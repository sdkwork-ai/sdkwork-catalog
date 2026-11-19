//! App browse/open catalog HTTP routes (owned by catalog capability).
//!
//! # Read-only by design
//!
//! This surface publishes the buyer-facing read model: category, attribute, product, and SKU
//! lookups. The cart and buyer-address collections that used to live here are **retired** — those
//! tables and their DTOs were removed from the merchandise catalog module, and
//! `sdkwork-merchandise-service/tests/catalog_standard.rs` asserts that neither `commerce_cart` nor
//! `commerce_user_address` reappears. A store that owns no cart cannot publish a cart route, so the
//! handlers are deleted rather than stubbed.
//!
//! Nothing here writes, which is why no handler reads `If-Match` and none answers `428`: the
//! optimistic-concurrency preconditions belong to the surfaces that actually mutate a row.

use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use sdkwork_database_id::IdGenerator;
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_merchandise_repository_sqlx::PostgresCommerceCatalogStore;
use sdkwork_merchandise_service::{
    AttributeListQuery, CatalogRepositoryPort, CategoryListQuery, CategoryRetrieveQuery,
    MediaListQuery, ProductSkuListQuery, ProductSkuRetrieveQuery, ProductSpuListQuery,
    ProductSpuRetrieveQuery,
};
use sdkwork_merchandise_web_support::{
    catalog_error_response, map_attribute, map_category, map_product, map_product_detail, map_sku,
    media_snapshot_url, not_found_response, success_offset_page, success_resource,
    unauthorized_response, validation_response, CatalogState,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::subject::app_storefront_subject_from_extension;

/// Mounts the app catalog routes over the authoritative PostgreSQL pool and the process identity.
///
/// The generator is a parameter, not a process global: the composition root owns the node identity,
/// and one process must not run two Snowflake sequences over one node id.
pub fn app_catalog_router_with_postgres_pool(pool: PgPool, ids: Arc<dyn IdGenerator>) -> Router {
    build_app_catalog_router(Arc::new(PostgresCommerceCatalogStore::new(pool, ids)))
}

/// Mounts the app catalog routes over a store the caller has already constructed.
///
/// The store arrives as the service-owned port, so this crate never names a database or a concrete
/// repository; construction belongs to the composition root.
pub fn build_app_catalog_router(store: Arc<dyn CatalogRepositoryPort>) -> Router {
    Router::new()
        .route("/app/v3/api/catalog/categories", get(app_list_categories))
        .route(
            "/app/v3/api/catalog/categories/{categoryId}",
            get(app_retrieve_category),
        )
        .route("/app/v3/api/catalog/attributes", get(app_list_attributes))
        .route("/app/v3/api/catalog/products", get(app_list_products))
        .route(
            "/app/v3/api/catalog/products/{productId}",
            get(app_retrieve_product),
        )
        .route(
            "/app/v3/api/catalog/products/{productId}/skus",
            get(app_list_product_skus),
        )
        .route("/app/v3/api/catalog/skus/{skuId}", get(app_retrieve_sku))
        .with_state(CatalogState { store })
}

async fn app_list_categories(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<AppCategoryListQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CategoryListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        params.parent_id.as_deref(),
        Some("active"),
        None,
        params.page,
        params.page_size,
        params.root_only.unwrap_or(false),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_categories_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_category).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_error_response("category list is unavailable", error),
    }
}

async fn app_retrieve_category(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(category_id): Path<String>,
    Query(params): Query<RetrieveQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CategoryRetrieveQuery::new(
        &subject.tenant_id,
        &category_id,
        params.locale.as_deref(),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message().to_string()),
    };
    match state.store.retrieve_category(query).await {
        Ok(Some(category)) => success_resource(map_category(category)),
        Ok(None) => not_found_response("category was not found"),
        Err(error) => catalog_error_response("category read model is unavailable", error),
    }
}

async fn app_list_attributes(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<OffsetListQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match AttributeListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        Some("active"),
        None,
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_attributes_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_attribute).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_error_response("attribute list is unavailable", error),
    }
}

/// Lists the buyer-visible products of the caller's scope.
///
/// `organization_id` comes from the authenticated context, with the caller-supplied `shop_id` as an
/// override — a shop is an organization-owned storefront, so `shop_id` narrows the same axis rather
/// than introducing a second one.
///
/// The domain's `q` filter is published on this collection: the storefront search box sends it and
/// the read model binds it as a parameter, building the `ILIKE` pattern inside the driver.
/// `attribute_value_id` stays `None` because the authored contract does not declare it, and a
/// filter the document does not publish must not be read from the transport. The status filter is
/// pinned to `active` because every route on this surface is the buyer's view of the catalog.
async fn app_list_products(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<AppProductListQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuListQuery::new(
        &subject.tenant_id,
        params
            .shop_id
            .as_deref()
            .or(subject.organization_id.as_deref()),
        params.q.as_deref(),
        params.category_id.as_deref(),
        params.brand_id.as_deref(),
        params.product_type.as_deref(),
        Some("active"),
        params.sort.as_deref(),
        None,
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_spus_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_product).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_error_response("product list is unavailable", error),
    }
}

#[derive(Debug, Deserialize)]
struct RetrieveQueryParams {
    locale: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProductSkuListQueryParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct OffsetListQueryParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AppCategoryListQueryParams {
    parent_id: Option<String>,
    root_only: Option<bool>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AppProductListQueryParams {
    q: Option<String>,
    shop_id: Option<String>,
    category_id: Option<String>,
    brand_id: Option<String>,
    product_type: Option<String>,
    sort: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

/// Lists the SKUs of one product.
///
/// `attribute_value_id` is passed as `None` for the same reason the product list passes `q`: the
/// authored contract for this route declares `page` and `page_size` only.
async fn app_list_product_skus(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
    Query(params): Query<ProductSkuListQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSkuListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        Some(&product_id),
        None,
        Some("active"),
        None,
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_skus_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_sku).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_error_response("product sku list is unavailable", error),
    }
}

async fn app_retrieve_product(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
    Query(params): Query<RetrieveQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuRetrieveQuery::new(
        &subject.tenant_id,
        &product_id,
        params.locale.as_deref(),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_spu(query).await {
        Ok(Some(data)) => {
            let sku_query = match ProductSkuListQuery::new(
                &subject.tenant_id,
                subject.organization_id.as_deref(),
                Some(product_id.as_str()),
                None,
                Some("active"),
                params.locale.as_deref(),
                Some(1),
                Some(200),
            ) {
                Ok(query) => query,
                Err(error) => return validation_response(error.message()),
            };
            match state.store.list_skus_page(sku_query).await {
                Ok(skus) => {
                    let images = match MediaListQuery::new(
                        &subject.tenant_id,
                        subject.organization_id.as_deref(),
                        Some("spu"),
                        Some(product_id.as_str()),
                        None,
                        Some("active"),
                        Some(1),
                        Some(50),
                    ) {
                        Ok(media_query) => match state.store.list_media_page(media_query).await {
                            Ok(page) => page
                                .items
                                .iter()
                                .filter_map(|media| media_snapshot_url(&media.resource_snapshot))
                                .collect::<Vec<_>>(),
                            // A gallery read failure must not block the PDP; the
                            // primary imageUrl still renders.
                            Err(_) => Vec::new(),
                        },
                        Err(_) => Vec::new(),
                    };
                    success_resource(map_product_detail(data, skus.items, images))
                }
                Err(error) => catalog_error_response("product sku read model is unavailable", error),
            }
        }
        Ok(None) => not_found_response("product was not found"),
        Err(error) => catalog_error_response("product read model is unavailable", error),
    }
}

async fn app_retrieve_sku(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(sku_id): Path<String>,
    Query(params): Query<RetrieveQueryParams>,
) -> Response {
    let subject = match app_storefront_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSkuRetrieveQuery::new(
        &subject.tenant_id,
        &sku_id,
        params.locale.as_deref(),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_sku(query).await {
        Ok(Some(data)) => success_resource(map_sku(data)),
        Ok(None) => not_found_response("sku was not found"),
        Err(error) => catalog_error_response("sku read model is unavailable", error),
    }
}
