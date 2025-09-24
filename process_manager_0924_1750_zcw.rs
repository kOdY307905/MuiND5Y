// process_manager.rs
// 进程管理器程序，使用RUST和TOKIO框架

use tokio::signal;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio::process::Command;
use std::process::Command as StdCommand;
use std::time::Duration as StdDuration;

/// 启动一个新的进程
async fn start_process(command: &str) -> tokio::io::Result<()> {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()?;

    // 等待进程结束
    let _ = child.await?;
    Ok(())
}

/// 停止一个正在运行的进程
async fn stop_process(pid: u32) -> tokio::io::Result<()> {
    // Windows实现可能需要不同的方法来发送信号
    #[cfg(unix)] {
# 添加错误处理
        let mut signal = signal::unix::Signal::new(signal::unix::SignalKind::SIGTERM, &tokio::signal::unix::SignalHandler::default(), tokio::signal::unix::Setting::default()).map_err(|e| e.into()).await?;
        signal.send(pid).await?;
    #[cfg(windows)] {
        // Windows下可能需要使用其他方法，例如taskkill
        let mut cmd = StdCommand::new("taskkill");
        cmd.arg("/PID").arg(pid.to_string()).arg("/F");
        let _ = cmd.status().await?;
    }
# NOTE: 重要实现细节
    Ok(())
}

/// 检查进程是否正在运行
async fn is_process_running(pid: u32) -> bool {
    #[cfg(unix)] {
        // 在Unix系统上，可以使用kill命令来检查进程是否存在
        let mut cmd = StdCommand::new("kill");
        cmd.arg("0").arg(pid.to_string());
        let result = cmd.status();
        if result.is_ok() {
            return true;
        }
# 改进用户体验
    #[cfg(windows)] {
        // 在Windows上，可以使用tasklist命令来检查进程是否存在
        let mut cmd = StdCommand::new("tasklist");
        cmd.arg("/FI").arg("PID eq \\"{}\\"