//! main display module
use crate::{
    command::Command,
    size::{DisplaySize, NewZeroed},
};
use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Size},
    pixelcolor::{Gray4, GrayColor},
    Pixel,
};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

/// Represents the SSD1327 Display.
///
/// Use this struct to initialize the driver.
pub struct Ssd1327<DI, SIZE>
where
    SIZE: DisplaySize,
{
    display: DI,
    buffer: SIZE::Buffer,
    // XXX: Figure out whether this duplicates buffer...
    _size: SIZE,
}

impl<DI, SIZE> Ssd1327<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Creates the SSD1327 Display.
    ///
    /// Make sure to reset and initialize the display before use!
    pub fn new(display: DI, _size: SIZE) -> Self {
        Self {
            display,
            _size,
            buffer: NewZeroed::new_zeroed(),
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
        self.send_command(Command::ColumnAddress {
            start: 0,
            end: SIZE::WIDTH - 1,
        })?;
        self.send_command(Command::RowAddress {
            start: 0,
            end: SIZE::HEIGHT - 1,
        })?;
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
        self.display.send_data(U8(&self.buffer.as_mut()))
    }
}

impl<DI, SIZE> DrawTarget for Ssd1327<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Color = Gray4;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(p, _c)| bb.contains(*p))
            .for_each(|Pixel(point, color)| {
                let idx = (point.x / 2 + point.y * 64) as usize;
                if let Some(byte) = self.buffer.as_mut().get_mut(idx) {
                    if point.x % 2 == 0 {
                        *byte = update_upper_half(*byte, color.luma());
                    } else {
                        *byte = update_lower_half(*byte, color.luma());
                    }
                }
            });

        Ok(())
    }

    fn clear(&mut self, fill: Gray4) -> Result<(), Self::Error> {
        let luma = fill.luma();
        let byte = (luma << 4) | luma;
        for b in self.buffer.as_mut() {
            *b = byte;
        }
        Ok(())
    }
}

impl<DI, SIZE> OriginDimensions for Ssd1327<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn size(&self) -> Size {
        Size::new(SIZE::WIDTH as u32, SIZE::HEIGHT as u32)
    }
}

#[inline]
fn update_upper_half(input: u8, color: u8) -> u8 {
    color << 4 | (input & 0x0F)
}

#[inline]
fn update_lower_half(input: u8, color: u8) -> u8 {
    color & 0x0f | (input & 0xF0)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn updates_upper_half_byte() {
        let input = 0b00000000;
        let color = 0b00001111;
        assert_eq!(0b11110000, update_upper_half(input, color));

        let input = 0b11110000;
        let color = 0b00000000;
        assert_eq!(0b00000000, update_upper_half(input, color));
    }

    #[test]
    fn leaves_lower_untouched_on_upper_change() {
        let input = 0b00000011;
        let color = 0b00001111;
        assert_eq!(0b11110011, update_upper_half(input, color));

        let input = 0b11111111;
        let color = 0b00000000;
        assert_eq!(0b00001111, update_upper_half(input, color));
    }

    #[test]
    fn updates_lower_half_byte() {
        let input = 0b00000000;
        let color = 0b00001111;
        assert_eq!(0b00001111, update_lower_half(input, color));

        let input = 0b00000000;
        let color = 0b00000000;
        assert_eq!(0b00000000, update_lower_half(input, color));
    }

    #[test]
    fn leaves_upper_untouched_on_lower_change() {
        let input = 0b11000011;
        let color = 0b00001111;
        assert_eq!(0b11001111, update_lower_half(input, color));

        let input = 0b11111111;
        let color = 0b00000000;
        assert_eq!(0b11110000, update_lower_half(input, color));
    }
}
