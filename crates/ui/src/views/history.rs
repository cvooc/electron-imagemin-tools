use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};
use imagemin_core::HistoryEntry;

#[derive(Debug, Clone)]
pub enum Message {
    /// 返回主界面
    Back,
    /// 打开某条历史记录的输出目录
    OpenDir(std::path::PathBuf),
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn view(entries: &[HistoryEntry]) -> Element<'static, Message> {
    if entries.is_empty() {
        return container(
            column![
                text("暂无压缩历史").size(20),
                text("完成压缩后会自动记录在这里").size(14),
            ]
            .spacing(12),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into();
    }

    let header_row = row![
        text("时间").width(Length::FillPortion(2)),
        text("文件数").width(Length::FillPortion(1)),
        text("原大小").width(Length::FillPortion(1)),
        text("压缩后").width(Length::FillPortion(1)),
        text("节省").width(Length::FillPortion(1)),
        text("操作").width(Length::FillPortion(1)),
    ]
    .padding(8);

    let mut rows = column![header_row];

    for entry in entries.iter().rev() {
        let file_count = entry.results.len();
        let success_count = entry.results.iter().filter(|r| r.success).count();
        let savings = entry.savings();
        let savings_str = if savings >= 0 {
            format!("-{}", format_size(savings as u64))
        } else {
            format!("+{}", format_size((-savings) as u64))
        };

        let entry_row = row![
            text(&entry.timestamp_str).width(Length::FillPortion(2)),
            text(format!("{} 个 (成功 {})", file_count, success_count))
                .width(Length::FillPortion(1)),
            text(format_size(entry.total_original)).width(Length::FillPortion(1)),
            text(format_size(entry.total_compressed)).width(Length::FillPortion(1)),
            text(savings_str).width(Length::FillPortion(1)),
            button(text("打开"))
                .on_press(Message::OpenDir(entry.output_dir.clone()))
                .width(Length::FillPortion(1)),
        ]
        .padding(4)
        .spacing(4);

        rows = rows.push(entry_row);
    }

    let back_btn = button(text("← 返回")).on_press(Message::Back);

    let content = column![back_btn, scrollable(rows)].spacing(12).padding(16);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
