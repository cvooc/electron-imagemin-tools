use iced::widget::{button, column, container, row, scrollable, text, text::Shaping};
use iced::{Color, Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

pub fn view() -> Element<'static, Message> {
    let back_btn = button(text("← 返回").shaping(Shaping::Advanced)).on_press(Message::Back);

    // 应用中使用的 emoji
    let app_emoji = vec![
        ("📂", "文件夹图标", "drop_zone 正常状态"),
        ("📥", "收件箱图标", "drop_zone hover 状态"),
        ("📋", "剪贴板", "header 按钮"),
        ("✅", "勾选", "压缩成功 / 暂无日志"),
        ("❌", "叉号", "压缩失败"),
        ("🔍", "放大镜", "预览按钮"),
        ("⏳", "沙漏", "正在压缩"),
        ("📭", "空邮箱", "暂无压缩历史"),
        ("—", "破折号", "最小化按钮"),
        ("✕", "乘号", "关闭按钮"),
    ];

    // Unicode 符号
    let unicode_symbols = vec![
        ("✓ (U+2713)", "check mark"),
        ("✗ (U+2717)", "ballot x"),
        ("★ (U+2605)", "star"),
        ("● (U+25CF)", "black circle"),
        ("◉ (U+25C9)", "fisheye"),
        ("► (U+25BA)", "pointer"),
        ("☑ (U+2611)", "ballot check"),
        ("☒ (U+2612)", "ballot x"),
    ];

    // 常见 emoji 分类
    let emoji_categories = vec![
        ("😊😄😃😀😁😂🤣", "笑脸"),
        ("❤️🧡💛💚💙💜🖤", "心形"),
        ("👍👎👌✌️🤞🤟🤘", "手势"),
        ("🐶🐱🐭🐹🐰🦊🐻", "动物"),
        ("🍎🍊🍋🍌🍉🍇🍓", "水果"),
        ("⚽🏀🏈⚾🎾🏐🏓", "运动"),
        ("🚗🚕🚙🚌🚎🏎️🚓", "车辆"),
        ("🌍🌎🌏🌐🗺️🧭", "地球"),
    ];

    let mut content = column![back_btn].spacing(12).padding(16);

    content = content.push(text("Emoji 测试页面").shaping(Shaping::Advanced).size(22));

    content = content.push(text("应用中使用的 emoji").shaping(Shaping::Advanced).size(16).style(Color::from_rgb(0.3, 0.3, 0.3)));
    for (emoji, name, location) in &app_emoji {
        content = content.push(
            row![
                text(*emoji).shaping(Shaping::Advanced).size(28).width(Length::Fixed(50.0)),
                text(*name).shaping(Shaping::Advanced).size(16).width(Length::FillPortion(1)),
                text(*location).shaping(Shaping::Advanced).size(12).style(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(8)
            .padding(4),
        );
    }

    content = content.push(text("").size(8));
    content = content.push(text("Unicode 符号测试").shaping(Shaping::Advanced).size(16).style(Color::from_rgb(0.3, 0.3, 0.3)));
    for (symbol, desc) in &unicode_symbols {
        content = content.push(
            row![
                text(*symbol).shaping(Shaping::Advanced).size(20).width(Length::Fixed(160.0)),
                text(*desc).shaping(Shaping::Advanced).size(14).style(Color::from_rgb(0.5, 0.5, 0.5)),
            ]
            .spacing(8)
            .padding(2),
        );
    }

    content = content.push(text("").size(8));
    content = content.push(text("常见 Emoji 分类").shaping(Shaping::Advanced).size(16).style(Color::from_rgb(0.3, 0.3, 0.3)));
    for (emojis, category) in &emoji_categories {
        content = content.push(
            row![
                text(*emojis).shaping(Shaping::Advanced).size(24).width(Length::FillPortion(1)),
                text(*category).shaping(Shaping::Advanced).size(14).style(Color::from_rgb(0.5, 0.5, 0.5)).width(Length::Fixed(60.0)),
            ]
            .spacing(8)
            .padding(2),
        );
    }

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
