use freya::prelude::*;
use freya::router::RouterContext;
use oneclient_common::VersionKey;
use oneclient_core::clusters::Cluster;
use oneclient_common::domain::GameLoader;

use crate::components::{
    Button, Dropdown, Icon, IconType, ScrollArea,  TextInput,
};
use crate::hooks::{settled_or_loading, use_active_cluster_id, use_clusters, use_dispatch, use_launcher, use_versions, versions_metadata};
use crate::routes::Route;
use crate::theme::colors;
use crate::ui::border_all_color;

const GRID_GAP_PX: f32 = 12.;
const MIN_CARD_WIDTH_PX: f32 = 280.;
const CARD_HEIGHT_PX: f32 = 240.;
const PLACEHOLDER_VERSION_INFO: &str = "Placeholder version info";

#[derive(PartialEq)]
pub struct Clusters;

impl Component for Clusters {
    fn render(&self) -> impl IntoElement {
        let clusters_query = use_clusters();
        let active_id = use_active_cluster_id();
        let mut grid_columns = use_state(|| 2_usize);
        let launcher = use_launcher();

        let mut show_create = use_state(|| false);
        let mut create_name = use_state(|| String::from("Custom Instance"));
        let mut create_version = use_state(|| String::from("1.21.1"));
        let create_loader = use_state(|| GameLoader::Fabric);

        let versions_query = use_versions();
        let metas = versions_metadata(&versions_query).unwrap_or_default();
        let mut version_options: Vec<String> = metas.iter().filter_map(|m| m.mc_version()).collect();
        if version_options.is_empty() {
            version_options.push("1.21.1".to_string());
        }

        let dispatch = use_dispatch();

        let clusters = settled_or_loading(&clusters_query).unwrap_or_default();

        let columns = *grid_columns.read();
        let grid_rows = chunk_clusters(&clusters, columns);

        let active = active_id.read().clone();
        let syncing = launcher.fetching || launcher.syncing_bundles;

        rect()
            .vertical()
            .width(Size::fill())
            .height(Size::fill())
            .overflow(Overflow::Clip)
            .padding(Gaps::new(0., 40., 40., 40.))
            .spacing(16.)
            .child(page_header(dispatch, show_create, create_name, create_version, create_loader, version_options))
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .height(Size::flex(1.0))
                    .content(Content::Flex)
                    .spacing(GRID_GAP_PX)
                    .on_sized(move |event: Event<SizedEventData>| {
                        let width = event.data().area.width();
                        let next = grid_columns_for_width(width);
                        if next != *grid_columns.peek() {
                            *grid_columns.write() = next;
                        }
                    })
                    .child(
                        rect()
                            .vertical()
                            .width(Size::flex(1.0))
                            .height(Size::fill())
                            .overflow(Overflow::Clip)
                            .child(
                                ScrollArea::new()
                                    .width(Size::fill())
                                    .height(Size::fill())
                                    .spacing(GRID_GAP_PX)
                                    .children(grid_rows.into_iter().map(|row| {
                                        let row_len = row.len();
                                        rect()
                                            .horizontal()
                                            .width(Size::fill())
                                            .content(Content::Flex)
                                            .spacing(GRID_GAP_PX)
                                            .children(row.into_iter().map(|cluster| {
                                                crate::view::app::clusters::InstanceCard {
                                                    cluster: cluster.clone(),
                                                    active: active == Some(cluster.id),
                                                    syncing,
                                                }
                                                .into_element()
                                            }))
                                            .children((row_len..columns).map(|_| {
                                                rect()
                                                    .width(Size::flex(1.0))
                                                    .height(Size::px(CARD_HEIGHT_PX))
                                                    .into_element()
                                            }))
                                            .into_element()
                                    })),
                            ),
                    )
            )
    }
}
fn page_header(
    dispatch: crate::Actions,
    mut show_create: State<bool>,
    mut create_name: State<String>,
    mut create_version: State<String>,
    create_loader: State<GameLoader>,
    version_options: Vec<String>,
) -> impl IntoElement {
    let show = *show_create.read();
    rect()
        .vertical()
        .spacing(6.)
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .main_align(Alignment::SpaceBetween)
                .cross_align(Alignment::Center)
                .child(
                    rect()
                        .vertical()
                        .spacing(6.)
                        .child(
                            label()
                                .text("Versions")
                                .font_size(36.)
                                .font_weight(FontWeight::BOLD)
                                .color(colors::fg_primary()),
                        )
                        .child(
                            label()
                                .text("Manage and create game instances.")
                                .font_size(12.)
                                .font_weight(FontWeight::MEDIUM)
                                .color(colors::fg_secondary()),
                        )
                )
                .child(
                    rect()
                        .horizontal()
                        .spacing(8.)
                        .cross_align(Alignment::Center)
                        .child(
                            Button::new()
                                .secondary()
                                .text("Browse Modpacks")
                                .on_press({
                                    let dispatch = dispatch.clone();
                                    move |_| {
                                        dispatch.browse_modpacks();
                                    }
                                })
                        )
                        .child(
                            Button::new()
                                .secondary()
                                .text("Import Modpack")
                                .on_press({
                                    let dispatch = dispatch.clone();
                                    move |_| {
                                        dispatch.import_modpack();
                                    }
                                })
                        )
                        .child(
                            Button::new()
                                .primary()
                                .text(if show { "Cancel" } else { "Add Custom Instance" })
                                .on_press(move |_| {
                                    let curr = *show_create.read();
                                    *show_create.write() = !curr;
                                })
                        )
                )
        )
        .child(
            if show {
                rect()
                    .vertical()
                    .spacing(8.)
                    .padding(Gaps::new_symmetric(12., 0.))
                    .child(
                        rect()
                            .horizontal()
                            .spacing(8.)
                            .cross_align(Alignment::Center)
                            .child(label().text("Name:").width(Size::px(60.)))
                            .child(TextInput::new(create_name).width(Size::px(200.)))
                    )
                    .child(
                        rect()
                            .horizontal()
                            .spacing(8.)
                            .cross_align(Alignment::Center)
                            .child(label().text("Version:").width(Size::px(60.)))
                            .child({
                                let current_version = create_version.read().to_string();
                                let mut create_version = create_version.clone();
                                let options = version_options.clone();
                                Dropdown::new(current_version, options.clone())
                                    .width(Size::px(120.))
                                    .on_select(move |idx: usize| {
                                        if let Some(v) = options.get(idx) {
                                            *create_version.write() = v.clone();
                                        }
                                    })
                            })
                    )
                    .child(
                        rect()
                            .horizontal()
                            .spacing(8.)
                            .cross_align(Alignment::Center)
                            .child(label().text("Loader:").width(Size::px(60.)))
                            .child({
                                let loader_options = vec![
                                    "Vanilla".to_string(),
                                    "Fabric".to_string(),
                                    "Forge".to_string(),
                                    "NeoForge".to_string(),
                                    "Quilt".to_string(),
                                ];
                                let current_loader = create_loader.read().to_string();
                                let mut create_loader = create_loader.clone();
                                Dropdown::new(current_loader, loader_options)
                                    .width(Size::px(120.))
                                    .on_select(move |idx| {
                                        let loader = match idx {
                                            0 => GameLoader::Vanilla,
                                            1 => GameLoader::Fabric,
                                            2 => GameLoader::Forge,
                                            3 => GameLoader::NeoForge,
                                            4 => GameLoader::Quilt,
                                            _ => GameLoader::Vanilla,
                                        };
                                        *create_loader.write() = loader;
                                    })
                            })
                    )
                    .child(
                        Button::new()
                            .primary()
                            .text("Create")
                            .on_press({
                                let dispatch = dispatch.clone();
                                let create_loader = create_loader.clone();
                                move |_| {
                                    let name = create_name.read().clone();
                                    let version = create_version.read().clone();
                                    let loader = *create_loader.read();
                                    dispatch.create_custom_cluster(name, version, loader);
                                    *show_create.write() = false;
                                }
                            })
                    )
                    .into_element()
            } else {
                rect().into_element()
            }
        )
}

fn chunk_clusters(clusters: &[Cluster], columns: usize) -> Vec<Vec<Cluster>> {
    let mut chunks = Vec::new();
    let mut current = Vec::new();
    for c in clusters {
        current.push(c.clone());
        if current.len() == columns {
            chunks.push(current);
            current = Vec::new();
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

fn grid_columns_for_width(available_width_px: f32) -> usize {
    const GAP: f32 = 16.;

    let grid_width = available_width_px - GAP;
    if grid_width < MIN_CARD_WIDTH_PX {
        return 1;
    }

    let cols = (grid_width / (MIN_CARD_WIDTH_PX + GRID_GAP_PX)).floor() as usize;
    cols.clamp(1, 3)
}



