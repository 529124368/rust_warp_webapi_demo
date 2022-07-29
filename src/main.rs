use std::sync::{Arc, Mutex};

use log::info;

mod controllers;
mod models;
mod routers;
use routers::router::get_router;

#[tokio::main]
async fn main() {
    //计数器
    let count = Arc::new(Mutex::new(0u32));
    //日志设置
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("starting HTTP server at http://localhost:80");
    //获取路由
    let route = get_router(count);
    //启动服务器
    warp::serve(route).run(([127, 0, 0, 1], 80)).await;
}
