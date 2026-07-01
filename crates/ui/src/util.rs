/// 将字节数转为可读格式（B/KB/MB）
pub fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// 将节省的字节数转为可读格式（支持负数，即增大）
pub fn format_savings(bytes: i64) -> String {
    if bytes.abs() < 1024 {
        format!("{} B", bytes)
    } else if bytes.abs() < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
