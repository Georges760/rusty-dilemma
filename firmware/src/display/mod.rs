use embassy_executor::Spawner;
use embassy_rp::{
    dma::AnyChannel,
    gpio::{Level, Output},
    peripherals::{PIN_11, PIN_12, PIN_13, PIN_16, PIN_22, PIN_23, PWM_CH0, SPI0},
    pwm::{self, Pwm},
    spi::{self, Async, Spi},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Delay;
use embedded_hal_async::spi::ExclusiveDevice;

use heapless::String;

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use mipidsi::Builder;

const DISPLAY_FREQ: u32 = 64_000_000;
const FERRIS_WIDTH: u32 = 86;

type DisplayInterface = SPIInterface<
    ExclusiveDevice<Spi<'static, SPI0, Async>, Output<'static, PIN_12>, Delay>,
    Output<'static, PIN_11>,
>;
type RstOutput = Output<'static, PIN_13>;

static BACKLIGHT_EVENT: Channel<ThreadModeRawMutex, u16, 2> = Channel::new();
static KEYBOARD_EVENT: Channel<ThreadModeRawMutex, String<24>, 2> = Channel::new();

pub async fn set_backlight(percent: u16) {
    BACKLIGHT_EVENT.send(percent).await;
}

pub async fn display_key_code(keycodes: String<24>) {
    KEYBOARD_EVENT.send(keycodes).await;
}

#[allow(clippy::too_many_arguments)]
pub fn init(
    spawner: &Spawner,
    spi: SPI0,
    clk: PIN_22,
    mosi: PIN_23,
    cs: PIN_12,
    dc: PIN_11,
    rst: PIN_13,
    bl: PIN_16,
    tx_dma: AnyChannel,
    bl_pwm: PWM_CH0,
) {
    let mut spi_config = spi::Config::default();
    spi_config.frequency = DISPLAY_FREQ;
    spi_config.phase = spi::Phase::CaptureOnSecondTransition;
    spi_config.polarity = spi::Polarity::IdleHigh;

    let spi = Spi::new_txonly(spi, clk, mosi, tx_dma, spi_config);

    let spi = ExclusiveDevice::new(spi, Output::new(cs, Level::High), Delay);

    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::Low);

    spawner.must_spawn(backlight_task(bl, bl_pwm));

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(spi, dc);

    spawner.must_spawn(display_task(di, rst));
}

#[embassy_executor::task]
async fn backlight_task(bl: PIN_16, pwm: PWM_CH0) {
    // Enable LCD backlight
    let mut pwm_config = pwm::Config::default();
    pwm_config.top = 100;
    pwm_config.compare_a = 50;
    let mut bl_pwm = Pwm::new_output_a(pwm, bl, pwm_config.clone());
    loop {
        pwm_config.compare_a = BACKLIGHT_EVENT.recv().await;
        bl_pwm.set_config(&pwm_config);
    }
}

#[embassy_executor::task]
async fn display_task(di: DisplayInterface, rst: RstOutput) {
    // Define the display from the display interface and initialize it
    #[cfg(feature = "gc9a01")]
    let mut display = Builder::gc9a01(di)
        .with_display_size(240, 240)
        .with_color_order(mipidsi::ColorOrder::Bgr)
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut Delay, Some(rst))
        .unwrap();

    // Make the display all black
    display.clear(Rgb565::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../../assets/ferris.raw"), FERRIS_WIDTH);
    let ferris = Image::new(&raw_image_data, Point::new(34, 68));
    // Display the image
    ferris.draw(&mut display).unwrap();

    loop {
        let keycodes = KEYBOARD_EVENT.recv().await;

        let style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
        Text::new(keycodes.as_str(), Point::new(20, 200), style)
            .draw(&mut display)
            .unwrap();
    }
}
