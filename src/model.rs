use serde::Serialize;
use tokio_pg_mapper_derive::PostgresMapper;

/// 待办列表模型
#[derive(PostgresMapper,Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    id: i32,
    title: String
}

/// 待办列表模型id
#[derive(PostgresMapper,Serialize)]
#[pg_mapper(table = "todo_list")]
pub struct TodoListID {
    id: i32,
}

/// 待办事项模型
#[derive(PostgresMapper,Serialize)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    id: i32,
    list_id: i32,
    checked: bool,
    title: String
}

/// 待办事项模型id
#[derive(PostgresMapper,Serialize)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItemID {
    id: i32,
}


#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool
}