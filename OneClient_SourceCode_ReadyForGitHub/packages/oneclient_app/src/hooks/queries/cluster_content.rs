use freya::query::{Query, QueryCapability, UseQuery, use_query};
use oneclient_content::packages::{ContentType, PackageStore};
use oneclient_core::{LauncherError, LinkedArtifactInfo};
use oneclient_db::models::ClusterId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClusterContentQuery {
    pub cluster_id: ClusterId,
    pub content_type: ContentType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClusterContentKeys {
    pub cluster_id: ClusterId,
    pub content_type: ContentType,
}

impl QueryCapability for ClusterContentQuery {
    type Ok = Vec<LinkedArtifactInfo>;
    type Err = LauncherError;
    type Keys = ClusterContentKeys;

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let state = crate::launcher::state()?;
        if let Ok(Some(row)) = oneclient_db::dao::cluster::get_by_id(&state.services.db, self.cluster_id).await {
            if let Ok(cluster) = oneclient_core::clusters::Cluster::try_from_row(row) {
                if let Ok(game_dir) = cluster.game_dir() {
                    oneclient_core::import_manual_content(&state.services, &cluster, &game_dir).await;
                }
            }
        }
        let all = PackageStore::list_linked_artifacts(self.cluster_id, &state.services.content()).await?;

        let mut active_items = Vec::new();
        if let Ok(Some(row)) = oneclient_db::dao::cluster::get_by_id(&state.services.db, self.cluster_id).await {
            if let Ok(game_dir) = oneclient_common::paths::cluster_game_dir(&row.folder_name) {
                let dest_dir = game_dir.join(self.content_type.folder_name());
                let disabled_dir = oneclient_common::paths::cluster_disabled_dir(&row.folder_name, self.content_type)
                    .unwrap_or_else(|_| game_dir.join(self.content_type.disabled_folder_name()));
                let _ = polyio::create_dir_all(&dest_dir).await;
                let _ = polyio::create_dir_all(&disabled_dir).await;

                for mut item in all {
                    if item.content_type == self.content_type {
                        let in_mods = dest_dir.join(&item.cluster_file_name).exists();
                        let in_disabled = disabled_dir.join(&item.cluster_file_name).exists();

                        if !in_mods && !in_disabled {
                            // File was deleted from disk by the user; drop the link
                            let _ = oneclient_db::dao::artifact::unlink_cluster_artifact(&state.services.db, self.cluster_id, &item.hash).await;
                        } else {
                            if in_disabled && !in_mods && item.enabled {
                                item.enabled = false;
                                let _ = oneclient_db::dao::artifact::update_cluster_artifact(&state.services.db, self.cluster_id, &item.hash, &item.cluster_file_name, 0).await;
                            } else if in_mods && !item.enabled && !in_disabled {
                                item.enabled = true;
                                let _ = oneclient_db::dao::artifact::update_cluster_artifact(&state.services.db, self.cluster_id, &item.hash, &item.cluster_file_name, 1).await;
                            }
                            active_items.push(item);
                        }
                    }
                }
            }
        } else {
            active_items = all.into_iter().filter(|item| item.content_type == self.content_type).collect();
        }

        Ok(active_items)
    }
}

pub fn use_cluster_content(
    cluster_id: ClusterId,
    content_type: ContentType,
) -> UseQuery<ClusterContentQuery> {
    use_query(Query::new(
        ClusterContentKeys {
            cluster_id,
            content_type,
        },
        ClusterContentQuery {
            cluster_id,
            content_type,
        },
    ))
}

pub fn cluster_content_items(query: &UseQuery<ClusterContentQuery>) -> Vec<LinkedArtifactInfo> {
    super::state::settled_or_loading(query).unwrap_or_default()
}


