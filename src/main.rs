#![no_std]
#![no_main]

/**
 * @file src/main.rs
 * @brief Count from 0 to 255 and display the value via LEDs.
 * @author Brendan Leber <n0bml@brendanleber.com>
 * @copyright Copyright 2023 by Brendan Leber.  Some rights reserved, see LICENSE.
 */
use arduino_hal::{
    hal::port::{PB3, PB4},
    port::{mode::Output, Pin},
};
use panic_halt as _;

fn shift_out(data_pin: &mut Pin<Output, PB3>, clock_pin: &mut Pin<Output, PB4>, data: &u8) {
    for i in 0..8 {
        let n = data & (1 << i);
        if n == 0 {
            data_pin.set_low();
        } else {
            data_pin.set_high();
        }

        clock_pin.set_high();
        clock_pin.set_low();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);

    let mut latch_pin = pins.d8.into_output();
    let mut data_pin = pins.d11.into_output();
    let mut clock_pin = pins.d12.into_output();

    loop {
        for num in 0..256 {
            latch_pin.set_low();
            shift_out(&mut data_pin, &mut clock_pin, &(num as u8));
            latch_pin.set_high();
            arduino_hal::delay_ms(1000);
        }
    }
}
