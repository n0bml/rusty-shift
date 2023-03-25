#![no_std]
#![no_main]

/**
 * @file src/bin/rusty-scan.rs
 * @brief Scan all LEDs in order.
 * @author Brendan Leber <n0bml@brendanleber.com>
 * @copyright Copyright 2023 by Brendan Leber.  Some rights reserved, see LICENSE.
 */
use arduino_hal::{
    hal::port::{PB3, PB4},
    port::{mode::Output, Pin},
};
use panic_halt as _;

#[derive(PartialEq)]
enum Direction {
    Forward,
    Backward,
}

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

    let mut bit: isize = 0;
    let mut direction: Direction = Direction::Forward;

    loop {
        if direction == Direction::Forward {
            bit += 1;
        } else {
            bit -= 1;
        }

        if bit > 7 {
            bit = 7;
            direction = Direction::Backward;
        } else if bit < 0 {
            bit = 0;
            direction = Direction::Forward;
        }

        let mask: u8 = (1 << bit) as u8;

        latch_pin.set_low();
        shift_out(&mut data_pin, &mut clock_pin, &mask);
        latch_pin.set_high();

        arduino_hal::delay_ms(100);
    }
}
