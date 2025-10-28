# hd44780-embedded-hal

Driver library for HD44780-LCD built ontop of [`embedded-hal`] or [`embedded-hal-async`].
If you want to make use of the [`embedded-hal-async`] traits, set the [`async`] feature.
This crate can then be used with crates that implement there traits.

* [on crates.io](https://crates.io/crates/hd44780-embedded-hal)
* [on github](https://github.com/wolkns/hd44780-embedded-hal)
* [Documentation](https://docs.rs/hd44780-embedded-hal/0.1.3/hd44780_embedded_hal/)

## Quick Example

```rust
// setup microcontroller environment
// ...

let i2c = ...;   // impl of embedded_hal_async::i2c::I2c       (or embedded_hal::i2c::I2c)
let delay = ...; // impl of embedded_hal_async::delay::DelayNs (or embedded_hal::delay::DelayNs)

let mut lcd = Hd44780::new(
    Pcf8574Interface::new(
        i2c,
        0x20,  // choose your address
        delay,
        Pcf8574EncoderDefault::new()
    ),
    DisplayTypeFont5x8::new(2,16,FnsetLines::Two)
);

lcd.init().await;

lcd.print_str("Hello World").await;

lcd.position(1,0).await; // set cursor to 2nd row

lcd_write!(
  lcd, 
  "{}={}", 
  hd44780_embedded_hal::characters::NonASCIIA00::GreekSmallPi, 
  3.1415
).await;

```

## Interfaces

This library implements the following interfaces to connect HD44780 with your microcontroller:

* PCF8574
  * 8-bit portexpander through I2C
* GPIO 4-bit data length
  * requires 8 GPIO pins
* GPIO 8-bit data length
  * requires 12 GPIO pins

## ToDo's

* There are some characters not defined in [`characters.rs`].
* Testing of GPIO Interfaces
