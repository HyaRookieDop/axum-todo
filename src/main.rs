mod config;
mod db;
mod error;
mod form;
mod handler;
mod model;
mod response;
use axum::{
    extract::Extension,
    routing::{get, post,delete,put},
    Router,
};
use dotenv::dotenv;
use handler::{todo_list, usage};
pub use response::Response;
pub type Result<T> = std::result::Result<T, error::AppError>;

#[tokio::main]
async fn main() {
    // 初始化日志
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "todo=debug");
    }
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let cfg = config::Config::from_env().expect("初始化配置失败");
    tracing::info!("服务器监听于：{}", &cfg.web.addr);
    let pool = cfg
        .pg
        .create_pool(tokio_postgres::NoTls)
        .expect("初始化数据库连接失败");
    // 路由
    let app = Router::new()
        .route("/", get(usage::usage))
        .route("/create", post(todo_list::create))
        .route("/all", get(todo_list::all))
        .route("/todo/:list_id", get(todo_list::find).delete(todo_list::delete).put(todo_list::update))
        .layer(Extension(model::AppState { pool }));
    // 绑定到配置文件设置的地址
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
