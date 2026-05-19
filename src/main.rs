// 隐藏 Windows 控制台窗口
#![cfg_attr(windows, windows_subsystem = "windows")]

use iced::widget::{column, container, row, rule, Space};
use iced::{Element, Fill, Task, Theme};

mod pages;

use pages::{
    animation::{AnimationMessage, AnimationPage},
    canvas::{CanvasMessage, CanvasPage},
    counter::{CounterMessage, CounterPage},
    layout::{LayoutMessage, LayoutPage},
    slider::{SliderMessage, SliderPage},
    text::{TextMessage, TextPage},
    theme::{ThemeMessage, ThemePage},
    Page,
};

// ─── 程序入口 ──────────────────────────────────────────────────────────────

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Rust Iced Demo")
        .theme(App::theme)
        .window_size((1000.0, 700.0))
        .run()
}

// ─── 应用状态 ──────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct App {
    current_page: Page,
    dark_mode: bool,
    counter: CounterPage,
    text: TextPage,
    slider: SliderPage,
    layout: LayoutPage,
    canvas: CanvasPage,
    animation: AnimationPage,
    theme: ThemePage,
}

// ─── 消息类型 ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    ToggleDarkMode(bool),
    Counter(CounterMessage),
    Text(TextMessage),
    Slider(SliderMessage),
    Layout(LayoutMessage),
    Canvas(CanvasMessage),
    Animation(AnimationMessage),
    Theme(ThemeMessage),
}

// ─── 核心逻辑 ──────────────────────────────────────────────────────────────

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.current_page = page,
            Message::ToggleDarkMode(v) => self.dark_mode = v,
            Message::Counter(msg) => self.counter.update(msg),
            Message::Text(msg) => self.text.update(msg),
            Message::Slider(msg) => self.slider.update(msg),
            Message::Layout(msg) => self.layout.update(msg),
            Message::Canvas(msg) => self.canvas.update(msg),
            Message::Animation(msg) => self.animation.update(msg),
            Message::Theme(msg) => self.theme.update(msg),
        }
        Task::none()
    }

    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let sidebar = pages::sidebar(self.current_page, self.dark_mode);

        let content = match self.current_page {
            Page::Welcome => welcome_view(self.dark_mode),
            Page::Counter => pages::content_wrapper("Counter", self.dark_mode, self.counter.view()),
            Page::TextInputs => pages::content_wrapper("Text Inputs", self.dark_mode, self.text.view()),
            Page::Sliders => pages::content_wrapper("Sliders & Progress", self.dark_mode, self.slider.view()),
            Page::Layout => pages::content_wrapper("Layout", self.dark_mode, self.layout.view()),
            Page::Canvas => pages::content_wrapper("Canvas Drawing", self.dark_mode, self.canvas.view()),
            Page::Animation => pages::content_wrapper("Animation", self.dark_mode, self.animation.view()),
            Page::Theme => pages::content_wrapper("Theme & Style", self.dark_mode, self.theme.view()),
        };

        row![sidebar, content].into()
    }
}

// ─── 欢迎页面 ──────────────────────────────────────────────────────────────

fn welcome_view(dark: bool) -> Element<'static, Message> {
    let title = iced::widget::text("Welcome to Rust Iced Demo")
        .size(32)
        .color(if dark {
            iced::Color::from_rgb(0.75, 0.85, 1.0)
        } else {
            iced::Color::from_rgb(0.1, 0.25, 0.55)
        });

    let subtitle = iced::widget::text("A Rust + Iced 0.14 GUI learning project")
        .size(15)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.55));

    let divider = rule::horizontal(2);

    let intro = iced::widget::text("The left sidebar contains multiple independent UI demo pages, each showcasing different Iced features:")
        .size(14)
        .color(iced::Color::from_rgb(0.4, 0.4, 0.45));

    let features = vec![
        ("Counter", "Basic counter, step counter, bounded counter and history"),
        ("Text Inputs", "Form inputs, password field, checkbox, live preview"),
        ("Sliders", "Slider controls, RGB color picker, volume control, simulated download"),
        ("Layout", "Row, Column, Center, Stack and more layout patterns"),
        ("Canvas", "Custom drawing, geometric shapes, grid, rotation"),
        ("Animation", "State-driven animation, position/size/color changes"),
        ("Theme", "Button styles, text styles, container radius, color preview"),
    ];

    let mut feature_list = column![].spacing(10);
    for (name, desc) in features {
        feature_list = feature_list.push(
            row![
                container(iced::widget::text("*").size(10).color(iced::Color::from_rgb(0.3, 0.6, 0.9)))
                    .width(iced::Length::Fixed(24.0))
                    .align_x(iced::Alignment::Center),
                column![
                    iced::widget::text(name).size(14).color(iced::Color::from_rgb(0.2, 0.4, 0.7)),
                    iced::widget::text(desc).size(12).color(iced::Color::from_rgb(0.5, 0.5, 0.55)),
                ]
                .spacing(2),
            ]
            .align_y(iced::Alignment::Center),
        );
    }

    let footer = iced::widget::text("Version 0.1.0 · Iced 0.14 · Rust 1.90+")
        .size(11)
        .color(iced::Color::from_rgb(0.55, 0.55, 0.6));

    let body = column![
        title,
        subtitle,
        divider,
        intro,
        feature_list,
        Space::new(),
        footer,
    ]
    .spacing(16)
    .padding(30);

    container(body)
        .width(Fill)
        .height(Fill)
        .into()
}
