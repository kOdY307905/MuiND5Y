// test_result_analyzer.rs
// 一个使用RUST和TOKIO框架的测试结果分析器

use tokio;
use std::fs;
use std::path::Path;
use std::io::{self, Error};

// 定义一个结构体，用于存储测试结果
struct TestResult {
    test_name: String,
    passed: bool,
    error_message: Option<String>,
}

impl TestResult {
    // 构造函数
    fn new(test_name: &str, passed: bool, error_message: Option<String>) -> Self {
        TestResult {
            test_name: test_name.to_string(),
            passed,
            error_message,
        }
    }

    // 输出测试结果
    fn print(&self) {
        println!("Test '{}': {}{}", self.test_name,
                if self.passed { "Passed" } else { "Failed" },
                match &self.error_message {
                    Some(msg) => format!(": {}", msg),
                    None => String::new(),
                }
        );
    }
}

// 分析测试结果
async fn analyze_results(results: Vec<TestResult>) {
    let mut passed = 0;
    let mut failed = 0;

    for result in results {
        result.print();
        if result.passed {
            passed += 1;
        } else {
            failed += 1;
        }
    }

    println!("Total tests: {}
Passed: {}
Failed: {}", results.len(), passed, failed);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 读取测试结果文件
    let test_results_path = "test_results.txt";
    let contents = fs::read_to_string(test_results_path).await?;

    // 解析测试结果
    let results: Vec<TestResult> = contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() < 3 {
                panic!("Invalid test result format");
            }

            let test_name = parts[0].trim().to_string();
            let passed = parts[1].trim() == "Passed";
            let error_message = if parts[2].starts_with("Error:") {
                Some(parts[2].trim().trim_start_matches("Error:").to_string())
            } else {
                None
            };

            TestResult::new(&test_name, passed, error_message)
        }).collect();

    // 分析并输出测试结果
    analyze_results(results).await;

    Ok(())
}
