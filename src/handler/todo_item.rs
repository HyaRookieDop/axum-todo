use crate::model::{AppState, TodoItem, TodoItemID};
use crate::{db, form, Response, Result};
use axum::extract::{Extension, Path};
use axum::Json;

use super::{get_client, log_error};


pub async fn create(
        Extension(state): Extension<AppState>,
        Json(payload): Json<form::CreateTodoItem>,
        ) -> Result<Json<Response<TodoItemID>>> {
    let handler_name = "todo_item create";
    let client = get_client(state, handler_name).await?;
    let result = db::todo_item::create(&client, payload).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn all(Extension(state): Extension<AppState>,Path(list_id): Path<i32>) -> Result<Json<Response<Vec<TodoItem>>>> {
    let handler_name = "todo_item all";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_item::all(&client,list_id).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn update(
        Extension(state): Extension<AppState>,
        Json(payload): Json<form::UpdateTodoItem>,
        ) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_item update";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_item::update(&client,payload).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn find(
        Extension(state): Extension<AppState>,
        Path((list_id,item_id)): Path<(i32,i32)>,
        ) -> Result<Json<Response<TodoItem>>> {
    let handler_name = "todo_item find";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_item::find(&client,list_id,item_id).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn delete(
        Extension(state): Extension<AppState>,
        Path((list_id,item_id)): Path<(i32,i32)>,
        ) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_item delete";
    let mut client = get_client(state,handler_name).await?;
    let result = db::todo_item::delete(&mut client,list_id,item_id).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
