#![recursion_limit = "256"]
#![allow(missing_docs)] // uf_app! / routes macros emit undocumented associated items
#![allow(clippy::too_long_first_doc_paragraph)]
#![deny(clippy::missing_errors_doc)]
//! `/auth` authentication routes and pages for Unified Field hosts.
//!
//! Registers sign-in, sign-up, logout, password reset, and OAuth callback pages under
//! `/auth` via [`LeptonAuthRoutes`]. Identity and session logic live in **lepton-auth** /
//! **higgs**; delivery adapters and OAuth client config are host concerns. Pair with
//! `lepton_shell::AppBarUserMenu` and `lepton_app::UserAppRoutes`. Workspace mount
//! recipe: crate README at the `lepton-uf-app` root.
//!
//! ## Concern → API
//!
//! | Concern | Page | Kit |
//! |---------|------|-----|
//! | Register `/auth` routes | [`LeptonAuthRoutes`] | `uf_app!` id `orbital-auth` |
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
//! ## Features
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `oauth-google` | Google OAuth sign-in buttons and callback wiring via `lepton-auth` / `lepton-auth-ui`. |
//! | `oauth-github` | GitHub OAuth sign-in buttons and callback wiring via `lepton-auth` / `lepton-auth-ui`. |
//! | `hydrate` / `ssr` | Standard Leptos client/server split for this crate and its uf-product / lepton-auth deps. |
//!
//! ## Route hosts
//!
//! Sign-in, sign-up, logout, and password-reset pages mount through eager
//! [`pages::AuthRouteHost`] and [`pages::PasswordResetRouteHost`] wrappers (not WASM
//! lazy routes like [`lepton_app::UserAppRoutes`]).
//!
//! [`pages::AuthRouteHost`] freezes the first specific `referer` query value with
//! [`lepton_shell::retain_frozen_post_auth_referer`] so remounts onto `/auth/signin`
//! cannot drop the gated path. On success it calls `trigger_refresh` so `/user/*`
//! gates see the authenticated session instead of the anonymous gate.
//!
//! [`pages::PasswordResetRouteHost`] sanitizes referer paths from the query string
//! (defaulting to sign-in when the sanitized path is `/`) and navigates back on close.
//! It does not freeze referers; each navigation re-reads `location.search`.
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
//! | Mid | [`examples/lepton-mount-host`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/examples/lepton-mount-host) (public `/auth` inventory smoke; Axum oneshot, no auth page mount) |
//! | Detailed | workspace [`lepton-uf-app-e2e`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/lepton-uf-app-e2e) (eager auth pages + Playwright); kit for deferred auth matrices |

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
    version: "0.2.0",
    routes: LeptonAuthRoutes,
    route_path: "/auth/signin",
}
