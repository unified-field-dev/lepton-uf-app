//! App-bar authentication menu, auth dialogs, and step-up for Unified Field hosts.
//!
//! [`AppBarUserMenu`] renders the signed-in/anonymous avatar menu (profile, account
//! settings, sign-in/up, logout), hosts `lepton_auth_ui::AuthDialog`, and mounts
//! `lepton_auth_ui::StepUpDialog` with a `lepton_auth_ui::StepUpController`.
//! Reads `uf_product::use_auth_context` for session state and an optional
//! `uf_product::AuthDialogController` from context so route gates elsewhere in the host
//! can open the same dialog.
//!
//! Call `uf_integrations::provide_shell_auth_menu` with [`AppBarUserMenu`], then mount
//! `lepton_app::UserAppRoutes` and `lepton_auth_app::LeptonAuthRoutes` under the host
//! `Router`. Session and identity backends live in **lepton-auth** / **higgs**. Workspace
//! overview: crate README at the `lepton-uf-app` root.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | App-bar auth menu + `AuthDialog` | [`AppBarUserMenu`] |
//! | Open sign-in / sign-up from a route gate | `uf_product::AuthDialogController` from `UnifiedFieldShellLayout` (same dialog as the menu — do not invent a second controller) |
//! | Step-up before a sensitive mutation | `lepton_auth_ui::StepUpController` (dialog mounted via [`AppBarUserMenu`]) |
//! | Post-auth navigate hardening | [`sanitize_post_auth_navigate_path`], [`retain_frozen_post_auth_referer`] |
//! | Compact app-bar extras | `uf_product::use_app_bar_menu_extras` / `AppBarCompactMenuExtras` |
//! | Session / dialog control | `uf_product::use_auth_context` / `AuthDialogController` |
//!
//! No crate-local server functions or typed error enums: identity errors stay in
//! **lepton-auth**; this crate is UI chrome only.
//!
//! ## Step-up
//!
//! [`AppBarUserMenu`] mounts `StepUpDialog` and provides `StepUpController` so hosts
//! that call `request` before a sensitive mutation get a working dialog. Account
//! Settings does not drive step-up today.
//!
//! ## Getting started
//!
//! ```rust,ignore
//! use lepton_shell::AppBarUserMenu;
//! use uf_integrations::provide_shell_auth_menu;
//!
//! provide_shell_auth_menu(|| view! { <AppBarUserMenu /> });
//! // Pair with <UserAppRoutes /> and <LeptonAuthRoutes /> under the host Router.
//! ```
//!
//! Pair with `lepton_app::UserAppRoutes` and `lepton_auth_app::LeptonAuthRoutes`.
//!
//! ## Examples
//!
//! | Level | Where |
//! |-------|--------|
//! | Highlight | Getting started snippet above |
//! | Mid | [`examples/lepton-mount-host`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/examples/lepton-mount-host) (path/auth/inventory smoke; Axum oneshot, no Leptos UI or menu mount) |
//! | Detailed | workspace [`lepton-uf-app-e2e`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/lepton-uf-app-e2e) (real mount of all three crates + Playwright) |
//!
//! Surface contracts: `tests/product_surface.rs`, `tests/shell_step_up_surface.rs`.

use lepton_auth::paths::{USER_ACCOUNT_SETTINGS, USER_PROFILE};
use lepton_auth_ui::{
    provide_step_up_controller, AuthDialog, AuthDialogCallbacks, AuthDialogKind, StepUpDialog,
};
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};
use orbital_base_components::initials_from_name;
use orbital_primitives::*;
use uf_product::provide_auth_dialog_controller;
use uf_product::use_app_bar_menu_extras;
use uf_product::use_auth_context;
use uf_product::AppBarCompactMenuExtras;
use uf_product::AuthDialogController;
use uf_product::AuthDialogIntent;

mod navigate_path;
mod session_menu_items;
use session_menu_items::SessionMenuItems;

pub use navigate_path::{retain_frozen_post_auth_referer, sanitize_post_auth_navigate_path};

/// Signed-in/anonymous avatar menu for the host app bar.
///
/// Call from `uf_integrations::provide_shell_auth_menu` (preferred) or nest under
/// `ShellAuthMenu`. Pair with `lepton_app::UserAppRoutes` and
/// `lepton_auth_app::LeptonAuthRoutes`. Requires `uf_product` auth context.
///
/// Bind the [`AuthDialogController`] from `UnifiedFieldShellLayout` when present so
/// [`uf_product::routes::RequireAuthenticated`] opens this same dialog. If no
/// controller is in context, this menu provides one for its own `AuthDialog` —
/// never a disconnected `Default` handle that leaves the gate talking to a
/// different signal.
///
/// # Examples
///
/// | Level | Where |
/// |-------|--------|
/// | Highlight | crate-root Getting started snippet |
/// | Mid | [`examples/lepton-mount-host`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/examples/lepton-mount-host) (inventory JSON names `AppBarUserMenu`; does not compile or mount this component) |
/// | Detailed | workspace [`lepton-uf-app-e2e`](https://github.com/unified-field-dev/lepton-uf-app/tree/main/lepton-uf-app-e2e) (`provide_shell_auth_menu` + Playwright avatar menu) |
#[component]
pub fn AppBarUserMenu() -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let auth = use_auth_context();
    let session = auth.session();
    let user_label = Memo::new(move |_| session.with(|session| session.display_label()));
    let current_path = Memo::new(move |_| {
        let mut path = location.pathname.get();
        let search = location.search.get();
        if !search.is_empty() {
            path.push_str(&search);
        }
        path
    });

    // Prefer the shell-provided controller so route gates open this same dialog.
    let controller =
        use_context::<AuthDialogController>().unwrap_or_else(provide_auth_dialog_controller);
    let _step_up = use_context::<lepton_auth_ui::StepUpController>()
        .unwrap_or_else(provide_step_up_controller);
    let auth_dialog_open = controller.open();
    let auth_dialog_intent = controller.intent();
    let auth_dialog_kind = Signal::derive(move || match auth_dialog_intent.get() {
        AuthDialogIntent::Signin => AuthDialogKind::Signin,
        AuthDialogIntent::Signup => AuthDialogKind::Signup,
        AuthDialogIntent::Logout => AuthDialogKind::Logout,
    });
    // Snapshot the gated path when the dialog opens. A live current_path
    // Signal follows `/auth/signin` (sanitizes to `/`) if the URL changes
    // under the open form.
    let post_auth_referer = RwSignal::new(String::from("/"));
    Effect::new(move |was_open: Option<bool>| {
        let open = auth_dialog_open.get();
        if open && was_open != Some(true) {
            post_auth_referer.set(sanitize_post_auth_navigate_path(Some(
                current_path.get_untracked(),
            )));
        }
        open
    });
    let referer = Signal::derive(move || post_auth_referer.get());
    let close_auth_dialog = Callback::new(move |_| {
        controller.close();
    });
    let switch_to_signin = Callback::new(move |_| {
        auth_dialog_intent.set(AuthDialogIntent::Signin);
    });
    let switch_to_signup = Callback::new(move |_| {
        auth_dialog_intent.set(AuthDialogIntent::Signup);
    });
    let pending_session_refresh = RwSignal::new(false);
    let refresh_and_close = {
        let auth = auth.clone();
        Callback::new(move |_| {
            auth.trigger_refresh();
            if auth_dialog_intent.get_untracked() == AuthDialogIntent::Logout {
                controller.close();
            } else {
                pending_session_refresh.set(true);
            }
        })
    };
    Effect::new(move |_| {
        if pending_session_refresh.get()
            && matches!(session.get(), uf_product::AuthSession::Authenticated(_))
        {
            pending_session_refresh.set(false);
            controller.close();
        }
    });

    view! {
        <Menu
            on_select=move |key: &str| {
                match key {
                    "profile" => {
                        navigate(USER_PROFILE, Default::default());
                    }
                    "signin" => {
                        controller.open_signin();
                    }
                    "logout" => {
                        controller.open_logout();
                    }
                    "signup" => {
                        controller.open_signup();
                    }
                    "account_settings" => {
                        navigate(USER_ACCOUNT_SETTINGS, Default::default());
                    }
                    "appearance_settings" => {
                        navigate(uf_product::paths::USER_APPEARANCE, Default::default());
                    }
                    _ => {}
                }
            }
        >
            <MenuTrigger slot>
                <div data-testid="user-avatar">
                    {move || {
                        let label = user_label.get();
                        view! {
                            <Avatar config=AvatarConfig {
                                initials: Some(initials_from_name(&label)),
                                name: Some(label),
                                shape: AvatarShape::Circular,
                                size: Some(32),
                                color: AvatarColor::Brand,
                                ..Default::default()
                            } />
                        }
                    }}
                </div>
            </MenuTrigger>
            {move || match use_app_bar_menu_extras() {
                // Build extras under this Menu's owner so MenuItem finds MenuInjection.
                Some(extras) if extras.show.get() => {
                    view! { <AppBarCompactMenuExtras /> }.into_any()
                }
                _ => ().into_any(),
            }}
            <SessionMenuItems session=session />
        </Menu>
        <AuthDialog
            open=auth_dialog_open.into()
            kind=auth_dialog_kind
            referer=referer
            callbacks=AuthDialogCallbacks {
                on_success: Some(refresh_and_close),
                on_close: Some(close_auth_dialog),
                on_switch_signin: Some(switch_to_signin),
                on_switch_signup: Some(switch_to_signup),
            }
        />
        <StepUpDialog/>
    }
}
