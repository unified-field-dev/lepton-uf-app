//! `/auth/signup` page.

use lepton_auth_ui::AuthDialogKind;

use crate::pages::AuthRouteHost;
use leptos::prelude::*;

/// `/auth/signup` page: hosts the sign-up dialog.
#[component]
pub fn SignupPage() -> impl IntoView {
    view! {
        <AuthRouteHost initial_kind=AuthDialogKind::Signup test_id="signup-container" />
    }
}
