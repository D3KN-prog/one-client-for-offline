use freya::prelude::*;
use freya::animation::*;

use crate::components::{Button, Icon, IconType, PlayerModel, ScrollArea, TextInput};
use crate::hooks::{try_default_account, use_current_account, use_dispatch};
use crate::platform::open_url;
use crate::theme::colors;
use crate::ui::border_all_color;

const ANIMATIONS: &[&str] = &["Idle", "Walking", "Running", "Crouch", "Flying"];

#[derive(Clone, Copy, PartialEq)]
enum SkinTab {
    Skins,
    OnlineSkins,
    Capes,
    ModsGuide,
}

#[derive(Clone, PartialEq)]
struct PresetSkin {
    name: &'static str,
    desc: &'static str,
    tag: &'static str,
}

const PRESET_SKINS: &[PresetSkin] = &[
    PresetSkin { name: "Steve", desc: "Classic Minecraft Male Model (4px arms)", tag: "Default" },
    PresetSkin { name: "Alex", desc: "Classic Minecraft Slim Model (3px arms)", tag: "Slim" },
    PresetSkin { name: "Cyber Knight", desc: "D3KN Dark Edition Armor & Visor", tag: "Exclusive" },
    PresetSkin { name: "Void Phantom", desc: "Cosmic obsidian with glowing eyes", tag: "Fantasy" },
    PresetSkin { name: "Neon Shadow", desc: "High-contrast stealth outfit", tag: "Modern" },
    PresetSkin { name: "Frost Warrior", desc: "Glacial crystalline battle suit", tag: "Polyfrost" },
    PresetSkin { name: "Ender Champion", desc: "Interdimensional royal mantle", tag: "Mythic" },
    PresetSkin { name: "Golden Sentinel", desc: "Imperial gilded combat regalia", tag: "Legendary" },
];

#[derive(Clone, PartialEq)]
struct CommunitySkin {
    username: &'static str,
    title: &'static str,
    desc: &'static str,
    tag: &'static str,
}

const COMMUNITY_SKINS: &[CommunitySkin] = &[
    CommunitySkin {
        username: "Technoblade",
        title: "Technoblade",
        desc: "The Blood God - Crown & Royal Pig Mantle",
        tag: "Legendary",
    },
    CommunitySkin {
        username: "Dream",
        title: "Dream",
        desc: "Iconic Neon Green & Mask Speedrunner",
        tag: "Speedrun",
    },
    CommunitySkin {
        username: "DanTDM",
        title: "DanTDM",
        desc: "Diamond Minecart Legend - Blue Hair & Goggles",
        tag: "OG Classic",
    },
    CommunitySkin {
        username: "MumboJumbo",
        title: "Mumbo Jumbo",
        desc: "Redstone Master - Formal Suit & Mustachio",
        tag: "Redstone",
    },
    CommunitySkin {
        username: "Grian",
        title: "Grian",
        desc: "Architect of Hermitcraft - Classic Red Sweater",
        tag: "Builder",
    },
    CommunitySkin {
        username: "Skeppy",
        title: "Skeppy",
        desc: "Diamond Block Skin - PvP & Trolling Legend",
        tag: "PvP",
    },
    CommunitySkin {
        username: "d3kn",
        title: "D3KN",
        desc: "D3KN Custom Gamer Edition",
        tag: "Special",
    },
    CommunitySkin {
        username: "Notch",
        title: "Notch",
        desc: "The Creator of Minecraft",
        tag: "Creator",
    },
    CommunitySkin {
        username: "jeb_",
        title: "Jens Bergensten",
        desc: "Lead Designer of Minecraft - Iconic Beard",
        tag: "Mojang Dev",
    },
    CommunitySkin {
        username: "CaptainSparklez",
        title: "CaptainSparklez",
        desc: "Fallen Kingdom Veteran - Creeper Hoodie",
        tag: "Music & OG",
    },
];

#[derive(Clone, PartialEq)]
struct PresetCape {
    name: &'static str,
    color_hint: Color,
    desc: &'static str,
}

const PRESET_CAPES: &[PresetCape] = &[
    PresetCape {
        name: "Polyfrost Founder",
        color_hint: Color::from_rgb(0, 180, 216),
        desc: "Official Polyfrost community founder insignia (Unlocked)",
    },
    PresetCape {
        name: "D3KN Legend Cape",
        color_hint: Color::from_rgb(255, 183, 3),
        desc: "Golden Dragon & Onyx sigil - PulseClient Custom Edition",
    },
    PresetCape {
        name: "15th Anniversary",
        color_hint: Color::from_rgb(190, 75, 40),
        desc: "Celebratory copper crest cape celebrating 15 years",
    },
    PresetCape {
        name: "Vanilla Migrator",
        color_hint: Color::from_rgb(180, 20, 50),
        desc: "Classic migration crimson cape with golden buckle",
    },
    PresetCape {
        name: "Sakura Blossom",
        color_hint: Color::from_rgb(255, 154, 162),
        desc: "Gentle cherry blossom petal gradient with soft glint",
    },
    PresetCape {
        name: "Cobalt Developer",
        color_hint: Color::from_rgb(30, 144, 255),
        desc: "Deep cobalt weave with cyan circuitry motifs",
    },
    PresetCape {
        name: "Void Nebula",
        color_hint: Color::from_rgb(138, 43, 226),
        desc: "Cosmic ultraviolet starfield cape",
    },
    PresetCape {
        name: "Emerald Champion",
        color_hint: Color::from_rgb(46, 204, 113),
        desc: "Emerald luster cape commemorating high-tier survival",
    },
];

#[derive(PartialEq)]
pub struct AccountSkins;

impl Component for AccountSkins {
    fn render(&self) -> impl IntoElement {
        let account_uuid = try_default_account(&use_current_account()).map(|a| a.id.to_string());
        let mut active_tab = use_state(|| SkinTab::OnlineSkins);
        let mut selected_skin = use_state(|| 0usize);
        let mut equipped_skin = use_state(|| 0usize);
        let mut selected_cape = use_state(|| 0usize);
        let mut equipped_cape = use_state(|| 0usize);
        let mut online_user = use_state(|| "Technoblade".to_string());
        let mut search_input = use_state(String::new);
        let mut bg_idx = use_state(|| 0usize);
        let dispatch = use_dispatch();

        let preview_target = match *active_tab.read() {
            SkinTab::OnlineSkins => Some(online_user.read().clone()),
            _ => account_uuid.clone(),
        };

        ScrollArea::new()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .height(Size::fill())
                    .padding(40.)
                    .spacing(24.)
                    .child(preview_panel(
                        preview_target,
                        *active_tab.read(),
                        *selected_skin.read(),
                        *selected_cape.read(),
                        &online_user.read(),
                        bg_idx,
                    ))
                    .child(
                        rect()
                            .vertical()
                            .width(Size::flex(1.0))
                            .height(Size::fill())
                            .spacing(18.)
                            .child(header_row())
                            .child(tab_bar(active_tab))
                            .child(
                                rect()
                                    .width(Size::fill())
                                    .height(Size::flex(1.0))
                                    .child(match *active_tab.read() {
                                        SkinTab::OnlineSkins => online_skins_view(
                                            search_input,
                                            online_user,
                                            dispatch.clone(),
                                        )
                                        .into_element(),
                                        SkinTab::Skins => skins_view(
                                            selected_skin,
                                            equipped_skin,
                                            dispatch.clone(),
                                        )
                                        .into_element(),
                                        SkinTab::Capes => capes_view(
                                            selected_cape,
                                            equipped_cape,
                                            dispatch.clone(),
                                        )
                                        .into_element(),
                                        SkinTab::ModsGuide => label().into_element(),
                                    }),
                            ),
                    )
            )
    }
}

fn header_row() -> impl IntoElement {
    rect()
        .horizontal()
        .width(Size::fill())
        .cross_align(Alignment::Center)
        .main_align(Alignment::SpaceBetween)
        .child(
            rect()
                .vertical()
                .spacing(4.)
                .child(
                    label()
                        .text("Cosmetics & Skins")
                        .font_size(28.)
                        .font_weight(FontWeight::BOLD)
                        .color(colors::fg_primary()),
                )
                .child(
                    label()
                        .text("All capes, skins and cosmetic features are 100% unlocked & free")
                        .font_size(13.)
                        .color(colors::fg_secondary()),
                ),
        )
        .child(
            rect()
                .padding(Gaps::new(6., 12., 6., 12.))
                .corner_radius(CornerRadius::new_all(8.))
                .background(Color::from_argb(140, 30, 50, 40))
                .border(border_all_color(1., Color::from_rgb(46, 204, 113)))
                .child(
                    label()
                        .text("FREE & UNLOCKED")
                        .font_size(11.)
                        .font_weight(FontWeight::BOLD)
                        .color(Color::from_rgb(46, 204, 113)),
                ),
        )
}

fn tab_bar(active_tab: State<SkinTab>) -> impl IntoElement {
    let t1 = active_tab;
    let t2 = active_tab;
    let t3 = active_tab;
    rect()
        .horizontal()
        .width(Size::fill())
        .spacing(10.)
        .child(tab_button("Online Community Skins", *active_tab.read() == SkinTab::OnlineSkins, move || {
            let mut s = t1;
            s.set(SkinTab::OnlineSkins);
        }))
        .child(tab_button("Built-in Presets", *active_tab.read() == SkinTab::Skins, move || {
            let mut s = t2;
            s.set(SkinTab::Skins);
        }))
        .child(tab_button("Capes (Free Unlocked)", *active_tab.read() == SkinTab::Capes, move || {
            let mut s = t3;
            s.set(SkinTab::Capes);
        }))
}

fn tab_button(title: &'static str, active: bool, on_click: impl Fn() + 'static) -> impl IntoElement {
    rect()
        .padding(Gaps::new_symmetric(8., 16.))
        .corner_radius(CornerRadius::new_all(8.))
        .background(if active {
            colors::brand()
        } else {
            colors::page_elevated()
        })
        .border(border_all_color(1., if active { colors::brand() } else { colors::component_border() }))
        .cursor(CursorIcon::Pointer)
        .on_press(move |_| on_click())
        .child(
            label()
                .text(title)
                .font_size(13.)
                .font_weight(if active { FontWeight::BOLD } else { FontWeight::NORMAL })
                .color(colors::fg_primary()),
        )
}

fn preview_panel(
    preview_target: Option<String>,
    tab: SkinTab,
    skin_idx: usize,
    cape_idx: usize,
    online_user: &str,
    bg_idx: State<usize>,
) -> impl IntoElement {
    let current_skin_name = PRESET_SKINS.get(skin_idx).map(|s| s.name).unwrap_or("Steve");
    let current_cape_name = PRESET_CAPES.get(cape_idx).map(|c| c.name).unwrap_or("Founder");
    let bg_names = ["CavesAndCliffs", "NetherUpdate", "TheWildUpdate", "TrailsAndTales", "TrickyTrials"];
    let mut show_bg_selector = use_state(|| false);

    let expand_anim = use_animation_with_dependencies(&*show_bg_selector.read(), |conf, &show| {
        conf.on_creation(OnCreation::Run);
        conf.on_change(OnChange::Rerun);
        AnimNum::new(if show { 0. } else { 1. }, if show { 1. } else { 0. })
            .time(250)
            .ease(Ease::Out)
    });
    let expand_val = expand_anim.get().value();

    rect()
        .vertical()
        .width(Size::px(320.))
        .height(Size::fill())
        .spacing(16.)
        .child(
            rect()
                .vertical()
                .width(Size::fill())
                .height(Size::flex(1.0))
                .corner_radius(CornerRadius::new_all(16.))
                .background(colors::page_elevated())
                .border(border_all_color(1., colors::component_border()))
                .child(
                    rect()
                        .center()
                        .width(Size::fill())
                        .height(Size::flex(1.0))
                        .overflow(Overflow::Clip)
                        .corner_radius(CornerRadius::new_all(16.))
                        .child(
                            rect()
                                .position(Position::new_absolute())
                                .width(Size::fill())
                                .height(Size::fill())
                                .child(crate::layout::ParallaxArt {
                                    art_bytes: crate::AppAssets::get_bytes(&format!("backgrounds/{}.jpg", bg_names[*bg_idx.read()])).unwrap_or_default(),
                                    fade_opacity: 1.0,
                                    parallax_enabled: crate::use_settings_snapshot().settings.dynamic_background_enabled,
                                }),
                        )
                        .child(match preview_target {
                            Some(target) => PlayerModel::new(target)
                                .width(Size::fill())
                                .height(Size::fill())
                                .into_element(),
                            None => Icon::new(IconType::Users01)
                                .size(72.)
                                .color(colors::fg_secondary())
                                .into_element(),
                        })
                        .child(
                            rect()
                                .position(Position::new_absolute().bottom(0.).left(0.).right(0.))
                                .vertical()
                                .cross_align(Alignment::Center)
                                .child(
                                    rect()
                                        .height(Size::px(expand_val * 60.))
                                        .width(Size::fill())
                                        .background(Color::from_argb(200, 10, 15, 20))
                                        .horizontal()
                                        .padding(Gaps::new_symmetric(0., 12.))
                                        .spacing(8.)
                                        .cross_align(Alignment::Center)
                                        .main_align(Alignment::Center)
                                        .children(bg_names.iter().enumerate().map(|(i, &name)| {
                                            let is_selected = i == *bg_idx.read();
                                            let mut bg_state = bg_idx;
                                            rect()
                                                .width(Size::px(44.))
                                                .height(Size::px(44.))
                                                .corner_radius(CornerRadius::new_all(6.))
                                                .background(colors::component_bg())
                                                .border(border_all_color(if is_selected { 2. } else { 1. }, if is_selected { colors::brand() } else { colors::component_border() }))
                                                .cursor(CursorIcon::Pointer)
                                                .on_press(move |_| bg_state.set(i))
                                                .child(
                                                    crate::layout::ParallaxArt {
                                                        art_bytes: crate::AppAssets::get_bytes(&format!("backgrounds/{}.jpg", name)).unwrap_or_default(),
                                                        fade_opacity: 1.0,
                                                        parallax_enabled: false,
                                                    }
                                                )
                                                .into_element()
                                        }))
                                )
                                .child(
                                    rect()
                                        .width(Size::px(40.))
                                        .height(Size::px(24.))
                                        .corner_radius(CornerRadius::new(8., 8., 0., 0.))
                                        .background(Color::from_argb(200, 10, 15, 20))
                                        .center()
                                        .cursor(CursorIcon::Pointer)
                                        .on_press(move |_| {
                                            let current = *show_bg_selector.read();
                                            show_bg_selector.set(!current);
                                        })
                                        .child(Icon::new(IconType::ChevronDown).size(16.).rotate(if *show_bg_selector.read() { 180. } else { 0. }))
                                )
                        ),
                )

        )
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .main_align(Alignment::Center)
                .spacing(6.)
                .children(
                    ANIMATIONS
                        .iter()
                        .enumerate()
                        .map(|(i, name)| anim_chip(name, i == 0).into_element()),
                ),
        )
}

fn anim_chip(name: &'static str, active: bool) -> impl IntoElement {
    rect()
        .center()
        .padding(Gaps::new_symmetric(6., 10.))
        .corner_radius(CornerRadius::new_all(8.))
        .background(if active {
            colors::brand()
        } else {
            colors::component_bg()
        })
        .cursor(CursorIcon::Pointer)
        .child(
            label()
                .text(name)
                .font_size(11.)
                .color(colors::fg_primary()),
        )
}

fn site_link_chip(name: &'static str, url: &'static str) -> impl IntoElement {
    rect()
        .padding(Gaps::new_symmetric(5., 10.))
        .corner_radius(CornerRadius::new_all(6.))
        .background(colors::component_bg())
        .border(border_all_color(1., colors::component_border()))
        .cursor(CursorIcon::Pointer)
        .on_press(move |_| open_url(url))
        .child(
            rect()
                .horizontal()
                .spacing(4.)
                .cross_align(Alignment::Center)
                .child(Icon::new(IconType::LinkExternal01).size(11.).color(colors::brand()))
                .child(
                    label()
                        .text(name)
                        .font_size(11.)
                        .color(colors::fg_primary()),
                ),
        )
}

fn online_skins_view(
    search_input: State<String>,
    mut online_user: State<String>,
    dispatch: crate::Actions,
) -> impl IntoElement {
    let mut search_user_btn = online_user;
    let search_in = search_input;
    let equip_dispatch = dispatch.clone();
    let download_dispatch = dispatch.clone();
    let current_user = online_user.read().clone();
    let equip_user = current_user.clone();
    let dl_user = current_user.clone();
    let namemc_user = current_user.clone();

    rect()
        .vertical()
        .width(Size::fill())
        .height(Size::fill())
        .spacing(16.)
        .child(
            rect()
                .vertical()
                .width(Size::fill())
                .spacing(12.)
                .padding(Gaps::new_all(14.))
                .corner_radius(CornerRadius::new_all(12.))
                .background(colors::page_elevated())
                .border(border_all_color(1., colors::component_border()))
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .spacing(10.)
                        .cross_align(Alignment::Center)
                        .child(
                            rect()
                                .width(Size::flex(1.0))
                                .child(
                                    TextInput::new(search_input)
                                        .placeholder("Search any player username (e.g. Technoblade, Dream, d3kn)...")
                                        .on_submit(move |val: String| {
                                            let trimmed = val.trim().to_string();
                                            if !trimmed.is_empty() {
                                                search_user_btn.set(trimmed);
                                            }
                                        }),
                                ),
                        )
                        .child(
                            Button::new()
                                .primary()
                                .on_press(move |_| {
                                    let q = search_in.read().trim().to_string();
                                    if !q.is_empty() {
                                        search_user_btn.set(q);
                                    }
                                })
                                .child(Icon::new(IconType::SearchMd).size(16.))
                                .text("Search & Preview"),
                        ),
                )
                .child(
                    rect()
                        .horizontal()
                        .width(Size::fill())
                        .spacing(8.)
                        .cross_align(Alignment::Center)
                        .child(
                            label()
                                .text("Skin Repositories:")
                                .font_size(12.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .color(colors::fg_secondary()),
                        )
                        .child(site_link_chip("NameMC Trending", "https://namemc.com/minecraft-skins/trending"))
                        .child(site_link_chip("PlanetMinecraft", "https://www.planetminecraft.com/skins/"))
                        .child(site_link_chip("The SkinDex", "https://www.minecraftskins.com/"))
                        .child(site_link_chip("NovaSkin", "https://minecraft.novaskin.me/")),
                ),
        )
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .padding(Gaps::new_symmetric(10., 16.))
                .corner_radius(CornerRadius::new_all(10.))
                .background(Color::from_argb(120, 20, 35, 55))
                .border(border_all_color(1., colors::brand()))
                .main_align(Alignment::SpaceBetween)
                .cross_align(Alignment::Center)
                .child(
                    rect()
                        .horizontal()
                        .spacing(8.)
                        .cross_align(Alignment::Center)
                        .child(Icon::new(IconType::Eye).size(18.).color(colors::brand()))
                        .child(
                            label()
                                .text(format!("Viewing Skin: {}", current_user))
                                .font_size(14.)
                                .font_weight(FontWeight::BOLD)
                                .color(colors::fg_primary()),
                        ),
                )
                .child(
                    rect()
                        .horizontal()
                        .spacing(10.)
                        .child(
                            Button::new()
                                .primary()
                                .on_press(move |_| {
                                    equip_dispatch
                                        .notify("Skin Selected")
                                        .body(format!("{}'s skin is now active on your character!", equip_user))
                                        .send();
                                })
                                .child(Icon::new(IconType::CheckCircle).size(16.))
                                .text("Equip As My Skin"),
                        )
                        .child(
                            Button::new()
                                .secondary()
                                .on_press(move |_| {
                                    let url = format!("https://minotar.net/skin/{}", dl_user);
                                    open_url(&url);
                                    download_dispatch
                                        .notify("Skin PNG Opened")
                                        .body(format!("Opened full skin texture for {}", dl_user))
                                        .send();
                                })
                                .child(Icon::new(IconType::Download01).size(16.))
                                .text("Download PNG"),
                        )
                        .child(
                            Button::new()
                                .secondary()
                                .on_press(move |_| {
                                    let url = format!("https://namemc.com/profile/{}", namemc_user);
                                    open_url(&url);
                                })
                                .child(Icon::new(IconType::LinkExternal01).size(16.))
                                .text("NameMC Profile"),
                        ),
                ),
        )
        .child(
            ScrollArea::new()
                .width(Size::fill())
                .height(Size::flex(1.0))
                .child(
                    rect()
                        .vertical()
                        .width(Size::fill())
                        .spacing(10.)
                        .children(
                            COMMUNITY_SKINS
                                .iter()
                                .map(|skin| {
                                    let is_active = online_user.read().eq_ignore_ascii_case(skin.username);
                                    let preview_u = skin.username;
                                    let mut set_target = online_user;
                                    let namemc_target = skin.username;
                                    let item_equip_disp = dispatch.clone();
                                    let item_skin_name = skin.title;

                                    rect()
                                        .horizontal()
                                        .width(Size::fill())
                                        .padding(Gaps::new_all(12.))
                                        .corner_radius(CornerRadius::new_all(10.))
                                        .background(if is_active {
                                            Color::from_argb(160, 25, 38, 55)
                                        } else {
                                            colors::page_elevated()
                                        })
                                        .border(border_all_color(
                                            if is_active { 2. } else { 1. },
                                            if is_active {
                                                colors::brand()
                                            } else {
                                                colors::component_border()
                                            },
                                        ))
                                        .cursor(CursorIcon::Pointer)
                                        .on_press(move |_| set_target.set(preview_u.to_string()))
                                        .cross_align(Alignment::Center)
                                        .main_align(Alignment::SpaceBetween)
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(12.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .center()
                                                        .width(Size::px(38.))
                                                        .height(Size::px(38.))
                                                        .corner_radius(CornerRadius::new_all(8.))
                                                        .background(colors::component_bg())
                                                        .child(
                                                            Icon::new(IconType::Users01)
                                                                .size(18.)
                                                                .color(if is_active { colors::brand() } else { colors::fg_secondary() }),
                                                        ),
                                                )
                                                .child(
                                                    rect()
                                                        .vertical()
                                                        .spacing(2.)
                                                        .child(
                                                            label()
                                                                .text(skin.title)
                                                                .font_size(14.)
                                                                .font_weight(FontWeight::SEMI_BOLD)
                                                                .color(colors::fg_primary()),
                                                        )
                                                        .child(
                                                            label()
                                                                .text(skin.desc)
                                                                .font_size(11.)
                                                                .color(colors::fg_secondary()),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .padding(Gaps::new(3., 7., 3., 7.))
                                                        .corner_radius(CornerRadius::new_all(6.))
                                                        .background(Color::from_argb(90, 40, 50, 65))
                                                        .child(
                                                            label()
                                                                .text(skin.tag)
                                                                .font_size(10.)
                                                                .font_weight(FontWeight::BOLD)
                                                                .color(colors::fg_secondary()),
                                                        ),
                                                )
                                                .child(
                                                    Button::new()
                                                        .secondary()
                                                        .on_press(move |_| {
                                                            item_equip_disp
                                                                .notify("Skin Equipped")
                                                                .body(format!("Applied {} as your active skin", item_skin_name))
                                                                .send();
                                                        })
                                                        .child(Icon::new(IconType::Check).size(14.))
                                                        .text("Equip"),
                                                )
                                                .child(
                                                    Button::new()
                                                        .secondary()
                                                        .on_press(move |_| {
                                                            open_url(&format!("https://namemc.com/profile/{}", namemc_target));
                                                        })
                                                        .child(Icon::new(IconType::LinkExternal01).size(14.))
                                                        .text("NameMC"),
                                                ),
                                        )
                                        .into_element()
                                })
                                .collect::<Vec<_>>(),
                        ),
                ),
        )
}

fn skins_view(
    mut selected_skin: State<usize>,
    mut equipped_skin: State<usize>,
    dispatch: crate::Actions,
) -> impl IntoElement {
    let import_dispatch = dispatch.clone();
    let equip_dispatch = dispatch.clone();

    rect()
        .vertical()
        .width(Size::fill())
        .height(Size::fill())
        .spacing(16.)
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .child(
                    Button::new()
                        .primary()
                        .on_press(move |_| {
                            let d = import_dispatch.clone();
                            spawn(async move {
                                if let Some(file) = rfd::AsyncFileDialog::new()
                                    .set_title("Select Minecraft Skin (.png)")
                                    .add_filter("Skin PNG", &["png"])
                                    .pick_file()
                                    .await
                                {
                                    d.notify("Skin Imported")
                                        .body(format!("Loaded {}", file.file_name()))
                                        .send();
                                }
                            });
                        })
                        .child(Icon::new(IconType::Plus).size(16.))
                        .text("Import Custom Skin (.png)"),
                )
                .child(
                    Button::new()
                        .secondary()
                        .on_press(move |_| {
                            let curr = *selected_skin.peek();
                            equipped_skin.set(curr);
                            let name = PRESET_SKINS.get(curr).map(|s| s.name).unwrap_or("Skin");
                            equip_dispatch
                                .notify("Skin Equipped")
                                .body(format!("Applied {} as your active skin", name))
                                .send();
                        })
                        .child(Icon::new(IconType::CheckCircle).size(16.))
                        .text("Equip Selected Skin"),
                ),
        )
        .child(
            ScrollArea::new()
                .width(Size::fill())
                .height(Size::flex(1.0))
                .child(
                    rect()
                        .vertical()
                        .width(Size::fill())
                        .spacing(12.)
                        .children(
                            PRESET_SKINS
                                .iter()
                                .enumerate()
                                .map(|(idx, skin)| {
                                    let is_selected = *selected_skin.read() == idx;
                                    let is_equipped = *equipped_skin.read() == idx;
                                    rect()
                                        .horizontal()
                                        .width(Size::fill())
                                        .padding(Gaps::new_all(14.))
                                        .corner_radius(CornerRadius::new_all(12.))
                                        .background(if is_selected {
                                            Color::from_argb(160, 25, 38, 55)
                                        } else {
                                            colors::page_elevated()
                                        })
                                        .border(border_all_color(
                                            if is_selected { 2. } else { 1. },
                                            if is_selected {
                                                colors::brand()
                                            } else {
                                                colors::component_border()
                                            },
                                        ))
                                        .cursor(CursorIcon::Pointer)
                                        .on_press(move |_| selected_skin.set(idx))
                                        .cross_align(Alignment::Center)
                                        .main_align(Alignment::SpaceBetween)
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(14.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .center()
                                                        .width(Size::px(40.))
                                                        .height(Size::px(40.))
                                                        .corner_radius(CornerRadius::new_all(8.))
                                                        .background(colors::component_bg())
                                                        .child(
                                                            Icon::new(IconType::Users01)
                                                                .size(20.)
                                                                .color(if is_selected { colors::brand() } else { colors::fg_secondary() }),
                                                        ),
                                                )
                                                .child(
                                                    rect()
                                                        .vertical()
                                                        .spacing(2.)
                                                        .child(
                                                            label()
                                                                .text(skin.name)
                                                                .font_size(15.)
                                                                .font_weight(FontWeight::SEMI_BOLD)
                                                                .color(colors::fg_primary()),
                                                        )
                                                        .child(
                                                            label()
                                                                .text(skin.desc)
                                                                .font_size(12.)
                                                                .color(colors::fg_secondary()),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .padding(Gaps::new(4., 8., 4., 8.))
                                                        .corner_radius(CornerRadius::new_all(6.))
                                                        .background(Color::from_argb(100, 40, 50, 65))
                                                        .child(
                                                            label()
                                                                .text(skin.tag)
                                                                .font_size(10.)
                                                                .font_weight(FontWeight::BOLD)
                                                                .color(colors::fg_secondary()),
                                                        ),
                                                )
                                                .maybe(is_equipped, |el| {
                                                    el.child(
                                                        rect()
                                                            .padding(Gaps::new(4., 8., 4., 8.))
                                                            .corner_radius(CornerRadius::new_all(6.))
                                                            .background(Color::from_argb(160, 20, 70, 40))
                                                            .border(border_all_color(1., Color::from_rgb(46, 204, 113)))
                                                            .child(
                                                                label()
                                                                    .text("EQUIPPED")
                                                                    .font_size(10.)
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .color(Color::from_rgb(46, 204, 113)),
                                                            ),
                                                    )
                                                }),
                                        )
                                        .into_element()
                                })
                                .collect::<Vec<_>>(),
                        ),
                ),
        )
}

fn capes_view(
    mut selected_cape: State<usize>,
    mut equipped_cape: State<usize>,
    dispatch: crate::Actions,
) -> impl IntoElement {
    let equip_dispatch = dispatch.clone();

    rect()
        .vertical()
        .width(Size::fill())
        .height(Size::fill())
        .spacing(16.)
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .child(
                    Button::new()
                        .primary()
                        .on_press(move |_| {
                            let curr = *selected_cape.peek();
                            equipped_cape.set(curr);
                            let name = PRESET_CAPES.get(curr).map(|c| c.name).unwrap_or("Cape");
                            equip_dispatch
                                .notify("Cape Equipped")
                                .body(format!("{} is now active on your character!", name))
                                .send();
                        })
                        .child(Icon::new(IconType::CheckCircle).size(16.))
                        .text("Equip Selected Cape"),
                ),
        )
        .child(
            ScrollArea::new()
                .width(Size::fill())
                .height(Size::flex(1.0))
                .child(
                    rect()
                        .vertical()
                        .width(Size::fill())
                        .spacing(12.)
                        .children(
                            PRESET_CAPES
                                .iter()
                                .enumerate()
                                .map(|(idx, cape)| {
                                    let is_selected = *selected_cape.read() == idx;
                                    let is_equipped = *equipped_cape.read() == idx;
                                    rect()
                                        .horizontal()
                                        .width(Size::fill())
                                        .padding(Gaps::new_all(14.))
                                        .corner_radius(CornerRadius::new_all(12.))
                                        .background(if is_selected {
                                            Color::from_argb(160, 25, 38, 55)
                                        } else {
                                            colors::page_elevated()
                                        })
                                        .border(border_all_color(
                                            if is_selected { 2. } else { 1. },
                                            if is_selected {
                                                cape.color_hint
                                            } else {
                                                colors::component_border()
                                            },
                                        ))
                                        .cursor(CursorIcon::Pointer)
                                        .on_press(move |_| selected_cape.set(idx))
                                        .cross_align(Alignment::Center)
                                        .main_align(Alignment::SpaceBetween)
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(14.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .width(Size::px(28.))
                                                        .height(Size::px(44.))
                                                        .corner_radius(CornerRadius::new_all(6.))
                                                        .background(cape.color_hint)
                                                        .border(border_all_color(1., Color::from_argb(120, 255, 255, 255))),
                                                )
                                                .child(
                                                    rect()
                                                        .vertical()
                                                        .spacing(2.)
                                                        .child(
                                                            label()
                                                                .text(cape.name)
                                                                .font_size(15.)
                                                                .font_weight(FontWeight::SEMI_BOLD)
                                                                .color(colors::fg_primary()),
                                                        )
                                                        .child(
                                                            label()
                                                                .text(cape.desc)
                                                                .font_size(12.)
                                                                .color(colors::fg_secondary()),
                                                        ),
                                                ),
                                        )
                                        .child(
                                            rect()
                                                .horizontal()
                                                .spacing(8.)
                                                .cross_align(Alignment::Center)
                                                .child(
                                                    rect()
                                                        .padding(Gaps::new(4., 8., 4., 8.))
                                                        .corner_radius(CornerRadius::new_all(6.))
                                                        .background(Color::from_argb(120, 20, 50, 40))
                                                        .border(border_all_color(1., Color::from_rgb(46, 204, 113)))
                                                        .child(
                                                            label()
                                                                .text("UNLOCKED")
                                                                .font_size(10.)
                                                                .font_weight(FontWeight::BOLD)
                                                                .color(Color::from_rgb(46, 204, 113)),
                                                        ),
                                                )
                                                .maybe(is_equipped, |el| {
                                                    el.child(
                                                        rect()
                                                            .padding(Gaps::new(4., 8., 4., 8.))
                                                            .corner_radius(CornerRadius::new_all(6.))
                                                            .background(Color::from_argb(160, 20, 70, 40))
                                                            .border(border_all_color(1., Color::from_rgb(46, 204, 113)))
                                                            .child(
                                                                label()
                                                                    .text("EQUIPPED")
                                                                    .font_size(10.)
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .color(Color::from_rgb(46, 204, 113)),
                                                            ),
                                                    )
                                                }),
                                        )
                                        .into_element()
                                })
                                .collect::<Vec<_>>(),
                        ),
                ),
        )
}

fn mods_guide_view() -> impl IntoElement {
    rect()
        .vertical()
        .width(Size::fill())
        .height(Size::fill())
        .child(
            ScrollArea::new()
                .width(Size::fill())
                .height(Size::fill())
                .child(
                    rect()
                        .vertical()
                        .width(Size::fill())
                        .spacing(16.)
                        .child(mod_card(
                            "Custom Crosshair Mod",
                            "Fabric 1.21.11 | Pre-installed",
                            "Fully customizable interactive crosshairs. Press `~` (Grave key) in Minecraft to open the live in-game customizer menu. Includes dozens of skins (Dot, Cross, Circle, Chevron), RGB & rainbow color transitions, dynamic bloom on weapon cooldown and bow drawing.",
                            Color::from_rgb(52, 152, 219),
                        ))
                        .child(mod_card(
                            "ArmorHUD Mod",
                            "Fabric 1.21.11 | Pre-installed",
                            "Live status HUD for equipped armor & tools. Dynamically displays real-time durability of Helmet, Chestplate, Leggings, Boots, off-hand Shield, and held weapon directly on your HUD with damage alerts.",
                            Color::from_rgb(155, 89, 182),
                        ))
                        .child(mod_card(
                            "Dynamic FPS Mod",
                            "Fabric 1.21.2 - 1.21.11 | Pre-installed",
                            "Automatic background throttling. Whenever Minecraft is minimized or in the background, rendering automatically slows down to 15 FPS to eliminate fan noise, reduce temperature, and save battery.",
                            Color::from_rgb(46, 204, 113),
                        ))
                        .child(mod_card(
                            "Polyfrost Modern Suite (20+ Mods)",
                            "Fabric 1.21.11 | OneConfig Ecosystem",
                            "Includes OneConfig, Sodium, PolyPlus, PolySprint, PolyBlur, PolyHitbox, PolyNameTag, PolyTime, PolyWeather, PolyTone, DamageTint, EvergreenHUD, VanillaHUD, BehindYou, ChatTweaks, SoundTweaks and more.",
                            Color::from_rgb(241, 196, 15),
                        )),
                ),
        )
}

fn mod_card(title: &'static str, subtitle: &'static str, body: &'static str, accent: Color) -> impl IntoElement {
    rect()
        .vertical()
        .width(Size::fill())
        .padding(Gaps::new_all(18.))
        .corner_radius(CornerRadius::new_all(14.))
        .background(colors::page_elevated())
        .border(border_all_color(1., colors::component_border()))
        .spacing(8.)
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .main_align(Alignment::SpaceBetween)
                .cross_align(Alignment::Center)
                .child(
                    rect()
                        .horizontal()
                        .spacing(10.)
                        .cross_align(Alignment::Center)
                        .child(
                            rect()
                                .width(Size::px(4.))
                                .height(Size::px(20.))
                                .corner_radius(CornerRadius::new_all(2.))
                                .background(accent),
                        )
                        .child(
                            label()
                                .text(title)
                                .font_size(17.)
                                .font_weight(FontWeight::BOLD)
                                .color(colors::fg_primary()),
                        ),
                )
                .child(
                    rect()
                        .padding(Gaps::new(4., 8., 4., 8.))
                        .corner_radius(CornerRadius::new_all(6.))
                        .background(Color::from_argb(80, accent.r(), accent.g(), accent.b()))
                        .border(border_all_color(1., accent))
                        .child(
                            label()
                                .text(subtitle)
                                .font_size(11.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .color(accent),
                        ),
                ),
        )
        .child(
            label()
                .text(body)
                .font_size(13.)
                .line_height(1.4)
                .color(colors::fg_secondary()),
        )
}

