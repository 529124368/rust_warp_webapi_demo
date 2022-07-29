use std::sync::{Arc, Mutex};

use serde_json::json;
use warp::Rejection;

//路由handle
pub async fn handle1(name: String, num: Arc<Mutex<u32>>) -> Result<String, Rejection> {
    let mut a = num.lock().unwrap();
    *a += 1;
    Ok(name + &a.to_string())
}

pub async fn handle2(num: Arc<Mutex<u32>>) -> Result<String, Rejection> {
    let mut a = num.lock().unwrap();
    *a += 1;
    Ok("当前被访问次数为:".to_string() + &a.to_string())
}

pub async fn handle3(name: String) -> Result<String, Rejection> {
    let re = json!( {"status": 200,
     "foo": {
         "name": name,
         "bar": [1, 3, "str", ["a", "b", 2]]
     },
     "result": null,
     "bool": [true, false],
     "float": 1.23
    });
    Ok(format!("{re}"))
}

pub async fn handle4(name: String, num: u32) -> Result<String, Rejection> {
    Ok(format!("{name}********{num}"))
}
