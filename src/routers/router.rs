use std::sync::{Arc, Mutex};

use warp::Filter;

use crate::controllers::user;

//web路由定义
pub fn get_router(
    count: Arc<Mutex<u32>>,
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let count1 = count.clone();
    let get_routes = warp::get().and(
        warp::path::end()
            .and_then(move || user::handle2(count.clone()))
            .or(warp::path!("hello" / String)
                .and_then(move |s: String| user::handle1(s, count1.clone())))
            .or(warp::path!("user" / String / u32).and_then(user::handle4))
            .or(warp::path!("name" / String).and_then(user::handle3)),
    );

    let post_routes = warp::post().and(
        warp::path!("user" / String / u32)
            .and_then(user::handle4)
            .or(warp::path!("name" / String).and_then(user::handle3)),
    );
    get_routes.or(post_routes)
}
