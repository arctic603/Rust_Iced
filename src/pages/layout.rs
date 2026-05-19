//! 布局演示页面 —— 各种布局方式

use iced::widget::{button, center, column, container, row, stack, Space};
use iced::{Alignment, Color, Element, Fill};

use crate::Message;

#[derive(Debug, Default)]
pub struct LayoutPage {
    pub selected_demo: usize,
}

#[derive(Debug, Clone)]
pub enum LayoutMessage {
    SelectDemo(usize),
}

impl LayoutPage {
    pub fn update(&mut self, msg: LayoutMessage) {
        match msg {
            LayoutMessage::SelectDemo(idx) => self.selected_demo = idx,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let demo_names = ["Row", "Column", "Center", "Stack", "Space"];

        let demo_tabs = row(
            demo_names
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    button(iced::widget::text(*name).size(12))
                        .padding([6, 12])
                        .on_press(Message::Layout(LayoutMessage::SelectDemo(i)))
                        .into()
                })
                .collect::<Vec<_>>(),
        )
        .spacing(8);

        let selected_content: Element<'static, Message> = match self.selected_demo {
            0 => row_demo(),
            1 => column_demo(),
            2 => center_demo(),
            3 => stack_demo(),
            4 => space_demo(),
            _ => iced::widget::text("Select a tab above").into(),
        };

        column![demo_tabs, selected_content]
            .spacing(16)
            .into()
    }
}

fn row_demo() -> Element<'static, Message> {
    let title = iced::widget::text("Row Horizontal").size(14).color(Color::from_rgb(0.4, 0.4, 0.5));

    let row1 = row![
        color_box(Color::from_rgb(0.9, 0.3, 0.3), "A", 60),
        color_box(Color::from_rgb(0.3, 0.9, 0.3), "B", 60),
        color_box(Color::from_rgb(0.3, 0.3, 0.9), "C", 60),
    ]
    .spacing(8);

    let row2 = row![
        color_box(Color::from_rgb(0.9, 0.7, 0.3), "Start", 80),
        color_box(Color::from_rgb(0.5, 0.8, 0.9), "Center", 80),
        color_box(Color::from_rgb(0.7, 0.5, 0.9), "End", 80),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    card(column![title, row1, row2].spacing(10).into())
}

fn column_demo() -> Element<'static, Message> {
    let title = iced::widget::text("Column Vertical").size(14).color(Color::from_rgb(0.4, 0.4, 0.5));

    let col1 = column![
        color_box(Color::from_rgb(0.9, 0.3, 0.3), "1", 40),
        color_box(Color::from_rgb(0.3, 0.9, 0.3), "2", 40),
        color_box(Color::from_rgb(0.3, 0.3, 0.9), "3", 40),
    ]
    .spacing(8);

    let col2 = column![
        color_box(Color::from_rgb(0.9, 0.5, 0.2), "Top", 50),
        color_box(Color::from_rgb(0.2, 0.7, 0.5), "Mid", 50),
        color_box(Color::from_rgb(0.5, 0.3, 0.8), "Bot", 50),
    ]
    .spacing(8)
    .align_x(Alignment::Center);

    let side_by_side = row![col1, Space::new(), col2].spacing(16);

    card(column![title, side_by_side].spacing(10).into())
}

fn center_demo() -> Element<'static, Message> {
    let title = iced::widget::text("Center").size(14).color(Color::from_rgb(0.4, 0.4, 0.5));

    let centered = center(
        container(iced::widget::text("Centered!").size(16).color(Color::WHITE))
            .width(180)
            .height(100)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.3, 0.5, 0.8))),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::new(8.0),
                },
                ..Default::default()
            }),
    )
    .width(Fill)
    .height(140)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.97))),
        border: iced::Border {
            color: Color::from_rgb(0.85, 0.85, 0.9),
            width: 1.0,
            radius: iced::border::Radius::new(6.0),
        },
        ..Default::default()
    });

    card(column![title, centered].spacing(10).into())
}

fn stack_demo() -> Element<'static, Message> {
    let title = iced::widget::text("Stack").size(14).color(Color::from_rgb(0.4, 0.4, 0.5));

    let layered = stack![
        container(Space::new())
            .width(200)
            .height(120)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.95))),
                ..Default::default()
            }),
        center(iced::widget::text("Back").size(12).color(Color::from_rgb(0.5, 0.5, 0.6))),
        container(iced::widget::text("Overlay").size(14).color(Color::WHITE))
            .padding(8)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.8, 0.3, 0.3))),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: iced::border::Radius::new(4.0),
                },
                ..Default::default()
            }),
    ];

    card(column![title, layered].spacing(10).into())
}

fn space_demo() -> Element<'static, Message> {
    let title = iced::widget::text("Space Fill").size(14).color(Color::from_rgb(0.4, 0.4, 0.5));

    let spaced = row![
        color_box(Color::from_rgb(0.3, 0.7, 0.4), "Left", 80),
        Space::new(),
        color_box(Color::from_rgb(0.7, 0.4, 0.3), "Right", 80),
    ]
    .align_y(Alignment::Center);

    card(column![title, spaced].spacing(10).into())
}

fn color_box(color: Color, label: &'static str, size: u16) -> Element<'static, Message> {
    container(iced::widget::text(label).size(12).color(Color::WHITE))
        .width(iced::Length::Fixed(size as f32))
        .height(iced::Length::Fixed(size as f32))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(color)),
            border: iced::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: iced::border::Radius::new(4.0),
            },
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

fn card(content: Element<'static, Message>) -> Element<'static, Message> {
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
