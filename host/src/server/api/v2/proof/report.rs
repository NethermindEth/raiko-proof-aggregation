use axum::{debug_handler, extract::State, routing::get, Json, Router};
use raiko_tasks::TaskManager;
use serde_json::Value;
use utoipa::OpenApi;

use crate::{interfaces::HostResult, ProverState};

#[utoipa::path(post, path = "/proof/report",
    tag = "Proving",
    responses (
        (status = 200, description = "Successfully listed all current tasks", body = CancelStatus)
    )
)]
#[debug_handler(state = ProverState)]
/// List all tasks.
///
/// Retrieve a list of `{ chain_id, blockhash, prover_type, prover, status }` items.
async fn report_handler(State(prover_state): State<ProverState>) -> HostResult<Json<Value>> {
    let mut manager = prover_state.task_manager();

    let task_report = manager.list_all_tasks().await?;

    Ok(Json(serde_json::to_value(task_report)?))
}

#[derive(OpenApi)]
#[openapi(paths(report_handler))]
struct Docs;

pub fn create_docs() -> utoipa::openapi::OpenApi {
    Docs::openapi()
}

pub fn create_router() -> Router<ProverState> {
    Router::new().route("/", get(report_handler))
}
