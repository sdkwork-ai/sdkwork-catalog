use axum::Extension;

use sdkwork_iam_context_service::IamAppContext;

#[derive(Debug, Clone)]

pub(crate) struct AppRuntimeSubject {
    pub tenant_id: String,

    pub organization_id: Option<String>,

    pub user_id: String,
}

pub(crate) fn app_runtime_subject_from_iam(
    context: &IamAppContext,
) -> Result<AppRuntimeSubject, String> {
    let tenant_id = required_context_text(&context.tenant_id, "tenant_id")?;

    let user_id = required_context_text(&context.user_id, "user_id")?;

    let organization_id = context
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    Ok(AppRuntimeSubject {
        tenant_id,

        organization_id,

        user_id,
    })
}

/// Tenant id resolved for optional-credential storefront routes.
///
/// App-api storefront reads (`DualTokenOrAnonymous`) admit visitors without a
/// user session, and the catalog read model lives in the platform's public
/// tenant. A deployment overrides the default with `CATALOG_PUBLIC_TENANT_ID`.
pub(crate) fn public_tenant_id() -> String {
    std::env::var("CATALOG_PUBLIC_TENANT_ID")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "100001".to_owned())
}

/// Resolves the storefront subject for optional-credential routes.
///
/// A signed-in dual-token request still resolves the real principal, so the
/// caller's tenant and organization keep applying. A request without a
/// principal is an anonymous catalog browse: it reads the public tenant with no
/// organization and no user, which is exactly the scope the storefront shows.
pub(crate) fn app_storefront_subject_from_extension(
    context: Option<Extension<IamAppContext>>,
) -> Result<AppRuntimeSubject, String> {
    match context {
        Some(Extension(context)) => app_runtime_subject_from_iam(&context),
        None => Ok(anonymous_public_subject()),
    }
}

fn anonymous_public_subject() -> AppRuntimeSubject {
    AppRuntimeSubject {
        tenant_id: public_tenant_id(),
        organization_id: None,
        user_id: String::new(),
    }
}

fn required_context_text(value: &str, field_name: &'static str) -> Result<String, String> {
    let value = value.trim();

    if value.is_empty() {
        return Err(format!(
            "authenticated runtime context {field_name} is required"
        ));
    }

    Ok(value.to_owned())
}
