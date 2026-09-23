pub mod app_catalog_router;
pub mod http_route_manifest;
pub mod routes;
pub mod subject;
pub mod web_bootstrap;

pub use app_catalog_router::{app_catalog_router_with_postgres_pool, build_app_catalog_router};
pub use http_route_manifest::gateway_route_manifest;
pub use routes::{
    build_catalog_app_router_with_framework_postgres, build_catalog_app_router_with_postgres_pool,
};
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

use axum::Router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use sdkwork_database_id::IdGenerator;
use sdkwork_database_sqlx::DatabasePool;
use std::sync::Arc;

/// Catalog standalone-gateway entry: resolves browse/open routes from the service host pool.
///
/// The identity travels with the pool: the app catalog routes build a merchandise catalog store to
/// mint merchandise ids, and that store must be wired to the process node id rather than to a second
/// generator.
pub async fn build_catalog_app_router_with_framework(host: Arc<CatalogServiceHost>) -> Router {
    // 服务端权威持久化仅支持 PostgreSQL（DATABASE_SPEC：authoritative-server）
    let DatabasePool::Postgres(pool, _) = host.database_pool() else {
        panic!("catalog app router requires a PostgreSQL database pool");
    };
    build_catalog_app_router_with_framework_postgres(pool.clone(), host.id_generator()).await
}

use sqlx::PgPool;

pub async fn gateway_mount(pool: PgPool, ids: Arc<dyn IdGenerator>) -> axum::Router {
    build_catalog_app_router_with_framework_postgres(pool, ids).await
}
