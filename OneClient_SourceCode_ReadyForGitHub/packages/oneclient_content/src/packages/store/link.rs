use std::path::Path;

use oneclient_db::models::ClusterRow;

use oneclient_common::domain::ContentType;
use oneclient_common::paths;
use crate::error::ContentResult;

use super::manifest;

#[tracing::instrument(level = "debug")]
pub async fn link_or_copy(src: &Path, dest: &Path) -> ContentResult<()> {
	if let Some(parent) = dest.parent() {
		polyio::create_dir_all(parent).await?;
	}

	remove_entry(dest).await?;

	if polyio::symlink_file(src, dest).await.is_ok() {
		return Ok(());
	}

	polyio::copy(src, dest).await?;
	Ok(())
}

/// `Path::exists` resolves symlinks so a link to an evicted artifact reads as absent and survives every unlink
pub async fn remove_entry(path: &Path) -> ContentResult<()> {
	if polyio::symlink_metadata(path).await.is_err() {
		return Ok(());
	}

	polyio::remove_file(path).await?;
	Ok(())
}

/// Best-effort only the folder is reconciled at the next launch regardless
/// A running game holds its jars open which on Windows blocks deletion so failure here is expected
#[tracing::instrument(level = "debug", skip(cluster), fields(cluster_id = cluster.id))]
pub async fn try_unlink_materialized(
	cluster: &ClusterRow,
	content_type: ContentType,
	file_name: &str,
) -> bool {
	let Ok(game_dir) = paths::cluster_game_dir(&cluster.folder_name) else {
		return false;
	};

	let path = game_dir.join(content_type.folder_name()).join(file_name);
	let _ = remove_entry(&path).await;

	if let Ok(disabled_dir) = paths::cluster_disabled_dir(&cluster.folder_name, content_type) {
		let dis_path = disabled_dir.join(file_name);
		let _ = remove_entry(&dis_path).await;
	}

	let relative = manifest::entry_path(content_type.folder_name(), file_name);
	if let Some(mut loaded) = manifest::load(&game_dir).await {
		loaded.entries.retain(|entry| entry.path != relative);
		manifest::save(&game_dir, &loaded).await;
	}

	true
}

#[tracing::instrument(level = "debug", skip(cluster), fields(cluster_id = cluster.id))]
pub async fn try_move_to_disabled(
	cluster: &ClusterRow,
	content_type: ContentType,
	file_name: &str,
) -> bool {
	let mut candidates = vec![
		paths::cluster_game_dir(&cluster.folder_name).ok(),
		paths::cluster_dir(&cluster.folder_name).ok(),
		paths::shared_minecraft_dir().ok(),
	];
	candidates.dedup();

	let mut moved = false;
	let clean_name = file_name.trim_end_matches(".disabled");
	let disabled_name = format!("{clean_name}.disabled");

	for dir_opt in candidates {
		let Some(game_dir) = dir_opt else { continue; };
		let active_dir = game_dir.join(content_type.folder_name());
		let disabled_dir = paths::cluster_disabled_dir(&cluster.folder_name, content_type)
			.unwrap_or_else(|_| game_dir.join(content_type.disabled_folder_name()));
		let _ = polyio::create_dir_all(&disabled_dir).await;

		let dest = disabled_dir.join(clean_name);
		let src1 = active_dir.join(file_name);
		let src2 = active_dir.join(clean_name);
		let src3 = active_dir.join(&disabled_name);

		for src in [src1, src2, src3] {
			if src.exists() {
				if polyio::copy(&src, &dest).await.is_ok() || dest.exists() {
					let _ = remove_entry(&src).await;
					if src.exists() {
						let _ = std::fs::remove_file(&src);
					}
					moved = true;
				}
			}
		}

		let relative = manifest::entry_path(content_type.folder_name(), clean_name);
		if let Some(mut loaded) = manifest::load(&game_dir).await {
			loaded.entries.retain(|entry| entry.path != relative);
			manifest::save(&game_dir, &loaded).await;
		}
	}

	moved
}

#[tracing::instrument(level = "debug", skip(cluster), fields(cluster_id = cluster.id))]
pub async fn try_move_from_disabled(
	cluster: &ClusterRow,
	content_type: ContentType,
	file_name: &str,
) -> bool {
	let mut candidates = vec![
		paths::cluster_game_dir(&cluster.folder_name).ok(),
		paths::cluster_dir(&cluster.folder_name).ok(),
		paths::shared_minecraft_dir().ok(),
	];
	candidates.dedup();

	let mut moved = false;
	let clean_name = file_name.trim_end_matches(".disabled");
	let disabled_name = format!("{clean_name}.disabled");

	for dir_opt in candidates {
		let Some(game_dir) = dir_opt else { continue; };
		let active_dir = game_dir.join(content_type.folder_name());
		let _ = polyio::create_dir_all(&active_dir).await;
		let dest = active_dir.join(clean_name);

		let disabled_dir = paths::cluster_disabled_dir(&cluster.folder_name, content_type)
			.unwrap_or_else(|_| game_dir.join(content_type.disabled_folder_name()));
		let src1 = disabled_dir.join(file_name);
		let src2 = disabled_dir.join(clean_name);
		let src3 = disabled_dir.join(&disabled_name);
		let src4 = active_dir.join(&disabled_name);

		for src in [src1, src2, src3, src4] {
			if src.exists() {
				if src != dest {
					if polyio::copy(&src, &dest).await.is_ok() || dest.exists() {
						let _ = remove_entry(&src).await;
						if src.exists() {
							let _ = std::fs::remove_file(&src);
						}
						moved = true;
					}
				}
			}
		}
	}

	moved
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn remove_entry_clears_a_dangling_link() {
		let root = polyio::testing::ScratchDir::new("dangling_link");
		let dir = root.path();
		polyio::create_dir_all(dir).await.unwrap();

		let target = dir.join("target.jar");
		let link = dir.join("link.jar");
		polyio::write(&target, b"jar".as_slice()).await.unwrap();
		polyio::symlink_file(&target, &link).await.unwrap();
		polyio::remove_file(&target).await.unwrap();

		assert!(!link.exists(), "the link resolves to nothing");
		assert!(
			polyio::symlink_metadata(&link).await.is_ok(),
			"but the link itself is still there"
		);

		remove_entry(&link).await.unwrap();
		assert!(polyio::symlink_metadata(&link).await.is_err());

		std::fs::remove_dir_all(root.path()).ok();
	}

	#[tokio::test]
	async fn remove_entry_is_fine_with_nothing_there() {
		let root = polyio::testing::ScratchDir::new("remove_missing");
		polyio::create_dir_all(root.path()).await.unwrap();

		remove_entry(&root.join("never_existed.jar")).await.unwrap();

		std::fs::remove_dir_all(root.path()).ok();
	}
}
