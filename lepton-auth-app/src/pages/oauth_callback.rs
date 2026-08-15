//! `/auth/oauth/callback` page.

use lepton_auth::routes::parse_referer_from_search;
use lepton_auth_ui::{AuthModalShell, OAuthCallbackContent};
use lepton_shell::sanitize_post_auth_navigate_path;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::pages::AuthPageShell;

fn query_param(search: &str, key: &str) -> String {
    let trimmed = search.trim_start_matches('?');
    url::form_urlencoded::parse(trimmed.as_bytes())
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.into_owned())
        .unwrap_or_default()
}

/// Hosts the OAuth callback dialog and completes the provider handoff.
#[component]
pub fn OAuthCallbackPage() -> impl IntoView {
    let location = use_location();
    let open = RwSignal::new(true);
    let title = Signal::derive(|| "Sign in".to_string());

    let provider = Memo::new(move |_| query_param(&location.search.get(), "provider"));
    let code = Memo::new(move |_| query_param(&location.search.get(), "code"));
    let state = Memo::new(move |_| query_param(&location.search.get(), "state"));
    let referer = Memo::new(move |_| {
        sanitize_post_auth_navigate_path(parse_referer_from_search(&location.search.get()))
    });

    view! {
        <AuthPageShell chrome_interactive=false>
            <div data-testid="oauth-callback-container">
                <AuthModalShell open=open.into() title=title>
                    <OAuthCallbackContent
                        provider=provider.into()
                        code=code.into()
                        state=state.into()
                        referer=referer.into()
                    />
                </AuthModalShell>
            </div>
        </AuthPageShell>
    }
}
