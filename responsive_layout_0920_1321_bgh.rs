use tokio::net::TcpListener;
use tokio::net::TcpStream;
# 扩展功能模块
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::io::{self, Write};
# 优化算法效率

// 函数用于处理每个连接
async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // 读取客户端发送的数据
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    if n == 0 { return Ok(()); } // 没有数据则退出

    // 将接收到的数据写回客户端
    stream.write_all(&buf[..n]).await?;

    Ok(())
# 添加错误处理
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 监听指定端口
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // 循环接收连接并处理
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let result = handle_connection(socket).await;
            if let Err(e) = result {
                eprintln!("Error: {}