use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router
};
use sea_orm::{ActiveValue::Set, DbErr};
use crate::{
    dto::items::{CreateItem, ItemResponse},
    services::items::ItemService,
    state::AppState,
};

pub fn items_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_items).post(create_item))
        .route("/{id}", get(get_item_by_id).delete(delete_item))
}

async fn get_all_items(
    State(state): State<AppState>
) -> Result<Json<Vec<ItemResponse>>, StatusCode> {
    let items = ItemService::get_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(items))
}

async fn get_item_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ItemResponse>, StatusCode> {
    let item = ItemService::get_by_id(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(item))
}

async fn create_item(
    State(state): State<AppState>,
    Json(data): Json<CreateItem>
) -> Result<(StatusCode, Json<ItemResponse>), StatusCode> {
    let item = ItemService::create(&state.db, data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(item)))
}

async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Result<StatusCode, StatusCode> {
    ItemService::delete(&state.db, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}