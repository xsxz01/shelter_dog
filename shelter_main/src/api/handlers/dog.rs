use crate::api::response::error::AppError;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;
use axum::extract::State;
use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, IntoActiveModel, TryIntoModel};
use std::sync::Arc;
use entity::dog::DogCreateRequest;
use crate::api::response::dog::DogCreateResponse;

pub async fn create(
    Extension(_claims): Extension<TokenClaims>,
    State(state): State<Arc<ApplicationState>>,
    Json(payload): Json<DogCreateRequest>,
) -> Result<Json<DogCreateResponse>, AppError> {
    let dog_active_model = payload.into_active_model();
    let dog_model = dog_active_model.save(state.db_conn.load().as_ref()).await?;
    let dog = dog_model.try_into_model()?;

    let response = DogCreateResponse {
        status: "success".to_string(),
        data: Some(dog),
    };

    Ok(Json(response))
}
