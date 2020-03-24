[![Build Status](https://github.com/rust-osdev/vga/workflows/Build/badge.svg)](https://github.com/rust-osdev/vga/actions?query=workflow%3ABuild) [![Docs.rs Badge](https://docs.rs/vga/badge.svg)](https://docs.rs/vga/)

# vga
This crate provides vga specific functions, data structures,
and access to various registers.

Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
this crate to work properly.

**Note: This crate is currently experimental and subject to change since it's in active development.**

## Text Mode
```rust
use vga::colors::{Color16, TextModeColor};
use vga::writers::{ScreenCharacter, TextWriter, Text80x25};

let text_mode = Text80x25::new();
let color = TextModeColor::new(Color16::Yellow, Color16::Black);
let screen_character = ScreenCharacter::new(b'T', color);

text_mode.set_mode();
text_mode.clear_screen();
text_mode.write_character(0, 0, screen_character);
```

## Graphics Mode
```rust
use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};

let mode = Graphics640x480x16::new();
mode.set_mode();
mode.clear_screen(Color16::Black);
mode.draw_line((80, 60), (80, 420), Color16::White);
mode.draw_line((80, 60), (540, 60), Color16::White);
mode.draw_line((80, 420), (540, 420), Color16::White);
mode.draw_line((540, 420), (540, 60), Color16::White);
mode.draw_line((80, 90), (540, 90), Color16::White);
for (offset, character) in "Hello World!".chars().enumerate() {
    mode.draw_character(270 + offset * 8, 72, character, Color16::White)
}
```
