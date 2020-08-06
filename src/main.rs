#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
use panic_semihosting as _;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{i2c::I2c, prelude::*, stm32};
use ht16k33::{HT16K33, Dimming, Display};
use adafruit_7segment::{SevenSegment, Index};
pub use ascii::{ToAsciiChar, AsciiChar};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        const DISP_I2C_ADDR: u8 = 112;

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        // as per the STM32F407 datasheet.
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb7.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        let mut ht16k33 = HT16K33::new(i2c, DISP_I2C_ADDR);

        ht16k33.initialize().expect("Failed to initialize ht16k33");
        ht16k33.set_display(Display::ON).expect("Err0");
        ht16k33.set_dimming(Dimming::BRIGHTNESS_MIN).expect("Could not set dimming!");
        ht16k33.update_buffer_with_char(Index::One, AsciiChar::new('A')).expect("ERR");
        ht16k33.update_buffer_with_char(Index::Two, AsciiChar::new('b')).expect("ERR");
        ht16k33.update_buffer_with_char(Index::Three, AsciiChar::new('c')).expect("ERR");
        ht16k33.update_buffer_with_char(Index::Four, AsciiChar::new('-')).expect("ERR");
        ht16k33.update_buffer_with_dot(Index::One, true);
        ht16k33.update_buffer_with_dot(Index::Two, true);
        ht16k33.update_buffer_with_dot(Index::Three, true);
        ht16k33.update_buffer_with_dot(Index::Four, true);
        ht16k33.update_buffer_with_colon(true);
        ht16k33.write_display_buffer().unwrap();
        delay.delay_ms(1000_u32);
        ht16k33.clear_display_buffer();
        ht16k33.update_buffer_with_float(Index::One, 99.99, 2, 10).expect("ERR");
        ht16k33.write_display_buffer().unwrap();
        delay.delay_ms(1000_u32);
        ht16k33.clear_display_buffer();
        ht16k33.update_buffer_with_float(Index::Two, 99.9, 2, 10).expect("ERR");
        ht16k33.write_display_buffer().unwrap();
        delay.delay_ms(1000_u32);
        ht16k33.clear_display_buffer();

        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpiod = dp.GPIOD.split();
        let mut led_green  = gpiod.pd12.into_push_pull_output();
        let mut led_orange = gpiod.pd13.into_push_pull_output();
        let mut led_red    = gpiod.pd14.into_push_pull_output();
        let mut led_blue   = gpiod.pd15.into_push_pull_output();
        loop
        {
            led_green.set_high().unwrap();
            delay.delay_ms(200_u32);
            led_green.set_low().unwrap();

            led_orange.set_high().unwrap();
            delay.delay_ms(200_u32);
            led_orange.set_low().unwrap();

            led_red.set_high().unwrap();
            delay.delay_ms(200_u32);
            led_red.set_low().unwrap();

            led_blue.set_high().unwrap();
            delay.delay_ms(200_u32);
            led_blue.set_low().unwrap();
        }
    }

    loop {}
}
