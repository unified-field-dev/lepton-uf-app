//! Change-password card for Account Settings.

use lepton_auth::actions::account::ChangePassword;
use lepton_auth::paths::RESET_PASSWORD_REQUEST;
use lepton_auth::security::password_requirement_results;
use leptos::prelude::*;
use orbital_base_components::Handler;
use uf_product::components::{Caption1, Card, CardContent, CardHeader, Subtitle2};
use uf_product::primitives::*;

#[component]
pub(super) fn ChangePasswordCard(
    change_password_action: ServerAction<ChangePassword>,
) -> impl IntoView {
    let password_preview = RwSignal::new(String::new());
    let password_blurred = RwSignal::new(false);
    let confirm_focused = RwSignal::new(false);
    let requirements = Memo::new(move |_| password_requirement_results(&password_preview.get()));
    let has_unmet_requirements =
        Memo::new(move |_| requirements.get().into_iter().any(|item| !item.satisfied));

    view! {
        <Card>
            <CardHeader>
                <Subtitle2>"Change current password"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true gap=FlexGap::Medium>
                    <Show when=move || matches!(change_password_action.value().get(), Some(Ok(())))>
                        <MessageBar intent=MessageBarIntent::Success>
                            "Password updated successfully."
                        </MessageBar>
                    </Show>
                    <Show when=move || matches!(change_password_action.value().get(), Some(Err(_)))>
                        <MessageBar intent=MessageBarIntent::Error>
                            {move || {
                                change_password_action
                                    .value()
                                    .get()
                                    .and_then(Result::err)
                                    .map(|e| e.to_string())
                                    .unwrap_or_default()
                            }}
                        </MessageBar>
                    </Show>
                    <ActionForm action=change_password_action>
                        <Flex vertical=true gap=FlexGap::Medium>
                            <Field label="Current password" required=true>
                                <Input bind=InputBind { name: "current_password".into(), ..InputBind::default() } appearance=InputAppearance { input_type: Signal::from(InputType::Password), ..Default::default() } />
                            </Field>
                            <Field label="New password" required=true>
                                <Input
                                    bind={
                                        let mut bind = InputBind::new(password_preview);
                                        bind.name = "new_password".into();
                                        bind
                                    }
                                    appearance=InputAppearance {
                                        input_type: Signal::from(InputType::Password),
                                        ..Default::default()
                                    }
                                    events=InputEvents {
                                        on_blur: Some(Handler::on(move |_: leptos::ev::FocusEvent| password_blurred.set(true))),
                                        ..InputEvents::default()
                                    }
                                />
                            </Field>
                            <Field label="Confirm new password" required=true>
                                <Input
                                    bind=InputBind { name: "confirm_password".into(), ..InputBind::default() }
                                    appearance=InputAppearance {
                                        input_type: Signal::from(InputType::Password),
                                        ..Default::default()
                                    }
                                    events=InputEvents {
                                        on_focus: Some(Handler::on(move |_: leptos::ev::FocusEvent| confirm_focused.set(true))),
                                        on_blur: Some(Handler::on(move |_: leptos::ev::FocusEvent| confirm_focused.set(false))),
                                        ..InputEvents::default()
                                    }
                                />
                            </Field>
                            <MessageBar
                                intent=Signal::derive(move || {
                                    if confirm_focused.get()
                                        && password_blurred.get()
                                        && has_unmet_requirements.get()
                                    {
                                        MessageBarIntent::Error
                                    } else {
                                        MessageBarIntent::Info
                                    }
                                })
                            >
                                <InfoLabel>
                                    "Password requirements (hover for details)"
                                    <InfoLabelInfo slot>
                                        <ul>
                                            <For
                                                each=move || requirements.get()
                                                key=|item| item.label
                                                children=move |item| view! { <li>{item.label}</li> }
                                            />
                                        </ul>
                                    </InfoLabelInfo>
                                </InfoLabel>
                            </MessageBar>
                            <Flex gap=FlexGap::Small>
                                <Button button_type=ButtonType::Submit>"Update password"</Button>
                            </Flex>
                        </Flex>
                    </ActionForm>
                    <Caption1>
                        "Need a reset link instead? "
                        <Link href=RESET_PASSWORD_REQUEST inline=true>
                            "Request password reset"
                        </Link>
                    </Caption1>
                </Flex>
            </CardContent>
        </Card>
    }
}
