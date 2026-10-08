//! Erasure for one data subject (BugLenz, ADR-0009), for administrators.

use actix_web::{web, HttpResponse};

use crate::auth::ApiActor;
use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use crate::services::privacy::PrivacyService;

#[cfg_attr(feature = "openapi", utoipa::path(
    delete,
    path = "/api/projects/{project_id}/privacy/users/{user_id}",
    tag = "Privacy",
    params(
        ("project_id" = i32, Path, description = "Project ID"),
        ("user_id" = String, Path, description = "The SDK's `user.id`, compared as an exact string"),
    ),
    responses(
        (status = 200, description = "How many events and transactions were erased", body = crate::services::privacy::Erasure),
        (status = 401, description = "Unauthorized", body = crate::error::ErrorResponse),
        (status = 403, description = "Forbidden", body = crate::error::ErrorResponse),
        (status = 404, description = "Project not found", body = crate::error::ErrorResponse),
    ),
    security(("bearer_auth" = [])),
))]
/// DELETE /api/projects/{project_id}/privacy/users/{user_id}
pub async fn erase_user(
    pool: web::Data<DbPool>,
    path: web::Path<(i32, String)>,
    actor: ApiActor,
) -> AppResult<HttpResponse> {
    if !actor.is_admin() {
        return Err(AppError::Forbidden("Admin privileges required".to_string()));
    }
    let (project_id, user_id) = path.into_inner();
    let erasure = PrivacyService::erase_user(&pool, project_id, &user_id).await?;
    Ok(HttpResponse::Ok().json(erasure))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/projects/{project_id}/privacy")
            .route("/users/{user_id}", web::delete().to(erase_user)),
    );
}
