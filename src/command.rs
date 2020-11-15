//! Contains all the commands that can be sent to the display

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

/// Holds commands which can be sent to the display.
pub enum Command {
    /// Turn display off (0xAE)
    DisplayOff,
    /// Turn display on (0xAF)
    DisplayOn,
    /// Set up column start and end address (0x15)
    ColumnAddress {
        /// The start column address
        start: u8,
        /// The end column address
        end: u8,
    },
    /// Set up row start and end address (0x75)
    RowAddress {
        /// The start row address
        start: u8,
        /// The end row address
        end: u8,
    },
    /// Contrast Control (0x81)
    Contrast(u8),
    /// Re-map setting in Graphic Display Data RAM  (0xA0)
    SetRemap(u8),
    /// Display Start Line (0xA1)
    StartLine(u8),
    /// Display Offset (0xA2)
    Offset(u8),
    /// Normal Display Mode (0xA4)
    DisplayModeNormal,
    /// Multiplex Ratio (0xA8)
    MuxRatio(u8),
    /// Phase Length (0xB1)
    PhaseLength(u8),
    /// Front Clock Divider / Oscillator Frequency (0xB3)
    FrontClockDivider(u8),
    /// Function Selection A (0xAB)
    FunctionSelectionA(u8),
    /// Second Pre-Charge Period (0xB6)
    SecondPreChargePeriod(u8),
    /// COM deselect voltage level (0xBE)
    ComVoltageLevel(u8),
    /// Pre-Charge Voltage (0xBC)
    PreChargeVoltage(u8),
    /// Function Selection B (0xD5)
    FunctionSelectionB(u8),
    /// Function Selection B (0xD5)
    CommandLock(u8),
}

impl Command {
    pub(crate) fn send<DI>(self, display: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        let (data, len) = match self {
            Self::DisplayOn => ([0xAF, 0, 0], 1),
            Self::DisplayOff => ([0xAE, 0, 0], 1),
            Self::ColumnAddress { start, end } => ([0x15, start, end], 3),
            Self::RowAddress { start, end } => ([0x75, start, end], 3),
            Self::Contrast(value) => ([0x81, value, 0], 2),
            Self::SetRemap(value) => ([0xA0, value, 0], 2),
            Self::StartLine(value) => ([0xA1, value, 0], 2),
            Self::Offset(value) => ([0xA2, value, 0], 2),
            Self::DisplayModeNormal => ([0xA4, 0, 0], 1),
            Self::MuxRatio(value) => ([0xA8, value, 0], 2),
            Self::PhaseLength(value) => ([0xB1, value, 0], 2),
            Self::FrontClockDivider(value) => ([0xB3, value, 0], 2),
            Self::FunctionSelectionA(value) => ([0xAB, value, 0], 2),
            Self::SecondPreChargePeriod(value) => ([0xB6, value, 0], 2),
            Self::ComVoltageLevel(value) => ([0xBE, value, 0], 2),
            Self::PreChargeVoltage(value) => ([0xBC, value, 0], 2),
            Self::FunctionSelectionB(value) => ([0xD5, value, 0], 2),
            Self::CommandLock(value) => ([0xFD, value, 0], 2),
        };
        display.send_commands(U8(&data[0..len]))
    }
}
