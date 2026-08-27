//! Path/auth/inventory contract smoke for lepton product mounts.
//!
//! Proves the same `/user` protect + `/auth` public split and `uf_app!` inventory
//! ids a host needs before mounting `lepton_app::UserAppRoutes`,
//! `lepton_auth_app::LeptonAuthRoutes`, and `lepton_shell::AppBarUserMenu`.
//!
//! This binary is an Axum oneshot: it does **not** depend on those crates and does
//! not run Leptos SSR/WASM. Copy the product Cargo features and Leptos sketch from
//! the host README, or open workspace `lepton-uf-app-e2e` for a real mount.
//!
//! ## When to use
//! Smoke paths + auth split + inventory without compiling the Leptos UI graph.
//!
//! ## Command
//! ```bash
//! export CARGO_BUILD_JOBS=1
//! export CARGO_TARGET_DIR=target-lepton-uf-app
//! cargo run -p lepton-mount-host
//! ```
//!
//! ## Success
//! Stdout prints `lepton_mount_host: OK — /user protect + /auth public + inventory`.
//!
//! ## Look next
//! `provide_shell_auth_menu(|| view! { <AppBarUserMenu /> })` plus
//! `<UserAppRoutes />` / `<LeptonAuthRoutes />` under the host Router. Boot
//! `provide_auth_services` on SSR.

#![allow(clippy::print_stdout, clippy::unwrap_used, clippy::expect_used)]

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::{from_fn, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use axum::Router;
use http_body_util::BodyExt;
use tower::ServiceExt;
use uf_product::routes::get_all_app_route_paths;
use uf_product::{inventory, AppRegistration};

// Match lepton-app / lepton-auth-app `uf_app!` ids and route_path values.
inventory::submit! {
    AppRegistration {
        id: "lepton-app",
        name: "User Settings",
        description: "User profile and account settings",
        icon: "User",
        route_path: "/user/account-settings",
        repository: Some("https://github.com/unified-field-dev/lepton-uf-app"),
        crate_name: Some("lepton-app"),
        brand_seed: None,
        permission_manifest: None,
    }
}

inventory::submit! {
    AppRegistration {
        id: "orbital-auth",
        name: "Lepton Auth",
        description: "Authentication routes for Unified Field hosts (lepton/higgs)",
        icon: "Lock",
        route_path: "/auth/signin",
        repository: Some("https://github.com/unified-field-dev/lepton-uf-app"),
        crate_name: Some("lepton-auth-app"),
        brand_seed: None,
        permission_manifest: None,
    }
}

#[derive(Clone)]
struct DemoSession {
    user_id: String,
}

async fn require_session(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    if req.extensions().get::<DemoSession>().is_some() {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn inject_demo_session(mut req: Request<Body>, next: Next) -> Response {
    if let Some(user) = req
        .headers()
        .get("x-demo-user")
        .and_then(|v| v.to_str().ok())
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
    {
        req.extensions_mut().insert(DemoSession { user_id: user });
    }
    next.run(req).await
}

async fn user_surface(Extension(session): Extension<DemoSession>) -> impl IntoResponse {
    let paths = get_all_app_route_paths();
    Json(serde_json::json!({
        "path": "/user",
        "user": session.user_id,
        "apps": paths,
        "shell_auth_menu": "AppBarUserMenu",
    }))
}

async fn auth_surface() -> impl IntoResponse {
    Json(serde_json::json!({
        "path": "/auth",
        "public": true,
        "routes": [
            "/auth/signin",
            "/auth/signup",
            "/auth/logout",
            "/auth/oauth/callback",
            "/auth/reset/request",
            "/auth/reset/confirm",
        ],
    }))
}

fn app() -> Router {
    Router::new()
        .route("/user", get(user_surface))
        .route_layer(from_fn(require_session))
        .route("/auth", get(auth_surface))
        .layer(from_fn(inject_demo_session))
}

async fn status_for(path: &str, user: Option<&str>) -> StatusCode {
    let mut builder = Request::builder().uri(path);
    if let Some(user) = user {
        builder = builder.header("x-demo-user", user);
    }
    app()
        .oneshot(builder.body(Body::empty()).expect("req"))
        .await
        .expect("oneshot")
        .status()
}

#[tokio::main]
async fn main() {
    let paths = get_all_app_route_paths();
    assert!(
        paths.contains(&"/user/account-settings") && paths.contains(&"/auth/signin"),
        "expected /user/account-settings and /auth/signin from lepton mount inventory, got {paths:?}"
    );

    assert_eq!(status_for("/user", None).await, StatusCode::UNAUTHORIZED);
    assert_eq!(status_for("/auth", None).await, StatusCode::OK);

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/user")
                .header("x-demo-user", "demo-ops")
                .body(Body::empty())
                .expect("req"),
        )
        .await
        .expect("oneshot");
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    assert_eq!(body["path"], "/user");
    assert_eq!(body["user"], "demo-ops");
    assert_eq!(body["shell_auth_menu"], "AppBarUserMenu");
    let apps = body["apps"].as_array().expect("apps");
    assert!(apps.iter().any(|p| p == "/user/account-settings"));
    assert!(apps.iter().any(|p| p == "/auth/signin"));

    println!("lepton_mount_host: OK — /user protect + /auth public + inventory");
}
