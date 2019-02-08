extern crate linux_embedded_hal;
use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};
use embedded_graphics::drawable::Pixel;
use embedded_graphics::unsignedcoord::UnsignedCoord;

extern crate bmp;
use bmp::consts::BLACK;

extern crate ssd1675;
use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};

// Graphics
extern crate embedded_graphics;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;
use std::{env, io};
use std::io::Write;
use std::time::{Duration, Instant};

// Font
extern crate profont;
use profont::ProFont14Point;

// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

const ROWS: u16 = 212;
const COLS: u8 = 104;

#[rustfmt::skip]
const LUT_FAST_YELLOW: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b11111010, 0b10010100, 0b10001100, 0b11000000, 0b11010000,  0b00000000, 0b00000000,  // LUT0 - Black
    0b11111010, 0b10010100, 0b00101100, 0b10000000, 0b11100000,  0b00000000, 0b00000000,  // LUTT1 - White
    0b11111010, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  0b00000000, 0b00000000,  // IGNORE
    0b11111010, 0b10010100, 0b11111000, 0b10000000, 0b01010000,  0b00000000, 0b11001100,  // LUT3 - Yellow (or Red)
    0b10111111, 0b01011000, 0b11111100, 0b10000000, 0b11010000,  0b00000000, 0b00010001,  // LUT4 - VCOM

    // Duration            | Repeat
    // A   B     C     D   |
    0,     0,   64,   16,    1,
    8,    16,    4,    4,    2,
    8,     8,    3,    8,    2,
    8,     4,    0,    0,    2,
    16,    8,    8,    0,    4,
    0,     0,    0,    0,    0,
    0,     0,    0,    0,    0,
];

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat
    // Configure Digital I/O Pins
    let cs = Pin::new(8); // BCM8
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(17); // BCM17
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(22); // BCM22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let reset = Pin::new(27); // BCM27
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset
        .set_direction(Direction::Out)
        .expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");
    println!("Pins configured");

    // Initialise display controller
    let mut delay = Delay {};

    let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

    let mut black_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let mut color_buffer = [0u8; ROWS as usize * COLS as usize / 8];

    let LUT = {
      if env::args().find(|arg| arg.ends_with("bmp")).is_some() {
        LUT_YELLOW.clone()
        } else {
        LUT_FAST_YELLOW.clone()
      }
    };
    let config = Builder::new()
        .dimensions(Dimensions {
            rows: ROWS,
            cols: COLS,
        })
        .rotation(Rotation::Rotate270)
        .lut(&LUT)
        .yellow(&true)
        .build()
        .expect("invalid configuration");
    let display = Display::new(controller, config);
    let mut display = GraphicDisplay::new(display, &mut black_buffer, &mut color_buffer);

    display.reset(&mut delay).expect("error resetting display");
    println!("Reset and initialised");

    display.clear(Color::White);
    println!("Clear");

    let interactive = env::args().count() == 1;

    let mut messages = if interactive {
      vec!["interactive", "mode", "use", "stdin"]
        .into_iter() // allocations!
        .map(String::from) // allocations!
        .collect()
    } else {
      env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ")
        .split(",")
        .map(String::from) // allocations!
        .collect::<Vec<String>>()
    };


    let coords = vec![0, 1, 2, 3].iter().map(|line|
      Coord::new(2, 4 + (line * (14 + 4)))
    ).collect::<Vec<Coord>>();

    let mut render_start;
    let mut last_render_time = Duration::new(0, 1);

    loop {
        render_start = Instant::now();
        while messages.iter().count() > coords.iter().count() {
            messages.pop();
        }

        display.reset(&mut delay).expect("error resetting display");
        display.clear(Color::White);

        if let Some(bitmap) = env::args().find(|arg| arg.ends_with("bmp")) {
          let img = bmp::open(bitmap).unwrap_or_else(|e| {
            panic!("Failed to open: {}", e);
          });
          let colors = img.coordinates()
            .map(|(x, y)|  {
              let color = match img.get_pixel(x, y) {
                BLACK => Color::Black,
                _ => Color::White
              };
              return Pixel(UnsignedCoord::new(x, y), color)
            })
            .collect::<Vec<_>>();
          display.draw(colors.into_iter());
        } else {
          for (message, &coord) in messages.iter().zip(coords.iter()) {
              println!("{:?}: `{}`", coord, message);
              display.draw(
                ProFont14Point::render_str(message)
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(coord)
                .into_iter(),
              );
          }
        }

        let pretty_time = format!(" {} seconds ", last_render_time.as_secs().to_string());
        display.draw(
          ProFont14Point::render_str(&pretty_time)
          .with_stroke(Some(Color::White))
          .with_fill(Some(Color::Black))
          .translate(Coord::new((ROWS - (14 * 8)) as i32, (COLS - 14 - 2) as i32))
          .into_iter(),
        );

        display.update(&mut delay).expect("error updating display");
        println!("Update...");

        println!("Finished - going to sleep");
        display.deep_sleep()?;

        if interactive {
          print!("> ");
          io::stdout().flush().unwrap();
          last_render_time = Instant::now() - render_start;

          let mut input = String::new();
          match io::stdin().read_line(&mut input) {
              Ok(_) => {
                messages.insert(0, input.trim().to_string());
              },
              Err(error) => println!("error: {}", error),
          }
        } else {
          return Ok(());
        }
    }
}
