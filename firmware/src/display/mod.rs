use embassy_executor::Spawner;
use embassy_rp::{
    dma::AnyChannel,
    gpio::{self, Level, Output},
    peripherals::{PIN_10, PIN_11, PIN_12, PIN_25, PIN_8, PIN_9, SPI1},
    spi::{self, Async, Spi},
};
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_hal_async::spi::ExclusiveDevice;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};
use mipidsi::Builder;

const DISPLAY_FREQ: u32 = 64_000_000;
const LCD_X_RES: i32 = 240;
const LCD_Y_RES: i32 = 240;
const FERRIS_WIDTH: u32 = 86;
const FERRIS_HEIGHT: u32 = 64;

type DisplayInterface = SPIInterface<
    ExclusiveDevice<Spi<'static, SPI1, Async>, Output<'static, PIN_9>, Delay>,
    Output<'static, PIN_8>,
>;
type RstOutput = Output<'static, PIN_12>;

#[allow(clippy::too_many_arguments)]
pub fn init(
    spawner: &Spawner,
    spi: SPI1,
    clk: PIN_10,
    mosi: PIN_11,
    cs: PIN_9,
    dc: PIN_8,
    rst: PIN_12,
    bl: PIN_25,
    tx_dma: AnyChannel,
) {
    let mut config = spi::Config::default();
    config.frequency = DISPLAY_FREQ;
    config.phase = spi::Phase::CaptureOnSecondTransition;
    config.polarity = spi::Polarity::IdleHigh;

    let spi = Spi::new_txonly(spi, clk, mosi, tx_dma, config);

    let spi = ExclusiveDevice::new(spi, Output::new(cs, gpio::Level::High), Delay);

    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::Low);

    // Enable LCD backlight
    let _bl = Output::new(bl, Level::High);

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(spi, dc);

    spawner.must_spawn(display_task(di, rst));
}

#[embassy_executor::task]
async fn display_task(di: DisplayInterface, rst: RstOutput) {
    // Define the display from the display interface and initialize it
    let mut display = Builder::gc9a01(di)
        .with_display_size(240, 240)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut Delay, Some(rst))
        .unwrap();

    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../../assets/ferris.raw"), FERRIS_WIDTH);
    let mut ferris = Image::new(&raw_image_data, Point::zero());

    let mut delta = Point { x: 5, y: 10 };
    loop {
        // Keep Ferris in the LCD area
        let bb = ferris.bounding_box();
        let tl = bb.top_left;
        let br = bb.bottom_right().unwrap();
        if tl.x < 0 || br.x > LCD_X_RES {
            delta.x = -delta.x;
        }
        if tl.y < 0 || br.y > LCD_Y_RES {
            delta.y = -delta.y;
        }

        // Erase ghosting
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();
        let mut off = Point { x: 0, y: 0 };
        if delta.x < 0 {
            off.x = FERRIS_WIDTH as i32;
        }
        Rectangle::new(tl + off, Size::new(delta.x as u32, FERRIS_HEIGHT))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();
        off = Point { x: 0, y: 0 };
        if delta.y < 0 {
            off.y = FERRIS_HEIGHT as i32;
        }
        Rectangle::new(tl + off, Size::new(FERRIS_WIDTH, delta.y as u32))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();
        // Translate Ferris
        ferris.translate_mut(delta);
        // Display Ferris
        ferris.draw(&mut display).unwrap();
        Timer::after(Duration::from_millis(50)).await;
    }
}
