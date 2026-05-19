// 隐藏 Windows 控制台窗口
// 加上这行后，运行程序时不会弹出额外的 CMD 黑框
#![cfg_attr(windows, windows_subsystem = "windows")]

use iced::widget::{column, container, row, rule, Space};
use iced::{Element, Fill, Task, Theme};

mod pages;

use pages::{
    animation::{AnimationMessage, AnimationPage},
    canvas::{CanvasMessage, CanvasPage},
    chart::{ChartMessage, ChartPage},
    counter::{CounterMessage, CounterPage},
    layout::{LayoutMessage, LayoutPage},
    slider::{SliderMessage, SliderPage},
    text::{TextMessage, TextPage},
    theme::{ThemeMessage, ThemePage},
    Page,
};

// ─── 程序入口 ──────────────────────────────────────────────────────────────

/// 程序入口函数
/// 使用 iced::application() 构建器创建应用，指定：
/// - 初始状态：App::default()
/// - 更新逻辑：App::update()
/// - 视图逻辑：App::view()
/// - 窗口标题、主题、默认尺寸
pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Rust Iced Demo")
        .theme(App::theme)
        .window_size((1000.0, 700.0))
        .run()
}

// ─── 应用状态 ──────────────────────────────────────────────────────────────

/// 应用全局状态结构体
/// 保存当前页面、深色模式开关，以及各个子页面的独立状态
#[derive(Debug, Default)]
struct App {
    current_page: Page,       // 当前显示的页面
    dark_mode: bool,          // 是否为深色模式
    counter: CounterPage,     // 计数器页面状态
    text: TextPage,           // 文本输入页面状态
    slider: SliderPage,       // 滑块页面状态
    layout: LayoutPage,       // 布局页面状态
    canvas: CanvasPage,       // Canvas 绘图页面状态
    chart: ChartPage,         // 折线图页面状态
    animation: AnimationPage, // 动画页面状态
    theme: ThemePage,         // 主题样式页面状态
}

// ─── 消息类型 ──────────────────────────────────────────────────────────────

/// 全局消息枚举
/// Iced 采用消息驱动架构：用户操作产生消息，update() 接收消息并更新状态
/// 每个子页面有自己的子消息类型，通过嵌套枚举聚合到 Message 中
#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),              // 切换页面导航
    ToggleDarkMode(bool),        // 切换深色/浅色模式
    Counter(CounterMessage),     // 计数器页面消息（嵌套）
    Text(TextMessage),           // 文本输入页面消息（嵌套）
    Slider(SliderMessage),       // 滑块页面消息（嵌套）
    Layout(LayoutMessage),       // 布局页面消息（嵌套）
    Canvas(CanvasMessage),       // Canvas 页面消息（嵌套）
    Chart(ChartMessage),         // 折线图页面消息（嵌套）
    Animation(AnimationMessage), // 动画页面消息（嵌套）
    Theme(ThemeMessage),         // 主题页面消息（嵌套）
}

// ─── 核心逻辑 ──────────────────────────────────────────────────────────────

impl App {
    /// update 是 Iced 应用的核心：接收消息，更新状态
    /// 返回值 Task<Message> 用于异步操作（如定时器、HTTP 请求），这里统一返回 Task::none()
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => self.current_page = page,
            Message::ToggleDarkMode(v) => self.dark_mode = v,
            // 将子消息转发给对应页面的 update 方法处理
            Message::Counter(msg) => self.counter.update(msg),
            Message::Text(msg) => self.text.update(msg),
            Message::Slider(msg) => self.slider.update(msg),
            Message::Layout(msg) => self.layout.update(msg),
            Message::Canvas(msg) => self.canvas.update(msg),
            Message::Chart(msg) => self.chart.update(msg),
            Message::Animation(msg) => self.animation.update(msg),
            Message::Theme(msg) => self.theme.update(msg),
        }
        Task::none()
    }

    /// 根据 dark_mode 状态返回当前主题（Light 或 Dark）
    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    /// view 负责根据当前状态构建 UI 树
    /// 左侧是导航栏，右侧是当前页面的内容区域
    fn view(&self) -> Element<'_, Message> {
        let sidebar = pages::sidebar(self.current_page, self.dark_mode);

        // 根据当前页面路由，渲染对应子页面的视图
        let content = match self.current_page {
            Page::Welcome => welcome_view(self.dark_mode),
            Page::Counter => pages::content_wrapper("Counter", self.dark_mode, self.counter.view()),
            Page::TextInputs => pages::content_wrapper("Text Inputs", self.dark_mode, self.text.view()),
            Page::Sliders => pages::content_wrapper("Sliders & Progress", self.dark_mode, self.slider.view()),
            Page::Layout => pages::content_wrapper("Layout", self.dark_mode, self.layout.view()),
            Page::Canvas => pages::content_wrapper("Canvas Drawing", self.dark_mode, self.canvas.view()),
            Page::Chart => pages::content_wrapper("Line Chart", self.dark_mode, self.chart.view()),
            Page::Animation => pages::content_wrapper("Animation", self.dark_mode, self.animation.view()),
            Page::Theme => pages::content_wrapper("Theme & Style", self.dark_mode, self.theme.view()),
        };

        // 左侧导航栏 + 右侧内容区，水平排列
        row![sidebar, content].into()
    }
}

// ─── 欢迎页面 ──────────────────────────────────────────────────────────────

/// 欢迎页面视图
/// 展示项目标题、简介和左侧各功能模块的索引说明
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
        ("Line Chart", "Line charts with multiple series, axes, grid, legend, data switching"),
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
