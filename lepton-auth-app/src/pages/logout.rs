//! `/auth/logout` page.

use lepton_auth_ui::AuthDialogKind;

use crate::pages::AuthRouteHost;
use leptos::prelude::*;

/// `/auth/logout` page: hosts the logout confirmation dialog.
#[component]
pub fn LogoutPage() -> impl IntoView {
    view! {
        <AuthRouteHost initial_kind=AuthDialogKind::Logout test_id="logout-container" />
    }
}
