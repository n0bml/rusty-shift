# Rusty Shift

My implementation of the [Arduino](https://www.arduino.cc/)
[Serial to Parallel Shifting-Out with a 74HC595](https://docs.arduino.cc/tutorials/communication/guide-to-shift-out),
written for the [SparkFun](https://www.sparkfun.com/)
[RedBoard](https://www.sparkfun.com/products/13975).
Written in Rust.

## rusty-shift

Count from 0 to 255 and display the values in binary via the 8 LEDs attached to the 74HC595 [shift register](https://en.wikipedia.org/wiki/Shift_register).

## rusty-scan

Display a single lit LED moving back and forth along the line of 8 attached LEDs.  Think the visor in an old school Cylon from the 1978 show [Battlestar Galactica](https://en.wikipedia.org/wiki/Battlestar_Galactica_(1978_TV_series)).
