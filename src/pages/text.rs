//! 文本输入页面 —— 各种输入控件演示

use iced::widget::{button, column, container, row, Space};
use iced::{Color, Element, Fill};

use crate::Message;

#[derive(Debug, Default)]
pub struct TextPage {
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub show_password: bool,
    pub agree_terms: bool,
    pub subscribe: bool,
    pub age: String,
}

#[derive(Debug, Clone)]
pub enum TextMessage {
    NameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    BioChanged(String),
    TogglePassword(bool),
    ToggleTerms(bool),
    ToggleSubscribe(bool),
    AgeChanged(String),
    Submit,
}

impl TextPage {
    pub fn update(&mut self, msg: TextMessage) {
        match msg {
            TextMessage::NameChanged(v) => self.name = v,
            TextMessage::EmailChanged(v) => self.email = v,
            TextMessage::PasswordChanged(v) => self.password = v,
            TextMessage::BioChanged(v) => self.bio = v,
            TextMessage::TogglePassword(v) => self.show_password = v,
            TextMessage::ToggleTerms(v) => self.agree_terms = v,
            TextMessage::ToggleSubscribe(v) => self.subscribe = v,
            TextMessage::AgeChanged(v) => {
                if v.chars().all(|c| c.is_ascii_digit()) {
                    self.age = v;
                }
            }
            TextMessage::Submit => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let form_title = iced::widget::text("User Registration Form")
            .size(18)
            .color(Color::from_rgb(0.3, 0.3, 0.5));

        let name_input = iced::widget::text_input("Enter name", &self.name)
            .on_input(|s| Message::Text(TextMessage::NameChanged(s)))
            .padding(10)
            .size(14);

        let email_input = iced::widget::text_input("Enter email", &self.email)
            .on_input(|s| Message::Text(TextMessage::EmailChanged(s)))
            .padding(10)
            .size(14);

        let password_input = iced::widget::text_input("Enter password", &self.password)
            .on_input(|s| Message::Text(TextMessage::PasswordChanged(s)))
            .padding(10)
            .size(14);

        let show_pwd = iced::widget::checkbox(self.show_password)
            .label("Show password")
            .on_toggle(|v| Message::Text(TextMessage::TogglePassword(v)));

        let age_input = iced::widget::text_input("Age (digits only)", &self.age)
            .on_input(|s| Message::Text(TextMessage::AgeChanged(s)))
            .padding(10)
            .size(14);

        let bio_input = iced::widget::text_input("Bio (one line)", &self.bio)
            .on_input(|s| Message::Text(TextMessage::BioChanged(s)))
            .padding(10)
            .size(14);

        let terms = iced::widget::checkbox(self.agree_terms)
            .label("I agree to the terms")
            .on_toggle(|v| Message::Text(TextMessage::ToggleTerms(v)));

        let subscribe = iced::widget::toggler(self.subscribe)
            .label("Subscribe to newsletter")
            .on_toggle(|v| Message::Text(TextMessage::ToggleSubscribe(v)));

        let submit_btn = button(iced::widget::text("Submit").size(14).align_x(iced::widget::text::Alignment::Center))
            .width(120)
            .padding([10, 20])
            .on_press(Message::Text(TextMessage::Submit));

        let form = card(column![
            form_title,
            iced::widget::text("Name").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
            name_input,
            iced::widget::text("Email").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
            email_input,
            iced::widget::text("Password").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
            password_input,
            show_pwd,
            iced::widget::text("Age").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
            age_input,
            iced::widget::text("Bio").size(12).color(Color::from_rgb(0.5, 0.5, 0.5)),
            bio_input,
            terms,
            subscribe,
            submit_btn,
        ]
        .spacing(8)
        .into());

        let preview_title = iced::widget::text("Live Preview")
            .size(18)
            .color(Color::from_rgb(0.3, 0.3, 0.5));

        let pwd_mask = if self.password.is_empty() {
            "(empty)".to_string()
        } else {
            "*".repeat(self.password.len())
        };
        let preview_content = format!(
            "Name: {}\nEmail: {}\nPassword: {}\nAge: {}\nBio: {}\nAgree: {}\nSubscribe: {}",
            if self.name.is_empty() { "(empty)" } else { &self.name },
            if self.email.is_empty() { "(empty)" } else { &self.email },
            pwd_mask,
            if self.age.is_empty() { "(empty)" } else { &self.age },
            if self.bio.is_empty() { "(empty)" } else { &self.bio },
            if self.agree_terms { "Yes" } else { "No" },
            if self.subscribe { "Yes" } else { "No" },
        );

        let preview_text = iced::widget::text(preview_content)
            .size(13)
            .color(Color::from_rgb(0.3, 0.35, 0.45));

        let preview = card(column![preview_title, preview_text].spacing(10).into());

        row![form, Space::new(), preview]
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
