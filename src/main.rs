use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use epd_waveshare::{
    graphics::VarDisplay,
    prelude::*,
    devices::EPD2in9,
    spi::{Spi, Interface},
};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi as RppalSpi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize SPI interface using `rppal`
    let spi = RppalSpi::new(Bus::Spi0, SlaveSelect::Ss0, 4_000_000, Mode::Mode0)?;
    let interface = Interface::new(spi, 25, 17, 27)?; // Connect relevant GPIO pins
    
    let mut epd = EPD2in9::new(&interface, &mut delay, None)?;
    let mut buffer = vec![0u8; 4736]; // Display buffer for 2.9-inch display

    // Create a drawing display
    let mut display = VarDisplay::new(epd.width(), epd.height(), &mut buffer);

    // Draw a rectangle
    Rectangle::new(Point::new(10, 10), Size::new(100, 50))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)?;

    // Update the display
    epd.update_frame(&mut interface, &display.buffer(), None)?;
    epd.display_frame(&mut interface)?;

    Ok(())
}
