//! 页面模块 —— 所有子页面 Demo 的入口

pub mod animation;
pub mod canvas;
pub mod counter;
pub mod layout;
pub mod slider;
pub mod text;
pub mod theme;

use iced::widget::{button, column, container, scrollable, Space};
use iced::{Element, Fill, Theme};

use crate::Message;

/// 页面路由枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Welcome,
    Counter,
    TextInputs,
    Sliders,
    Layout,
    Canvas,
    Animation,
    Theme,
}

impl Page {
    /// 页面标题
    pub fn title(self) -> &'static str {
        match self {
            Page::Welcome => "Welcome",
            Page::Counter => "Counter",
            Page::TextInputs => "Text Inputs",
            Page::Sliders => "Sliders",
            Page::Layout => "Layout",
            Page::Canvas => "Canvas",
            Page::Animation => "Animation",
            Page::Theme => "Theme",
        }
    }

    /// 页面图标
    pub fn icon(self) -> &'static str {
        match self {
            Page::Welcome => "*",
            Page::Counter => "#",
            Page::TextInputs => "T",
            Page::Sliders => "=",
            Page::Layout => "[]",
            Page::Canvas => "@",
            Page::Animation => "~",
            Page::Theme => "O",
        }
    }

    /// 所有页面列表
    pub const ALL: [Page; 8] = [
        Page::Welcome,
        Page::Counter,
        Page::TextInputs,
        Page::Sliders,
        Page::Layout,
        Page::Canvas,
        Page::Animation,
        Page::Theme,
    ];
}

/// 左侧导航侧边栏
pub fn sidebar<'a>(current: Page, dark: bool) -> Element<'a, Message> {
    let logo = iced::widget::text("Iced Demo")
        .size(22)
        .color(if dark {
            iced::Color::from_rgb(0.8, 0.85, 0.95)
        } else {
            iced::Color::from_rgb(0.15, 0.25, 0.5)
        });

    let subtitle = iced::widget::text("Rust GUI Learning")
        .size(11)
        .color(iced::Color::from_rgb(0.5, 0.5, 0.5));

    let mut nav_items = column![logo, subtitle].spacing(4).padding(16);

    for page in Page::ALL {
        let is_active = page == current;
        let label = format!("{}  {}", page.icon(), page.title());

        let btn = button(iced::widget::text(label).size(13))
            .width(Fill)
            .padding([8, 12])
            .style(move |theme: &Theme, status| {
                let mut style = if is_active {
                    button::primary(theme, status)
                } else {
                    button::secondary(theme, status)
                };
                if is_active {
                    style.background = Some(iced::Background::Color(
                        if dark {
                            iced::Color::from_rgb(0.2, 0.35, 0.65)
                        } else {
                            iced::Color::from_rgb(0.2, 0.4, 0.8)
                        },
                    ));
                } else if dark {
                    style.background = Some(iced::Background::Color(
                        iced::Color::from_rgb(0.12, 0.12, 0.14),
                    ));
                }
                style
            })
            .on_press(Message::Navigate(page));

        nav_items = nav_items.push(btn);
    }

    container(scrollable(nav_items))
        .width(180)
        .height(Fill)
        .style(move |_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(if dark {
                iced::Color::from_rgb(0.08, 0.08, 0.1)
            } else {
                iced::Color::from_rgb(0.96, 0.96, 0.98)
            })),
            border: iced::Border {
                color: if dark {
                    iced::Color::from_rgb(0.15, 0.15, 0.18)
                } else {
                    iced::Color::from_rgb(0.85, 0.85, 0.9)
                },
                width: 1.0,
                radius: iced::border::Radius::new(0.0),
            },
            ..Default::default()
        })
        .into()
}

/// 页面内容包装器 —— 添加统一的标题和滚动
pub fn content_wrapper<'a>(
    title: &'static str,
    dark: bool,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    let header = iced::widget::text(title)
        .size(28)
        .color(if dark {
            iced::Color::from_rgb(0.85, 0.9, 1.0)
        } else {
            iced::Color::from_rgb(0.1, 0.2, 0.5)
        });

    let body = column![header, Space::new(), content]
        .spacing(16)
        .padding(24);

    scrollable(container(body).width(Fill).height(Fill)).into()
}
