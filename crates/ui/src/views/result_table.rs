use iced::widget::{button, column, horizontal_space, row, text};
use iced::{Element, Length};

use crate::util::{format_savings, format_size};

#[derive(Debug, Clone)]
pub struct Row {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub status: Result<(), String>,
    /// 输入文件路径（用于预览）
    pub input_path: Option<std::path::PathBuf>,
    /// 输出文件路径（用于预览）
    pub output_path: Option<std::path::PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenOutputDir,
    RetryCompress,
    ClearResults,
    /// 在系统图片查看器中预览原图和压缩后的图
    Preview(usize),
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

    for (i, result) in results.iter().enumerate() {
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

        let can_preview = result.input_path.is_some() || result.output_path.is_some();

        let row = row![
            text(&result.name).width(Length::FillPortion(3)),
            text(format_size(result.original_size)).width(Length::FillPortion(1)),
            text(format_size(result.compressed_size)).width(Length::FillPortion(1)),
            text(savings_text).width(Length::FillPortion(1)),
            text(status_text).width(Length::FillPortion(1)),
        ]
        .padding(4);

        let row = if can_preview {
            row.push(button(text("预览").size(12)).on_press(Message::Preview(i)))
        } else {
            row
        };

        rows = rows.push(row);
    }

    let total_savings = total_original as i64 - total_compressed as i64;
    let ratio = if total_original > 0 {
        format!(" ({:.0}%)", (total_savings as f64 / total_original as f64 * 100.0))
    } else {
        String::new()
    };
    let summary = text(format!(
        "成功 {} / 失败 {} / 共节省 {}{}",
        success_count,
        fail_count,
        format_savings(total_savings),
        ratio,
    ));

    let mut footer = row![horizontal_space(), summary].spacing(16).padding(12);

    if has_output_dir {
        footer = footer.push(button(text("打开文件夹")).on_press(Message::OpenOutputDir));
    }
    footer = footer.push(button(text("再次压缩")).on_press(Message::RetryCompress));
    footer = footer.push(button(text("清空列表")).on_press(Message::ClearResults));

    column![rows, footer].into()
}
