//! `/auth/reset/*` page components hosting the password reset dialog.

use lepton_auth_ui::PasswordResetDialogKind;
use leptos::prelude::*;

use crate::pages::PasswordResetRouteHost;

/// `/auth/reset/request` page: hosts the password reset request dialog.
#[component]
pub fn PasswordResetRequestPage() -> impl IntoView {
    view! {
        <PasswordResetRouteHost
            initial_kind=PasswordResetDialogKind::Request
            test_id="password-reset-request-container"
        />
    }
}

/// `/auth/reset/confirm` page: hosts the password reset confirm dialog.
#[component]
pub fn PasswordResetConfirmPage() -> impl IntoView {
    view! {
        <PasswordResetRouteHost
            initial_kind=PasswordResetDialogKind::Confirm
            test_id="password-reset-confirm-container"
        />
    }
}
