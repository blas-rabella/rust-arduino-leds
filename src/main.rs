#![no_std]
#![no_main]

use crate::ws2812::prerendered::Ws2812;
use arduino_hal::spi;
use cichlid::{rgb_gradient, ColorRGB};
use panic_halt as _;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi as ws2812;

#[arduino_hal::entry]
fn main() -> ! {
    const NUM_PINS: usize = 50;
    const COLORS: &'static [ColorRGB] = &[
        ColorRGB::Green,
        ColorRGB::DarkGreen,
        ColorRGB::Orange,
        ColorRGB::Turquoise,
        ColorRGB::DarkRed,
        ColorRGB::GreenYellow,
        ColorRGB::Red,
        ColorRGB::Cyan,
        ColorRGB::Purple, // yellow
        ColorRGB::SeaGreen,
        ColorRGB::Violet,
        ColorRGB::Chartreuse,
        ColorRGB::HotPink,
        ColorRGB::Blue,
        ColorRGB::DarkOliveGreen,
        ColorRGB::Black,
        // To be filled...
        ColorRGB::PapayaWhip,
        ColorRGB::Plum,
        ColorRGB::Teal,
        ColorRGB::Thistle,
        ColorRGB::SpringGreen,
        ColorRGB::SteelBlue,
        ColorRGB::LimeGreen,
        ColorRGB::Tomato,
        ColorRGB::Indigo,
        ColorRGB::IndianRed,
        ColorRGB::Honeydew,
        ColorRGB::Fuchsia,
        ColorRGB::Gold,
        ColorRGB::Amethyst,
        ColorRGB::NavajoWhite,
        ColorRGB::OrangeRed,
    ];
    let num_colors = 32;
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings {
            clock: spi::SerialClockRate::OscfOver4,
            ..Default::default()
        },
    );

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);

    let mut output_buffer = [0; 20 + (NUM_PINS * 12)];
    let mut data: [RGB8; NUM_PINS] = [RGB8::default(); NUM_PINS];
    let mut colors = [ColorRGB::new(27, 77, 62); NUM_PINS];

    let mut shifted: [RGB8; NUM_PINS] = [RGB8::default(); NUM_PINS];
    let mut ws = Ws2812::new(spi, &mut output_buffer);
    let mut j = 0;
    loop {
        let raw_a0: u16 = a0.analog_read(&mut adc);
        let raw_a1 = a1.analog_read(&mut adc);
        let raw_a2 = a2.analog_read(&mut adc);
        // Empirically the values where between 0 and 1024
        // so, shifting 5 will set them arround 32
        let color1: usize = (raw_a0 >> 5) as usize;
        let color2 = (raw_a1 >> 5) as usize;
        let speed = 350 + raw_a2;

        rgb_gradient(
            &mut colors,
            NUM_PINS,
            COLORS[color1 % num_colors],
            COLORS[color2 % num_colors], // module just in case
        );
        // probably manage if there is a change or not...
        for x in 0..NUM_PINS {
            data[x] = RGB8 {
                r: colors[x].r,
                b: colors[x].b,
                g: colors[x].g,
            }
        }

        for x in 0..NUM_PINS {
            shifted[(x + j) % NUM_PINS] = data[x];
        }
        ws.write(shifted.iter().cloned()).unwrap();
        arduino_hal::delay_ms(speed as u16);
        j += 1;
    }
}
