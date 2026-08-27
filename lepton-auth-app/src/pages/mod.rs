//! Page components for the `/auth/*` routes (signin, signup, logout, password reset)
//! and their shared shell/host wrappers.

/// Shared wrapper for `/auth/signin`, `/auth/signup`, and `/auth/logout` routes.
pub mod auth_route_host;
/// Shared page chrome for the `/auth/*` routes.
pub mod auth_shell;
/// `/auth/logout` page.
pub mod logout;
/// `/auth/oauth/callback` page.
pub mod oauth_callback;
/// `/auth/reset/*` page components hosting the password reset dialog.
pub mod password_reset;
/// Shared wrapper for `/auth/reset/request` and `/auth/reset/confirm` routes.
pub mod password_reset_route_host;
/// `/auth/signin` page.
pub mod signin;
/// `/auth/signup` page.
pub mod signup;

pub use auth_route_host::AuthRouteHost;
pub use auth_shell::AuthPageShell;
pub use logout::LogoutPage;
pub use oauth_callback::OAuthCallbackPage;
pub use password_reset::{PasswordResetConfirmPage, PasswordResetRequestPage};
pub use password_reset_route_host::PasswordResetRouteHost;
pub use signin::SigninPage;
pub use signup::SignupPage;
