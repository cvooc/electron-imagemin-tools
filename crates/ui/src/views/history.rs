use iced::widget::{button, column, container, row, scrollable, text, text::Shaping};
use iced::{Color, Element, Length};
use imagemin_core::HistoryEntry;

use crate::util::format_size;

#[derive(Debug, Clone)]
pub enum Message {
    /// 返回主界面
    Back,
    /// 打开某条历史记录的输出目录
    OpenDir(std::path::PathBuf),
    /// 清空全部历史
    ClearAll,
}

pub fn view(entries: &[HistoryEntry]) -> Element<'static, Message> {
    if entries.is_empty() {
        return container(
            column![
                text("📭").shaping(Shaping::Advanced).size(48),
                text("暂无压缩历史").size(20),
                text("完成压缩后会自动记录在这里").size(14).style(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(12)
            .align_items(iced::Alignment::Center),
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

    let back_btn = button(text("← 返回").shaping(Shaping::Advanced)).on_press(Message::Back);
    let clear_btn = button(text("清空历史")).on_press(Message::ClearAll).style(iced::theme::Button::Destructive);

    let top_bar = row![back_btn, iced::widget::horizontal_space(), clear_btn].spacing(8);

    let content = column![top_bar, scrollable(rows)].spacing(12).padding(16);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
