use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tokio::stream::StreamExt;
use futures::stream;
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::io::Cursor;
use regex::Regex;

// 定义一个异步的文件扫描器
pub struct FileScanner {
    path: String,
    pattern: Regex,
}

impl FileScanner {
    // 创建一个新的文件扫描器
    pub fn new<P: Into<String>>(path: P, pattern: Regex) -> Self {
        FileScanner {
            path: path.into(),
            pattern,
        }
    }

    // 异步扫描文件并找出匹配的行
    pub async fn scan(&self) -> io::Result<Vec<String>> {
        let mut buf_reader = File::open(&self.path).await?;
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer).await?;

        let lines = self.pattern
            .find_iter(Cursor::new(String::from_utf8_lossy(&buffer)))
            .map(|mat| mat.as_str().to_string())
            .collect();

        Ok(lines)
    }
}

// 定义一个扫描任务，用于处理单个文件的扫描
pub struct ScanTask {
    scanner: FileScanner,
}

impl ScanTask {
    // 创建一个新的扫描任务
    pub fn new(scanner: FileScanner) -> Self {
        ScanTask { scanner }
    }

    // 执行扫描任务
    pub async fn execute(self) -> io::Result<Vec<String>> {
        self.scanner.scan().await
    }
}

// 定义一个异步的扫描器，用于扫描目录中的所有文件
pub struct AsyncScanner {
    path: String,
    pattern: Regex,
}

impl AsyncScanner {
    // 创建一个新的异步扫描器
    pub fn new<P: Into<String>>(path: P, pattern: Regex) -> Self {
        AsyncScanner {
            path,
            pattern,
        }
    }

    // 异步扫描目录中的所有文件
    pub async fn scan_dir(&self) -> io::Result<Vec<String>> {
        let mut results = Vec::new();
        let entries = tokio::fs::read_dir(&self.path).await?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let scanner = FileScanner::new(path.to_str().unwrap().to_string(), self.pattern.clone());
                let task = ScanTask::new(scanner);
                let mut lines = task.execute().await?;
                results.append(&mut lines);
            }
        }

        Ok(results)
    }
}

// 主程序入口
#[tokio::main]
async fn main() -> io::Result<()> {
    // 定义要扫描的目录和模式
    let path = "./"; // 替换为实际路径
    let pattern = Regex::new(r"[0-9]{3}-[0-9]{2}-[0-9]{4}").unwrap(); // 示例模式，匹配类似123-45-6789格式的字符串

    // 创建一个异步扫描器
    let scanner = AsyncScanner::new(path, pattern);

    // 执行扫描
    let matches = scanner.scan_dir().await?;

    // 打印匹配结果
    for line in matches {
        println!("Found match: {}", line);
    }

    Ok(())
}