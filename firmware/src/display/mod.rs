use embassy_executor::Spawner;
use embassy_rp::{
    dma::AnyChannel,
    gpio::{self, Level, Output},
    peripherals::{PIN_10, PIN_11, PIN_12, PIN_25, PIN_8, PIN_9, SPI1},
    spi::{self, Async, Spi},
};
use embassy_time::{Delay, Duration};
use embedded_hal_async::spi::ExclusiveDevice;

use crate::utils::Ticker;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    pixelcolor::Rgb565,
    prelude::*,
};
use mipidsi::Builder;

const DISPLAY_FREQ: u32 = 64_000_000;

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

    let spi = ExclusiveDevice::new(spi, Output::new(cs, gpio::Level::Low), Delay);

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
    let mut display = Builder::gc9a01(di).init(&mut Delay, Some(rst)).unwrap();

    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../../assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(34, 68));
    // Display the image
    ferris.draw(&mut display).unwrap();

    let mut ticker = Ticker::every(Duration::from_hz(24));

    loop {
        //TODO display infos here
        ticker.next().await;
    }
}
