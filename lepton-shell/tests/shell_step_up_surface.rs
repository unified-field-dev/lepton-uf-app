//! Shell mounts AuthDialog + StepUpDialog and app-bar menu contracts.

#![allow(clippy::expect_used)]

use std::fs;
use std::path::PathBuf;

fn lib_src() -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs"))
        .expect("lepton-shell src/lib.rs")
}

fn shell_ui_src() -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let lib = fs::read_to_string(root.join("lib.rs")).expect("lepton-shell src/lib.rs");
    let menu = fs::read_to_string(root.join("session_menu_items.rs"))
        .expect("lepton-shell src/session_menu_items.rs");
    format!("{lib}\n{menu}")
}

#[test]
fn shell_mounts_auth_and_step_up_dialogs_happy_path() {
    let src = lib_src();
    assert!(src.contains("AuthDialog"), "shell must mount AuthDialog");
    assert!(
        src.contains("StepUpDialog"),
        "shell must mount StepUpDialog"
    );
    assert!(
        src.contains("provide_step_up_controller") || src.contains("StepUpController"),
        "shell must provide or reuse StepUpController"
    );
}

#[test]
fn shell_step_up_docs_host_consumers_happy_path() {
    let src = lib_src();
    assert!(
        src.contains("StepUpController") && src.contains("request"),
        "crate docs should say hosts call StepUpController::request"
    );
    assert!(
        src.contains("does not drive step-up"),
        "docs should clarify Account Settings does not drive step-up today"
    );
}

#[test]
fn shell_menu_session_branches_and_paths_happy_path() {
    let src = shell_ui_src();
    for id in [
        "user-avatar",
        "user-menu-profile",
        "user-menu-account-settings",
        "user-menu-logout",
        "user-menu-signin",
        "user-menu-signup",
    ] {
        assert!(
            src.contains(id),
            "AppBarUserMenu missing kit-aligned testid `{id}`"
        );
    }
    for path in ["USER_PROFILE", "USER_ACCOUNT_SETTINGS", "USER_APPEARANCE"] {
        assert!(
            src.contains(path),
            "AppBarUserMenu must navigate via `{path}`"
        );
    }
    assert!(
        src.contains("AuthSession::Authenticated") && src.contains("AuthSession::Anonymous"),
        "menu must branch authenticated vs anonymous sessions"
    );
    assert!(
        src.contains("sanitize_post_auth_navigate_path"),
        "AuthDialog referer must go through sanitize_post_auth_navigate_path"
    );
    assert!(
        src.contains("post_auth_referer"),
        "AuthDialog referer must snapshot the gated path when the dialog opens"
    );
    assert!(
        src.contains("provide_auth_dialog_controller"),
        "AppBarUserMenu must provide AuthDialogController when shell context is missing"
    );
    assert!(
        !src.contains("unwrap_or_default()"),
        "orphan Default AuthDialogController disconnects RequireAuthenticated from AuthDialog"
    );
}

#[test]
fn shell_menu_drops_anon_branch_sad_path() {
    let src = shell_ui_src();
    assert!(
        src.contains("user-menu-signin") && src.contains("user-menu-signup"),
        "dropping anon sign-in/up leaves guests without an auth entry"
    );
    assert!(
        src.contains("open_signin") && src.contains("open_signup"),
        "anon menu items must open AuthDialog intents"
    );
    assert!(
        !src.contains("unimplemented!"),
        "shell menu must not ship unimplemented placeholders"
    );
}
