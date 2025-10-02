use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use serde::Serialize;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Display;
use thiserror::Error;

// 定义一个错误类型，用于处理可能的错误
#[derive(Error, Debug)]
enum TestReportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

// 测试报告数据结构
#[derive(Serialize)]
struct TestReport {
    // 测试报告信息字段
    timestamp: u64,
    results: Vec<TestCase>,
}

// 测试用例数据结构
#[derive(Serialize)]
struct TestCase {
    name: String,
    passed: bool,
}

// 生成测试报告的异步函数
async fn generate_test_report<P: AsRef<Path> + Display>(path: P) -> Result<(), TestReportError> {
    // 创建测试报告数据
    let report = TestReport {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        results: vec![
            TestCase { name: "Test Case 1".to_string(), passed: true },
            TestCase { name: "Test Case 2".to_string(), passed: false },
        ],
    };

    // 将测试报告数据序列化为JSON
    let report_json = serde_json::to_string(&report)?;

    // 打开文件并写入测试报告
    let mut file = File::create(path).await?;
    file.write_all(report_json.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // 设置测试报告生成路径
    let report_path = "./test_report.json";

    // 调用异步函数生成测试报告
    match generate_test_report(report_path).await {
        Ok(_) => println!("Test report generated successfully"),
        Err(e) => eprintln!("Failed to generate test report: {}", e),
    }
}
