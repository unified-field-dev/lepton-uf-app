//! Danger Zone card: wipe legal account via [`lepton_auth::actions::account::WipeAccount`].

use lepton_auth::account_api::WIPE_CONFIRM_PHRASE;
use lepton_auth::actions::account::WipeAccount;
use leptos::prelude::*;
use uf_product::components::{Body1, Caption1, Card, CardContent, CardHeader, Subtitle2};
use uf_product::primitives::*;

/// Account wipe form (Owner): password, type-to-confirm, optional TOTP.
#[component]
pub fn AccountWipeSection() -> impl IntoView {
    let wipe_action = ServerAction::<WipeAccount>::new();

    view! {
        <Card>
            <CardHeader>
                <Subtitle2>"Delete account"</Subtitle2>
            </CardHeader>
            <CardContent>
                <Flex vertical=true gap=FlexGap::Medium>
                    <div data-testid="account-wipe-section">
                        <Body1>
                            "This permanently deletes your account, emails, personas, and sign-in data. It cannot be undone."
                        </Body1>
                    </div>
                    <Caption1>
                        "Type " {WIPE_CONFIRM_PHRASE} " to confirm. Enter your current password. If you use an authenticator app, enter a code too."
                    </Caption1>
                    <Show when=move || matches!(wipe_action.value().get(), Some(Err(_)))>
                        <div data-testid="account-wipe-error">
                            <MessageBar intent=MessageBarIntent::Error>
                                {move || {
                                    wipe_action
                                        .value()
                                        .get()
                                        .and_then(Result::err)
                                        .map_or_else(
                                            || "Unable to delete the account right now.".to_string(),
                                            |e| e.to_string(),
                                        )
                                }}
                            </MessageBar>
                        </div>
                    </Show>
                    <ActionForm action=wipe_action>
                        <div data-testid="account-wipe-form">
                            <Flex vertical=true gap=FlexGap::Medium>
                                <Field label="Type DELETE to confirm" required=true>
                                    <Input
                                        bind=InputBind {
                                            name: "confirm_phrase".into(),
                                            ..InputBind::default()
                                        }
                                        appearance=InputAppearance {
                                            input_type: Signal::from(InputType::Text),
                                            ..Default::default()
                                        }
                                    />
                                </Field>
                                <Field label="Current password" required=true>
                                    <Input
                                        bind=InputBind {
                                            name: "current_password".into(),
                                            ..InputBind::default()
                                        }
                                        appearance=InputAppearance {
                                            input_type: Signal::from(InputType::Password),
                                            ..Default::default()
                                        }
                                    />
                                </Field>
                                <Field label="Authenticator code (if enabled)">
                                    <Input
                                        bind=InputBind {
                                            name: "totp_code".into(),
                                            ..InputBind::default()
                                        }
                                        appearance=InputAppearance {
                                            input_type: Signal::from(InputType::Text),
                                            ..Default::default()
                                        }
                                    />
                                </Field>
                                <Flex gap=FlexGap::Small>
                                    <Button
                                        button_type=ButtonType::Submit
                                        appearance=ButtonAppearance::Primary
                                        disabled=wipe_action.pending()
                                        attr:data-testid="account-wipe-submit"
                                    >
                                        {move || {
                                            if wipe_action.pending().get() {
                                                "Deleting…"
                                            } else {
                                                "Delete account"
                                            }
                                        }}
                                    </Button>
                                </Flex>
                            </Flex>
                        </div>
                    </ActionForm>
                </Flex>
            </CardContent>
        </Card>
    }
}
