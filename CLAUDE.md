# Scientific Calculator

A desktop scientific calculator built with Rust and egui/eframe.

## Project Structure

codeblock-start
src/
├── main.rs                  # Entry point, eframe window setup
├── app.rs                   # eframe::App impl (UI layout + keyboard input)
├── calculator/
│   ├── mod.rs               # Module root, exports Calculator + format_number
│   ├── state.rs             # Calculator struct + SciOp enum + all state mutation methods
│   └── ops.rs               # handle_basic() / handle_sci() button dispatchers
└── ui/
    ├── mod.rs               # Module root
    ├── buttons.rs           # BtnKind enum, btn_colors, calc_button, basic_row, sci_row
    └── display.rs           # render_display() — main number + formula line
codeblock-end

## Tech Stack

- **Rust 2024 edition**
- **eframe 0.31** — native app framework
- **egui 0.31** — immediate mode GUI

## Key Details

- Dark theme (iOS calculator-inspired): gb(28,28,30) background, gb(255,149,0) orange operators, gb(58,58,60) gray digits, gb(10,132,255) blue scientific
- Responsive layout: buttons fill remaining vertical space proportionally
- 4-column grid, 5 basic rows + 5 scientific rows (toggleable)
- Keyboard input: digits, +-*/=(), Enter, Backspace, Escape
- DEG/RAD mode toggle for trig functions
- Parenthesis support via paren_stack (nested parens)
- Formula line shows full expression chain (e.g. 23 − 6 × (34 − 55) + (3 − 1) ÷ 3)
- Factorial limited to n <= 20 (u64 range)
- Number formatting: integers displayed as whole numbers, floats trimmed to 10 decimal places

## Scientific Operations

### Unary (instant)
| Button | Description | Example |
|--------|-------------|---------|
| x² | Square | 5 → x² = 25 |
| x³ | Cube | 3 → x³ = 27 |
| √x | Square root | 9 → √x = 3 |
| ∛x | Cube root | 27 → ∛x = 3 |
| ln | Natural log | |
| log | Base-10 log | |
| 1/x | Reciprocal | |
| x! | Factorial (0–20) | |
| 10ˣ | Base-10 exponential | 2 → 10ˣ = 100 |
| |x| | Absolute value | −5 → |x| = 5 |
| π | Pi constant | |
|  | Euler's constant | |

### Binary (two-argument: first number, press button, second number, then =)
| Button | Description | Example |
|--------|-------------|---------|
| xʸ | Custom power | 2 → xʸ → 3 → = → 8 |
| ʸ√x | Custom nth root | 8 → ʸ√x → 3 → = → 2 |

### Trig (DEG/RAD aware)
sin cos 	an sin cos tan

## Build & Run

codeblock-startash
cargo run          # Debug build
cargo run --release # Optimized build (opt-level = 2)
codeblock-end

## graphify

This project has a graphify knowledge graph at graphify-out/.

Rules:
- Before answering architecture or codebase questions, read graphify-out/GRAPH_REPORT.md for god nodes and community structure
- If graphify-out/wiki/index.md exists, navigate it instead of reading raw files
- After modifying code files in this session, run graphify update . to keep the graph current (AST-only, no API cost)
