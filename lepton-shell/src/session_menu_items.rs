//! Avatar menu item lists for signed-in vs anonymous sessions.

use leptos::prelude::*;
use orbital_primitives::*;
use uf_product::AuthSession;

#[component]
pub(super) fn SessionMenuItems(#[prop(into)] session: Signal<AuthSession>) -> impl IntoView {
    move || match session.get() {
        AuthSession::Authenticated(_) => view! {
            <div data-testid="user-menu-profile">
                <MenuItem value="profile">"Profile"</MenuItem>
            </div>
            <MenuItem disabled=true value="preferences">"Preferences"</MenuItem>
            <div data-testid="user-menu-account-settings">
                <MenuItem value="account_settings">"Account Settings"</MenuItem>
            </div>
            <Divider />
            <div data-testid="user-menu-logout">
                <MenuItem value="logout">"Log Out"</MenuItem>
            </div>
        }
        .into_any(),
        AuthSession::Anonymous(_) => view! {
            <div data-testid="user-menu-signin">
                <MenuItem value="signin">"Sign In"</MenuItem>
            </div>
            <div data-testid="user-menu-signup">
                <MenuItem value="signup">"Sign Up"</MenuItem>
            </div>
        }
        .into_any(),
    }
}
