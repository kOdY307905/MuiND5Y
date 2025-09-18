use std::error::Error;
use tokio;
use warp::Filter;

// 定义一个响应结构体，用于格式化API响应
#[derive(Debug, serde::Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
    error: Option<String>,
}

// 定义一个简单的数据结构，用于演示
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 定义一个简单的用户数据
    let user = User {
        id: 1,
        name: "John Doe".to_string(),
    };

    // 定义一个路由，返回JSON格式化的响应
    let api = warp::path!("api" / "user")
        .map(move || {
            // 使用ApiResponse结构体包装响应数据
            ApiResponse {
                success: true,
                data: Some(user.clone()),
                message: Some("User fetched successfully".to_string()),
                error: None,
            }
        }).and_then(respond_json);

    // 启动服务器
    warp::serve(api)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

// 定义一个过滤器，用于将响应转换为JSON格式
fn respond_json<T: serde::Serialize>(item: T) -> Result<impl warp::Reply, warp::Rejection> {
    let reply = warp::reply::json(&item);
    Ok(reply)
}

// 定义一个函数，用于处理错误并返回格式化的响应
async fn handle_error(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    let code = 500;
    let message = "Internal Server Error";
    let error_response = ApiResponse {
        success: false,
        data: None,
        message: None,
        error: Some(message.to_string()),
    };
    // 将错误信息转换为JSON格式
    let reply = warp::reply::json(&error_response);
    Ok(reply)
}
