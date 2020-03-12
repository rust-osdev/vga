# vga
This crate provides vga specific functions, data structures,
and access to various registers.

Memory addresses `0xA0000 -> 0xBFFFF` must be readable and writeable
this crate to work properly.

## Usage
```rust
use vga::colors::{Color16Bit, TextModeColor};
use vga::Text80x25;

let text_mode = Text80x25::new();
let color = TextModeColor::new(Color16Bit::Yellow, Color16Bit::Black);

text_mode.set_mode();
text_mode.clear_screen();
text_mode.write_character(0, 0, b'H', color);
```
