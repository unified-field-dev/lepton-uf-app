//! Appearance settings page — color mode, brand source, and live theme preview.

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use orbital_theme::{OrbitalThemeProvider, Theme, ThemeInjection};
use uf_product::components::{Caption1, Card, ContentContainer, Subtitle2, Title3};
use uf_product::primitives::*;
use uf_product::services::{get_my_appearance, save_my_appearance};
use uf_product::theme::{
    apply_appearance_preferences, use_appearance_preferences, write_local_appearance,
    AppearancePreferences, PRODUCT_BRAND_PRESETS, UF_SHELL_BRAND_SEED,
};
use uf_product::{use_auth_context, use_auth_state};

use crate::appearance_preview::AppearancePreviewGallery;

const SETTINGS_PRODUCT_BRAND: &str = UF_SHELL_BRAND_SEED;

/// `/user/appearance` page: color mode, brand source, and a live theme preview.
#[component]
pub fn AppearancePage() -> impl IntoView {
    let auth_state = use_auth_state();
    let navigate = use_navigate();
    let auth = use_auth_context();
    let global_prefs = use_appearance_preferences();

    Effect::new(move |_| {
        if !auth_state.with(|s| s.is_authenticated()) {
            navigate(lepton_auth::paths::SIGNIN, Default::default());
        }
    });

    let saved_resource = Resource::new(|| (), |_| get_my_appearance());
    let draft = RwSignal::new(AppearancePreferences::light_product());
    let preview_theme = RwSignal::new(Theme::light());
    let brand_hex = RwSignal::new(String::new());
    let (message, set_message) = signal(None::<String>);

    let color_mode = RwSignal::new(Some("light".to_string()));
    let brand_source = RwSignal::new(Some("product".to_string()));

    Effect::new(move |_| {
        if let Some(Ok(data)) = saved_resource.get() {
            let prefs: AppearancePreferences = data.into();
            draft.set(prefs.clone());
            color_mode.set(Some(prefs.color_mode));
            brand_source.set(Some(prefs.brand_source));
            brand_hex.set(
                prefs
                    .brand_seed_color
                    .unwrap_or_else(|| "#4f6bed".to_string()),
            );
        }
    });

    Effect::new(move |_| {
        if let Some(mode) = color_mode.get() {
            draft.update(|d| d.color_mode = mode);
        }
    });

    Effect::new(move |_| {
        if let Some(source) = brand_source.get() {
            draft.update(|d| d.brand_source = source);
        }
    });

    Effect::new(move |_| {
        let hex = brand_hex.get();
        if !hex.is_empty() {
            draft.update(|d| d.brand_seed_color = Some(hex));
        }
    });

    Effect::new(move |_| {
        let prefs = draft.get();
        apply_appearance_preferences(preview_theme, &prefs, SETTINGS_PRODUCT_BRAND);
    });

    let saving = RwSignal::new(false);

    let on_save = Callback::new(move |_| {
        set_message.set(None);
        let prefs = draft.get_untracked();
        if let Some(ctx) = global_prefs {
            ctx.set(prefs.clone());
            write_global_theme(&prefs);
        }
        saving.set(true);
        let auth = auth.clone();
        leptos::task::spawn_local_scoped(async move {
            match save_my_appearance(prefs.color_mode, prefs.brand_source, prefs.brand_seed_color)
                .await
            {
                Ok(()) => {
                    set_message.set(Some("Appearance preferences saved.".to_string()));
                    saved_resource.refetch();
                    auth.trigger_refresh();
                }
                Err(err) => set_message.set(Some(format!("Save failed: {err}"))),
            }
            saving.set(false);
        });
    });

    view! {
        <ContentContainer max_width="900px" data_testid="appearance-page">
            <Flex vertical=true gap=FlexGap::Large>
                <Title3>"Appearance"</Title3>
                <Caption1>"Customize color mode and brand color. Preview updates live before you save."</Caption1>

                <Show when=move || message.get().is_some()>
                    <MessageBar intent=MessageBarIntent::Info>
                        {move || message.get().unwrap_or_default()}
                    </MessageBar>
                </Show>

                <Card>
                    <Flex vertical=true gap=FlexGap::Medium padding=SpacingInset::all_l()>
                        <Subtitle2>"Color mode"</Subtitle2>
                        <RadioGroup bind=RadioGroupBind::from(color_mode)>
                            <Radio value="light".to_string() label="Light".to_string() />
                            <Radio value="dark".to_string() label="Dark".to_string() />
                        </RadioGroup>

                        <Subtitle2>"Brand color"</Subtitle2>
                        <RadioGroup bind=RadioGroupBind::from(brand_source)>
                            <Radio value="product".to_string() label="Follow product brand".to_string() />
                            <Radio value="custom".to_string() label="Custom brand color".to_string() />
                        </RadioGroup>

                        <Show when=move || brand_source.get() == Some("custom".to_string())>
                            <Flex wrap=FlexWrap::Wrap gap=FlexGap::Small align=FlexAlign::Center>
                                {PRODUCT_BRAND_PRESETS
                                    .iter()
                                    .map(|(name, hex)| {
                                        let hex = (*hex).to_string();
                                        view! {
                                            <div data-testid=format!("appearance-preset-{name}")>
                                                <Button
                                                    appearance=ButtonAppearance::Subtle
                                                    on_click=Callback::new({
                                                        let hex = hex.clone();
                                                        move |_| brand_hex.set(hex.clone())
                                                    })
                                                >
                                                    {*name}
                                                </Button>
                                            </div>
                                        }
                                    })
                                    .collect_view()}
                                <Input
                                    bind=InputBind::new(brand_hex)
                                    appearance=InputAppearance::with_placeholder("#RRGGBB")
                                />
                            </Flex>
                        </Show>

                        <Show when=move || brand_source.get() == Some("product".to_string())>
                            <Caption1>
                                "On product pages, the UI uses that product's brand color. In Settings, the shell brand applies to the preview."
                            </Caption1>
                        </Show>
                    </Flex>
                </Card>

                <Card>
                    <Flex vertical=true gap=FlexGap::Medium padding=SpacingInset::all_l()>
                        <Subtitle2>"Preview"</Subtitle2>
                        <OrbitalThemeProvider theme=preview_theme>
                            <AppearancePreviewGallery />
                        </OrbitalThemeProvider>
                    </Flex>
                </Card>

                <div data-testid="appearance-save">
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=on_save
                        disabled=Signal::derive(move || saving.get())
                    >
                        {move || if saving.get() { "Saving…" } else { "Save preferences" }}
                    </Button>
                </div>
            </Flex>
        </ContentContainer>
    }
}

fn write_global_theme(prefs: &AppearancePreferences) {
    let theme = ThemeInjection::use_rw_theme();
    apply_appearance_preferences(theme, prefs, SETTINGS_PRODUCT_BRAND);
    write_local_appearance(prefs);
}
