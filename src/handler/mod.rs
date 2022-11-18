use deadpool_postgres::Client;

use crate::{model::AppState, error::AppError,Result};

pub mod todo_list;
pub mod todo_item;
pub mod usage;

pub async fn get_client(state: AppState,handler_name: &str) -> Result<Client> {
    state.pool.get().await.map_err(|err| {
        tracing::error!("{}: {:?}", handler_name, err);
        AppError::db_error(err)
    })
}

pub fn log_error(handler_name: String) -> Box<dyn Fn(AppError) -> AppError>{
    Box::new(move |err| {
        tracing::debug!("{}: {:?}",handler_name,err);
        err
    })
}
