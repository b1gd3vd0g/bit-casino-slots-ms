//! This module handles the HTTP router serving all routes for the application. It utilizes the
//! handlers found in the `handlers` module. The documentation for the API can be found described in
//! the `openapi.yaml` file in the root, or served with a GET request to the base path.

use axum::Router;

/// Provide the HTTP router for the app **without** its required state.
pub fn router() -> Router {
    Router::new()
}
