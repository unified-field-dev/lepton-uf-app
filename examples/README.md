# Examples

| Host | When to use | Command | Success | Look next |
|------|-------------|---------|---------|-----------|
| [`lepton-mount-host`](lepton-mount-host/) | Path/auth/inventory **contract smoke** (no Leptos UI) | `cargo run -p lepton-mount-host` | Deny/allow + OK line | Product mount sketch in host README, or `lepton-uf-app-e2e` |

## `lepton-mount-host`

**Teaches:** session gate on `/user`, public `/auth`, matching `uf_app!` inventory
ids (`lepton-app`, `orbital-auth`), and the shell auth-menu slot name
(`AppBarUserMenu`). Does **not** compile or mount the three product crates.

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-lepton-uf-app
cargo run -p lepton-mount-host
```

**Success:** stdout prints `lepton_mount_host: OK — /user protect + /auth public + inventory`.

**Next step:** Copy the Leptos sketch from the host README (`provide_shell_auth_menu`
+ `<UserAppRoutes />` / `<LeptonAuthRoutes />`), or run the in-repo product host
[`lepton-uf-app-e2e`](../lepton-uf-app-e2e/).
