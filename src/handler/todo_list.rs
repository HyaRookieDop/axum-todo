use crate::model::{AppState, TodoList, TodoListID};
use crate::{db, form, Response, Result};
use axum::extract::{Extension, Path};
use axum::Json;

use super::{get_client, log_error};


pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::CreateTodoList>,
) -> Result<Json<Response<TodoListID>>> {
    let handler_name = "todo_list create";
    let client = get_client(state, handler_name).await?;
    let result = db::todo_list::create(&client, payload).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn all(Extension(state): Extension<AppState>) -> Result<Json<Response<Vec<TodoList>>>> {
    let handler_name = "todo_list all";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_list::all(&client).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn update(
    Extension(state): Extension<AppState>,
    Json(payload): Json<form::UpdateTodoList>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_list update";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_list::update(&client,payload).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn find(
        Extension(state): Extension<AppState>,
        Path(list_id): Path<i32>,
        ) -> Result<Json<Response<TodoList>>> {
    let handler_name = "todo_list find";
    let client = get_client(state,handler_name).await?;
    let result = db::todo_list::find(&client,list_id).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}

pub async fn delete(
    Extension(state): Extension<AppState>,
    Path(list_id): Path<i32>,
) -> Result<Json<Response<bool>>> {
    let handler_name = "todo_list delete";
    let mut client = get_client(state,handler_name).await?;
    let result = db::todo_list::delete(&mut client,list_id).await.map_err(log_error(handler_name.to_string()))?;
    Ok(Json(Response::ok(result)))
}
