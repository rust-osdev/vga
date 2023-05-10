//! Provides access to the vga graphics card.

use super::{
    configurations::{
        VgaConfiguration, MODE_320X200X256_CONFIGURATION, MODE_320X240X256_CONFIGURATION,
        MODE_40X25_CONFIGURATION, MODE_40X50_CONFIGURATION, MODE_640X480X16_CONFIGURATION,
        MODE_80X25_CONFIGURATION,
    },
    fonts::VgaFont,
    registers::{
        AttributeControllerRegisters, ColorPaletteRegisters, CrtcControllerIndex,
        CrtcControllerRegisters, EmulationMode, GeneralRegisters, GraphicsControllerIndex,
        GraphicsControllerRegisters, PlaneMask, SequencerIndex, SequencerRegisters,
    },
};
use crate::configurations::MODE_1280X800X256_CONFIGURATION;
use conquer_once::spin::Lazy;
use spinning_top::Spinlock;

/// Provides mutable access to the vga graphics card.
pub static VGA: Lazy<Spinlock<Vga>> = Lazy::new(|| Spinlock::new(Vga::new()));

/// Represents the starting address of the frame buffer for
/// various video modes.
#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum FrameBuffer {
    /// The starting address for graphics modes.
    GraphicsMode(usize),
    /// The starting address for color text modes.
    CgaMode(usize),
    /// The starting address for monochrome text modes.
    MdaMode(usize),
}

impl FrameBuffer {
    fn new(memory_map_mode: u8, video_memory_start: usize) -> FrameBuffer {
        match memory_map_mode {
            0x1 => FrameBuffer::GraphicsMode(video_memory_start),
            0x2 => FrameBuffer::MdaMode(video_memory_start + 0x10000),
            0x3 => FrameBuffer::CgaMode(video_memory_start + 0x18000),
            _ => panic!("{:X} is not a valid FrameBuffer map mode", memory_map_mode),
        }
    }
}

impl From<FrameBuffer> for usize {
    fn from(value: FrameBuffer) -> usize {
        match value {
            FrameBuffer::GraphicsMode(addr) => addr,
            FrameBuffer::CgaMode(addr) => addr,
            FrameBuffer::MdaMode(addr) => addr,
        }
    }
}

/// Represents a specified vga video mode.
#[derive(Debug, Clone, Copy)]
pub enum VideoMode {
    /// Represents text mode 40x25.
    Mode40x25,
    /// Represents text mode 40x50.
    Mode40x50,
    /// Represents text mode 80x25.
    Mode80x25,
    /// Represents graphics mode 320x200x256.
    Mode320x200x256,
    /// Represents graphics mode 320x240x256.
    Mode320x240x256,
    /// Represents graphics mode 640x480x16.
    Mode640x480x16,
    /// Represents graphics mode 1280x800x256.
    Mode1280x800x256,
}

/// Represents a vga graphics card with it's common registers,
/// as well as the most recent video mode.
pub struct Vga {
    /// Represents the general registers on vga hardware.
    pub general_registers: GeneralRegisters,
    /// Represents the sequencer registers on vga hardware.
    pub sequencer_registers: SequencerRegisters,
    /// Represents the graphics controller registers on vga hardware.
    pub graphics_controller_registers: GraphicsControllerRegisters,
    /// Represents the attribute controller registers on vga hardware.
    pub attribute_controller_registers: AttributeControllerRegisters,
    /// Represents the crtc controller registers on vga hardware.
    pub crtc_controller_registers: CrtcControllerRegisters,
    /// Represents the color palette registers on vga hardware.
    pub color_palette_registers: ColorPaletteRegisters,
    most_recent_video_mode: Option<VideoMode>,
    /// Memory start. 0xa0000 for physical memory mapping
    video_memory_start: usize,
}

impl Vga {
    fn new() -> Vga {
        Vga {
            general_registers: GeneralRegisters::new(),
            sequencer_registers: SequencerRegisters::new(),
            graphics_controller_registers: GraphicsControllerRegisters::new(),
            attribute_controller_registers: AttributeControllerRegisters::new(),
            crtc_controller_registers: CrtcControllerRegisters::new(),
            color_palette_registers: ColorPaletteRegisters::new(),
            most_recent_video_mode: None,
            video_memory_start: 0xa0000,
        }
    }

    /// Set the start of video memory
    ///
    /// The default is 0xA0000 for identity map to physical memory
    ///
    /// # Example
    ///
    /// ```
    /// use vga;
    ///
    /// vga::vga::VGA.lock().set_memory_start(0xa0000);
    /// ```
    pub fn set_memory_start(&mut self, video_memory_start: usize) {
        self.video_memory_start = video_memory_start;
    }

    /// Sets the vga graphics card to the given `VideoMode`.
    pub fn set_video_mode(&mut self, video_mode: VideoMode) {
        match video_mode {
            VideoMode::Mode40x25 => self.set_video_mode_40x25(),
            VideoMode::Mode40x50 => self.set_video_mode_40x50(),
            VideoMode::Mode80x25 => self.set_video_mode_80x25(),
            VideoMode::Mode320x200x256 => self.set_video_mode_320x200x256(),
            VideoMode::Mode320x240x256 => self.set_video_mode_320x240x256(),
            VideoMode::Mode640x480x16 => self.set_video_mode_640x480x16(),
            VideoMode::Mode1280x800x256 => self.set_video_mode_1280x800x256(),
        }
    }

    /// Gets the `FrameBuffer` address as specified by the
    /// `Miscellaneous Output Register`.
    pub fn get_frame_buffer(&mut self) -> FrameBuffer {
        let miscellaneous_graphics = self
            .graphics_controller_registers
            .read(GraphicsControllerIndex::Miscellaneous);
        let memory_map_mode = (miscellaneous_graphics >> 0x2) & 0x3;
        FrameBuffer::new(memory_map_mode, self.video_memory_start)
    }

    /// Returns the most recent video mode, or `None` if no
    /// video mode has been set yet.
    pub fn get_most_recent_video_mode(&self) -> Option<VideoMode> {
        self.most_recent_video_mode
    }

    /// Returns the current `EmulationMode` as determined by the miscellaneous output register.
    pub fn get_emulation_mode(&mut self) -> EmulationMode {
        EmulationMode::from(self.general_registers.read_msr() & 0x1)
    }

    /// Loads a vga text mode font as specified by `vga_font`.
    pub fn load_font(&mut self, vga_font: &VgaFont) {
        // Save registers
        let (
            plane_mask,
            sequencer_memory_mode,
            read_plane_select,
            graphics_mode,
            miscellaneous_graphics,
        ) = self.save_font_registers();

        // Switch to flat addressing
        self.sequencer_registers
            .write(SequencerIndex::MemoryMode, sequencer_memory_mode | 0x04);

        // Disable Even/Odd addressing
        self.graphics_controller_registers
            .write(GraphicsControllerIndex::GraphicsMode, graphics_mode & !0x10);
        self.graphics_controller_registers.write(
            GraphicsControllerIndex::Miscellaneous,
            miscellaneous_graphics & !0x02,
        );

        // Write font to plane
        self.sequencer_registers.set_plane_mask(PlaneMask::PLANE2);

        let frame_buffer = usize::from(self.get_frame_buffer()) as *mut u8;

        for character in 0..vga_font.characters {
            for row in 0..vga_font.character_height {
                let offset = (character * 32) + row;
                let font_offset = (character * vga_font.character_height) + row;
                unsafe {
                    frame_buffer
                        .offset(offset as isize)
                        .write_volatile(vga_font.font_data[font_offset as usize]);
                }
            }
        }

        self.restore_font_registers(
            plane_mask,
            sequencer_memory_mode,
            read_plane_select,
            graphics_mode,
            miscellaneous_graphics,
        );
    }

    fn restore_font_registers(
        &mut self,
        plane_mask: u8,
        sequencer_memory_mode: u8,
        read_plane_select: u8,
        graphics_mode: u8,
        miscellaneous_graphics: u8,
    ) {
        self.sequencer_registers
            .write(SequencerIndex::PlaneMask, plane_mask);
        self.sequencer_registers
            .write(SequencerIndex::MemoryMode, sequencer_memory_mode);
        self.graphics_controller_registers
            .write(GraphicsControllerIndex::ReadPlaneSelect, read_plane_select);
        self.graphics_controller_registers
            .write(GraphicsControllerIndex::GraphicsMode, graphics_mode);
        self.graphics_controller_registers.write(
            GraphicsControllerIndex::Miscellaneous,
            miscellaneous_graphics,
        );
    }

    fn save_font_registers(&mut self) -> (u8, u8, u8, u8, u8) {
        (
            self.sequencer_registers.read(SequencerIndex::PlaneMask),
            self.sequencer_registers.read(SequencerIndex::MemoryMode),
            self.graphics_controller_registers
                .read(GraphicsControllerIndex::ReadPlaneSelect),
            self.graphics_controller_registers
                .read(GraphicsControllerIndex::GraphicsMode),
            self.graphics_controller_registers
                .read(GraphicsControllerIndex::Miscellaneous),
        )
    }

    fn set_registers(&mut self, configuration: &VgaConfiguration) {
        let emulation_mode = self.get_emulation_mode();

        // Set miscellaneous output
        self.general_registers
            .write_msr(configuration.miscellaneous_output);

        // Set the sequencer registers.
        for (index, value) in configuration.sequencer_registers {
            self.sequencer_registers.write(*index, *value);
        }

        // Unlock the crtc registers.
        self.unlock_crtc_registers(emulation_mode);

        // Set the crtc registers.
        for (index, value) in configuration.crtc_controller_registers {
            self.crtc_controller_registers
                .write(emulation_mode, *index, *value);
        }

        // Set the grx registers.
        for (index, value) in configuration.graphics_controller_registers {
            self.graphics_controller_registers.write(*index, *value);
        }

        // Blank the screen so the palette registers are unlocked.
        self.attribute_controller_registers
            .blank_screen(emulation_mode);

        // Set the arx registers.
        for (index, value) in configuration.attribute_controller_registers {
            self.attribute_controller_registers
                .write(emulation_mode, *index, *value);
        }

        // Unblank the screen so the palette registers are locked.
        self.attribute_controller_registers
            .unblank_screen(emulation_mode);
    }

    /// Sets the video card to Mode 40x25.
    fn set_video_mode_40x25(&mut self) {
        self.set_registers(&MODE_40X25_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode40x25);
    }

    /// Sets the video card to Mode 40x50.
    fn set_video_mode_40x50(&mut self) {
        self.set_registers(&MODE_40X50_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode40x50);
    }

    /// Sets the video card to Mode 80x25.
    fn set_video_mode_80x25(&mut self) {
        self.set_registers(&MODE_80X25_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode80x25);
    }

    /// Sets the video card to Mode 320x200x256.
    fn set_video_mode_320x200x256(&mut self) {
        self.set_registers(&MODE_320X200X256_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode320x200x256);
    }

    /// Sets the video card to Mode 320x200x256x.
    fn set_video_mode_320x240x256(&mut self) {
        self.set_registers(&MODE_320X240X256_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode320x240x256);
    }

    /// Sets the video card to Mode 640x480x16.
    fn set_video_mode_640x480x16(&mut self) {
        self.set_registers(&MODE_640X480X16_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode640x480x16);
    }

    /// Sets the video card to Mode 1280x800x256.
    fn set_video_mode_1280x800x256(&mut self) {
        self.set_registers(&MODE_1280X800X256_CONFIGURATION);
        self.most_recent_video_mode = Some(VideoMode::Mode1280x800x256);
    }

    /// Unlocks the CRTC registers by setting bit 7 to 0 `(value & 0x7F)`.
    ///
    /// `Protect Registers [0:7]`: Note that the ability to write to Bit 4 of the Overflow Register (CR07)
    /// is not affected by this bit (i.e., bit 4 of the Overflow Register is always writeable).
    ///
    /// 0 = Enable writes to registers `CR[00:07]`
    ///
    /// 1 = Disable writes to registers `CR[00:07]`
    fn unlock_crtc_registers(&mut self, emulation_mode: EmulationMode) {
        // Setting bit 7 to 1 used to be required for `VGA`, but says it's
        // ignored in modern hardware. Setting it to 1 just to be safe for older
        // hardware. More information can be found here
        // https://01.org/sites/default/files/documentation/intel-gfx-prm-osrc-hsw-display.pdf
        // under `CR03 - Horizontal Blanking End Register`.
        let horizontal_blanking_end = self
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::HorizontalBlankingEnd);
        self.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::HorizontalBlankingEnd,
            horizontal_blanking_end | 0x80,
        );

        let vertical_sync_end = self
            .crtc_controller_registers
            .read(emulation_mode, CrtcControllerIndex::VerticalSyncEnd);
        self.crtc_controller_registers.write(
            emulation_mode,
            CrtcControllerIndex::VerticalSyncEnd,
            vertical_sync_end & 0x7F,
        );
    }
}
