use freya::prelude::*;
use freya::router::RouterContext;
use oneclient_core::clusters::Cluster;

use crate::components::{ART_PREVIEW_EDGE, DynamicArt, Icon, IconType, Button};
use crate::routes::Route;
use crate::theme::colors;
use crate::ui::border_all_color;

const CARD_HEIGHT_PX: f32 = 240.;

#[derive(PartialEq)]
pub struct InstanceCard {
    pub cluster: Cluster,
    pub active: bool,
    pub syncing: bool,
}

impl Component for InstanceCard {
    fn render(&self) -> impl IntoElement {
        let mut hovering = use_state(|| false);
        let a11y_id = use_a11y();
        let focus = use_focus(a11y_id);

        let active = self.active;
        let hovered = *hovering.read();
        let focused = focus().is_focused();

        let border_color = if active || focused {
            colors::brand()
        } else if hovered {
            colors::component_border_hover()
        } else {
            colors::component_border()
        };

        let cluster_id = self.cluster.id;
        let c_name = self.cluster.name.clone();

        rect()
            .key(self.cluster.id)
            .width(Size::flex(1.0))
            .height(Size::px(CARD_HEIGHT_PX))
            .a11y_id(a11y_id)
            .a11y_focusable(true)
            .a11y_role(AccessibilityRole::Button)
            .on_press(move |_| {
                let _ = RouterContext::get().push(Route::ClusterOverview { cluster_id });
            })
            .on_pointer_enter(move |_| {
                *hovering.write() = true;
            })
            .on_pointer_leave(move |_| {
                *hovering.write() = false;
            })
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::fill())
                    .overflow(Overflow::Clip)
                    .corner_radius(CornerRadius::new_all(12.))
                    .background(colors::page_elevated())
                    .border(border_all_color(if active { 2. } else { 1. }, border_color))
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::fill())
                            .position(Position::new_absolute())
                            .child(DynamicArt::for_cluster(&self.cluster).max_edge(ART_PREVIEW_EDGE)),
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .height(Size::fill())
                            .position(Position::new_absolute())
                            .padding(Gaps::new_symmetric(14., 16.))
                            .main_align(Alignment::End)
                            .cross_align(Alignment::Start)
                            .background(
                                LinearGradient::new()
                                    .angle(0.0)
                                    .stop((Color::TRANSPARENT, 0.0))
                                    .stop((Color::from_argb(60, 10, 15, 20), 30.0))
                                    .stop((Color::from_argb(180, 10, 15, 20), 65.0))
                                    .stop((Color::from_argb(240, 10, 15, 20), 100.0)),
                            )
                            .child(
                                rect()
                                    .vertical()
                                    .spacing(4.)
                                    .padding(Gaps::new(8., 12., 8., 12.))
                                    .corner_radius(CornerRadius::new_all(8.))
                                    .background(Color::from_argb(200, 12, 18, 26))
                                    .border(border_all_color(1., Color::from_argb(50, 255, 255, 255)))
                                    .child(
                                        label()
                                            .text(c_name)
                                            .font_size(20.)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::WHITE),
                                    )
                                    .child(
                                        label()
                                            .text(format!("{} {}", self.cluster.mc_version, self.cluster.mc_loader))
                                            .font_size(13.)
                                            .font_weight(FontWeight::SEMI_BOLD)
                                            .color(Color::from_rgb(185, 205, 235)),
                                    ),
                            ),
                    )
                    .maybe_child(self.active.then(|| {
                        rect()
                            .position(Position::new_absolute())
                            .padding(Gaps::new_symmetric(12., 14.))
                            .child(
                                rect()
                                    .padding(Gaps::new_symmetric(4., 8.))
                                    .corner_radius(CornerRadius::new_all(6.))
                                    .background(colors::brand())
                                    .child(
                                        label()
                                            .text("ACTIVE")
                                            .font_size(10.)
                                            .font_weight(FontWeight::BOLD)
                                            .color(Color::WHITE),
                                    ),
                            )
                    }))
                    .child(
                        rect()
                            .position(Position::new_absolute().top(8.).right(8.))
                            .child(
                                Button::new()
                                    .on_press({
                                        let dispatch = crate::hooks::use_dispatch();
                                        move |e: Event<PressEventData>| {
                                            e.stop_propagation();
                                            dispatch.delete_cluster(cluster_id);
                                        }
                                    })
                                    .child(Icon::new(IconType::Trash01).size(16.))
                            )
                    ),
            )
            .into_element()
    }
}


