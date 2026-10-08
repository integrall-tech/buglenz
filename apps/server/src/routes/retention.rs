//! Admin API for retention (BugLenz, ADR-0009, invariant I5).

use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::auth::ApiActor;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use crate::services::retention::{
    Effective, ProjectRetention, RetentionDefaults, RetentionService, RetentionUpdate,
};
use crate::services::ProjectService;
use crate::workers::retention::{RetentionReport, RetentionState};

fn require_admin(actor: &ApiActor) -> AppResult<()> {
    if actor.is_admin() {
        Ok(())
    } else {
        Err(AppError::Forbidden("Admin privileges required".to_string()))
    }
}

/// One project's periods.
#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ProjectRetentionView {
    pub project_id: i32,
    pub name: String,
    /// What the project sets itself.
    pub own: ProjectRetention,
    /// What applies: its own, else the instance default.
    pub effective: Effective,
    /// Every type has a period.
    pub protected: bool,
    /// The types with no period at all.
    pub missing: Vec<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RetentionOverview {
    pub defaults: RetentionDefaults,
    pub interval_hours: u64,
    /// The last pass since the server started, if there was one.
    pub last_run: Option<RetentionReport>,
    pub projects: Vec<ProjectRetentionView>,
}

#[cfg_attr(feature = "openapi", utoipa::path(
    get,
    path = "/api/retention",
    tag = "Retention",
    responses(
        (status = 200, description = "Instance defaults, every project's periods and the last pass", body = RetentionOverview),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::error::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
))]
/// GET /api/retention
pub async fn get_overview(
    pool: web::Data<DbPool>,
    state: web::Data<Arc<RetentionState>>,
    actor: ApiActor,
) -> AppResult<HttpResponse> {
    require_admin(&actor)?;
    let projects = RetentionService::all(pool.get_ref())
        .await?
        .into_iter()
        .map(|p| {
            let own = p.own();
            let effective = Effective::resolve(&state.defaults, &own);
            ProjectRetentionView {
                project_id: p.project_id,
                name: p.name,
                own,
                protected: effective.is_protected(),
                missing: effective
                    .missing()
                    .iter()
                    .map(|m| (*m).to_string())
                    .collect(),
                effective,
            }
        })
        .collect();
    Ok(HttpResponse::Ok().json(RetentionOverview {
        defaults: state.defaults,
        interval_hours: state.interval.as_secs() / 3600,
        last_run: state.last_run(),
        projects,
    }))
}

#[cfg_attr(feature = "openapi", utoipa::path(
    put,
    path = "/api/projects/{project_id}/retention",
    tag = "Retention",
    params(("project_id" = i32, Path, description = "Project ID")),
    request_body = RetentionUpdate,
    responses(
        (status = 200, description = "The project's own periods after the update", body = ProjectRetention),
        (status = 400, description = "A period is outside 7..3650 days", body = crate::error::ErrorResponse),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::error::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::error::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
))]
/// PUT /api/projects/{project_id}/retention
/// A number sets a period, `null` clears it (the instance default applies), an absent field is left alone.
pub async fn put_project(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<RetentionUpdate>,
    actor: ApiActor,
) -> AppResult<HttpResponse> {
    require_admin(&actor)?;
    let project_id = path.into_inner();
    ProjectService::get_by_id(pool.get_ref(), project_id).await?;
    let updated = RetentionService::set(pool.get_ref(), project_id, &body).await?;
    Ok(HttpResponse::Ok().json(updated))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/retention", web::get().to(get_overview))
        .route(
            "/api/projects/{project_id}/retention",
            web::put().to(put_project),
        );
}
