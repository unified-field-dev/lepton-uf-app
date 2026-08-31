//! Shared page chrome for the `/auth/*` routes.

use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use uf_integrations::{ShellAppBar, ShellAuthMenu, UnifiedFieldAppBar, UnifiedFieldShellLayout};

/// Auth route shell: full Unified Field app bar on a blank canvas.
#[component]
pub fn AuthPageShell(
    #[prop(default = true)] chrome_interactive: bool,
    children: Children,
) -> impl IntoView {
    let (style_sheet, class_names) = turf::inline_style_sheet_values! {
        .Canvas {
            width: 100%;
            min-height: calc(100vh - 48px);
            box-sizing: border-box;
        }
    };

    view! {
        <style>{style_sheet}</style>
        <div data-testid="auth-page-shell-root">
            <UnifiedFieldShellLayout>
                <ShellAppBar slot>
                    <UnifiedFieldAppBar
                        app_name="Unified Field".to_string()
                        app_logo_initial="U".to_string()
                        homepage_url="/".to_string()
                        interactive=chrome_interactive
                    >
                        <ShellAuthMenu slot:auth_menu>
                            <AppBarUserMenu />
                        </ShellAuthMenu>
                    </UnifiedFieldAppBar>
                </ShellAppBar>
                <div class=class_names.canvas>
                    {children()}
                </div>
            </UnifiedFieldShellLayout>
        </div>
    }
}
