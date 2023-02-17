# rust-arduino-leds
A simple project that drives some color LEDS with an Atmega328

# How it works

I use [`embedded-hal`](https://crates.io/crates/embedded-hal) to use the Arduino Nano hardware.

I use [`smart-leds`](https://crates.io/crates/smart-leds) and the `ws2812-spi` to actually communicate with those leds.

I use [`cichlid`](https://crates.io/crates/cichlid) for the color effects

## Important variables

- `NUM_PINS`: Number of leds to drive
- `COLORS`: Static predefined colors
- `num_colors` number of colors in `COLORS`

## Hardware stuff

- Data for the leds is written to pin `D11`
- `A0` and `A1` are setting the start and end color in the gradient
- `A2` sets the "animation" speed

I found that for my board the `clock` configuration was to be set at `OscfOver4` but that might be different for you. 
