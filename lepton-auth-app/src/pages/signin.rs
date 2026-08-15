//! `/auth/signin` page.

use lepton_auth_ui::AuthDialogKind;

use crate::pages::AuthRouteHost;
use leptos::prelude::*;

/// `/auth/signin` page: hosts the sign-in dialog.
#[component]
pub fn SigninPage() -> impl IntoView {
    view! {
        <AuthRouteHost initial_kind=AuthDialogKind::Signin test_id="signin-container" />
    }
}
