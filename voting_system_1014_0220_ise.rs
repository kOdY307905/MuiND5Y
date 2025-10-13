use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::stream::StreamExt;
use std::io::Result;
use std::net::SocketAddr;
use std::str;
use serde_json::json;
use serde_json::Value;
use serde_json::Result as JsonResult;

// 定义投票项数据结构
struct VoteOption {
    name: String,
    votes: u32,
}

// 定义投票系统
struct VotingSystem {
    options: RwLock<HashMap<String, VoteOption>>,
}

impl VotingSystem {
    fn new() -> Self {
        VotingSystem {
            options: RwLock::new(HashMap::new()),
        }
    }

    async fn add_option(&self, name: String) -> JsonResult<Value> {
        let mut options = self.options.write().await;
        if options.contains_key(&name) {
            Err(serde_json::Error::custom("Option already exists"))
        } else {
            options.insert(name.clone(), VoteOption {
                name: name.clone(),
                votes: 0,
            });
            Ok(json!({"message": "Option added"}))
        }
    }

    async fn vote(&self, option_name: String) -> JsonResult<Value> {
        let mut options = self.options.write().await;
        if let Some(option) = options.get_mut(&option_name) {
            option.votes += 1;
            Ok(json!({"message": "Vote counted"}))
        } else {
            Err(serde_json::Error::custom("Option not found"))
        }
    }

    async fn get_results(&self) -> JsonResult<Value> {
        let options = self.options.read().await;
        let mut results = HashMap::new();
        for (key, value) in options.iter() {
            results.insert(key.clone(), value.votes);
        }
        Ok(json!(results))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let voting_system = Arc::new(VotingSystem::new());

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on port 8080");

    while let Ok((socket, _)) = listener.accept().await {
        let voting_system = Arc::clone(&voting_system);
        tokio::spawn(async move {
            handle_client(socket, voting_system).await;
        });
    }
    Ok(())
}

async fn handle_client(mut socket: tokio::net::TcpStream, voting_system: Arc<VotingSystem>) {
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => return, // Connection closed
            Ok(n) => {
                let request = match str::from_utf8(&buffer[..n]) {
                    Ok(v) => v,
                    Err(e) => {
                        socket.write_all(b"Invalid UTF-8").await.expect("Failed to write