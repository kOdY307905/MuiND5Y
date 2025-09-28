use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, Instant};

/// UserBehaviorAnalysis 是一个用于分析用户行为的程序。
/// 它记录用户的活动，并提供一些基本的分析功能。
struct UserBehaviorAnalysis {
    /// 用户活动日志记录
    activity_log: Arc<Mutex<HashMap<String, Vec<Instant>>>,
}

impl UserBehaviorAnalysis {
    /// 创建一个新的 UserBehaviorAnalysis 实例。
    fn new() -> Self {
        UserBehaviorAnalysis {
            activity_log: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 记录用户活动
    async fn record_activity(&self, user_id: &str) {
        let mut log = self.activity_log.lock().await;
        if let Some(entry) = log.get_mut(user_id) {
            entry.push(Instant::now());
        } else {
            log.insert(user_id.to_string(), vec![Instant::now()]);
        }
    }

    /// 获取用户的活动频率（每分钟的活动次数）
    async fn activity_frequency(&self, user_id: &str) -> Option<f64> {
        let log = self.activity_log.lock().await;
        let activities = log.get(user_id)?;
        let duration = Duration::from_secs(60);
        let first_activity = activities.first()?.elapsed();
        let last_activity = Instant::now().duration_since(activities.last()?.saturating_sub(duration));
        let mut count = 0;
        for activity in activities.iter() {
            if activity.elapsed() <= duration {
                count += 1;
            }
        }
        Some(count as f64 / 60.0)
    }
}

#[tokio::main]
async fn main() {
    let analysis = UserBehaviorAnalysis::new();
    analysis.record_activity("user1").await;
    tokio::time::sleep(Duration::from_secs(10)).await;
    analysis.record_activity("user1").await;
    tokio::time::sleep(Duration::from_secs(10)).await;
    analysis.record_activity("user1").await;

    if let Some(frequency) = analysis.activity_frequency("user1").await {
        println!("User1 activity frequency: {:.2} activities per minute", frequency);
    } else {
        println!("No activities found for user1");
    }
}
