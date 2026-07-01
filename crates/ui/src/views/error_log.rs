use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Color, Element, Length};

use crate::app::LogEntry;

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    CopyLog,
}

fn log_item_bg(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(Color::from_rgba(0.8, 0.2, 0.2, 0.08))),
        border: iced::Border { radius: 4.0.into(), ..Default::default() },
        ..Default::default()
    }
}

pub fn view(logs: &[LogEntry]) -> Element<'static, Message> {
    let back_btn = button(text("← 返回")).on_press(Message::Back);

    let mut content = column![back_btn].spacing(12).padding(16);

    content = content.push(text("压缩失败日志").size(20));

    if logs.is_empty() {
        content = content.push(
            text("暂无失败日志")
                .size(16)
                .style(Color::from_rgb(0.5, 0.5, 0.5)),
        );
    } else {
        let count = logs.len();
        content = content.push(
            row![
                text(format!("共 {} 条失败记录", count))
                    .size(14)
                    .style(Color::from_rgb(0.7, 0.3, 0.3)),
                button(text("一键复制日志").size(13))
                    .on_press(Message::CopyLog),
            ]
            .spacing(12),
        );

        for log in logs.iter().rev() {
            let item = column![
                text(&log.timestamp).size(11).style(Color::from_rgb(0.5, 0.5, 0.5)),
                text(&log.filename).size(13),
                text(&log.input_path).size(11).style(Color::from_rgb(0.5, 0.5, 0.5)),
                text(&log.error).size(12).style(Color::from_rgb(0.8, 0.2, 0.2)),
            ]
            .spacing(2)
            .padding([6, 8]);

            content = content.push(
                container(item)
                    .style(log_item_bg)
                    .padding(2),
            );
        }
    }

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
