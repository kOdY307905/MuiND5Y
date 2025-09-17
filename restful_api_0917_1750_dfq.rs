use warp::Filter;

// 导入warp库中所需的模块
use warp::Rejection;
use warp::Reply;
use warp::http::StatusCode;
use warp::reject::Reject;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

// 定义一个简单的数据结构，用于序列化和反序列化
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    name: String,
}

// 定义API错误类型
#[derive(Debug, Reject)]
struct ApiError {
    status: StatusCode,
    message: String,
}

// 定义全局用户数据库
static USERS: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(vec![]));

// 创建一个新的用户
fn create_user(user: User) -> Result<impl Reply, Rejection> {
    USERS.lock().map(|mut users| {
        users.push(user);
        Ok(warp::reply::json(&user))
    }).map_err(|e| warp::reject::custom(ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        message: e.to_string(),
    }))
}

// 获取所有用户
async fn get_users() -> Result<impl Reply, Rejection> {
    let users = USERS.lock().await;
    Ok(warp::reply::json(&users))
}

// 启动warp服务器的main函数
#[tokio::main]
async fn main() {
    // 定义路由和过滤器
    let api = warp::path("users")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|user: User| create_user(user))
        .or(warp::path("users")
            .and(warp::get()).map(|| get_users()))
        .recover(handle_rejection);

    // 定义错误处理器
    fn handle_rejection(err: Rejection) -> impl Reply {
        if err.is_not_found() {
            warp::reply::with_status(warp::reply::json(&"Not Found"), StatusCode::NOT_FOUND)
        } else {
            match err.find::<ApiError>() {
                Some(api_err) => warp::reply::with_status(warp::reply::json(&api_err), api_err.status),
                None => warp::reply::with_status(warp::reply::json(&"Internal Server Error"), StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }

    // 启动服务器
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

