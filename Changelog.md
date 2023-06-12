# 0.2.9

- Added `Graphics1280x800x256`.
- Added `PrimitiveDrawing` with the ability to `draw_line` and `draw_rect`.

# 0.2.8

- Added the ability to set the graphics memory start with `vga::vga::VGA.lock().set_memory_start(0xa0000);`.
- `TextModeColor::set_foreground` now correctly saves the background color instead of resetting to black.

# 0.2.7

- Updated various out of date crates.

# 0.2.6

- Updated `x86_64` to build with lastest rust nightly.

# 0.2.5

- Updated various out of date crates.

# 0.2.4

- Updated `x86_64` to fix deprecated `asm!` macro.

# 0.2.3

- Added support for 320x240x256 mode via `Graphics320x240x256`.
- Added the ability to easily get a pointer to the modes frame buffer via `GraphicsWriter::get_frame_buffer`.

# 0.2.2

## Breaking

- `Screen::get_width()` now accessed by associated constant `i.e Text80x25::WIDTH`.
- `Screen::get_height()` now accessed by associated constant `i.e Text80x25::HEIGHT`.
- `Screen::get_size()` now accessed by associated constant `i.e Text80x25::SIZE`.
- `Graphics320x200x256::new`, `Graphics640x480x16::new`, `Text40x25::new`, `Text40x50::new`, `Text80x25::new` and `ScreenCharacter::new` are now `const fn`.
- `Graphics320x200x256`, `Graphics640x480x16`, `Text40x25`, `Text40x50`, and `Text80x25` now derive `Copy` and `Clone`.

## Other

- Added `TextWriter::fill_screen(ScreenCharacter)` for convenience.


# 0.2.1

- Added `Graphics320x200x256` mode.
- Implemented `Screen` for `Graphics640x480x16`.

# 0.2.0

## Breaking

- Registers moved `vga::registers`.
- `Plane` converted to `ReadPlane` and `PlaneMask`.
- Register read/write ability removed from `Vga`.
- Public access added to `Vga` fields.
- `TextWriter::get_width` and `TextWriter::get_height` moved to a `Screen` trait.
- `Color16Bit` renamed to `Color16`.

## Other

- Added a new `Screen` trait for dealing with the size of a screen.
- Added a `GraphicsWriter` trait for dealing with vga graphics.
- Added `Graphics640x480x16::clear_screen`.
- Added `Graphics640x480x16::draw_line`.
- Added `Graphics640x480x16::draw_character`.
- Added `vga::drawing::Point` for drawing lines.

# 0.1.2

## Breaking

- `ScreenCharacter::new` now takes a `TextModeColor` instead of 2 `Color16Bit`.

## Other

- Added `ScreenCharacter::get_character`.
- Added `ScreenCharacter::get_color`.
- Added `TextWriter::read_character`.
