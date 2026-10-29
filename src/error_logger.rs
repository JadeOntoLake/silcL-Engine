use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};

static INIT: Once = Once::new();

/// 写入错误日志到 error.log
pub fn log_error(msg: &str) {

    // 清空旧日志
    INIT.call_once(|| {
        let _ = std::fs::write("error.log", "");
    });

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_line = format!("[{}] {}\n", timestamp, msg);

    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open("error.log")
        .and_then(|mut file| file.write_all(log_line.as_bytes()));
}