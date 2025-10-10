use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::stream::StreamExt;
use std::task::Context;
use std::task::Poll;
use std::io::Error;
use tokio::io::AsyncWrite;
use tokio::sync::Notify;
use std::collections::VecDeque;
use tokio::io::{self, AsyncRead, AsyncWrite};

#[derive(Debug, Clone)]
pub struct LightningNode {
    peers: Arc<Mutex<HashMap<String, TcpStream>>>,
    addr: String,
    notify: Arc<Notify>,
    queue: Arc<Mutex<VecDeque<String>>>,
}

impl LightningNode {
    // 创建一个新的闪电网络节点
    pub fn new(addr: &str) -> Self {
        LightningNode {
            peers: Arc::new(Mutex::new(HashMap::new())),
            addr: addr.to_string(),
            notify: Arc::new(Notify::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // 启动节点监听
    pub async fn listen(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(&self.addr).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            let node = self.clone();
            tokio::spawn(async move {
                node.handle_connection(socket).await;
            });
        }
    }

    // 处理连接
    async fn handle_connection(&self, mut socket: TcpStream) {
        let mut buf = [0u8; 128];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => return, // socket closed
                Ok(_) => {
                    // Process message and write response
                    self.notify.notify_one();
                    if let Err(e) = socket.write_all(&buf).await {
                        eprintln!("Failed to write to socket: {}", e);
                        return;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                    return;
                },
            }
        }
    }

    // 添加对等节点
    pub async fn add_peer(&self, addr: &str, socket: TcpStream) {
        let mut peers = self.peers.lock().await;
        peers.insert(addr.to_string(), socket);
    }

    // 发送消息给所有对等节点
    pub async fn broadcast(&self, msg: &[u8]) {
        let peers = self.peers.lock().await;
        for (_, socket) in peers.iter() {
            // Send message to each peer
            if let Err(e) = socket.write_all(msg).await {
                eprintln!("Failed to send message to peer: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let node = LightningNode::new("127.0.0.1:8080");
    match node.listen().await {
        Ok(()) => println!("Node started successfully"),
        Err(e) => eprintln!("Failed to start node: {}", e),
    }
}
