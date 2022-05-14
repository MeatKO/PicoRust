#![no_std]
#![no_main]

mod pixel_ops;
mod framebuffer;
mod display;
mod model;
mod vector;
mod text;

use core::u8;

use cortex_m_rt::entry;

use framebuffer::{draw_line_horizontal, draw_line};
use panic_halt as _;
use rp2040_hal as hal;

use hal::clocks::Clock;
use hal::pac;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::rate::*;
use embedded_time::duration::*;
use embedded_time::clock;
use embedded_time::Instant;
use embedded_time::Timer;
use embedded_time::fixed_point::FixedPoint;
use display_interface_spi::SPIInterfaceNoCS;
use crate::vector::Vec2D;
use crate::pixel_ops::colors;
use crate::text::print_text;
use core::time;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! 
{
	let mut framebuffer = [0u8; display::SCREEN_WIDTH * display::SCREEN_HEIGHT];

	let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
	let core = pac::CorePeripherals::take().unwrap();
	
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

	let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

	let _spi_sclk = pins.gpio18.into_mode::<hal::gpio::FunctionSpi>(); // sclk
    let _spi_mosi = pins.gpio19.into_mode::<hal::gpio::FunctionSpi>(); // mosi
	let _spi_data_command = pins.gpio16.into_push_pull_output();
	let _spi_chip_select = pins.gpio17.into_push_pull_output();
	let spi = hal::Spi::<_, _, 8>::new(pac.SPI0).init(&mut pac.RESETS, clocks.peripheral_clock.freq(), 62_500_000u32.Hz(), &embedded_hal::spi::MODE_2);

	let mut led_pin = pins.gpio25.into_push_pull_output();
	let mut r_pin = pins.gpio6.into_push_pull_output();
	let mut g_pin = pins.gpio7.into_push_pull_output();
	let mut b_pin = pins.gpio8.into_push_pull_output();
	let mut reset_pin = pins.gpio20.into_push_pull_output();

	r_pin.set_high().unwrap();
	g_pin.set_high().unwrap();
	b_pin.set_high().unwrap();

	// ferris image
	let ferris_width = 86usize;
	const ferris_buffer: &[u8] = include_bytes!("ferris.raw");

	let mut out_img: [u8; ferris_buffer.len() / 2] = [0u8; ferris_buffer.len() / 2];

	for i in 0..(ferris_buffer.len() / 2)
	{
		out_img[i] = pixel_ops::make_pixel2(ferris_buffer[(i * 2) + 0], ferris_buffer[(i * 2) + 1]);
	}

	// display init
	let mut display_interface = SPIInterfaceNoCS::new(spi, _spi_data_command);

	display::Init(&mut display_interface, &mut delay,& mut reset_pin);

	let cube = model::Cube2D::New();
	let triangle = model::Triangle::New();

	let mut y_offset: u16 = 0u16;
	let mut degrees: f32 = 0f32;
	let mut scale: f32 = 0f32;
	let mut counter: f32 = 0f32;

	#[allow(clippy::empty_loop)]
	loop
	{
		// draw framebuffer to screen
		display::set_pixels(&mut display_interface, 0, 0, 239, 319, framebuffer.iter().map(|&v| pixel_ops::convert_pixel(v)));
		led_pin.set_high().unwrap();

		// prepare the framebuffer for a new frame
		framebuffer::clear(&mut framebuffer);

		// draw to framebuffer
		let rot_triangle = triangle.Scale(7.0f32).Rotate(degrees).Translate(Vec2D{x: 100f32, y: 200f32});
		rot_triangle.Draw(&mut framebuffer, colors::WHITE as u8);

		// let rot_cube = cube.Scale(scale).Rotate(degrees).Translate(Vec2D{x: 100f32, y: 100f32});
		// rot_cube.Draw(&mut framebuffer);

		draw_line(&mut framebuffer, &Vec2D{x: 0.0f32, y: 0.0f32}, &rot_triangle.points[0], colors::RED as u8);
		draw_line(&mut framebuffer, &Vec2D{x: 0.0f32, y: 0.0f32}, &rot_triangle.points[1], colors::GREEN as u8);
		draw_line(&mut framebuffer, &Vec2D{x: 0.0f32, y: 0.0f32}, &rot_triangle.points[2], colors::BLUE as u8);

		print_text(&mut framebuffer, b"ANCIENT", Vec2D{x: 30.0f32, y: 30.0f32}, scale * 30.0f32, colors::WHITE as u8);
		print_text(&mut framebuffer, b"RUNES", Vec2D{x: 30.0f32, y: scale * 30.0f32 + 30.0f32}, scale * 30.0f32, colors::WHITE as u8);
		// print_text(&mut framebuffer, b"QRSTUVWXYZ", Vec2D{x: 30.0f32, y: scale * 30.0f32 * 2.0f32 + 30.0f32}, scale * 30.0f32, colors::WHITE as u8);
		// print_text(&mut framebuffer, b"0123456789", Vec2D{x: 30.0f32, y: scale * 30.0f32 * 3.0f32 + 30.0f32}, scale * 30.0f32, colors::WHITE as u8);


		// misc.
		if y_offset >= 320
		{
			y_offset = 0;
		}

		if degrees >= 360f32
		{
			degrees = 0f32;
		}

		//counter += 1.0f32;
		degrees += 0.025f32;
		y_offset += 4;

		scale = libm::sin(degrees as f64 * 3.0f64) as f32 * 1.5f32;

		scale *= scale;
		scale += 1.0f32;
		scale /= 2.0f32;

		scale = 0.65f32;

		// if scale < 0.0f32
		// {
		// 	scale *= -1.0f32;
		// }
	}
	
	#[allow(clippy::empty_loop)]
	loop 
	{
        led_pin.set_high().unwrap();
        delay.delay_ms(250);
        led_pin.set_low().unwrap();
        delay.delay_ms(1500);
    }
}

// End of file
