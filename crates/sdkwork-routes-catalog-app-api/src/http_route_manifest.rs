//! Catalog app-api gateway route manifest (materialized from the authored
//! OpenAPI contract; business routes are protected by the default framework
//! profile, so dual-token auth is the manifest default).
//!
//! The cart and buyer-address collections are absent because the routes are: those tables were
//! retired from the merchandise catalog module, and a manifest entry without a route would make the
//! gateway authorize a path that nothing answers.

use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/attributes",
        "catalog",
        "attributes.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/categories",
        "catalog",
        "categories.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/categories/{categoryId}",
        "catalog",
        "categories.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/products",
        "catalog",
        "products.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/products/{productId}",
        "catalog",
        "products.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/products/{productId}/skus",
        "catalog",
        "products.skus.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/catalog/skus/{skuId}",
        "catalog",
        "skus.retrieve",
    ),
];

pub fn gateway_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
