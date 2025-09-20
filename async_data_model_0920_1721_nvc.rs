use tokio::sync::Mutex;

/// 定义一个异步数据模型，使用Mutex来保证线程安全。
struct AsyncDataModel {
    /// 数据存储，这里以Vec<String>为例
    data: Mutex<Vec<String>>,
}

impl AsyncDataModel {
    /// 创建一个新的AsyncDataModel实例
    pub fn new() -> Self {
        AsyncDataModel {
            data: Mutex::new(Vec::new()),
        }
    }

    /// 添加数据到模型中
    #[allow(dead_code)]
    pub async fn add_data(&self, value: String) -> Result<(), String> {
        let mut data = self.data.lock().await;
        data.push(value);
        Ok(())
    }

    /// 获取模型中的所有数据
    #[allow(dead_code)]
    pub async fn get_data(&self) -> Result<Vec<String>, String> {
        let data = self.data.lock().await;
        Ok(data.clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建数据模型实例
    let model = AsyncDataModel::new();

    // 添加数据
    model.add_data("Data 1".to_string()).await?;
    model.add_data("Data 2".to_string()).await?;

    // 获取并打印数据
    let data = model.get_data().await?;
    for d in data {
        println!("Data: {}", d);
    }

    Ok(())
}