use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_id::IdGenerator;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

pub mod identity;
pub mod runtime_env;

pub use identity::{shared_identity, CatalogIdentity};

/// The catalog process host.
///
/// Catalog is an **API-only dependency surface**: every route reads tables owned by
/// `sdkwork-merchandise`, so the process needs a shared pool — for the merchandise catalog store and
/// for the Snowflake node lease — but no schema of its own. There is deliberately no `database/`
/// root and no `*-database-host` module here.
///
/// `sdkwork-database-lifecycle` names this shape `skipped_without_database_assets` (see
/// `discovery.rs`), so a federated host that materializes `SDKWORK_CATALOG_APP_ROOT` discovers the
/// root and discovers no database assets — rather than loading an empty module whose contract no
/// table satisfies. The framework's two tables the catalog module used to own
/// (`commerce_cart` / `commerce_cart_item` / `commerce_user_address`) were buyer-side baggage with no
/// route behind them, and were retired; `commerce_` itself belongs to `sdkwork-merchandise`.
pub struct CatalogServiceHost {
    pool: DatabasePool,
    identity: Arc<CatalogIdentity>,
}

impl CatalogServiceHost {
    pub async fn new() -> Self {
        Self::from_env()
            .await
            .expect("catalog service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        let _ = dotenvy::dotenv();
        let config = DatabaseConfig::from_env("CATALOG")
            .map_err(|error| format!("read catalog database config failed: {error}"))?;
        let pool = create_pool_from_config(config)
            .await
            .map_err(|error| format!("create catalog database pool failed: {error}"))?;
        // One identity for the whole process: the app catalog routes build a merchandise catalog
        // store to mint merchandise ids, and those ids must come from this node id rather than from
        // a second, divergent generator.
        let identity = shared_identity(CatalogIdentity::from_pool(&pool).await?);
        Ok(Self { pool, identity })
    }

    pub fn database_pool(&self) -> &DatabasePool {
        &self.pool
    }

    /// The process Snowflake identity.
    #[must_use]
    pub fn identity(&self) -> Arc<CatalogIdentity> {
        Arc::clone(&self.identity)
    }

    /// The injectable Snowflake port repositories consume.
    ///
    /// One process, one node id: this returns the same generator to every caller, so a later
    /// repository cannot be wired to a divergent sequence.
    #[must_use]
    pub fn id_generator(&self) -> Arc<dyn IdGenerator> {
        self.identity.clone()
    }
}
