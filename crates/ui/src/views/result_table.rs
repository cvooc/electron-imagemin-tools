use iced::widget::{button, column, horizontal_space, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct CompressResult {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenOutputDir,
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

pub fn view(results: &[CompressResult]) -> Element<'static, Message> {
    let header = row![
        text("图片名").width(Length::FillPortion(3)),
        text("原图大小").width(Length::FillPortion(1)),
        text("压缩后大小").width(Length::FillPortion(1)),
        text("压缩率").width(Length::FillPortion(1)),
    ]
    .padding(8);

    let mut rows = column![header];

    for result in results {
        let ratio = if result.original_size > 0 {
            ((result.compressed_size as f64 - result.original_size as f64)
                / result.original_size as f64
                * 100.0) as i64
        } else {
            0
        };

        let row = row![
            text(&result.name).width(Length::FillPortion(3)),
            text(format_size(result.original_size)).width(Length::FillPortion(1)),
            text(format_size(result.compressed_size)).width(Length::FillPortion(1)),
            text(format!("{}%", ratio)).width(Length::FillPortion(1)),
        ]
        .padding(4);

        rows = rows.push(row);
    }

    let summary = text(format!("共成功压缩 {} 个文件", results.len()));
    let open_btn = button(text("打开文件夹")).on_press(Message::OpenOutputDir);

    let footer = row![horizontal_space(), summary, open_btn,]
        .spacing(16)
        .padding(12);

    column![rows, footer].into()
}