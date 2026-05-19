//! 动画效果页面 —— 简单的动画演示

use iced::widget::{button, column, container, row, slider, Space};
use iced::{Alignment, Color, Element, Fill};

use crate::Message;

#[derive(Debug, Default)]
pub struct AnimationPage {
    pub animate_enabled: bool,
    pub box_position: f32,
    pub box_size: f32,
    pub box_color_phase: f32,
    pub pulse_phase: f32,
}

#[derive(Debug, Clone)]
pub enum AnimationMessage {
    ToggleAnimate,
    PositionChanged(f32),
    SizeChanged(f32),
    ColorPhaseChanged(f32),
    PulseTick,
    Reset,
}

impl AnimationPage {
    pub fn update(&mut self, msg: AnimationMessage) {
        match msg {
            AnimationMessage::ToggleAnimate => self.animate_enabled = !self.animate_enabled,
            AnimationMessage::PositionChanged(v) => self.box_position = v,
            AnimationMessage::SizeChanged(v) => self.box_size = v,
            AnimationMessage::ColorPhaseChanged(v) => self.box_color_phase = v,
            AnimationMessage::PulseTick => {
                if self.animate_enabled {
                    self.pulse_phase = (self.pulse_phase + 0.05) % 1.0;
                }
            }
            AnimationMessage::Reset => {
                self.box_position = 0.0;
                self.box_size = 50.0;
                self.box_color_phase = 0.0;
                self.pulse_phase = 0.0;
                self.animate_enabled = false;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let stage = container(
            container(Space::new())
                .width(iced::Length::Fixed(self.box_size + self.pulse_phase * 30.0))
                .height(iced::Length::Fixed(self.box_size + self.pulse_phase * 30.0))
                .style(move |_theme: &iced::Theme| container::Style {
                    background: Some(iced::Background::Color(hsl_color(
                        self.box_color_phase,
                        0.7,
                        0.5,
                    ))),
                    border: iced::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: iced::border::Radius::new(8.0),
                    },
                    ..Default::default()
                }),
        )
        .width(Fill)
        .height(iced::Length::Fixed(160.0))
        .padding(iced::Padding {
            top: 20.0,
            right: 20.0 + self.box_position * 3.0,
            bottom: 20.0,
            left: 20.0,
        })
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.97))),
            border: iced::Border {
                color: Color::from_rgb(0.85, 0.85, 0.9),
                width: 1.0,
                radius: iced::border::Radius::new(8.0),
            },
            ..Default::default()
        })
        .align_x(if self.box_position < 33.0 {
            Alignment::Start
        } else if self.box_position < 66.0 {
            Alignment::Center
        } else {
            Alignment::End
        });

        let controls_title = iced::widget::text("Animation Controls")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let pos_row = row![
            iced::widget::text("Position:").size(13).width(60),
            slider(0.0..=100.0, self.box_position, |v| Message::Animation(AnimationMessage::PositionChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.box_position)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let size_row = row![
            iced::widget::text("Size:").size(13).width(60),
            slider(20.0..=100.0, self.box_size, |v| Message::Animation(AnimationMessage::SizeChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.box_size)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let color_row = row![
            iced::widget::text("Hue:").size(13).width(60),
            slider(0.0..=360.0, self.box_color_phase, |v| Message::Animation(AnimationMessage::ColorPhaseChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.box_color_phase)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let btn_row = row![
            button(iced::widget::text(if self.animate_enabled { "Stop" } else { "Start" }).size(13))
                .padding([8, 16])
                .on_press(Message::Animation(AnimationMessage::ToggleAnimate)),
            button(iced::widget::text("Reset").size(13))
                .padding([8, 16])
                .on_press(Message::Animation(AnimationMessage::Reset)),
        ]
        .spacing(10);

        let pulse_info = iced::widget::text(format!("Pulse phase: {:.2}", self.pulse_phase))
            .size(12)
            .color(Color::from_rgb(0.5, 0.5, 0.5));

        let controls = card(
            column![controls_title, pos_row, size_row, color_row, btn_row, pulse_info].spacing(10).into(),
        );

        let info = card(column![
            iced::widget::text("About Animation").size(16).color(Color::from_rgb(0.4, 0.4, 0.5)),
            iced::widget::text("This demo shows state-driven animation. In real apps, combine Task and time module for smoother animations.").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
        ].spacing(8).into());

        column![stage, controls, info]
            .spacing(16)
            .into()
    }
}

fn hsl_color(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::from_rgb(r + m, g + m, b + m)
}

fn card(content: Element<'_, Message>) -> Element<'_, Message> {
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
