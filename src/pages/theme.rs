//! 主题样式页面 —— 颜色、字体、圆角等样式演示
//!
//! 本页面演示 Iced 中常见的样式定制方式：
//! - 按钮预设样式（Primary / Secondary）
//! - 自定义按钮背景色
//! - Container 背景色、边框、圆角
//! - 不同大小和颜色的文本样式

use iced::widget::{button, column, container, row};
use iced::{Color, Element, Fill, Theme};

use crate::Message;

/// 主题样式页面状态
#[derive(Debug, Default)]
pub struct ThemePage {
    pub custom_color_r: f32,  // 自定义颜色 R 分量（0~255）
    pub custom_color_g: f32,  // 自定义颜色 G 分量（0~255）
    pub custom_color_b: f32,  // 自定义颜色 B 分量（0~255）
}

/// 主题样式页面消息
#[derive(Debug, Clone)]
pub enum ThemeMessage {
    RedChanged(f32),
    GreenChanged(f32),
    BlueChanged(f32),
}

impl ThemePage {
    /// 更新主题页面状态
    pub fn update(&mut self, msg: ThemeMessage) {
        match msg {
            ThemeMessage::RedChanged(v) => self.custom_color_r = v,
            ThemeMessage::GreenChanged(v) => self.custom_color_g = v,
            ThemeMessage::BlueChanged(v) => self.custom_color_b = v,
        }
    }

    /// 构建主题样式页面视图
    pub fn view(&self) -> Element<'_, Message> {
        let preset_title = iced::widget::text("Button Styles")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let btn_primary = button(iced::widget::text("Primary").size(13).align_x(iced::widget::text::Alignment::Center))
            .padding([8, 18])
            .style(|theme: &Theme, status| button::primary(theme, status));

        let btn_secondary = button(iced::widget::text("Secondary").size(13).align_x(iced::widget::text::Alignment::Center))
            .padding([8, 18])
            .style(|theme: &Theme, status| button::secondary(theme, status));

        let btn_success = button(iced::widget::text("Success").size(13).align_x(iced::widget::text::Alignment::Center))
            .padding([8, 18])
            .style(|theme: &Theme, status| {
                let mut s = button::primary(theme, status);
                s.background = Some(iced::Background::Color(Color::from_rgb(0.2, 0.7, 0.35)));
                s
            });

        let btn_danger = button(iced::widget::text("Danger").size(13).align_x(iced::widget::text::Alignment::Center))
            .padding([8, 18])
            .style(|theme: &Theme, status| {
                let mut s = button::primary(theme, status);
                s.background = Some(iced::Background::Color(Color::from_rgb(0.9, 0.25, 0.25)));
                s
            });

        let btn_row = row![btn_primary, btn_secondary, btn_success, btn_danger]
            .spacing(10);

        let preset_card = card(column![preset_title, btn_row].spacing(12).into());

        let custom_title = iced::widget::text("Custom Color Preview")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let custom_preview = container(
            iced::widget::text(format!(
                "R: {:.0}  G: {:.0}  B: {:.0}",
                self.custom_color_r, self.custom_color_g, self.custom_color_b
            ))
            .size(14)
            .color(Color::WHITE),
        )
        .width(Fill)
        .height(80)
        .align_x(iced::Alignment::Center)
        .align_y(iced::Alignment::Center)
        .style(move |_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(
                self.custom_color_r / 255.0,
                self.custom_color_g / 255.0,
                self.custom_color_b / 255.0,
            ))),
            border: iced::Border {
                color: Color::from_rgb(0.8, 0.8, 0.85),
                width: 2.0,
                radius: iced::border::Radius::new(8.0),
            },
            ..Default::default()
        });

        let custom_card = card(column![custom_title, custom_preview].spacing(10).into());

        let text_title = iced::widget::text("Text Styles")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let text_examples = column![
            iced::widget::text("Default size text").size(14),
            iced::widget::text("Large title text").size(22).color(Color::from_rgb(0.2, 0.4, 0.7)),
            iced::widget::text("Small note text").size(11).color(Color::from_rgb(0.5, 0.5, 0.5)),
            iced::widget::text("Red warning text").size(14).color(Color::from_rgb(0.9, 0.2, 0.2)),
            iced::widget::text("Green success text").size(14).color(Color::from_rgb(0.1, 0.7, 0.3)),
        ]
        .spacing(6);

        let text_card = card(column![text_title, text_examples].spacing(10).into());

        let container_title = iced::widget::text("Container Radius")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let radius_examples = row![
            rounded_box("None", 0.0, Color::from_rgb(0.7, 0.3, 0.3)),
            rounded_box("Small", 4.0, Color::from_rgb(0.3, 0.7, 0.3)),
            rounded_box("Medium", 8.0, Color::from_rgb(0.3, 0.3, 0.7)),
            rounded_box("Large", 16.0, Color::from_rgb(0.7, 0.5, 0.2)),
        ]
        .spacing(10);

        let radius_card = card(column![container_title, radius_examples].spacing(10).into());

        column![preset_card, custom_card, text_card, radius_card]
            .spacing(16)
            .into()
    }
}

/// 辅助函数：创建一个带圆角和文字的彩色方块，用于展示不同圆角效果
fn rounded_box(label: &'static str, radius: f32, color: Color) -> Element<'static, Message> {
    container(
        iced::widget::text(label)
            .size(12)
            .color(Color::WHITE)
            .align_x(iced::widget::text::Alignment::Center),
    )
    .width(iced::Length::Fixed(80.0))
    .height(iced::Length::Fixed(50.0))
    .align_x(iced::Alignment::Center)
    .align_y(iced::Alignment::Center)
    .style(move |_theme: &Theme| container::Style {
        background: Some(iced::Background::Color(color)),
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: iced::border::Radius::new(radius),
        },
        ..Default::default()
    })
    .into()
}

/// 辅助函数：带浅灰背景和圆角的卡片容器
fn card(content: Element<'_, Message>) -> Element<'_, Message> {
    container(content)
        .width(Fill)
        .padding(16)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.98, 0.98, 0.99))),
            border: iced::Border {
                color: Color::from_rgb(0.88, 0.88, 0.92),
                width: 1.0,
                radius: iced::border::Radius::new(8.0),
            },
            ..Default::default()
        })
        .into()
}
