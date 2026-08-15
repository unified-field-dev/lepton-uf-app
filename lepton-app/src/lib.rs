#![recursion_limit = "256"]
#![allow(missing_docs)] // uf_app! / routes macros emit undocumented associated items
#![allow(clippy::too_long_first_doc_paragraph)]
//! User settings app routes and UI composition (lepton/higgs; Orbital is UI-only).
//!
//! Mount via the host `uf-product` registry under `/user`. Pair with
//! `lepton_shell::AppBarUserMenu` and `lepton_auth_app::LeptonAuthRoutes`.
//! Workspace overview: crate README at the `lepton-uf-app` root.
//!
//! **Owns:** `/user` pages (profile, appearance, account settings, confirm-account).
//! **Does not own:** session/backends (`lepton-auth` / higgs), `/auth` routes
//! (`lepton-auth-app`), or app-bar chrome (`lepton-shell`).
//!
//! ## Concern → API
//!
//! | Concern | Start here | Server / kit |
//! |---------|------------|--------------|
//! | Profile / photo | [`ProfilePage`] | [`get_my_profile`] / [`update_my_profile`] → [`ProfileData`] |
//! | Appearance | [`AppearancePage`] | `uf-product` / `orbital-theme` |
//! | Account settings shell | [`AccountSettingsPage`] | `lepton_auth::actions::account` |
//! | TOTP enroll / disable | [`TotpSettingsSection`] | `lepton_auth::actions::totp` |
//! | OAuth link / unlink | [`ConnectedAccountsSection`] | `lepton_auth::actions::oauth_settings` |
//! | Devices / passkeys | [`SecurityDevicesSection`] | `lepton_auth::actions::devices` |
//! | Owner wipe | `wipe_section` (private; composed by [`AccountSettingsPage`]) | `lepton_auth::actions::account::WipeAccount` |
//! | Soft confirm funnel | [`ConfirmAccountPage`] | `lepton_auth_ui::ConfirmAccountPage` |
//! | `/user/*` path constants | [`paths`] | generated beside [`UserAppRoutes`] |
//!
//! Profile and account mutations return `ServerFnError` at the Leptos boundary.
//!
//! ## Organized by task
//!
//! | Task | Start here |
//! |------|------------|
//! | Register `/user` routes | [`UserAppRoutes`] (`uf_app!` id `lepton-app`) |
//! | Navigate to a `/user` leaf | [`paths`] (`PROFILE`, `APPEARANCE`, `ACCOUNT_SETTINGS`, and siblings) |
//! | Profile / photo | [`ProfilePage`] / [`ProfileData`] |
//! | Appearance preferences | [`AppearancePage`] |
//! | Account settings (TOTP, OAuth, devices, wipe) | [`AccountSettingsPage`] |
//! | Soft confirm funnel | [`ConfirmAccountPage`] |
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
//! `uf_app!` already registers id `lepton-app` at `/user/account-settings`. Enable `lepton-auth/totp`
//! on the SSR graph for authenticator UI (included in this crate's `ssr` feature).
//! OAuth link CTAs need `oauth-google` / `oauth-github` on this crate (and matching
//! hydrate features).
//!
//! ## Pages
//!
//! | Path | Page | Module (private) |
//! |------|------|-------------------|
//! | `/user` | Redirect to [`paths::ACCOUNT_SETTINGS`] | — |
//! | `/user/profile` | [`ProfilePage`] | `profile` |
//! | `/user/appearance` | [`AppearancePage`] | `appearance` (+ `appearance_preview` gallery) |
//! | `/user/account-settings` | [`AccountSettingsPage`] | `account_settings` (+ `connected_accounts_section`, `devices_section`, `totp_section`, `wipe_section`) |
//! | `/user/confirm-account` | [`ConfirmAccountPage`] | `confirm_account` |
//!
//! [`UserLayout`] wraps these under the app-bar/left-nav shell; WASM code-split lazy
//! views ([`AccountSettingsRoute`], [`AppearanceRoute`], [`ProfileRoute`],
//! [`ConfirmAccountRoute`]) live in the private `lazy_routes` module.
//!
//! ## Examples
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Getting started snippet above |
//! | Mid | `examples/lepton-mount-host` (path/auth/inventory smoke; no Leptos UI) |
//! | Detailed | workspace `lepton-uf-app-e2e` (real mount + Playwright); kit for deferred domain matrices |

use leptos::prelude::*;
use leptos_router::{components::*, path, Lazy};
use uf_product_macros::uf_app;

mod account_settings;
mod appearance;
mod appearance_preview;
mod confirm_account;
mod connected_accounts_section;
mod devices_section;
mod layout;
mod lazy_routes;
mod profile;
mod profile_photo_display;
mod profile_photo_upload;
mod totp_section;
mod wipe_section;

pub use account_settings::AccountSettingsPage;
pub use appearance::AppearancePage;
pub use confirm_account::ConfirmAccountPage;
pub use connected_accounts_section::ConnectedAccountsSection;
pub use devices_section::SecurityDevicesSection;
pub use layout::UserLayout;
pub use lazy_routes::{
    prefetch_family, AccountSettingsRoute, AppearanceRoute, ConfirmAccountRoute, ProfileRoute,
};
pub use profile::{
    get_my_profile, update_my_profile, validate_display_name, ProfileData, ProfilePage,
    MAX_DISPLAY_NAME_CHARS,
};
pub use totp_section::TotpSettingsSection;

uf_app! {
    name: "User Settings",
    id: "lepton-app",
    description: "User profile and account settings",
    icon: "👤",
    version: "0.1.0",
    routes: UserAppRoutes,
    route_path: "/user/account-settings",
}

/// Nested `/user` routes for a Unified Field host `Router` / `Routes` tree.
///
/// Drop in as a route child alongside `lepton_auth_app::LeptonAuthRoutes`. Pair
/// with `lepton_shell::AppBarUserMenu` via `uf_integrations::provide_shell_auth_menu`.
/// Leaf path constants live in [`paths`] (`PROFILE`, `APPEARANCE`,
/// `ACCOUNT_SETTINGS`, `CONFIRM_ACCOUNT`).
#[orbital_macros::orbital_routes_extract]
#[component(transparent)]
pub fn UserAppRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("user") view=UserLayout>
            <Route path=path!("") view=|| view! { <Redirect path=paths::ACCOUNT_SETTINGS /> } />
            <Route path=path!("profile") view={Lazy::<ProfileRoute>::new()} />
            <Route path=path!("appearance") view={Lazy::<AppearanceRoute>::new()} />
            <Route path=path!("account-settings") view={Lazy::<AccountSettingsRoute>::new()} />
            <Route path=path!("confirm-account") view={Lazy::<ConfirmAccountRoute>::new()} />
        </ParentRoute>
    }
    .into_inner()
}
