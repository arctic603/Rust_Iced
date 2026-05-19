//! 滑块与进度页面 —— 各种进度控件

use iced::widget::{button, column, container, progress_bar, row, slider, Space};
use iced::{Alignment, Color, Element, Fill};

use crate::Message;

#[derive(Debug, Default)]
pub struct SliderPage {
    pub basic: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub volume: f32,
    pub is_playing: bool,
    pub download_progress: f32,
    pub is_downloading: bool,
}

#[derive(Debug, Clone)]
pub enum SliderMessage {
    BasicChanged(f32),
    RedChanged(f32),
    GreenChanged(f32),
    BlueChanged(f32),
    VolumeChanged(f32),
    TogglePlay,
    Download,
    DownloadTick,
}

impl SliderPage {
    pub fn update(&mut self, msg: SliderMessage) {
        match msg {
            SliderMessage::BasicChanged(v) => self.basic = v,
            SliderMessage::RedChanged(v) => self.red = v,
            SliderMessage::GreenChanged(v) => self.green = v,
            SliderMessage::BlueChanged(v) => self.blue = v,
            SliderMessage::VolumeChanged(v) => self.volume = v,
            SliderMessage::TogglePlay => self.is_playing = !self.is_playing,
            SliderMessage::Download => {
                self.is_downloading = true;
                self.download_progress = 0.0;
            }
            SliderMessage::DownloadTick => {
                if self.is_downloading {
                    self.download_progress += 2.0;
                    if self.download_progress >= 100.0 {
                        self.download_progress = 100.0;
                        self.is_downloading = false;
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let basic_label = iced::widget::text(format!("Basic Slider: {:.0}%", self.basic))
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let basic_slider = slider(0.0..=100.0, self.basic, |v| Message::Slider(SliderMessage::BasicChanged(v)))
            .width(Fill)
            .step(1.0);
        let basic_bar = progress_bar(0.0..=100.0, self.basic);
        let basic_card = card(column![basic_label, basic_slider, basic_bar].spacing(10).into());

        let rgb_label = iced::widget::text("RGB Color Picker")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));

        let red_row = row![
            iced::widget::text("R").size(14).color(Color::from_rgb(0.9, 0.2, 0.2)).width(20),
            slider(0.0..=255.0, self.red, |v| Message::Slider(SliderMessage::RedChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.red)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let green_row = row![
            iced::widget::text("G").size(14).color(Color::from_rgb(0.2, 0.8, 0.2)).width(20),
            slider(0.0..=255.0, self.green, |v| Message::Slider(SliderMessage::GreenChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.green)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let blue_row = row![
            iced::widget::text("B").size(14).color(Color::from_rgb(0.2, 0.4, 0.9)).width(20),
            slider(0.0..=255.0, self.blue, |v| Message::Slider(SliderMessage::BlueChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.blue)).size(12).width(30),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        let color_preview = container(Space::new())
            .width(Fill)
            .height(60)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(
                    self.red / 255.0,
                    self.green / 255.0,
                    self.blue / 255.0,
                ))),
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.8, 0.85),
                    width: 2.0,
                    radius: iced::border::Radius::new(6.0),
                },
                ..Default::default()
            });

        let rgb_card = card(column![rgb_label, red_row, green_row, blue_row, color_preview].spacing(10).into());

        let vol_label = iced::widget::text(format!("Volume: {:.0}%", self.volume))
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let vol_slider = slider(0.0..=100.0, self.volume, |v| Message::Slider(SliderMessage::VolumeChanged(v)))
            .width(Fill)
            .step(1.0);
        let play_btn = button(iced::widget::text(if self.is_playing { "Pause" } else { "Play" }).size(13))
            .padding([8, 20])
            .on_press(Message::Slider(SliderMessage::TogglePlay));
        let vol_card = card(column![vol_label, vol_slider, play_btn].spacing(10).into());

        let dl_label = iced::widget::text("Simulated Download")
            .size(16)
            .color(Color::from_rgb(0.4, 0.4, 0.5));
        let dl_bar = progress_bar(0.0..=100.0, self.download_progress);
        let dl_status = iced::widget::text(format!("{:.0}% {}",
            self.download_progress,
            if self.is_downloading { "Downloading..." } else if self.download_progress >= 100.0 { "Complete" } else { "Waiting" }
        )).size(13).color(Color::from_rgb(0.4, 0.4, 0.5));
        let dl_btn = button(iced::widget::text(if self.is_downloading { "Downloading..." } else { "Start Download" }).size(13))
            .padding([8, 20])
            .on_press(Message::Slider(SliderMessage::Download));
        let dl_card = card(column![dl_label, dl_bar, dl_status, dl_btn].spacing(10).into());

        column![basic_card, rgb_card, vol_card, dl_card]
            .spacing(16)
            .into()
    }
}

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
