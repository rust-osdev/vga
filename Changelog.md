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
