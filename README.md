# Scientific Calculator

A desktop scientific calculator built with Rust and egui/eframe.

![Calculator Screenshot](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## Features

- **Dark theme** inspired by iOS calculator with orange operators, gray digits, and blue scientific buttons
- **Basic operations**: addition, subtraction, multiplication, division
- **Scientific functions**:
  - Trig: sin, cos, tan, asin, acos, atan (DEG/RAD aware)
  - Powers: x², x³, xʸ (custom power)
  - Roots: √x, ∛x, ʸ√x (custom nth root)
  - Logarithms: ln, log, 10ˣ
  - Other: 1/x, x!, π, e, |x|
- **Formula chain**: shows full expression as you type (e.g. 23 − 6 × (34 − 55) + (3 − 1) ÷ 3)
- **Parenthesis support** with nesting
- **Keyboard input**: digits, operators, Enter, Backspace, Escape
- **Responsive layout** that fills available space

## Screenshot

## Build & Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

### Commands

`ash
# Debug build
cargo run

# Optimized release build
cargo run --release
`

## Usage

### Basic Operations
- Click buttons or type on keyboard
- Use +, -, *, / for operations
- Press Enter or = to evaluate
- Use Escape to clear, Backspace to delete

### Scientific Functions
- Click **▼ Scientific** to toggle scientific mode
- **Unary ops** (sin, cos, x², √x, etc.) apply instantly to the current value
- **Binary ops** (xʸ, ʸ√x): enter first value, press the button, enter second value, then press =

### Trig Functions
- Toggle between **DEG** and **RAD** mode when scientific panel is open
- In DEG mode, input is treated as degrees; in RAD mode, as radians

## Project Structure

`
src/
├── main.rs                  # Entry point, eframe window setup
├── app.rs                   # eframe::App impl (UI layout + keyboard input)
├── calculator/
│   ├── mod.rs               # Module root, exports Calculator + format_number
│   ├── state.rs             # Calculator struct + SciOp enum + state mutation
│   └── ops.rs               # handle_basic() / handle_sci() button dispatchers
└── ui/
    ├── mod.rs               # Module root
    ├── buttons.rs           # BtnKind enum, btn_colors, calc_button, rows
    └── display.rs           # render_display() — main number + formula line
`

## Tech Stack

- **Rust 2024 edition**
- [eframe 0.31](https://crates.io/crates/eframe) — native app framework
- [egui 0.31](https://crates.io/crates/egui) — immediate mode GUI

## License

MIT
