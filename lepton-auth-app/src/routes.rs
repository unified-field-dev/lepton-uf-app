//! Nested `/auth` route wiring for the auth pages.
//!
//! [`LeptonAuthRoutes`] is the uf-product registry entrypoint. The
//! `orbital_routes_extract` attribute also emits [`paths`] (`SIGNIN`, `SIGNUP`, …)
//! beside this module; path constants are also published as `lepton_auth_app::paths`.

use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Redirect, Route};

#[component]
fn AuthRoutesLayout() -> impl IntoView {
    view! {
        <div data-testid="auth-routes-layout-root">
            <Outlet />
        </div>
    }
}

/// Nested `/auth` routes for a Unified Field host `Router` / `Routes` tree.
///
/// Drop in as a route child alongside `lepton_app::UserAppRoutes`. Pair with
/// `lepton_shell::AppBarUserMenu` via `uf_integrations::provide_shell_auth_menu`.
/// Path constants for leaves live in [`paths`] (crate root: `lepton_auth_app::paths`).
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn LeptonAuthRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    use crate::pages::{
        LogoutPage, OAuthCallbackPage, PasswordResetConfirmPage, PasswordResetRequestPage,
        SigninPage, SignupPage,
    };
    use leptos_router::path;

    view! {
        <ParentRoute path=path!("auth") view=AuthRoutesLayout>
            <Route path=path!("") view=|| view! { <Redirect path=crate::paths::SIGNIN /> } />
            <Route path=path!("signup") view=SignupPage />
            <Route path=path!("signin") view=SigninPage />
            <Route path=path!("logout") view=LogoutPage />
            <Route path=path!("oauth/callback") view=OAuthCallbackPage />
            <Route path=path!("reset/request") view=PasswordResetRequestPage />
            <Route path=path!("reset/confirm") view=PasswordResetConfirmPage />
        </ParentRoute>
    }
    .into_inner()
}
