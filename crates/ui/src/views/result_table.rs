use iced::widget::{button, column, horizontal_space, row, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub struct Row {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub status: Result<(), String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenOutputDir,
    RetryCompress,
    ClearResults,
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

fn format_savings(bytes: i64) -> String {
    if bytes.abs() < 1024 {
        format!("{} B", bytes)
    } else if bytes.abs() < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

pub fn view(results: &[Row], has_output_dir: bool) -> Element<'static, Message> {
    let header = row![
        text("图片名").width(Length::FillPortion(3)),
        text("原图大小").width(Length::FillPortion(1)),
        text("压缩后大小").width(Length::FillPortion(1)),
        text("节省").width(Length::FillPortion(1)),
        text("状态").width(Length::FillPortion(1)),
    ]
    .padding(8);

    let mut rows = column![header];

    let mut total_original: u64 = 0;
    let mut total_compressed: u64 = 0;
    let mut success_count = 0;
    let mut fail_count = 0;

    for result in results {
        let status_text = match &result.status {
            Ok(()) => {
                total_original += result.original_size;
                total_compressed += result.compressed_size;
                success_count += 1;
                "成功".to_string()
            }
            Err(e) => {
                fail_count += 1;
                format!("失败: {}", e)
            }
        };

        let savings = result.original_size as i64 - result.compressed_size as i64;
        let savings_text = if result.status.is_ok() {
            format_savings(savings)
        } else {
            "-".to_string()
        };

        let row = row![
            text(&result.name).width(Length::FillPortion(3)),
            text(format_size(result.original_size)).width(Length::FillPortion(1)),
            text(format_size(result.compressed_size)).width(Length::FillPortion(1)),
            text(savings_text).width(Length::FillPortion(1)),
            text(status_text).width(Length::FillPortion(1)),
        ]
        .padding(4);

        rows = rows.push(row);
    }

    let total_savings = total_original as i64 - total_compressed as i64;
    let summary = text(format!(
        "成功 {} / 失败 {} / 共节省 {}",
        success_count,
        fail_count,
        format_savings(total_savings)
    ));

    let mut footer = row![horizontal_space(), summary].spacing(16).padding(12);

    if has_output_dir {
        footer = footer.push(button(text("打开文件夹")).on_press(Message::OpenOutputDir));
    }
    footer = footer.push(button(text("再次压缩")).on_press(Message::RetryCompress));
    footer = footer.push(button(text("清空列表")).on_press(Message::ClearResults));

    column![rows, footer].into()
}
