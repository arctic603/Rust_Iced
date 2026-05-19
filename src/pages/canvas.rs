//! Canvas 绘图页面 —— 自定义绘制
//!
//! 本页面演示 Iced Canvas 的高级用法：
//! - Cache：缓存几何图形，避免每帧重新计算
//! - Frame：绘制路径（矩形、圆形、直线）
//! - 坐标变换：translate、rotate
//! - 自定义样式：fill 填充、stroke 描边
//! - 文字绘制：在 Canvas 上输出文本

use iced::widget::canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::widget::{column, container, row, slider};
use iced::{Color, Element, Fill, Length, Point, Rectangle, Renderer, Theme, Vector};

use crate::Message;

/// Canvas 绘图页面状态
#[derive(Debug)]
pub struct CanvasPage {
    pub cache: Cache,         // 绘制缓存，用于性能优化
    pub circle_radius: f32,   // 圆形半径（10~120）
    pub rotation: f32,        // 旋转角度（0~360 度）
}

impl Default for CanvasPage {
    fn default() -> Self {
        Self {
            cache: Cache::default(),
            circle_radius: 50.0,
            rotation: 0.0,
        }
    }
}

/// Canvas 页面消息
#[derive(Debug, Clone)]
pub enum CanvasMessage {
    RadiusChanged(f32),    // 圆形半径滑块变化
    RotationChanged(f32),  // 旋转角度滑块变化
    ClearCache,            // 手动清除绘制缓存
}

impl CanvasPage {
    /// 更新 Canvas 页面状态
    /// 当半径或旋转角度变化时，需要清除缓存以触发重绘
    pub fn update(&mut self, msg: CanvasMessage) {
        match msg {
            CanvasMessage::RadiusChanged(v) => {
                self.circle_radius = v;
                self.cache.clear();
            }
            CanvasMessage::RotationChanged(v) => {
                self.rotation = v;
                self.cache.clear();
            }
            CanvasMessage::ClearCache => self.cache.clear(),
        }
    }

    /// 构建 Canvas 页面视图
    /// 上方是 Canvas 绘图区，下方是参数控制滑块
    pub fn view(&self) -> Element<'_, Message> {
        let canvas_widget = Canvas::new(self)
            .width(Fill)
            .height(Length::Fixed(300.0));

        let radius_slider = row![
            iced::widget::text("Radius:").size(13),
            slider(10.0..=120.0, self.circle_radius, |v| Message::Canvas(CanvasMessage::RadiusChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.circle_radius)).size(12).width(30),
        ]
        .spacing(8);

        let rot_slider = row![
            iced::widget::text("Rotation:").size(13),
            slider(0.0..=360.0, self.rotation, |v| Message::Canvas(CanvasMessage::RotationChanged(v))).width(Fill),
            iced::widget::text(format!("{:.0}", self.rotation)).size(12).width(30),
        ]
        .spacing(8);

        let clear_btn = iced::widget::button(iced::widget::text("Clear Cache").size(12))
            .padding([6, 14])
            .on_press(Message::Canvas(CanvasMessage::ClearCache));

        let controls = column![radius_slider, rot_slider, clear_btn].spacing(10);

        column![
            container(canvas_widget)
                .width(Fill)
                .height(Length::Fixed(300.0))
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.95, 0.95, 0.97))),
                    border: iced::Border {
                        color: Color::from_rgb(0.85, 0.85, 0.9),
                        width: 1.0,
                        radius: iced::border::Radius::new(8.0),
                    },
                    ..Default::default()
                }),
            controls,
        ]
        .spacing(12)
        .into()
    }
}

impl canvas::Program<Message> for CanvasPage {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);

            draw_grid(frame, &bounds);

            frame.with_save(|frame| {
                frame.translate(Vector::new(center.x, center.y));
                frame.rotate(iced::Degrees(self.rotation));
                frame.fill(
                    &Path::rectangle(Point::new(-40.0, -40.0), iced::Size::new(80.0, 80.0)),
                    Color::from_rgb(0.3, 0.6, 0.9),
                );
                frame.stroke(
                    &Path::rectangle(Point::new(-40.0, -40.0), iced::Size::new(80.0, 80.0)),
                    Stroke::default().with_width(2.0).with_color(Color::from_rgb(0.1, 0.4, 0.7)),
                );
            });

            frame.fill(
                &Path::circle(center, self.circle_radius),
                Color::from_rgba(0.9, 0.4, 0.3, 0.6),
            );
            frame.stroke(
                &Path::circle(center, self.circle_radius),
                Stroke::default().with_width(2.0).with_color(Color::from_rgb(0.7, 0.2, 0.1)),
            );

            frame.stroke(
                &Path::line(
                    Point::new(0.0, center.y),
                    Point::new(bounds.width, center.y),
                ),
                Stroke::default().with_width(1.0).with_color(Color::from_rgb(0.7, 0.7, 0.75)),
            );
            frame.stroke(
                &Path::line(
                    Point::new(center.x, 0.0),
                    Point::new(center.x, bounds.height),
                ),
                Stroke::default().with_width(1.0).with_color(Color::from_rgb(0.7, 0.7, 0.75)),
            );

            let text = Text {
                content: format!("Radius: {:.0}px", self.circle_radius),
                position: Point::new(10.0, 20.0),
                color: Color::from_rgb(0.3, 0.3, 0.4),
                size: iced::Pixels(13.0),
                ..Default::default()
            };
            frame.fill_text(text);
        });

        vec![geometry]
    }
}

/// 在 Canvas 上绘制网格背景
/// step: 网格间距（像素）
fn draw_grid(frame: &mut Frame, bounds: &Rectangle) {
    let grid_color = Color::from_rgb(0.9, 0.9, 0.92);
    let step = 30.0;

    for x in (0..=bounds.width as i32).step_by(step as usize) {
        frame.stroke(
            &Path::line(
                Point::new(x as f32, 0.0),
                Point::new(x as f32, bounds.height),
            ),
            Stroke::default().with_width(0.5).with_color(grid_color),
        );
    }

    for y in (0..=bounds.height as i32).step_by(step as usize) {
        frame.stroke(
            &Path::line(
                Point::new(0.0, y as f32),
                Point::new(bounds.width, y as f32),
            ),
            Stroke::default().with_width(0.5).with_color(grid_color),
        );
    }
}
