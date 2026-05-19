# Rust Iced Demo

A multi-page GUI demo application built with [Iced](https://github.com/iced-rs/iced) 0.14 and Rust.

## Features

This project demonstrates various Iced UI components across 8 independent pages:

| Page | Description |
|---|---|
| **Welcome** | Overview and feature list |
| **Counter** | Basic counter, step counter, bounded counter (-10~10), history tracking |
| **Text Inputs** | Form inputs, password field, checkbox, toggler, live preview |
| **Sliders** | Slider controls, RGB color picker, volume control, simulated download |
| **Layout** | Row, Column, Center, Stack, Space layout patterns |
| **Canvas** | Custom drawing with Cache, geometric shapes, grid, rotation |
| **Animation** | State-driven animation with position/size/color changes |
| **Theme** | Button styles, text styles, container radius, color preview |

## Requirements

- Rust 1.75+ (stable recommended)
- Windows / macOS / Linux

## Quick Start

```bash
git clone https://github.com/arctic603/Rust_Iced.git
cd Rust_Iced
cargo run --release
```

> First build downloads Iced dependencies, may take 2-5 minutes.

## Project Structure

```
Rust_Iced/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # Application entry, routing, theme
    └── pages/
        ├── mod.rs           # Page enum, sidebar navigation, content wrapper
        ├── counter.rs       # Counter demo page
        ├── text.rs          # Text input demo page
        ├── slider.rs        # Slider & progress demo page
        ├── layout.rs        # Layout patterns demo page
        ├── canvas.rs        # Canvas drawing demo page
        ├── animation.rs     # Animation demo page
        └── theme.rs         # Theme & style demo page
```

## Dependencies

| Crate | Version | Description |
|---|---|---|
| [iced](https://crates.io/crates/iced) | 0.14 | Cross-platform GUI framework |

## Architecture

- **Page routing**: `Page` enum with 8 variants, switched via left sidebar
- **State management**: Each page has its own state struct and message enum
- **Navigation**: Left sidebar with active page highlighting
- **Theming**: Light/Dark mode support via `Theme` toggle

## License

MIT
