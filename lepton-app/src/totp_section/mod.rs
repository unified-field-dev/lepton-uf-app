//! Authenticator (TOTP) enroll card for Account Settings.

mod helpers;
mod steps;

use helpers::TotpUiStep;
use lepton_auth::actions::totp::{get_totp_settings_status, PendingTotpEnrollView};
use leptos::prelude::*;
use steps::{
    TotpConfirmStep, TotpDisableStep, TotpEnabledStep, TotpIdleStep, TotpRecoveryStep,
    TotpRegenStep, TotpScanStep, TotpState,
};
use uf_product::components::{
    Card, CardContent, CardHeader, SkeletonItemSize, StepStatus, Subtitle2,
};
use uf_product::primitives::*;

/// Authenticator app enroll / disable / recovery for Account Settings.
#[component]
pub fn TotpSettingsSection() -> impl IntoView {
    let refresh = RwSignal::new(0u32);
    let status = Resource::new(
        move || refresh.get(),
        |_| async move { get_totp_settings_status().await },
    );
    let step = RwSignal::new(TotpUiStep::Idle);
    let pending = RwSignal::new(Option::<PendingTotpEnrollView>::None);
    let recovery_codes = RwSignal::new(Vec::<String>::new());
    let code_input = RwSignal::new(String::new());
    let recovery_ack = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);
    let busy = RwSignal::new(false);

    let bump = Callback::new(move |()| refresh.update(|n| *n = n.wrapping_add(1)));

    Effect::new(move |_| {
        if let Some(Ok(s)) = status.get() {
            match step.get_untracked() {
                TotpUiStep::Idle | TotpUiStep::Enabled => {
                    step.set(if s.totp_enabled {
                        TotpUiStep::Enabled
                    } else {
                        TotpUiStep::Idle
                    });
                }
                _ => {}
            }
        }
    });

    let scan_status = Signal::derive(move || match step.get() {
        TotpUiStep::Scan => StepStatus::Active,
        TotpUiStep::Confirm | TotpUiStep::Recovery | TotpUiStep::Enabled => StepStatus::Done,
        _ => StepStatus::Pending,
    });
    let confirm_status = Signal::derive(move || match step.get() {
        TotpUiStep::Confirm => StepStatus::Active,
        TotpUiStep::Recovery | TotpUiStep::Enabled => StepStatus::Done,
        _ => StepStatus::Pending,
    });
    let recovery_status = Signal::derive(move || match step.get() {
        TotpUiStep::Recovery => StepStatus::Active,
        TotpUiStep::Enabled => StepStatus::Done,
        _ => StepStatus::Pending,
    });

    let state = TotpState {
        step,
        pending,
        recovery_codes,
        code_input,
        recovery_ack,
        error,
        busy,
        bump,
        scan_status,
        confirm_status,
        recovery_status,
    };

    view! {
        <Card>
            <CardHeader>
                <Flex align=FlexAlign::Center gap=FlexGap::Small wrap=FlexWrap::Wrap>
                    <Subtitle2>"Authenticator app"</Subtitle2>
                    {move || match step.get() {
                        TotpUiStep::Enabled => view! {
                            <Badge appearance=BadgeAppearance::Filled>"enabled"</Badge>
                        }.into_any(),
                        TotpUiStep::Idle => view! {
                            <Badge appearance=BadgeAppearance::Outline>"not set up"</Badge>
                        }.into_any(),
                        _ => ().into_any(),
                    }}
                </Flex>
            </CardHeader>
            <CardContent>
                <div data-testid="totp-settings-section">
                    <Flex vertical=true gap=FlexGap::Medium>
                        <Show when=move || error.get().is_some()>
                            <MessageBar intent=MessageBarIntent::Error>
                                <div data-testid="totp-settings-error">
                                    {move || error.get().unwrap_or_default()}
                                </div>
                            </MessageBar>
                        </Show>

                        <Suspense fallback=move || view! {
                            <Skeleton>
                                <SkeletonItem size=Signal::from(SkeletonItemSize::S48) />
                            </Skeleton>
                        }>
                            {move || {
                                let _ = status.get();
                                match step.get() {
                                    TotpUiStep::Idle => view! { <TotpIdleStep state=state /> }.into_any(),
                                    TotpUiStep::Scan => view! { <TotpScanStep state=state /> }.into_any(),
                                    TotpUiStep::Confirm => view! { <TotpConfirmStep state=state /> }.into_any(),
                                    TotpUiStep::Recovery => view! { <TotpRecoveryStep state=state /> }.into_any(),
                                    TotpUiStep::Enabled => view! { <TotpEnabledStep state=state /> }.into_any(),
                                    TotpUiStep::Disable => view! { <TotpDisableStep state=state /> }.into_any(),
                                    TotpUiStep::RegenConfirm => view! { <TotpRegenStep state=state /> }.into_any(),
                                }
                            }}
                        </Suspense>
                    </Flex>
                </div>
            </CardContent>
        </Card>
    }
}
