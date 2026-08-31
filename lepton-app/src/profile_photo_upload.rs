use leptos::prelude::*;
use orbital_base_components::{Handler, UploadFileList};
use uf_product::components::Caption1;
use uf_product::primitives::*;

const MAX_FILE_SIZE: f64 = 5.0 * 1024.0 * 1024.0; // 5 MB
const ALLOWED_EXTENSIONS: &[&str] = &["png", "jpeg", "jpg", "gif", "webp"];

/// Profile photo upload control.
///
/// Posts multipart form data to host `/api/files/upload`, including a client
/// `profile_id` field for convenience. That field is **not** authorization: the
/// host must bind the blob to the session user's profile (verify ownership or
/// ignore the client id and derive it server-side).
#[component]
pub fn ProfilePhotoUpload(
    #[prop(into)] profile_id: Signal<String>,
    #[prop(into)] on_upload: Callback<()>,
) -> impl IntoView {
    let (uploading, set_uploading) = signal(false);
    let (error_msg, set_error_msg) = signal(Option::<String>::None);
    let (success_msg, set_success_msg) = signal(Option::<String>::None);

    let on_file_selected = Handler::on({
        move |files: UploadFileList| {
            set_error_msg.set(None);
            set_success_msg.set(None);

            // web-sys 0.3.103 has no `FileList::first`; `get(0)` is the DOM API.
            let Some(file) = files.get(0) else {
                return;
            };

            if file.size() > MAX_FILE_SIZE {
                set_error_msg.set(Some(format!(
                    "File too large ({:.1} MB). Maximum is 5 MB.",
                    file.size() / 1024.0 / 1024.0
                )));
                return;
            }

            let name = file.name();
            let extension = name.rsplit('.').next().unwrap_or("").to_lowercase();
            if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
                set_error_msg.set(Some(format!(
                    "File type '.{}' not allowed. Use: {}",
                    extension,
                    ALLOWED_EXTENSIONS.join(", ")
                )));
                return;
            }

            let pid = profile_id.get_untracked();
            set_uploading.set(true);

            leptos::task::spawn_local_scoped(async move {
                match upload_file(file, &pid).await {
                    Ok(()) => {
                        set_success_msg.set(Some("Photo uploaded successfully.".to_string()));
                        on_upload.run(());
                    }
                    Err(e) => {
                        set_error_msg.set(Some(e));
                    }
                }
                set_uploading.set(false);
            });
        }
    });

    view! {
        <div data-testid="profile-photo-upload">
        <Flex
            vertical=true
            align=FlexAlign::Center
            gap=FlexGap::Small
        >
            <Upload
                config=UploadConfig::accept("image/png,image/jpeg,image/gif,image/webp")
                on_change=on_file_selected
            >
                <Button
                    appearance=ButtonAppearance::Secondary
                    disabled=uploading
                    icon=icondata::AiCameraOutlined
                >
                    {move || if uploading.get() { "Uploading..." } else { "Upload Photo" }}
                </Button>
            </Upload>
            <Caption1>"PNG, JPEG, GIF, or WebP. Max 5 MB."</Caption1>
            {move || error_msg.get().map(|msg| view! {
                <MessageBar intent=MessageBarIntent::Error>{msg}</MessageBar>
            })}
            {move || success_msg.get().map(|msg| view! {
                <MessageBar intent=MessageBarIntent::Success>{msg}</MessageBar>
            })}
        </Flex>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
async fn upload_file(file: web_sys::File, profile_id: &str) -> Result<(), String> {
    use wasm_bindgen::JsCast;

    let form_data =
        web_sys::FormData::new().map_err(|_| "Failed to create FormData".to_string())?;
    form_data
        .append_with_blob_and_filename("file", &file, &file.name())
        .map_err(|_| "Failed to append file".to_string())?;
    form_data
        .append_with_str("profile_id", profile_id)
        .map_err(|_| "Failed to append profile_id".to_string())?;

    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&form_data);

    let request = web_sys::Request::new_with_str_and_init("/api/files/upload", &opts)
        .map_err(|_| "Failed to create request".to_string())?;

    let window = leptos::web_sys::window().ok_or_else(|| "No window".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "Network error".to_string())?;

    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Invalid response".to_string())?;

    if !resp.ok() {
        let text = wasm_bindgen_futures::JsFuture::from(
            resp.text()
                .map_err(|_| "Failed to read error body".to_string())?,
        )
        .await
        .map_err(|_| "Failed to read error body".to_string())?;

        let msg = text
            .as_string()
            .unwrap_or_else(|| "Upload failed".to_string());
        return Err(msg);
    }

    Ok(())
}

// Signature (async, owned `File`) must mirror the wasm implementation awaited above.
#[cfg(not(target_arch = "wasm32"))]
async fn upload_file(_file: web_sys::File, _profile_id: &str) -> Result<(), String> {
    Err("Upload only available in browser".to_string())
}
