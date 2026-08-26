//! Post-auth navigate-path hardening for Lepton UI hosts.
//!
//! Wraps [`lepton_auth::routes::sanitize_referer_path`] and rejects additional
//! open-redirect bypasses (control characters, `://` smuggling) that some
//! browsers treat as off-origin navigations.

use lepton_auth::routes::sanitize_referer_path;

/// Sanitize a post-auth redirect / AuthDialog referer for in-app navigation.
///
/// See the crate guide [Sanitize post-auth navigate path](index.html#sanitize-post-auth-navigate-path).
/// Invalid values fall back to `"/"`.
#[must_use]
pub fn sanitize_post_auth_navigate_path(referer: Option<String>) -> String {
    let sanitized = sanitize_referer_path(referer);
    if is_extra_safe_in_app_path(&sanitized) {
        sanitized
    } else {
        "/".to_string()
    }
}

/// Keep a captured in-app path when a later read sanitizes to `"/"`.
///
/// See the crate guide [Freeze post-auth referer](index.html#freeze-post-auth-referer).
/// Split WASM remounts the hidden `referer` field, and the live URL can become
/// `/auth/signin` (rejected by sanitize) after the user started on a gated
/// product route such as `/tag`.
#[must_use]
pub fn retain_frozen_post_auth_referer(frozen: &str, incoming: &str) -> String {
    if frozen != "/" && !frozen.is_empty() {
        frozen.to_string()
    } else {
        sanitize_post_auth_navigate_path(Some(incoming.to_string()))
    }
}

/// Extra checks beyond lepton-auth: ASCII controls/whitespace and `://` smuggling.
fn is_extra_safe_in_app_path(path: &str) -> bool {
    if !path.starts_with('/') || path.starts_with("//") {
        return false;
    }
    if path.contains('\\') {
        return false;
    }
    if path.bytes().any(|b| b <= 0x20 || b == 0x7f) {
        return false;
    }
    if path.contains("://") {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::{
        is_extra_safe_in_app_path, retain_frozen_post_auth_referer,
        sanitize_post_auth_navigate_path,
    };

    #[test]
    fn sanitize_keeps_safe_in_app_path_happy() {
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/user/profile".to_string())),
            "/user/profile"
        );
    }

    #[test]
    fn sanitize_rejects_protocol_relative_sad() {
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("//evil.example".to_string())),
            "/"
        );
    }

    #[test]
    fn sanitize_rejects_backslash_control_and_url_smuggle_sad() {
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/\\evil.example".to_string())),
            "/"
        );
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/\tevil.example".to_string())),
            "/"
        );
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/https://evil.example".to_string())),
            "/"
        );
        // Embedded whitespace (not only leading/trailing trim edges).
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/user/\x0bprofile".to_string())),
            "/"
        );
    }

    #[test]
    fn extra_safe_allows_query_on_path_happy() {
        assert!(is_extra_safe_in_app_path("/counter?tab=1"));
    }

    #[test]
    fn sanitize_keeps_gated_product_paths_happy() {
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/tag".to_string())),
            "/tag"
        );
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/tag/".to_string())),
            "/tag/"
        );
        assert_eq!(
            sanitize_post_auth_navigate_path(Some("/gate/auth-required".to_string())),
            "/gate/auth-required"
        );
    }

    #[test]
    fn retain_frozen_keeps_gated_path_when_live_url_becomes_signin_sad() {
        assert_eq!(retain_frozen_post_auth_referer("/tag/", "/"), "/tag/");
        assert_eq!(
            retain_frozen_post_auth_referer("/tag", "/auth/signin"),
            "/tag"
        );
        assert_eq!(
            retain_frozen_post_auth_referer("/", "/gate/auth-required"),
            "/gate/auth-required"
        );
        assert_eq!(retain_frozen_post_auth_referer("/", "/auth/signin"), "/");
    }
}
