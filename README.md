# 🦀 Rust Iced Demo

一个基于 [Iced](https://github.com/iced-rs/iced) GUI 框架的 Rust 桌面应用演示程序。

## 功能展示

- **计数器**：点击 ＋ / － 按钮增减计数，支持重置，负数红色、正数绿色高亮
- **进度滑块**：拖动滑块实时更新进度条（0–100%）
- **文本输入**：实时回显输入内容
- **深色/浅色主题**：右上角 toggler 一键切换

## 环境要求

- Rust 1.75+（推荐 stable）
- Windows / macOS / Linux

## 快速运行

```bash
git clone https://github.com/arctic603/Rust_Iced.git
cd Rust_Iced
cargo run --release
```

> 首次编译需要下载 iced 依赖，约需 1–3 分钟，请耐心等待。

## 项目结构

```
Rust_Iced/
├── Cargo.toml        # 依赖配置
└── src/
    └── main.rs       # 应用逻辑（单文件 Demo）
```

## 依赖

| 库 | 版本 | 说明 |
|---|---|---|
| [iced](https://crates.io/crates/iced) | 0.13 | Rust GUI 框架 |

## 截图

> 运行 `cargo run` 后即可看到如下界面：
> - 顶部：标题 + 主题切换开关
> - 中部：计数器、进度滑块
> - 底部：文本输入回显

## License

MIT
