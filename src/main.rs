use anyhow::Result;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::iso_8859_14::FONT_8X13;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyleBuilder;
use embedded_graphics::Drawable;
use embedded_hal::blocking::delay::DelayMs;
use epd_waveshare::epd2in13_v2::Display2in13;
use epd_waveshare::epd2in13bc::Display2in13bc;
use epd_waveshare::epd2in13bc::Epd2in13bc;
use epd_waveshare::graphics::Display;
use epd_waveshare::prelude::TriColor;
use epd_waveshare::prelude::WaveshareDisplay;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::Gpio10;
use esp_idf_hal::gpio::Gpio7;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use std::process::Output; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let mut delay = FreeRtos;

    let sclk = peripherals.pins.gpio6;
    let serial_out = peripherals.pins.gpio7;
    let cs = peripherals.pins.gpio10.into_output()?;
    let dc = peripherals.pins.gpio1.into_output()?;
    let busy = peripherals.pins.gpio3.into_input()?;
    let rst = peripherals.pins.gpio2.into_output()?;

    let config = <spi::config::Config as Default>::default().baudrate(8.MHz().into());

    let mut spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
        spi,
        spi::Pins {
            sclk,
            sdo: serial_out,
            sdi: None::<Gpio7<Output>>,
            cs: None::<Gpio10<Output>>,
        },
        config,
    )?;

    let mut driver = Epd2in13bc::new(&mut spi, cs, busy, dc, rst, &mut delay)?;
    let mut buf = Display2in13::default();

    let style = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);
    Text::new(
        "Look up at a star\nits incredibly far\nbut further away altogether...",
        Point::new(0, 0),
        style,
    )
    .draw(&mut buf)?;
    driver.update_and_display_frame(&mut spi, buf.buffer(), &mut delay)?;
    println!("Hello, world!");
    loop {
        delay.delay_ms(1000 as u32);
    }
}
