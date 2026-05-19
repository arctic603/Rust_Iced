//! 计数器页面 —— 多种计数器变体
//!
//! 本页面演示 Iced 中按钮和状态交互的基本模式：
//! - 基础计数器：+ / - / Reset
//! - 步进计数器：可调整每次增减的步长
//! - 边界计数器：数值被限制在 [-10, 10] 区间内
//! - 历史记录：追踪基础计数器的最近操作记录

use iced::widget::{button, column, container, row, Space};
use iced::{Alignment, Color, Element, Fill};

use crate::Message;

/// 计数器页面状态
#[derive(Debug, Default)]
pub struct CounterPage {
    pub basic: i32,       // 基础计数器值
    pub step: i32,        // 步进计数器值
    pub step_size: i32,   // 当前步长（默认 1）
    pub bounded: i32,     // 边界计数器值（限制在 -10~10）
    pub history: Vec<i32>, // 基础计数器的历史记录
}

/// 计数器页面消息
#[derive(Debug, Clone)]
pub enum CounterMessage {
    BasicInc,             // 基础计数器 +1
    BasicDec,             // 基础计数器 -1
    BasicReset,           // 基础计数器归零并清空历史
    StepInc,              // 步进计数器 +step_size
    StepDec,              // 步进计数器 -step_size
    StepSizeChanged(i32), // 修改步长
    BoundedInc,           // 边界计数器 +1（受限制）
    BoundedDec,           // 边界计数器 -1（受限制）
    BoundedReset,         // 边界计数器归零
}

impl CounterPage {
    /// 更新计数器页面状态
    pub fn update(&mut self, msg: CounterMessage) {
        match msg {
            CounterMessage::BasicInc => {
                self.basic += 1;
                self.history.push(self.basic);
            }
            CounterMessage::BasicDec => {
                self.basic -= 1;
                self.history.push(self.basic);
            }
            CounterMessage::BasicReset => {
                self.basic = 0;
                self.history.clear();
            }
            CounterMessage::StepInc => self.step += self.step_size,
            CounterMessage::StepDec => self.step -= self.step_size,
            CounterMessage::StepSizeChanged(v) => self.step_size = v,
            // 使用 min/max 确保数值不越界
            CounterMessage::BoundedInc => self.bounded = (self.bounded + 1).min(10),
            CounterMessage::BoundedDec => self.bounded = (self.bounded - 1).max(-10),
            CounterMessage::BoundedReset => self.bounded = 0,
        }
    }

    /// 构建计数器页面视图
    pub fn view(&self) -> Element<'_, Message> {
        let basic_label = iced::widget::text("Basic Counter")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let basic_value = iced::widget::text(self.basic.to_string())
            .size(48)
            .color(if self.basic >= 0 {
                Color::from_rgb(0.1, 0.7, 0.3)
            } else {
                Color::from_rgb(0.9, 0.2, 0.2)
            });
        let basic_btns = row![
            btn("-").on_press(Message::Counter(CounterMessage::BasicDec)),
            basic_value,
            btn("+").on_press(Message::Counter(CounterMessage::BasicInc)),
        ]
        .align_y(Alignment::Center)
        .spacing(16);
        let basic_reset = button(iced::widget::text("Reset").size(13))
            .padding([6, 16])
            .on_press(Message::Counter(CounterMessage::BasicReset));
        let basic_card = card(column![basic_label, basic_btns, basic_reset].spacing(12).into());

        let step_label = iced::widget::text("Step Counter")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let step_value = iced::widget::text(self.step.to_string())
            .size(36)
            .color(Color::from_rgb(0.2, 0.5, 0.8));
        let step_info = iced::widget::text(format!("Step size: {}", self.step_size))
            .size(12)
            .color(Color::from_rgb(0.5, 0.5, 0.5));
        let step_btns = row![
            btn("-").on_press(Message::Counter(CounterMessage::StepDec)),
            step_value,
            btn("+").on_press(Message::Counter(CounterMessage::StepInc)),
        ]
        .align_y(Alignment::Center)
        .spacing(16);
        let step_controls = row![
            button(iced::widget::text("Step=1").size(11)).padding([4, 10]).on_press(Message::Counter(CounterMessage::StepSizeChanged(1))),
            button(iced::widget::text("Step=5").size(11)).padding([4, 10]).on_press(Message::Counter(CounterMessage::StepSizeChanged(5))),
            button(iced::widget::text("Step=10").size(11)).padding([4, 10]).on_press(Message::Counter(CounterMessage::StepSizeChanged(10))),
        ]
        .spacing(8);
        let step_card = card(column![step_label, step_info, step_btns, step_controls].spacing(10).into());

        let bound_label = iced::widget::text("Bounded Counter (-10 ~ 10)")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let bound_value = iced::widget::text(self.bounded.to_string())
            .size(36)
            .color(if self.bounded == 10 || self.bounded == -10 {
                Color::from_rgb(0.9, 0.5, 0.1)
            } else {
                Color::from_rgb(0.2, 0.5, 0.8)
            });
        let bound_bar = container(Space::new())
            .width(iced::Length::Fixed((self.bounded.abs() as f32 / 10.0) * 200.0))
            .height(iced::Length::Fixed(6.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.6, 0.9))),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::new(3.0),
                },
                ..Default::default()
            });
        let bound_btns = row![
            btn("-").on_press(Message::Counter(CounterMessage::BoundedDec)),
            bound_value,
            btn("+").on_press(Message::Counter(CounterMessage::BoundedInc)),
        ]
        .align_y(Alignment::Center)
        .spacing(16);
        let bound_reset = button(iced::widget::text("Reset").size(13))
            .padding([6, 16])
            .on_press(Message::Counter(CounterMessage::BoundedReset));
        let bound_card = card(column![bound_label, bound_bar, bound_btns, bound_reset].spacing(10).into());

        let history_label = iced::widget::text("History")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let history_text = if self.history.is_empty() {
            iced::widget::text("No records yet").size(13).color(Color::from_rgb(0.5, 0.5, 0.5))
        } else {
            let recent: Vec<String> = self.history.iter().rev().take(20).map(|v| v.to_string()).collect();
            iced::widget::text(format!("Recent: {}", recent.join(", ")))
                .size(13)
                .color(Color::from_rgb(0.3, 0.3, 0.4))
        };
        let history_card = card(column![history_label, history_text].spacing(8).into());

        column![basic_card, step_card, bound_card, history_card]
            .spacing(16)
            .into()
    }
}

/// 辅助函数：创建统一风格的 + / - 小按钮
fn btn<'a>(label: &'a str) -> button::Button<'a, Message> {
    button(iced::widget::text(label).size(18).align_x(iced::widget::text::Alignment::Center))
        .width(50)
        .padding(8)
}

/// 辅助函数：带浅灰背景和圆角的卡片容器
fn card<'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    container(content)
        .width(Fill)
        .padding(16)
        .style(|_theme: &iced::Theme| container::Style {
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
