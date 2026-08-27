//! TOTP enroll / manage step views.

use lepton_auth::actions::totp::{
    begin_totp_enroll_ui, confirm_totp_enroll_ui, disable_totp_ui,
    regenerate_totp_recovery_codes_ui, PendingTotpEnrollView,
};
use leptos::prelude::*;
use leptos::task::spawn_local_scoped;
use orbital_base_components::input_event_value;
use uf_product::components::{Body1, Caption1, Step, StepStatus, Stepper};
use uf_product::primitives::*;

use super::helpers::{copy_text, safe_totp_qr_svg, server_err, TotpUiStep};

/// Shared signals for the authenticator settings card.
#[derive(Clone, Copy)]
pub(super) struct TotpState {
    pub step: RwSignal<TotpUiStep>,
    pub pending: RwSignal<Option<PendingTotpEnrollView>>,
    pub recovery_codes: RwSignal<Vec<String>>,
    pub code_input: RwSignal<String>,
    pub recovery_ack: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub busy: RwSignal<bool>,
    pub bump: Callback<()>,
    pub scan_status: Signal<StepStatus>,
    pub confirm_status: Signal<StepStatus>,
    pub recovery_status: Signal<StepStatus>,
}

#[component]
fn EnrollStepper(state: TotpState) -> impl IntoView {
    view! {
        <Stepper vertical=false>
            <Step slot:steps label="Scan".to_string() status=state.scan_status.get() />
            <Step slot:steps label="Confirm".to_string() status=state.confirm_status.get() />
            <Step slot:steps label="Recovery".to_string() status=state.recovery_status.get() />
        </Stepper>
    }
}

#[component]
pub(super) fn TotpIdleStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-idle">
            <Flex vertical=true gap=FlexGap::Medium>
                <Caption1>
                    "Add a time-based code from an authenticator app. You'll need it when you sign in, and for sensitive account actions."
                </Caption1>
                <div data-testid="totp-settings-setup">
                    <Button
                        attr:data-testid="totp-settings-setup-btn"
                        disabled=Signal::derive(move || state.busy.get())
                        on_click=Callback::new(move |_| {
                            state.busy.set(true);
                            state.error.set(None);
                            spawn_local_scoped(async move {
                                match begin_totp_enroll_ui().await {
                                    Ok(view) => {
                                        state.pending.set(Some(view));
                                        state.step.set(TotpUiStep::Scan);
                                    }
                                    Err(e) => state.error.set(Some(server_err(&e))),
                                }
                                state.busy.set(false);
                            });
                        })
                    >
                        "Set up authenticator"
                    </Button>
                </div>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpScanStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-scan">
            <Flex vertical=true gap=FlexGap::Medium>
                <EnrollStepper state=state />
                <Body1>"Scan this QR with your authenticator app."</Body1>
                {move || state.pending.get().map(|p| {
                    let secret = p.manual_secret.clone();
                    let svg = p.qr_svg.clone();
                    view! {
                        {match safe_totp_qr_svg(&svg) {
                            Some(safe) => view! {
                                <div data-testid="totp-settings-qr" data-otpauth=p.otpauth_uri.clone() inner_html=safe.to_string()></div>
                            }.into_any(),
                            None => view! {
                                <MessageBar intent=MessageBarIntent::Error>
                                    "QR image could not be displayed. Use the manual key below."
                                </MessageBar>
                            }.into_any(),
                        }}
                        <Caption1>"Can't scan? Enter this key manually:"</Caption1>
                        <div data-testid="totp-settings-manual-secret">
                            <Code text=secret.clone() />
                        </div>
                        <Button
                            appearance=ButtonAppearance::Secondary
                            on_click=Callback::new({
                                let secret = secret;
                                move |_| copy_text(&secret.replace(' ', ""))
                            })
                        >
                            "Copy key"
                        </Button>
                    }
                })}
                <Flex gap=FlexGap::Small>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.pending.set(None);
                            state.error.set(None);
                            state.step.set(TotpUiStep::Idle);
                        })
                    >
                        "Cancel"
                    </Button>
                    <Button
                        attr:data-testid="totp-settings-continue"
                        on_click=Callback::new(move |_| {
                            state.code_input.set(String::new());
                            state.error.set(None);
                            state.step.set(TotpUiStep::Confirm);
                        })
                    >
                        "Continue"
                    </Button>
                </Flex>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpConfirmStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-confirm">
            <Flex vertical=true gap=FlexGap::Medium>
                <EnrollStepper state=state />
                <Body1>"Enter the 6-digit code from your app to finish setup."</Body1>
                <Field label="Code" required=true>
                    <input
                        type="text"
                        inputmode="numeric"
                        autocomplete="one-time-code"
                        data-testid="totp-settings-code"
                        prop:value=move || state.code_input.get()
                        on:input=move |ev| {
                            if let Some(v) = input_event_value(&ev) {
                                state.code_input.set(v);
                            }
                        }
                    />
                </Field>
                <Flex gap=FlexGap::Small>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.error.set(None);
                            state.step.set(TotpUiStep::Scan);
                        })
                    >
                        "Back"
                    </Button>
                    <Button
                        attr:data-testid="totp-settings-confirm-submit"
                        disabled=Signal::derive(move || state.busy.get())
                        on_click=Callback::new(move |_| {
                            let Some(p) = state.pending.get() else {
                                state.error.set(Some("Setup expired. Start again.".into()));
                                state.step.set(TotpUiStep::Idle);
                                return;
                            };
                            let factor_id = p.factor_id.clone();
                            let code = state.code_input.get();
                            state.busy.set(true);
                            state.error.set(None);
                            spawn_local_scoped(async move {
                                match confirm_totp_enroll_ui(factor_id, code).await {
                                    Ok(codes) => {
                                        state.recovery_codes.set(codes);
                                        state.recovery_ack.set(false);
                                        state.pending.set(None);
                                        state.step.set(TotpUiStep::Recovery);
                                    }
                                    Err(e) => state.error.set(Some(server_err(&e))),
                                }
                                state.busy.set(false);
                            });
                        })
                    >
                        "Confirm"
                    </Button>
                </Flex>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpRecoveryStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-recovery">
            <Flex vertical=true gap=FlexGap::Medium>
                <EnrollStepper state=state />
                <MessageBar intent=MessageBarIntent::Warning>
                    "Save these recovery codes now. We only show them once. Each code works one time if you lose your authenticator."
                </MessageBar>
                <div data-testid="totp-settings-recovery-list">
                    {move || {
                        let text = state
                            .recovery_codes
                            .get()
                            .chunks(2)
                            .map(|pair| pair.join("     "))
                            .collect::<Vec<_>>()
                            .join("\n");
                        view! { <Code text=text /> }
                    }}
                </div>
                <Button
                    appearance=ButtonAppearance::Secondary
                    on_click=Callback::new(move |_| {
                        copy_text(&state.recovery_codes.get().join("\n"));
                    })
                >
                    "Copy all"
                </Button>
                <div data-testid="totp-settings-recovery-ack">
                    <Checkbox
                        checked=state.recovery_ack
                        label="I saved these codes".to_string()
                        size=Signal::from(CheckboxSize::Medium)
                    />
                </div>
                <Button
                    attr:data-testid="totp-settings-recovery-done"
                    disabled=Signal::derive(move || !state.recovery_ack.get() || state.busy.get())
                    on_click=Callback::new(move |_| {
                        state.recovery_codes.set(Vec::new());
                        state.recovery_ack.set(false);
                        state.error.set(None);
                        state.step.set(TotpUiStep::Enabled);
                        state.bump.run(());
                    })
                >
                    "Done"
                </Button>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpEnabledStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-enabled">
            <Flex vertical=true gap=FlexGap::Medium>
                <Caption1>
                    "Sign-in and sensitive actions can ask for a code from your app."
                </Caption1>
                <Flex gap=FlexGap::Small wrap=FlexWrap::Wrap>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.code_input.set(String::new());
                            state.error.set(None);
                            state.step.set(TotpUiStep::RegenConfirm);
                        })
                    >
                        "Get new recovery codes"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.code_input.set(String::new());
                            state.error.set(None);
                            state.step.set(TotpUiStep::Disable);
                        })
                    >
                        "Disable authenticator"
                    </Button>
                </Flex>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpDisableStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-disable">
            <Flex vertical=true gap=FlexGap::Medium>
                <Body1>"Disable authenticator? You can set it up again later."</Body1>
                <Field label="Current code" required=true>
                    <input
                        type="text"
                        inputmode="numeric"
                        autocomplete="one-time-code"
                        data-testid="totp-settings-disable-code"
                        prop:value=move || state.code_input.get()
                        on:input=move |ev| {
                            if let Some(v) = input_event_value(&ev) {
                                state.code_input.set(v);
                            }
                        }
                    />
                </Field>
                <Flex gap=FlexGap::Small>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.error.set(None);
                            state.step.set(TotpUiStep::Enabled);
                        })
                    >
                        "Keep enabled"
                    </Button>
                    <Button
                        disabled=Signal::derive(move || state.busy.get())
                        on_click=Callback::new(move |_| {
                            let code = state.code_input.get();
                            state.busy.set(true);
                            state.error.set(None);
                            spawn_local_scoped(async move {
                                match disable_totp_ui(code).await {
                                    Ok(()) => {
                                        state.pending.set(None);
                                        state.recovery_codes.set(Vec::new());
                                        state.step.set(TotpUiStep::Idle);
                                        state.bump.run(());
                                    }
                                    Err(e) => state.error.set(Some(server_err(&e))),
                                }
                                state.busy.set(false);
                            });
                        })
                    >
                        "Disable"
                    </Button>
                </Flex>
            </Flex>
        </div>
    }
}

#[component]
pub(super) fn TotpRegenStep(state: TotpState) -> impl IntoView {
    view! {
        <div data-testid="totp-settings-regen">
            <Flex vertical=true gap=FlexGap::Medium>
                <MessageBar intent=MessageBarIntent::Warning>
                    "New recovery codes replace the old ones. Enter a current authenticator code to continue."
                </MessageBar>
                <Field label="Current code" required=true>
                    <input
                        type="text"
                        inputmode="numeric"
                        autocomplete="one-time-code"
                        data-testid="totp-settings-regen-code"
                        prop:value=move || state.code_input.get()
                        on:input=move |ev| {
                            if let Some(v) = input_event_value(&ev) {
                                state.code_input.set(v);
                            }
                        }
                    />
                </Field>
                <Flex gap=FlexGap::Small>
                    <Button
                        appearance=ButtonAppearance::Secondary
                        on_click=Callback::new(move |_| {
                            state.error.set(None);
                            state.step.set(TotpUiStep::Enabled);
                        })
                    >
                        "Cancel"
                    </Button>
                    <Button
                        disabled=Signal::derive(move || state.busy.get())
                        on_click=Callback::new(move |_| {
                            let code = state.code_input.get();
                            state.busy.set(true);
                            state.error.set(None);
                            spawn_local_scoped(async move {
                                match regenerate_totp_recovery_codes_ui(code).await {
                                    Ok(codes) => {
                                        state.recovery_codes.set(codes);
                                        state.recovery_ack.set(false);
                                        state.step.set(TotpUiStep::Recovery);
                                    }
                                    Err(e) => state.error.set(Some(server_err(&e))),
                                }
                                state.busy.set(false);
                            });
                        })
                    >
                        "Generate"
                    </Button>
                </Flex>
            </Flex>
        </div>
    }
}
