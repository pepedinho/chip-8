
# CHIP-8 Emulator in Rust 🕹️

A **CHIP-8 emulator** written in **Rust**, designed for learning and experimenting with low-level programming concepts, CPU emulation, and classic game development. This project implements a CHIP-8 interpreter, display rendering, and keyboard input handling.

---

## Overview

CHIP-8 is a simple interpreted programming language originally developed in the 1970s to run games on 8-bit microcomputers. Its small instruction set and simple graphics make it an excellent starting point for emulator development.

This emulator aims to:

- Correctly parse and execute CHIP-8 opcodes.
- Emulate CPU registers, stack, timers, and memory.
- Render graphics on a pixel grid with proper sprite drawing and collision detection.
- Handle keyboard input with a mapping from physical keys to CHIP-8 keys.
- Load and run CHIP-8 game ROMs (.ch8 files).
- Provide a clean and well-commented Rust codebase for easy understanding and extension.

---

## Features ⭐

- Full opcode set implemented (0x00E0, 0x1NNN, 0x6XKK, 0xDXYN, 0xFX0A, and others).
- Accurate sprite rendering with pixel toggling and collision flag.
- Stack and program counter management for subroutine calls and returns.
- Timers for delay and sound emulation.
- Keyboard input with SDL2 integration (or chosen backend).
- Simple main loop controlling CPU cycles and display refresh.
- Support for classic CHIP-8 games like **15PUZZLE**, **X-MIRROR**, etc.

---

## Getting Started 🚀

### Prerequisites

- Rust toolchain ([install Rust](https://rust-lang.org/tools/install))
- SDL2 library (or equivalent) installed on your system for windowing and input

### Build & Run

1. Clone this repository:

```bash
git clone https://github.com/yourusername/chip8-emulator-rust.git
cd chip8-emulator-rust
```

2. Build the project:

```bash
cargo build --release
```

3. Run with a CHIP-8 ROM file:

```bash
cargo run --release -- path/to/game.ch8
```

---

## Usage 🎮

- Use your physical keyboard to control the CHIP-8 keys (mapping explained below).
- The emulator handles key press, release, and waiting for key input.
- The display window will show the classic 64x32 monochrome pixels.
- The game should run as expected, including sound and timers.

---

## Keyboard Mapping ⌨️

The CHIP-8 uses a 16-key hexadecimal keypad:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

Mapped to your physical keyboard as:

| CHIP-8 Key | Physical Key |
|------------|--------------|
| 0x0        | X            |
| 0x1        | 1            |
| 0x2        | 2            |
| 0x3        | 3            |
| 0x4        | Q            |
| 0x5        | W            |
| 0x6        | E            |
| 0x7        | A            |
| 0x8        | S            |
| 0x9        | D            |
| 0xA        | Z            |
| 0xB        | C            |
| 0xC        | 4            |
| 0xD        | R            |
| 0xE        | F            |
| 0xF        | V            |

---

## Code Structure 🛠️

- `cpu.rs`: Implements the CPU, registers, stack, opcode interpretation, and timers.
- `display.rs`: Handles pixel rendering, sprite drawing, and screen clearing.
- `keyboard.rs`: Manages keyboard state, key presses/releases, and mapping from physical keys to CHIP-8 keys.
- `main.rs`: Sets up the window, event loop, loads ROMs, and runs the CPU cycles.

---

## Known Issues & To Do 📝

- Sound is currently a placeholder (no actual audio output yet).
- Timing is approximate; some games might run faster or slower than on original hardware.
- Improve input responsiveness for certain key combinations.
- Add debugger support and step-through opcode execution.
- Extend support to SCHIP (Super CHIP-8) instructions.

---

## Learning Outcomes 📚

This project helped me deepen my understanding of:

- Bitwise operations and opcode decoding.
- Low-level CPU design concepts.
- Event-driven input handling.
- Graphics rendering with pixel buffers.
- Rust ownership, borrowing, and safe concurrency.

---

## Contributions 🤝

Contributions, bug reports, and feature requests are welcome! Feel free to open issues or submit pull requests.

---

## License ⚖️

MIT License — see `LICENSE` file for details.

---

If you want, I can also help generate a `Cargo.toml` example or setup instructions for SDL2 bindings in Rust. Let me know!
