//! main display module
use crate::command::Command;
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics::{
    drawable::Pixel,
    pixelcolor::{Gray4, GrayColor},
    prelude::Size,
    DrawTarget,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 128;
const BUFFER_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGHT * 4 / 8;

/// Represents the SSD1327 Display.
///
/// Use this struct to initialize the driver.
pub struct Ssd1327<DI> {
    display: DI,
    buffer: [u8; BUFFER_SIZE],
}

impl<DI: WriteOnlyDataCommand> Ssd1327<DI> {
    /// Creates the SSD1327 Display.
    ///
    /// Make sure to reset and initialize the display before use!
    pub fn new(display: DI) -> Self {
        Self {
            display,
            buffer: [0; BUFFER_SIZE],
        }
    }

    /// Resets the display.
    pub fn reset<RST, DELAY>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), DisplayError>
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(100);

        rst.set_low().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(100);

        rst.set_high().map_err(|_| DisplayError::BusWriteError)?;
        delay.delay_ms(100);

        Ok(())
    }

    /// Initializes the display.
    pub fn init(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOff)?;
        self.send_command(Command::ColumnAddress { start: 0, end: 127 })?;
        self.send_command(Command::RowAddress { start: 0, end: 127 })?;
        self.send_command(Command::Contrast(0x80))?;
        self.send_command(Command::SetRemap(0x51))?;
        self.send_command(Command::StartLine(0x00))?;
        self.send_command(Command::Offset(0x00))?;
        self.send_command(Command::DisplayModeNormal)?;
        self.send_command(Command::MuxRatio(0x7f))?;
        self.send_command(Command::PhaseLength(0xf1))?;
        self.send_command(Command::FrontClockDivider(0x00))?;
        self.send_command(Command::FunctionSelectionA(0x01))?;
        self.send_command(Command::SecondPreChargePeriod(0x0f))?;
        self.send_command(Command::ComVoltageLevel(0x0f))?;
        self.send_command(Command::PreChargeVoltage(0x08))?;
        self.send_command(Command::FunctionSelectionB(0x62))?;
        self.send_command(Command::CommandLock(0x12))?;
        self.send_command(Command::DisplayOn)?;

        Ok(())
    }

    /// Allows to send custom commands to the display.
    pub fn send_command(&mut self, command: Command) -> Result<(), DisplayError> {
        command.send(&mut self.display)
    }

    /// Flushes the display, and makes the output visible on the screen.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.display.send_data(U8(&self.buffer))
    }
}

impl<DI> DrawTarget<Gray4> for Ssd1327<DI> {
    type Error = DisplayError;

    fn draw_pixel(&mut self, pixel: Pixel<Gray4>) -> Result<(), Self::Error> {
        let Pixel(point, color) = pixel;

        let idx = (point.x / 2 + point.y * 64) as usize;
        if point.x % 2 == 0 {
            self.buffer[idx] = (color.luma() << 4) | self.buffer[idx];
        } else {
            self.buffer[idx] = (color.luma() & 0x0f) | self.buffer[idx];
        }

        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
    }
}
