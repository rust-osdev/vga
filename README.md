[![Build Status](https://github.com/rust-osdev/vga/workflows/Build/badge.svg)](https://github.com/rust-osdev/vga/actions?query=workflow%3ABuild) [![Docs.rs Badge](https://docs.rs/vga/badge.svg)](https://docs.rs/vga/)

# vga
This crate provides vga specific functions, data structures,
and access to various registers.

Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
this crate to work properly.

**Note: This crate is currently experimental and subject to change since it's in active development.**

## Usage
```rust
use vga::colors::Color16Bit;
use vga::writers::{ScreenCharacter, TextWriter, Text80x25};

let text_mode = Text80x25::new();
let screen_character = ScreenCharacter::new(b'T'Color16Bit::Yellow, Color16Bit::Black);

text_mode.set_mode();
text_mode.clear_screen();
text_mode.write_character(0, 0, screen_character);
```
