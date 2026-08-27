//! `/user/confirm-account` product page (Orbital / uf_product shell).

use lepton_auth_ui::ConfirmAccountPage as ConfirmAccountFunnel;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use uf_product::components::ContentContainer;
use uf_product::use_auth_state;

/// Guided account confirm funnel under the user settings shell.
#[component]
pub fn ConfirmAccountPage() -> impl IntoView {
    let auth_state = use_auth_state();
    let navigate = use_navigate();
    Effect::new(move |_| {
        if !auth_state.with(|s| s.is_authenticated()) {
            navigate(lepton_auth::paths::SIGNIN, Default::default());
        }
    });

    view! {
        <ContentContainer max_width="900px">
            <ConfirmAccountFunnel />
        </ContentContainer>
    }
}
