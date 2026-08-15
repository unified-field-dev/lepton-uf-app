#![recursion_limit = "256"]
#![allow(missing_docs)] // uf_app! / routes macros emit undocumented associated items
#![allow(clippy::too_long_first_doc_paragraph)]
//! Lepton authentication app routes under `/auth`.
//!
//! Composes sign-in / sign-up / logout / password-reset / OAuth callback pages for
//! Unified Field hosts. Identity and session logic live in **lepton-auth** /
//! **higgs**; Orbital is UI-only. Pair with `lepton_shell::AppBarUserMenu` and
//! `lepton_app::UserAppRoutes`. Workspace mount recipe: crate README at the
//! `lepton-uf-app` root.
//!
//! **Owns:** nested `/auth/*` route table and page chrome.
//! **Does not own:** delivery adapters, OAuth client config, or Account Settings.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | Register `/auth` routes | [`LeptonAuthRoutes`] (`uf_app!` id `orbital-auth`) |
//! | Navigate to an `/auth` leaf | [`paths`] |
//! | Sign-in / sign-up / logout pages | [`SigninPage`] / [`SignupPage`] / [`LogoutPage`] |
//! | Password reset | [`PasswordResetRequestPage`] / [`PasswordResetConfirmPage`] |
//! | OAuth callback | [`OAuthCallbackPage`] |
//!
//! ## Concern → API
//!
//! | Concern | Page | Kit |
//! |---------|------|-----|
//! | Sign-in | [`SigninPage`] | `lepton_auth_ui` + `lepton_auth::actions` |
//! | Sign-up | [`SignupPage`] | `lepton_auth_ui` + `lepton_auth::actions` |
//! | Logout | [`LogoutPage`] | `lepton_auth_ui` + `lepton_auth::actions` |
//! | Password reset | [`PasswordResetRequestPage`] / [`PasswordResetConfirmPage`] | `lepton_auth::actions::password_reset` |
//! | OAuth callback | [`OAuthCallbackPage`] | `lepton_auth_ui::OAuthCallbackContent` |
//! | `/auth/*` path constants | [`paths`] | generated beside [`LeptonAuthRoutes`] |
//!
//! Auth mutations live in **lepton-auth** / **lepton-auth-ui** and return
//! `ServerFnError` at the Leptos boundary; this crate mounts pages only.
//!
//! ## Getting started
//!
//! ```rust,ignore
//! use lepton_app::UserAppRoutes;
//! use lepton_auth_app::LeptonAuthRoutes;
//! use lepton_shell::AppBarUserMenu;
//! use leptos::prelude::*;
//! use leptos_router::components::{Router, Routes};
//! use uf_integrations::provide_shell_auth_menu;
//!
//! provide_shell_auth_menu(|| view! { <AppBarUserMenu /> });
//!
//! view! {
//!     <Router>
//!         <Routes fallback=|| view! { /* host 404 */ }>
//!             <UserAppRoutes />
//!             <LeptonAuthRoutes />
//!         </Routes>
//!     </Router>
//! }
//! ```
//!
//! `uf_app!` registers id `orbital-auth` at `/auth/signin` (Apps Product link).
//! Bare `/auth` redirects to that leaf. To refuse new accounts, set
//! `UF_LEPTON_SIGNUP_DISABLED=1` (see lepton `SECURITY.md`).
//!
//! ## Routes
//!
//! | Path | Page |
//! |------|------|
//! | `/auth` | Redirect to [`paths::SIGNIN`] |
//! | `/auth/signin` | [`SigninPage`] |
//! | `/auth/signup` | [`SignupPage`] |
//! | `/auth/logout` | [`LogoutPage`] |
//! | `/auth/oauth/callback` | [`OAuthCallbackPage`] |
//! | `/auth/reset/request` | [`PasswordResetRequestPage`] |
//! | `/auth/reset/confirm` | [`PasswordResetConfirmPage`] |
//!
//! ## Examples
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Getting started snippet above |
//! | Mid | `examples/lepton-mount-host` (path/auth/inventory smoke; no Leptos UI) |
//! | Detailed | workspace `lepton-uf-app-e2e` (real mount + Playwright); kit for deferred auth matrices |

use uf_product_macros::uf_app;

/// Page components for the `/auth/*` routes (signin, signup, logout, OAuth callback, password reset).
pub mod pages;
/// Nested `/auth` route wiring; see [`LeptonAuthRoutes`].
pub mod routes;

pub use pages::{
    LogoutPage, OAuthCallbackPage, PasswordResetConfirmPage, PasswordResetRequestPage, SigninPage,
    SignupPage,
};
pub use routes::paths;
pub use routes::LeptonAuthRoutes;

uf_app! {
    name: "Lepton Auth",
    id: "orbital-auth",
    description: "Authentication routes for Unified Field hosts (lepton/higgs)",
    icon: "🔐",
    version: "0.1.0",
    routes: LeptonAuthRoutes,
    route_path: "/auth/signin",
}
