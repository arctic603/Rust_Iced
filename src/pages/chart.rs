//! 折线图页面 —— 使用 Canvas 自定义绘制多种数据可视化图表
//!
//! 本页面演示如何在 Iced 中利用 Canvas 实现数据可视化：
//! - 坐标轴与网格线
//! - 多条折线叠加显示
//! - 数据点标记
//! - 图例说明
//! - 动态切换数据集

use iced::widget::canvas::{self, Cache, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::widget::{button, column, container, row, Space};
use iced::{Color, Element, Fill, Length, Point, Rectangle, Renderer, Size, Theme, Vector};

use crate::Message;

/// 图表页面状态
#[derive(Debug)]
pub struct ChartPage {
    /// Canvas 绘制缓存，用于优化重绘性能
    cache: Cache,
    /// 当前选中的数据集索引
    dataset_index: usize,
    /// 是否显示数据点
    show_points: bool,
    /// 是否显示网格
    show_grid: bool,
}

impl Default for ChartPage {
    fn default() -> Self {
        Self {
            cache: Cache::default(),
            dataset_index: 0,
            show_points: true,
            show_grid: true,
        }
    }
}

/// 图表页面的消息类型
#[derive(Debug, Clone)]
pub enum ChartMessage {
    /// 切换到指定数据集
    SwitchDataset(usize),
    /// 切换数据点显示状态
    TogglePoints,
    /// 切换网格显示状态
    ToggleGrid,
}

impl ChartPage {
    /// 更新页面状态
    pub fn update(&mut self, msg: ChartMessage) {
        match msg {
            ChartMessage::SwitchDataset(idx) => {
                self.dataset_index = idx;
                // 切换数据集后清除缓存，触发重绘
                self.cache.clear();
            }
            ChartMessage::TogglePoints => {
                self.show_points = !self.show_points;
                self.cache.clear();
            }
            ChartMessage::ToggleGrid => {
                self.show_grid = !self.show_grid;
                self.cache.clear();
            }
        }
    }

    /// 构建页面视图
    pub fn view(&self) -> Element<'_, Message> {
        // Canvas 绘图区域
        let canvas_widget = Canvas::new(self)
            .width(Fill)
            .height(Length::Fixed(360.0));

        // 数据集切换按钮
        let datasets = get_all_datasets();
        let mut dataset_buttons = row![].spacing(8);
        for (idx, ds) in datasets.iter().enumerate() {
            let is_active = idx == self.dataset_index;
            let btn = button(iced::widget::text(ds.name).size(12))
                .padding([6, 12])
                .style(move |theme: &Theme, status| {
                    if is_active {
                        button::primary(theme, status)
                    } else {
                        button::secondary(theme, status)
                    }
                })
                .on_press(Message::Chart(ChartMessage::SwitchDataset(idx)));
            dataset_buttons = dataset_buttons.push(btn);
        }

        // 控制按钮行
        let point_btn_text = if self.show_points {
            "隐藏数据点"
        } else {
            "显示数据点"
        };
        let grid_btn_text = if self.show_grid {
            "隐藏网格"
        } else {
            "显示网格"
        };

        let point_btn = button(iced::widget::text(point_btn_text).size(12))
            .padding([6, 12])
            .on_press(Message::Chart(ChartMessage::TogglePoints));
        let grid_btn = button(iced::widget::text(grid_btn_text).size(12))
            .padding([6, 12])
            .on_press(Message::Chart(ChartMessage::ToggleGrid));

        let controls = row![dataset_buttons, Space::new(), point_btn, grid_btn]
            .align_y(iced::Alignment::Center);

        // 当前数据集信息
        let current_ds = &datasets[self.dataset_index];
        let info_text = iced::widget::text(format!("{} — {}", current_ds.name, current_ds.description))
            .size(13)
            .color(Color::from_rgb(0.5, 0.5, 0.55));

        column![
            container(canvas_widget)
                .width(Fill)
                .height(Length::Fixed(360.0))
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.98, 0.98, 0.99))),
                    border: iced::Border {
                        color: Color::from_rgb(0.85, 0.85, 0.9),
                        width: 1.0,
                        radius: iced::border::Radius::new(8.0),
                    },
                    ..Default::default()
                }),
            info_text,
            controls,
        ]
        .spacing(12)
        .into()
    }
}

// ─── Canvas 绘制实现 ─────────────────────────────────────────────────────────

impl canvas::Program<Message> for ChartPage {
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
            let dataset = &get_all_datasets()[self.dataset_index];

            // 图表内边距：上下左右留出坐标轴和标签空间
            let padding = ChartPadding {
                top: 30.0,
                right: 40.0,
                bottom: 50.0,
                left: 60.0,
            };

            // 实际绘图区域
            let plot_area = PlotArea::new(&bounds, &padding);

            // 绘制背景
            frame.fill(
                &Path::rectangle(Point::new(0.0, 0.0), bounds.size()),
                Color::from_rgb(0.98, 0.98, 0.99),
            );

            // 绘制网格
            if self.show_grid {
                draw_grid(frame, &plot_area, dataset);
            }

            // 绘制坐标轴
            draw_axes(frame, &plot_area, dataset);

            // 绘制每条折线
            for line in &dataset.lines {
                draw_line(frame, &plot_area, line, dataset);
            }

            // 绘制数据点
            if self.show_points {
                for line in &dataset.lines {
                    draw_points(frame, &plot_area, line, dataset);
                }
            }

            // 绘制图例
            draw_legend(frame, &plot_area, dataset);
        });

        vec![geometry]
    }
}

// ─── 数据结构 ────────────────────────────────────────────────────────────────

/// 单条折线的数据
#[derive(Debug, Clone)]
struct LineData {
    /// 折线名称（用于图例）
    name: &'static str,
    /// 数据点数值
    values: Vec<f32>,
    /// 折线颜色
    color: Color,
}

/// 完整数据集
#[derive(Debug, Clone)]
struct Dataset {
    /// 数据集名称
    name: &'static str,
    /// 数据集描述
    description: &'static str,
    /// X 轴标签
    x_labels: Vec<&'static str>,
    /// 包含的所有折线
    lines: Vec<LineData>,
    /// Y 轴最小值（为 None 则自动计算）
    y_min: Option<f32>,
    /// Y 轴最大值（为 None 则自动计算）
    y_max: Option<f32>,
    /// Y 轴标题
    y_title: &'static str,
}

/// 图表边距
struct ChartPadding {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

/// 实际绘图区域
struct PlotArea {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl PlotArea {
    fn new(bounds: &Rectangle, padding: &ChartPadding) -> Self {
        Self {
            x: padding.left,
            y: padding.top,
            width: bounds.width - padding.left - padding.right,
            height: bounds.height - padding.top - padding.bottom,
        }
    }
}

// ─── 预设数据集 ──────────────────────────────────────────────────────────────

fn get_all_datasets() -> Vec<Dataset> {
    vec![
        // 数据集 1：城市月平均气温
        Dataset {
            name: "月平均气温",
            description: "北京与上海的月平均气温对比（单位：°C）",
            x_labels: vec!["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"],
            lines: vec![
                LineData {
                    name: "北京",
                    values: vec![-3.0, 0.5, 7.0, 15.5, 21.5, 26.0, 27.5, 26.5, 21.0, 14.0, 5.5, -1.0],
                    color: Color::from_rgb(0.9, 0.3, 0.2),
                },
                LineData {
                    name: "上海",
                    values: vec![4.5, 6.5, 10.5, 16.0, 21.0, 24.5, 29.0, 28.5, 25.0, 19.5, 13.5, 7.0],
                    color: Color::from_rgb(0.2, 0.5, 0.9),
                },
            ],
            y_min: Some(-5.0),
            y_max: Some(35.0),
            y_title: "温度 (°C)",
        },
        // 数据集 2：模拟股票走势
        Dataset {
            name: "股票走势",
            description: "某科技公司模拟股价走势（单位：元）",
            x_labels: vec!["周一", "周二", "周三", "周四", "周五", "周一", "周二", "周三", "周四", "周五"],
            lines: vec![
                LineData {
                    name: "开盘价",
                    values: vec![120.5, 122.0, 119.5, 125.0, 128.5, 127.0, 130.5, 132.0, 129.5, 135.0],
                    color: Color::from_rgb(0.2, 0.7, 0.3),
                },
                LineData {
                    name: "收盘价",
                    values: vec![122.0, 119.5, 125.0, 128.5, 127.0, 130.5, 132.0, 129.5, 135.0, 138.5],
                    color: Color::from_rgb(0.9, 0.5, 0.1),
                },
            ],
            y_min: Some(110.0),
            y_max: Some(145.0),
            y_title: "价格 (元)",
        },
        // 数据集 3：网站访问量
        Dataset {
            name: "网站流量",
            description: "某网站过去 12 个月的访问量趋势（单位：万）",
            x_labels: vec!["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"],
            lines: vec![
                LineData {
                    name: "PC端",
                    values: vec![45.0, 42.0, 48.0, 52.0, 58.0, 55.0, 62.0, 68.0, 72.0, 78.0, 85.0, 92.0],
                    color: Color::from_rgb(0.5, 0.3, 0.8),
                },
                LineData {
                    name: "移动端",
                    values: vec![30.0, 32.0, 38.0, 45.0, 52.0, 58.0, 65.0, 72.0, 80.0, 88.0, 95.0, 105.0],
                    color: Color::from_rgb(0.1, 0.7, 0.8),
                },
                LineData {
                    name: "小程序",
                    values: vec![10.0, 12.0, 15.0, 18.0, 22.0, 28.0, 35.0, 42.0, 50.0, 58.0, 65.0, 75.0],
                    color: Color::from_rgb(0.9, 0.7, 0.2),
                },
            ],
            y_min: Some(0.0),
            y_max: Some(120.0),
            y_title: "访问量 (万)",
        },
    ]
}

// ─── 绘制函数 ────────────────────────────────────────────────────────────────

/// 计算数据范围
fn compute_range(dataset: &Dataset) -> (f32, f32) {
    let min = dataset.y_min.unwrap_or_else(|| {
        let mut min = f32::INFINITY;
        for line in &dataset.lines {
            for &v in &line.values {
                if v < min { min = v; }
            }
        }
        min
    });
    let max = dataset.y_max.unwrap_or_else(|| {
        let mut max = f32::NEG_INFINITY;
        for line in &dataset.lines {
            for &v in &line.values {
                if v > max { max = v; }
            }
        }
        max
    });
    (min, max)
}

/// 将数据值映射到 Canvas Y 坐标
fn map_y(value: f32, plot: &PlotArea, min: f32, max: f32) -> f32 {
    let ratio = (value - min) / (max - min);
    plot.y + plot.height - ratio * plot.height
}

/// 将数据索引映射到 Canvas X 坐标
fn map_x(index: usize, count: usize, plot: &PlotArea) -> f32 {
    if count <= 1 {
        plot.x + plot.width / 2.0
    } else {
        plot.x + (index as f32 / (count - 1) as f32) * plot.width
    }
}

/// 绘制网格线
fn draw_grid(frame: &mut Frame, plot: &PlotArea, dataset: &Dataset) {
    let grid_color = Color::from_rgb(0.9, 0.9, 0.92);
    let count = dataset.x_labels.len();

    // 垂直网格线（对应 X 轴标签位置）
    for i in 0..count {
        let x = map_x(i, count, plot);
        frame.stroke(
            &Path::line(
                Point::new(x, plot.y),
                Point::new(x, plot.y + plot.height),
            ),
            Stroke::default().with_width(0.5).with_color(grid_color),
        );
    }

    // 水平网格线（5 等分）
    for i in 0..=5 {
        let ratio = i as f32 / 5.0;
        let y = plot.y + plot.height - ratio * plot.height;
        frame.stroke(
            &Path::line(
                Point::new(plot.x, y),
                Point::new(plot.x + plot.width, y),
            ),
            Stroke::default().with_width(0.5).with_color(grid_color),
        );
    }
}

/// 绘制坐标轴
fn draw_axes(frame: &mut Frame, plot: &PlotArea, dataset: &Dataset) {
    let axis_color = Color::from_rgb(0.5, 0.5, 0.55);
    let text_color = Color::from_rgb(0.35, 0.35, 0.4);
    let count = dataset.x_labels.len();
    let (y_min, y_max) = compute_range(dataset);

    // X 轴线
    frame.stroke(
        &Path::line(
            Point::new(plot.x, plot.y + plot.height),
            Point::new(plot.x + plot.width, plot.y + plot.height),
        ),
        Stroke::default().with_width(1.5).with_color(axis_color),
    );

    // Y 轴线
    frame.stroke(
        &Path::line(
            Point::new(plot.x, plot.y),
            Point::new(plot.x, plot.y + plot.height),
        ),
        Stroke::default().with_width(1.5).with_color(axis_color),
    );

    // X 轴标签
    for (i, label) in dataset.x_labels.iter().enumerate() {
        let x = map_x(i, count, plot);
        let text = Text {
            content: label.to_string(),
            position: Point::new(x, plot.y + plot.height + 18.0),
            color: text_color,
            size: iced::Pixels(11.0),
            font: iced::Font::with_name("Microsoft YaHei"),
            align_x: iced::alignment::Horizontal::Center.into(),
            align_y: iced::alignment::Vertical::Top.into(),
            ..Default::default()
        };
        frame.fill_text(text);
    }

    // Y 轴标签（5 等分）
    for i in 0..=5 {
        let ratio = i as f32 / 5.0;
        let value = y_min + ratio * (y_max - y_min);
        let y = plot.y + plot.height - ratio * plot.height;

        let label = if (y_max - y_min) >= 100.0 {
            format!("{:.0}", value)
        } else if (y_max - y_min) >= 10.0 {
            format!("{:.1}", value)
        } else {
            format!("{:.2}", value)
        };

        let text = Text {
            content: label,
            position: Point::new(plot.x - 10.0, y),
            color: text_color,
            size: iced::Pixels(10.0),
            align_x: iced::alignment::Horizontal::Right.into(),
            align_y: iced::alignment::Vertical::Center.into(),
            ..Default::default()
        };
        frame.fill_text(text);
    }

    // Y 轴标题需要旋转 90 度，放在左侧
    frame.with_save(|frame| {
        frame.translate(Vector::new(14.0, plot.y + plot.height / 2.0));
        frame.rotate(iced::Degrees(-90.0));
        let rotated_text = Text {
            content: dataset.y_title.to_string(),
            position: Point::new(0.0, 0.0),
            color: text_color,
            size: iced::Pixels(11.0),
            font: iced::Font::with_name("Microsoft YaHei"),
            align_x: iced::alignment::Horizontal::Center.into(),
            align_y: iced::alignment::Vertical::Center.into(),
            ..Default::default()
        };
        frame.fill_text(rotated_text);
    });
}

/// 绘制折线
fn draw_line(frame: &mut Frame, plot: &PlotArea, line: &LineData, dataset: &Dataset) {
    let count = line.values.len();
    if count < 2 {
        return;
    }
    let (y_min, y_max) = compute_range(dataset);

    // 构建折线路径
    let mut points = Vec::with_capacity(count);
    for (i, &value) in line.values.iter().enumerate() {
        let x = map_x(i, dataset.x_labels.len(), plot);
        let y = map_y(value, plot, y_min, y_max);
        points.push(Point::new(x, y));
    }

    // 使用线段连接各点
    for i in 0..(points.len() - 1) {
        frame.stroke(
            &Path::line(points[i], points[i + 1]),
            Stroke::default()
                .with_width(2.5)
                .with_color(line.color),
        );
    }

    // 填充折线下方的半透明区域（面积图效果）
    let mut area_points = vec![Point::new(points[0].x, plot.y + plot.height)];
    for p in &points {
        area_points.push(*p);
    }
    area_points.push(Point::new(points[points.len() - 1].x, plot.y + plot.height));

    let area_path = Path::new(|builder| {
        if let Some(first) = area_points.first() {
            builder.move_to(*first);
            for p in &area_points[1..] {
                builder.line_to(*p);
            }
            builder.close();
        }
    });

    frame.fill(
        &area_path,
        Color::from_rgba(line.color.r, line.color.g, line.color.b, 0.08),
    );
}

/// 绘制数据点
fn draw_points(frame: &mut Frame, plot: &PlotArea, line: &LineData, dataset: &Dataset) {
    let (y_min, y_max) = compute_range(dataset);

    for (i, &value) in line.values.iter().enumerate() {
        let x = map_x(i, dataset.x_labels.len(), plot);
        let y = map_y(value, plot, y_min, y_max);

        // 外圈白色边框
        frame.fill(
            &Path::circle(Point::new(x, y), 5.5),
            Color::from_rgb(0.98, 0.98, 0.99),
        );
        // 内圈着色
        frame.fill(
            &Path::circle(Point::new(x, y), 4.0),
            line.color,
        );
    }
}

/// 绘制图例
fn draw_legend(frame: &mut Frame, plot: &PlotArea, dataset: &Dataset) {
    let legend_y = plot.y - 18.0;
    let mut legend_x = plot.x + plot.width;
    let _item_spacing = 100.0;
    let text_color = Color::from_rgb(0.35, 0.35, 0.4);

    // 从右向左排列图例
    for line in dataset.lines.iter().rev() {
        // 计算文字宽度（粗略估算：每个字符约 7px）
        let text_width = line.name.len() as f32 * 7.0 + 20.0;
        legend_x -= text_width;

        // 色块
        frame.fill(
            &Path::rectangle(
                Point::new(legend_x, legend_y - 4.0),
                Size::new(12.0, 8.0),
            ),
            line.color,
        );

        // 文字
        let text = Text {
            content: line.name.to_string(),
            position: Point::new(legend_x + 16.0, legend_y),
            color: text_color,
            size: iced::Pixels(11.0),
            font: iced::Font::with_name("Microsoft YaHei"),
            align_x: iced::alignment::Horizontal::Left.into(),
            align_y: iced::alignment::Vertical::Center.into(),
            ..Default::default()
        };
        frame.fill_text(text);
    }
}
