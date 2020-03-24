//! Common registers used in vga programming.

mod attribute_controller;
mod color_palette;
mod crtc_controller;
mod general;
mod graphics_controller;
mod sequencer;

use crate::colors::{Color16, PALETTE_SIZE};

pub use attribute_controller::{AttributeControllerIndex, AttributeControllerRegisters};
pub use color_palette::ColorPaletteRegisters;
pub use crtc_controller::{CrtcControllerIndex, CrtcControllerRegisters};
pub use general::GeneralRegisters;
pub use graphics_controller::{GraphicsControllerIndex, GraphicsControllerRegisters, WriteMode};
pub use sequencer::{PlaneMask, SequencerIndex, SequencerRegisters};

const ST00_READ_ADDRESS: u16 = 0x3C2;
const ST01_READ_CGA_ADDRESS: u16 = 0x3DA;
const ST01_READ_MDA_ADDRESS: u16 = 0x3BA;
const FCR_READ_ADDRESS: u16 = 0x3CA;
const FCR_CGA_WRITE_ADDRESS: u16 = 0x3DA;
const FCR_MDA_WRITE_ADDRESS: u16 = 0x3BA;
const MSR_READ_ADDRESS: u16 = 0x3CC;
const MSR_WRITE_ADDRESS: u16 = 0x3C2;

const SRX_INDEX_ADDRESS: u16 = 0x3C4;
const SRX_DATA_ADDRESS: u16 = 0x3C5;

const GRX_INDEX_ADDRESS: u16 = 0x3CE;
const GRX_DATA_ADDRESS: u16 = 0x3CF;

const ARX_INDEX_ADDRESS: u16 = 0x3C0;
const ARX_DATA_ADDRESS: u16 = 0x3C1;

const CRX_INDEX_CGA_ADDRESS: u16 = 0x3D4;
const CRX_INDEX_MDA_ADDRESS: u16 = 0x3B4;
const CRX_DATA_CGA_ADDRESS: u16 = 0x3D5;
const CRX_DATA_MDA_ADDRESS: u16 = 0x3B5;

const COLOR_PALETTE_DATA_ADDRESS: u16 = 0x3C9;
const COLOR_PALETTE_INDEX_READ_ADDRESS: u16 = 0x3C7;
const COLOR_PALETTE_INDEX_WRITE_ADDRESSS: u16 = 0x3C8;

/// Represents a vga emulation mode.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum EmulationMode {
    /// Represents a monochrome emulation mode.
    Mda = 0x0,
    /// Respresents a color emulation mode.
    Cga = 0x1,
}

impl From<u8> for EmulationMode {
    fn from(value: u8) -> EmulationMode {
        match value {
            0x0 => EmulationMode::Mda,
            0x1 => EmulationMode::Cga,
            _ => panic!("{} is an invalid emulation mode", value),
        }
    }
}
