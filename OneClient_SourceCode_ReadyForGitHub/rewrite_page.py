import re

with open("packages/oneclient_app/src/view/app/clusters/page.rs", "r", encoding="utf-8") as f:
    content = f.read()

# Replace imports
content = content.replace("use crate::components::{", "use crate::components::{InstanceCard, ")
content = content.replace("VersionCard,", "")

# Define chunk_clusters
content = content.replace("fn chunk_lines", "fn chunk_clusters(clusters: &[Cluster], columns: usize) -> Vec<Vec<Cluster>> {\n    let mut chunks = Vec::new();\n    let mut current = Vec::new();\n    for c in clusters {\n        current.push(c.clone());\n        if current.len() == columns {\n            chunks.push(current);\n            current = Vec::new();\n        }\n    }\n    if !current.is_empty() {\n        chunks.push(current);\n    }\n    chunks\n}\n\nfn chunk_lines")

# Find Clusters::render
render_start = content.find("        let active_cluster = active_id")
page_header_def = content.find("fn page_header")

# Re-write Clusters::render
new_render_body = """
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
"""

content = content[:render_start] + new_render_body + content[page_header_def:]

with open("packages/oneclient_app/src/view/app/clusters/page.rs", "w", encoding="utf-8") as f:
    f.write(content)
