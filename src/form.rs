use serde::Deserialize;

/// 创建待办列表
#[derive(Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

/// 修改待办列表
#[derive(Deserialize)]
pub struct UpdateTodoList {
    pub id: i32,
    pub title: String,
}

/// 创建待办事项
#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}

/// 修改待办事项
#[derive(Deserialize)]
pub struct UpdateTodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32
}