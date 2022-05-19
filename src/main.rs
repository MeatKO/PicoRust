#![no_std]
#![no_main]

mod pixel_ops;
mod framebuffer;
mod display;
mod model;
mod vector;
mod text;
mod matrix;

use core::u8;
use cortex_m_rt::entry;
use panic_halt as _;
use rp2040_hal as hal;
use hal::clocks::Clock;
use hal::pac;
use embedded_hal::digital::v2::{OutputPin,InputPin};
use embedded_time::rate::*;
use embedded_time::fixed_point::FixedPoint;
use display_interface_spi::SPIInterfaceNoCS;
use crate::vector::vec2f;
use crate::pixel_ops::colors;
use crate::text::print_text;
use crate::matrix::mat4x4;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! 
{
	let mut framebuffer = [0u8; display::SCREEN_WIDTH * display::SCREEN_HEIGHT]; // pixel color information

	// init peripherals
	let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
	let core = pac::CorePeripherals::take().unwrap();
	
	// init clocks
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

	// init output pins
	let mut led_pin = pins.gpio25.into_push_pull_output();
	let mut r_pin = pins.gpio6.into_push_pull_output();
	let mut g_pin = pins.gpio7.into_push_pull_output();
	let mut b_pin = pins.gpio8.into_push_pull_output();
	let mut reset_pin = pins.gpio20.into_push_pull_output();

	// init input pins
	let input_a = pins.gpio12.into_pull_up_input();
	let input_b = pins.gpio13.into_pull_up_input();
	let input_x = pins.gpio14.into_pull_up_input();
	let input_y = pins.gpio15.into_pull_up_input();

	// turn off LEDs (set_high actually turns them off)
	r_pin.set_high().unwrap();
	g_pin.set_high().unwrap();
	b_pin.set_high().unwrap();

	// display init
	let mut display_interface = SPIInterfaceNoCS::new(spi, _spi_data_command);
	display::Init(&mut display_interface, &mut delay,& mut reset_pin);

	// mesh & matrix init
	let cube = model::cube::new();

	let mut degrees_x: f32 = 0f32;
	let mut degrees_y: f32 = 0f32;
	let mut degrees_z: f32 = 0f32;

	// 3D projection matrix vars
	let fov = 90.0f32; // Field of view
	let near_plane_distance = 0.1f32; // min render distance
	let far_plane_distance = 100.0f32; // max render distance

	let projection_mat: mat4x4 = mat4x4::new_projection(fov, near_plane_distance, far_plane_distance);

	// 3D model translation matrix
	let mut world_to_camera_mat: mat4x4 = mat4x4::new();
	world_to_camera_mat.data[3][0] = 0.0f32; // X translation
	world_to_camera_mat.data[3][1] = 0.0f32; // Y translation
	world_to_camera_mat.data[3][2] = -40.0f32; // Z translation

	let mut toggled: bool = false;
	let mut render_wireframe: bool = false;

	#[allow(clippy::empty_loop)]
	loop
	{
		// draw framebuffer to screen
		display::set_pixels(&mut display_interface, 0, 0, 239, 319, framebuffer.iter().map(|&v| pixel_ops::convert_pixel(v)));
		led_pin.set_high().unwrap();

		// prepare the framebuffer for a new frame
		framebuffer::clear(&mut framebuffer);

		// text doesn't influence the z buffer and should be rendered last
		print_text(&mut framebuffer, b"3D", vec2f{x: 30.0f32, y: 30.0f32}, 25.0f32, colors::WHITE as u8);
		print_text(&mut framebuffer, b"CUBE", vec2f{x: 30.0f32, y: 25.0f32 + 30.0f32}, 25.0f32, colors::WHITE as u8);

		let rot_matrix_x = mat4x4::new_rot_x(degrees_x);
		let rot_matrix_y = mat4x4::new_rot_y(degrees_y);
		let rot_matrix_z = mat4x4::new_rot_z(degrees_z);

		let mvp_matrix = projection_mat * &(world_to_camera_mat * &((rot_matrix_z * &rot_matrix_y) * &rot_matrix_x));

		let new_cube = cube.scale(10.0f32);

		if render_wireframe
		{
			new_cube.rasterize_wireframe(&mut framebuffer, &mvp_matrix, colors::WHITE as u8);
		}
		else 
		{
			new_cube.rasterize(&mut framebuffer, &mvp_matrix, colors::WHITE as u8);
		}

		// take user input
		if input_a.is_low().unwrap() 
		{
			degrees_x += 0.05f32;
		}
		if input_x.is_low().unwrap() 
		{
			degrees_y += 0.05f32;
		}
		if input_y.is_low().unwrap() 
		{
			degrees_z += 0.05f32;
		}

		if input_b.is_low().unwrap() 
		{
			toggled = true;
		}
		else
		{
			if toggled
			{
				render_wireframe = !render_wireframe;
			}

			toggled = false;
		}
	}
}