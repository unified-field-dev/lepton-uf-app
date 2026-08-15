use lepton_auth::paths::SIGNIN;
use lepton_auth::routes::parse_referer_from_search;
#[cfg(feature = "hydrate")]
use lepton_auth::token_url::{
    read_token_from_window_location, strip_legacy_token_query_from_address_bar,
};
use lepton_auth_ui::{PasswordResetDialog, PasswordResetDialogKind};
use lepton_shell::sanitize_post_auth_navigate_path;
use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

use crate::pages::AuthPageShell;

/// Shared wrapper for `/auth/reset/request` and `/auth/reset/confirm` routes.
#[component]
pub fn PasswordResetRouteHost(
    initial_kind: PasswordResetDialogKind,
    test_id: &'static str,
) -> impl IntoView {
    let navigate = use_navigate();
    let location = use_location();
    let referer = Memo::new(move |_| {
        let search = location.search.get();
        let parsed = sanitize_post_auth_navigate_path(parse_referer_from_search(&search));
        if parsed == "/" {
            SIGNIN.to_string()
        } else {
            parsed
        }
    });

    let token_from_query = Memo::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            read_token_from_window_location()
        }
        #[cfg(not(feature = "hydrate"))]
        {
            let raw = location.search.get();
            let query = raw.trim_start_matches('?');
            url::form_urlencoded::parse(query.as_bytes())
                .find(|(k, _)| k == "token")
                .map(|(_, v)| v.to_string())
                .unwrap_or_default()
        }
    });

    #[cfg(feature = "hydrate")]
    Effect::new(move |_| {
        if !token_from_query.get().is_empty() {
            strip_legacy_token_query_from_address_bar();
        }
    });

    let dialog_open = RwSignal::new(true);
    let dialog_kind = RwSignal::new(initial_kind);

    Effect::new(move |was_open: Option<bool>| {
        let open = dialog_open.get();
        if was_open == Some(true) && !open {
            let target = referer.get();
            navigate(&target, Default::default());
        }
        open
    });

    view! {
        <AuthPageShell chrome_interactive=false>
            <div data-testid=test_id>
                <PasswordResetDialog
                    open=dialog_open.into()
                    kind=dialog_kind.into()
                    token_from_query=token_from_query
                />
            </div>
        </AuthPageShell>
    }
}
