use leptos::prelude::*;
use uf_product::primitives::{Avatar, AvatarConfig};

#[component]
pub fn ProfilePhotoDisplay(
    #[prop(into)] photo_url: MaybeProp<String>,
    #[prop(into)] display_name: Signal<String>,
) -> impl IntoView {
    view! {
        <div data-testid="profile-photo-display">
            <Avatar config=AvatarConfig {
                src: photo_url.get(),
                name: Some(display_name.get()),
                size: Some(96),
                ..Default::default()
            } />
        </div>
    }
}
