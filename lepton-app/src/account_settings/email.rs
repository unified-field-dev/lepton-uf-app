//! Email overview, verify, and change-email cards for Account Settings.

use lepton_auth::actions::account::{
    RequestEmailChange, RequestEmailVerification, VerifyEmailToken,
};
use leptos::prelude::*;
use orbital_base_components::input_event_value;
use uf_product::components::{Body1, Card, CardContent, CardHeader, Subtitle2};
use uf_product::primitives::*;

#[component]
pub(super) fn VerifyEmailForm(
    verify_action: ServerAction<VerifyEmailToken>,
    verify_token_input: RwSignal<String>,
) -> impl IntoView {
    view! {
        <ActionForm action=verify_action>
            <div data-testid="account-verify-email-form">
                <Flex vertical=true gap=FlexGap::Medium>
                <Field label="Verification token" required=true>
                    // Native input so `ActionForm` posts `token` (Thaw `Input` + `prop:value` did not serialize).
                    <input
                        type="text"
                        name="token"
                        autocomplete="off"
                        prop:value=move || verify_token_input.get()
                        on:input=move |ev| {
                            if let Some(v) = input_event_value(&ev) {
                                verify_token_input.set(v);
                            }
                        }
                    />
                </Field>
                <Flex gap=FlexGap::Small>
                    <Button button_type=ButtonType::Submit disabled=verify_action.pending()>
                        {move || {
                            if verify_action.pending().get() {
                                "Verifying..."
                            } else {
                                "Verify email"
                            }
                        }}
                    </Button>
                </Flex>
                </Flex>
            </div>
        </ActionForm>
    }
}

#[component]
pub(super) fn EmailOverviewCard(
    masked_email: String,
    role_badge: String,
    email_verified: bool,
    token_from_query: Memo<String>,
    verify_action: ServerAction<VerifyEmailToken>,
    verify_token_input: RwSignal<String>,
    resend_action: ServerAction<RequestEmailVerification>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Subtitle2>"Email"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true gap=FlexGap::Medium>
                    <Flex align=FlexAlign::Center gap=FlexGap::Small wrap=FlexWrap::Wrap>
                        <div data-testid="account-masked-email">
                            <Body1>{masked_email}</Body1>
                        </div>
                        <Badge appearance=BadgeAppearance::Filled>
                            {role_badge}
                        </Badge>
                    </Flex>
                    <Show
                        when=move || !email_verified
                        fallback=move || view! {
                            <div data-testid="account-email-verified-banner">
                                <MessageBar intent=MessageBarIntent::Success>
                                    "Email is verified."
                                </MessageBar>
                            </div>
                            <Show when=move || !token_from_query.get().is_empty()>
                                <Show when=move || matches!(verify_action.value().get(), Some(Err(_)))>
                                    <MessageBar intent=MessageBarIntent::Error>
                                        {move || {
                                            verify_action
                                                .value()
                                                .get()
                                                .and_then(Result::err)
                                                .map_or_else(|| "Unable to verify email right now.".to_string(), |e| e.to_string())
                                        }}
                                    </MessageBar>
                                </Show>
                                <VerifyEmailForm
                                    verify_action=verify_action
                                    verify_token_input=verify_token_input
                                />
                            </Show>
                        }
                    >
                        <div data-testid="account-email-unverified-banner">
                            <MessageBar intent=MessageBarIntent::Warning>
                                "Email is not verified. Some routes are restricted."
                            </MessageBar>
                        </div>
                        <Show when=move || matches!(verify_action.value().get(), Some(Err(_)))>
                            <MessageBar intent=MessageBarIntent::Error>
                                {move || {
                                    verify_action
                                        .value()
                                        .get()
                                        .and_then(Result::err)
                                        .map_or_else(|| "Unable to verify email right now.".to_string(), |e| e.to_string())
                                }}
                            </MessageBar>
                        </Show>
                        <VerifyEmailForm
                            verify_action=verify_action
                            verify_token_input=verify_token_input
                        />
                        <ActionForm action=resend_action>
                            <Button
                                appearance=ButtonAppearance::Secondary
                                button_type=ButtonType::Submit
                                disabled=resend_action.pending()
                            >
                                {move || {
                                    if resend_action.pending().get() {
                                        "Sending verification email..."
                                    } else {
                                        "Resend verification email"
                                    }
                                }}
                            </Button>
                        </ActionForm>
                        <Show when=move || matches!(resend_action.value().get(), Some(Ok(())))>
                            <MessageBar intent=MessageBarIntent::Success>
                                "Verification email sent."
                            </MessageBar>
                        </Show>
                        <Show when=move || matches!(resend_action.value().get(), Some(Err(_)))>
                            <MessageBar intent=MessageBarIntent::Error>
                                {move || {
                                    resend_action
                                        .value()
                                        .get()
                                        .and_then(Result::err)
                                        .map_or_else(|| "Unable to send verification email right now.".to_string(), |e| e.to_string())
                                }}
                            </MessageBar>
                        </Show>
                    </Show>
                </Flex>
            </CardContent>
        </Card>
    }
}

#[component]
pub(super) fn ChangeEmailCard(
    change_email_action: ServerAction<RequestEmailChange>,
) -> impl IntoView {
    view! {
        <Card>
            <CardHeader>
                <Subtitle2>"Change email"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true gap=FlexGap::Medium>
                    <Show when=move || matches!(change_email_action.value().get(), Some(Ok(())))>
                        <MessageBar intent=MessageBarIntent::Success>
                            "Verification sent for the new email. Your current email stays active until verification."
                        </MessageBar>
                    </Show>
                    <Show when=move || matches!(change_email_action.value().get(), Some(Err(_)))>
                        <MessageBar intent=MessageBarIntent::Error>
                            {move || {
                                change_email_action
                                    .value()
                                    .get()
                                    .and_then(Result::err)
                                    .map(|e| e.to_string())
                                    .unwrap_or_default()
                            }}
                        </MessageBar>
                    </Show>
                    <ActionForm action=change_email_action>
                        <Flex vertical=true gap=FlexGap::Medium>
                            <Field label="New email" required=true>
                                <Input bind=InputBind { name: "new_email".into(), ..InputBind::default() } appearance=InputAppearance { input_type: Signal::from(InputType::Email), ..Default::default() } />
                            </Field>
                            <Field label="Current password" required=true>
                                <Input bind=InputBind { name: "current_password".into(), ..InputBind::default() } appearance=InputAppearance { input_type: Signal::from(InputType::Password), ..Default::default() } />
                            </Field>
                            <Flex gap=FlexGap::Small>
                                <Button button_type=ButtonType::Submit>
                                    "Request email change"
                                </Button>
                            </Flex>
                        </Flex>
                    </ActionForm>
                </Flex>
            </CardContent>
        </Card>
    }
}
