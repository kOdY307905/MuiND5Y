// data_analyzer.rs
// 这是一个使用RUST和TOKIO框架实现的简单数据统计分析器
// 它包含了错误处理、注释和文档，遵循RUST最佳实践，确保代码的可维护性和可扩展性

use tokio;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// 统计数据分析器
pub struct DataAnalyzer {
    /// 存储数据的HashMap, 键为String, 值为i32
    data: Arc<Mutex<HashMap<String, i32>>>,
}

impl DataAnalyzer {
    /// 创建一个新的DataAnalyzer实例
    pub fn new() -> Self {
        DataAnalyzer {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 添加数据到分析器
    pub async fn add_data(&self, key: String, value: i32) -> Result<(), String> {
        let mut data = self.data.lock().await;
        data.entry(key).and_modify(|e| *e += value).or_insert(value);
        Ok(())
    }

    /// 获取数据的平均值
    pub async fn get_average(&self) -> Result<f64, String> {
        let data = self.data.lock().await;
        let total: i32 = data.values().sum();
        let count = data.len() as i32;

        if count == 0 {
            Err("No data available".to_string())
        } else {
            Ok(total as f64 / count as f64)
        }
    }
}

#[tokio::main]
async fn main() {
    let analyzer = DataAnalyzer::new();

    // 添加一些数据
    if let Err(e) = analyzer.add_data("test".to_string(), 10).await {
        eprintln!("Error adding data: {}", e);
    }
    if let Err(e) = analyzer.add_data("test".to_string(), 20).await {
        eprintln!("Error adding data: {}", e);
    }

    // 获取平均值
    match analyzer.get_average().await {
        Ok(average) => println!("Average: {}", average),
        Err(e) => eprintln!("Error calculating average: {}", e),
    }
}