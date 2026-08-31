# lepton-mount-host

Axum oneshot under **`/user`** (session-gated) and **`/auth`** (public): inventory
ids match `lepton-app` / `lepton-auth-app` `uf_app!` registrations, and the JSON
names the shell slot (`AppBarUserMenu`).

This binary is a **path / auth / inventory contract smoke**. It does not depend on
`lepton-shell`, `lepton-app`, or `lepton-auth-app`, and it does not run Leptos
SSR/WASM. Production hosts mount those crates; copy the Leptos sketch below (or
open [`lepton-uf-app-e2e/src/app.rs`](../../lepton-uf-app-e2e/src/app.rs)).

| | |
|---|---|
| **When to use** | First smoke of path + auth split + inventory ids without compiling the UI graph |
| **Command** | `CARGO_BUILD_JOBS=1 CARGO_TARGET_DIR=target-lepton-uf-app cargo run -p lepton-mount-host` |
| **Success** | Stdout: `lepton_mount_host: OK — /user protect + /auth public + inventory` |
| **Look next** | Mount the three crates (`provide_shell_auth_menu` + route children); boot `provide_auth_services` |

**Open first:** [`src/main.rs`](src/main.rs)

## Copy into your host

| File | What to take |
|------|----------------|
| This [`Cargo.toml`](Cargo.toml) | Axum oneshot shape + `uf-product` `ssr` for inventory smoke only |
| Product mount `Cargo.toml` (below) | The three lepton crates with `ssr` / `hydrate` features |
| [`src/main.rs`](src/main.rs) | Auth split idea: protect `/user`, keep `/auth` public; inventory ids |
| Leptos sketch (below) | Route children + `provide_shell_auth_menu` |

### Product mount dependencies

```toml
[dependencies]
lepton-shell = { git = "https://github.com/unified-field-dev/lepton-uf-app", package = "lepton-shell", rev = "REPLACE_WITH_PIN", default-features = false }
lepton-app = { git = "https://github.com/unified-field-dev/lepton-uf-app", package = "lepton-app", rev = "REPLACE_WITH_PIN", default-features = false }
lepton-auth-app = { git = "https://github.com/unified-field-dev/lepton-uf-app", package = "lepton-auth-app", rev = "REPLACE_WITH_PIN", default-features = false }
uf-product = { /* your pin */, default-features = false }
uf-integrations = { /* your pin */, default-features = false }

[features]
ssr = [
    "lepton-shell/ssr",
    "lepton-app/ssr",
    "lepton-auth-app/ssr",
    "uf-product/ssr",
    "uf-integrations/ssr",
]
hydrate = [
    "lepton-shell/hydrate",
    "lepton-app/hydrate",
    "lepton-auth-app/hydrate",
    "uf-product/hydrate",
    "uf-integrations/hydrate",
]
```

### Leptos mount sketch

```rust,ignore
use lepton_app::UserAppRoutes;
use lepton_auth_app::LeptonAuthRoutes;
use lepton_shell::AppBarUserMenu;
use leptos::prelude::*;
use leptos_router::components::{Router, Routes};
use uf_integrations::provide_shell_auth_menu;

provide_shell_auth_menu(|| view! { <AppBarUserMenu /> });

view! {
    <Router>
        <Routes fallback=|| view! { /* host 404 */ }>
            <UserAppRoutes />
            <LeptonAuthRoutes />
        </Routes>
    </Router>
}
// SSR boot: lepton_auth::services::provide_auth_services(...);
```

For shell chrome (layout, fonts, Axum + Leptos boot), copy
[`shell-chrome-host`](https://github.com/unified-field-dev/unified-field-product/tree/main/examples/shell-chrome-host)
from unified-field-product, then replace the Demo auth menu with
`provide_shell_auth_menu(|| view! { <AppBarUserMenu /> })`.

## Run (documented gate)

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-lepton-uf-app
cargo check -p lepton-mount-host
cargo run -p lepton-mount-host
```

**Success:** stdout prints `lepton_mount_host: OK — /user protect + /auth public + inventory`.

## Hydrate / browser

Out of gate for this host. Full UI in this workspace: [`lepton-uf-app-e2e`](../../lepton-uf-app-e2e/).
Domain matrices deferred to the lepton kit stay listed in that README.
