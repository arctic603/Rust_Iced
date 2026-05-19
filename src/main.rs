use iced::{
    alignment,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, progress_bar, row, slider,
        text, text_input, toggler,
    },
    Alignment, Color, Element, Fill, Length, Task, Theme,
};

/// 程序入口 —— iced 0.13 新式函数调用
pub fn main() -> iced::Result {
    iced::application("🦀 Rust Iced Demo", IcedDemo::update, IcedDemo::view)
        .theme(IcedDemo::theme)
        .window_size((800.0, 600.0))
        .run()
}

// ─── 应用状态 ──────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct IcedDemo {
    /// 计数器值
    count: i32,
    /// 滑块值 0.0 ~ 100.0
    slider_value: f32,
    /// 文本输入框内容
    input_text: String,
    /// 是否开启深色主题
    dark_mode: bool,
}

// ─── 消息类型 ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
    SliderChanged(f32),
    InputChanged(String),
    ToggleTheme(bool),
}

// ─── 核心逻辑 ──────────────────────────────────────────────────────────────────

impl IcedDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
            Message::Reset => self.count = 0,
            Message::SliderChanged(v) => self.slider_value = v,
            Message::InputChanged(s) => self.input_text = s,
            Message::ToggleTheme(v) => self.dark_mode = v,
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
        // ── 标题区域 ─────────────────────────────────────────────────────────
        let title = text("🦀 Rust Iced Demo")
            .size(32)
            .color(if self.dark_mode {
                Color::from_rgb(0.6, 0.85, 1.0)
            } else {
                Color::from_rgb(0.1, 0.3, 0.65)
            });

        let subtitle = text("一个用 Rust + Iced 0.13 构建的 GUI 演示程序")
            .size(15)
            .color(Color::from_rgb(0.5, 0.5, 0.5));

        // ── 主题切换 ─────────────────────────────────────────────────────────
        let theme_toggle = row![
            text("深色模式").size(14),
            horizontal_space(),
            toggler(self.dark_mode)
                .label("Dark Mode")
                .on_toggle(Message::ToggleTheme),
        ]
        .align_y(Alignment::Center)
        .spacing(8);

        // ── 计数器面板 ───────────────────────────────────────────────────────
        let counter_label = text("📊 计数器").size(18);

        let count_color = if self.count > 0 {
            Color::from_rgb(0.1, 0.7, 0.3)
        } else if self.count < 0 {
            Color::from_rgb(0.9, 0.2, 0.2)
        } else {
            Color::from_rgb(0.45, 0.45, 0.45)
        };

        let count_display = text(self.count.to_string())
            .size(56)
            .color(count_color);

        let btn_dec = button(
            text("－")
                .size(22)
                .align_x(alignment::Horizontal::Center),
        )
        .width(60)
        .padding(10)
        .on_press(Message::Decrement);

        let btn_inc = button(
            text("＋")
                .size(22)
                .align_x(alignment::Horizontal::Center),
        )
        .width(60)
        .padding(10)
        .on_press(Message::Increment);

        let btn_reset = button(
            text("重置")
                .size(15)
                .align_x(alignment::Horizontal::Center),
        )
        .width(90)
        .padding([8, 16])
        .on_press(Message::Reset);

        let counter_btns = row![btn_dec, count_display, btn_inc]
            .align_y(Alignment::Center)
            .spacing(20);

        let counter_panel = column![counter_label, counter_btns, btn_reset]
            .spacing(12)
            .align_x(Alignment::Center);

        // ── 滑块面板 ─────────────────────────────────────────────────────────
        let slider_label = text(format!("🎚 进度滑块: {:.0}%", self.slider_value)).size(18);

        let slider_widget = slider(0.0..=100.0, self.slider_value, Message::SliderChanged)
            .width(Fill)
            .step(1.0);

        let progress = progress_bar(0.0..=100.0, self.slider_value).height(18);

        let slider_panel = column![slider_label, slider_widget, progress].spacing(10);

        // ── 文本输入面板 ─────────────────────────────────────────────────────
        let input_label = text("📝 文本输入").size(18);

        let input = text_input("在此输入文字…", &self.input_text)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(15);

        let echo = if self.input_text.is_empty() {
            text("（等待输入…）")
                .size(14)
                .color(Color::from_rgb(0.6, 0.6, 0.6))
        } else {
            text(format!("✅ 你输入了：{}", self.input_text))
                .size(14)
                .color(Color::from_rgb(0.15, 0.65, 0.4))
        };

        let input_panel = column![input_label, input, echo].spacing(10);

        // ── 状态信息栏 ───────────────────────────────────────────────────────
        let status = text(format!(
            "Rust Iced Demo v{} · iced 0.13 · 主题: {}",
            env!("CARGO_PKG_VERSION"),
            if self.dark_mode { "🌙 深色" } else { "☀️ 浅色" }
        ))
        .size(12)
        .color(Color::from_rgb(0.5, 0.5, 0.5));

        // ── 整体布局 ─────────────────────────────────────────────────────────
        let header = row![title, horizontal_space(), theme_toggle]
            .align_y(Alignment::Center);

        let content = column![
            header,
            subtitle,
            horizontal_rule(2),
            counter_panel,
            horizontal_rule(1),
            slider_panel,
            horizontal_rule(1),
            input_panel,
            horizontal_rule(1),
            status,
        ]
        .spacing(20)
        .padding(30);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
