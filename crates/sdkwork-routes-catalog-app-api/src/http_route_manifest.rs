//! Catalog app-api gateway route manifest (materialized from the authored
//! OpenAPI contract).
//!
//! Every route on this surface is a buyer-facing read, so browsing the catalog
//! must never be gated behind a login: `DualTokenOrAnonymous` admits anonymous
//! visitors while still resolving a signed-in principal, which keeps per-user
//! scope such as the caller's organization working. Handlers fall back to the
//! platform public tenant when no principal is present
//! (`subject::app_storefront_subject_from_extension`).
//!
//! The cart and buyer-address collections are absent because the routes are: those tables were
//! retired from the merchandise catalog module, and a manifest entry without a route would make the
//! gateway authorize a path that nothing answers.

use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/attributes",
        "catalog",
        "attributes.list",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/categories",
        "catalog",
        "categories.list",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/categories/{categoryId}",
        "catalog",
        "categories.retrieve",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/products",
        "catalog",
        "products.list",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/products/{productId}",
        "catalog",
        "products.retrieve",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/products/{productId}/skus",
        "catalog",
        "products.skus.list",
    ),
    HttpRoute::dual_token_or_anonymous(
        HttpMethod::Get,
        "/app/v3/api/catalog/skus/{skuId}",
        "catalog",
        "skus.retrieve",
    ),
];

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}

#[cfg(test)]
mod tests {
    use sdkwork_web_core::RouteAuth;

    use super::*;

    fn route_auth(method: &str, path: &str) -> RouteAuth {
        gateway_route_manifest()
            .match_route(method, path)
            .unwrap_or_else(|| panic!("{method} {path} must be registered in the catalog manifest"))
            .auth
    }

    #[test]
    fn storefront_catalog_browsing_never_requires_a_login() {
        for path in [
            "/app/v3/api/catalog/attributes",
            "/app/v3/api/catalog/categories",
            "/app/v3/api/catalog/categories/{categoryId}",
            "/app/v3/api/catalog/products",
            "/app/v3/api/catalog/products/{productId}",
            "/app/v3/api/catalog/products/{productId}/skus",
            "/app/v3/api/catalog/skus/{skuId}",
        ] {
            assert_eq!(
                RouteAuth::DualTokenOrAnonymous,
                route_auth("GET", path),
                "GET {path} must admit anonymous visitors while still resolving a signed-in principal"
            );
        }
    }
}
