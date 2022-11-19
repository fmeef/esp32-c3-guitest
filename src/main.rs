use anyhow::Result;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::mono_font::iso_8859_13::FONT_8X13_BOLD;
use embedded_graphics::mono_font::iso_8859_14::FONT_8X13;
use embedded_graphics::mono_font::iso_8859_3::FONT_7X13;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Triangle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyleBuilder;
use embedded_graphics::Drawable;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::prelude::_embedded_hal_blocking_spi_Write;
use embedded_hal::spi::Mode;
use epd_waveshare::epd2in13_v3::Display2in13;
use epd_waveshare::epd2in13_v3::Epd2in13;
use epd_waveshare::graphics::Display;
use epd_waveshare::prelude::Color;
use epd_waveshare::prelude::WaveshareDisplay;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;

fn setup_display() -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();
    esp_idf_logger::init().expect("failed to setup logging");
    let peripherals = Peripherals::take().unwrap();
    let spi = peripherals.spi2;

    let mut delay = FreeRtos;

    println!("delay setup");

    let sclk = peripherals.pins.gpio6.into_output()?;
    let serial_out = peripherals.pins.gpio7.into_output()?;
    let cs = peripherals.pins.gpio10.into_output()?;
    let dc = peripherals.pins.gpio8.into_output()?;
    let busy = peripherals.pins.gpio1.into_input()?;
    let rst = peripherals.pins.gpio0.into_output()?;
    println!("gpio set");
    let config = <spi::config::Config as Default>::default().baudrate(1.MHz().into());

    println!("spi config");
    let mut spi = spi::Master::<spi::SPI2, _, _, _, _>::new(
        spi,
        spi::Pins {
            sclk,
            sdo: serial_out,
            sdi: None::<Gpio2<Input>>,
            cs: None::<Gpio1<Output>>,
        },
        config,
    )?;

    println!("spi setup");

    let mut driver = Epd2in13::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("cry");
    let mut buf = Display2in13::default();
    buf.set_rotation(epd_waveshare::graphics::DisplayRotation::Rotate90);
    let style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
    Text::new(
        "Look up at a star\nits incredibly far\nbut further away altogether...\nI make a many tired noise\nzzz",
        Point::new(0, 10),
        style,
    )
    .draw(&mut buf)?;

    Triangle::new(Point::new(20, 60), Point::new(40, 60), Point::new(30, 75))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut buf)?;

    Triangle::new(Point::new(20, 70), Point::new(40, 70), Point::new(30, 55))
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut buf)?;

    driver.set_refresh(
        &mut spi,
        &mut delay,
        epd_waveshare::prelude::RefreshLut::Full,
    )?;
    driver.clear_frame(&mut spi, &mut delay)?;
    driver.display_frame(&mut spi, &mut delay)?;
    driver.update_and_display_frame(&mut spi, buf.buffer(), &mut delay)?;
    driver.sleep(&mut spi, &mut delay)?;

    Ok(())
}
