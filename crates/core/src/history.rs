use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 单次压缩历史记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// 时间戳（毫秒）
    pub timestamp_ms: u64,
    /// 可读时间字符串
    pub timestamp_str: String,
    /// 文件结果列表
    pub results: Vec<HistoryResult>,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 原始总大小
    pub total_original: u64,
    /// 压缩后总大小
    pub total_compressed: u64,
}

/// 单个文件的历史结果（精简版，不含输出路径）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResult {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub success: bool,
}

impl HistoryEntry {
    pub fn savings(&self) -> i64 {
        self.total_original as i64 - self.total_compressed as i64
    }
}

/// 历史记录集合
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    /// 历史文件路径
    pub fn history_path() -> PathBuf {
        crate::Config::config_dir().join("history.json")
    }

    /// 加载历史记录
    pub fn load() -> Self {
        let path = Self::history_path();
        if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// 保存历史记录
    pub fn save(&self) -> Result<(), std::io::Error> {
        let dir = crate::Config::config_dir();
        std::fs::create_dir_all(&dir)?;
        let path = Self::history_path();
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(&path, content)
    }

    /// 添加一条记录。保持最多 100 条，自动丢弃最旧的记录。
    pub fn add(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
        if self.entries.len() > 100 {
            self.entries.remove(0);
        }
    }

    /// 清空所有历史记录
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
