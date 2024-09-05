#![no_std]
#![no_main]

use core::sync::atomic::AtomicU32;

use cortex_m_rt::{exception};
use embedded_hal::digital::OutputPin;
use rp2040_hal::{pac, rom_data::reset_to_usb_boot, Sio};

#[cfg(feature = "binaryinfo")]
pub mod binary_info;

extern "C" {
    static __bootloader_application_start: u32;
    static __bootloader_application_end: u32;
}

#[link_section = ".uninit.bootloader_magic"]
#[used]
static BOOTLOADER_MAGIC: AtomicU32 = AtomicU32::new(0);

const MAGIC_TOKEN: u32 = 0xCAFEB0BA;

unsafe fn check_bootloader() {
    const CYCLES_PER_US: usize = 125;
    const WAIT_CYCLES: usize = 100 * 1000 * CYCLES_PER_US;

    if BOOTLOADER_MAGIC.load(atomic_polyfill::Ordering::SeqCst) != MAGIC_TOKEN {
        BOOTLOADER_MAGIC.store(MAGIC_TOKEN, atomic_polyfill::Ordering::SeqCst);

        cortex_m::asm::delay(WAIT_CYCLES as u32);
        BOOTLOADER_MAGIC.store(0, atomic_polyfill::Ordering::SeqCst);
        return;
    }

    BOOTLOADER_MAGIC.store(0, atomic_polyfill::Ordering::SeqCst);

    reset_to_usb_boot(1 << 17, 0);
}

pub const FLASH_BASE: *const u32 = 0x10000000 as _;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut s = pins.gpio17.into_push_pull_output();
    s.set_high().unwrap();

    unsafe {
        check_bootloader();
    }

    unsafe {
        let p = cortex_m::Peripherals::steal();
        let start = FLASH_BASE as u32 + &__bootloader_application_start as *const u32 as u32;
        s.set_low().unwrap();
        p.SCB.vtor.write(start);
        cortex_m::asm::bootload(start as *const u32);
    }
}

#[no_mangle]
#[cfg_attr(target_os = "none", link_section = ".HardFault.user")]
unsafe extern "C" fn HardFault() {
    cortex_m::peripheral::SCB::sys_reset();
}

#[exception]
unsafe fn DefaultHandler(_: i16) -> ! {
    const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;
    let irqn = core::ptr::read_volatile(SCB_ICSR) as u8 as i16 - 16;

    panic!("DefaultHandler #{:?}", irqn);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
