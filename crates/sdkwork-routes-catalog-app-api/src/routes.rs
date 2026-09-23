use std::sync::Arc;

use axum::Router;
use sdkwork_database_id::IdGenerator;
use sqlx::PgPool;

pub fn build_catalog_app_router_with_postgres_pool(
    pool: PgPool,
    ids: Arc<dyn IdGenerator>,
) -> Router {
    crate::app_catalog_router::app_catalog_router_with_postgres_pool(pool, ids)
}

pub async fn build_catalog_app_router_with_framework_postgres(
    pool: PgPool,
    ids: Arc<dyn IdGenerator>,
) -> Router {
    crate::web_bootstrap::wrap_router_with_web_framework_from_env(
        build_catalog_app_router_with_postgres_pool(pool, ids),
    )
    .await
}
