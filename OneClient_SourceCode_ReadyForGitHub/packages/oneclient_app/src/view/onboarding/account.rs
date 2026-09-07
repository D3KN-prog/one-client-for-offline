use freya::prelude::*;
use freya::query::MutationStateData;
use oneclient_auth::MinecraftAccount;

use crate::components::{Avatar, Button, Icon, IconType, TextInput};
use crate::hooks::{
    AddOfflineAccountKeys, try_default_account, use_add_offline_account, use_current_account,
};
use crate::routes::Route;
use crate::theme::colors;
use crate::view::onboarding::{
    onboarding_illustration, onboarding_nav, onboarding_page, step_heading,
};

#[derive(PartialEq)]
pub struct OnboardingAccount;

impl Component for OnboardingAccount {
    fn render(&self) -> impl IntoElement {
        let account_query = use_current_account();
        let add_offline = use_add_offline_account();
        let mut username = use_state(String::new);
        let mut submitted = use_state(|| false);

        use_side_effect(move || {
            if !*submitted.read() {
                return;
            }
            match &*add_offline.read().state() {
                MutationStateData::Settled { .. } => {
                    submitted.set(false);
                }
                _ => {}
            }
        });

        let account = try_default_account(&account_query);
        let has_account = account.is_some();
        let offline_name = username.read().trim().to_string();
        let can_continue = has_account || offline_name.len() >= 3;

        let add_offline2 = add_offline.clone();
        let on_confirm = move |_: Event<PressEventData>| {
            let name = username.peek().trim().to_string();
            if name.len() < 3 {
                return;
            }
            add_offline2.mutate(AddOfflineAccountKeys { username: name });
            submitted.set(true);
        };

        let error: Option<String> = match &*add_offline.read().state() {
            MutationStateData::Settled { res: Err(err), .. } => Some(err.to_string()),
            _ => None,
        };

        let content = rect()
            .vertical()
            .width(Size::fill())
            .spacing(24.)
            .child(step_heading(
                "Account",
                "Enter a username to play offline. No Microsoft account needed.",
            ))
            .child(match &account {
                Some(account) => account_preview(account).into_element(),
                None => offline_input_card(username, error, on_confirm).into_element(),
            })
            .into_element();

        let page = onboarding_page(
            onboarding_illustration(IconType::OnboardingAccount),
            content,
            onboarding_nav(
                Some(Route::OnboardingLanguage {}),
                Route::OnboardingBundles {},
                can_continue,
            ),
        );

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .child(page)
    }
}

fn account_preview(account: &MinecraftAccount) -> impl IntoElement {
    rect()
        .horizontal()
        .width(Size::fill())
        .spacing(24.)
        .child(
            rect()
                .horizontal()
                .spacing(12.)
                .cross_align(Alignment::Center)
                .child(
                    Avatar::new(account.id.to_string())
                        .width(Size::px(48.))
                        .height(Size::px(48.)),
                )
                .child(
                    rect()
                        .vertical()
                        .spacing(4.)
                        .child(
                            label()
                                .text(account.username.clone())
                                .font_size(16.)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .color(colors::fg_primary()),
                        )
                        .child(
                            label()
                                .text(account.id.to_string())
                                .font_size(12.)
                                .color(colors::fg_secondary()),
                        ),
                ),
        )
        .into_element()
}

fn offline_input_card(
    username: State<String>,
    error: Option<String>,
    on_confirm: impl FnMut(Event<PressEventData>) + 'static,
) -> impl IntoElement {
    rect()
        .vertical()
        .width(Size::fill())
        .spacing(12.)
        .child(
            label()
                .text("Username")
                .font_size(11.)
                .font_weight(FontWeight::MEDIUM)
                .color(colors::fg_secondary()),
        )
        .child(TextInput::new(username).placeholder("Enter your username (3-16 chars)"))
        .map(error, |el, msg| {
            el.child(
                rect()
                    .horizontal()
                    .cross_align(Alignment::Center)
                    .spacing(6.)
                    .child(
                        Icon::new(IconType::AlertTriangle)
                            .size(13.)
                            .color(colors::danger()),
                    )
                    .child(label().text(msg).font_size(12.).color(colors::danger()))
                    .into_element(),
            )
        })
        .child(
            Button::new()
                .primary()
                .large()
                .on_press(on_confirm)
                .child(Icon::new(IconType::Plus).size(16.))
                .text("Continue"),
        )
        .into_element()
}


