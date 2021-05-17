use {
    display_interface_spi::SPIInterface,
    embedded_graphics::{
        fonts::{Font24x32, Text},
        pixelcolor::Gray4,
        prelude::*,
        text_style,
    },
    linux_embedded_hal::Delay,
    rppal::{
        gpio::Gpio,
        spi::{Bus, Mode, SlaveSelect, Spi},
    },
    ssd1327,
};

fn main() {
    // Configure gpio
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).unwrap();
    let gpio = Gpio::new().unwrap();
    let cs = gpio.get(8).unwrap().into_output();
    let dc = gpio.get(5).unwrap().into_output();
    let mut rst = gpio.get(6).unwrap().into_output();

    // Init SPI
    let spii = SPIInterface::new(spi, dc, cs);
    let mut disp = ssd1327::display::Ssd1327::new(spii);

    // Reset & init
    disp.reset(&mut rst, &mut Delay).unwrap();
    disp.init().unwrap();

    // Clear the display
    disp.clear(Gray4::new(0)).unwrap();
    disp.flush().unwrap();

    // Write "Hello" to the display
    Text::new("Hello", Point::new(0, 0))
        .into_styled(text_style!(
            font = Font24x32,
            text_color = Gray4::new(0b0000_1111),
            background_color = Gray4::new(0),
        ))
        .draw(&mut disp)
        .unwrap();
    disp.flush().unwrap();
}
