use crate::{Result, model::{TodoItem, TodoItemID}, form, error::AppError};
use tokio_postgres::Client;

pub async fn create(client: &Client,frm: form::CreateTodoItem) -> Result<TodoItemID> {
    let result = super::query_one(client, "INSERT INTO table_item (title,checked,list_id) VALUES ($1,$2,$3)", &[&frm.title,&frm.checked,&frm.list_id]).await?;
    Ok(result)
}

pub async fn all(client: &Client,list_id: i32) -> Result<Vec<TodoItem>> {
    let result = super::query(client, "SELECT * FROM todo_item where list_id=$1", &[&list_id]).await?;
    Ok(result)
}

pub async fn find(client: &Client,list_id: i32,item_id: i32) -> Result<TodoItem> {
    let result = super::query_one(client, "SELECT * FROM todo_item where list_id=$1 AND id=$2", &[&list_id,&item_id]).await?;
    Ok(result)
}

pub async fn update(client: &Client,frm: form::UpdateTodoItem) -> Result<bool> {
    let result = super::excute(client, "UPDATE todo_item SET title=$1,checked=$2,list_id=$3", &[&frm.title,&frm.checked,&frm.list_id]).await?;
    Ok(result > 0)
}

pub async fn delete(client: &mut Client,list_id: i32,item_id: i32) -> Result<bool> {
    let tx = client.transaction().await.map_err(AppError::from)?;
    let result = super::excute(&tx, "DELETE todo_item WHERE list_id=$1 AND item_id=$2", &[&list_id,&item_id]).await;
    if let Err(err) = result {
        tx.rollback().await.map_err(AppError::from)?;
        return Err(AppError::db_error(err))
    }
    tx.commit().await.map_err(AppError::from)?;
    Ok(true)
}
