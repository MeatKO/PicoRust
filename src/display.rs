
use embedded_graphics::pixelcolor::raw::RawU16;
use display_interface::{WriteOnlyDataCommand, DataFormat::U8Iter, DataFormat::U16BEIter};
use embedded_hal::blocking::delay::DelayUs;
use core::iter::once;
use embedded_hal::digital::v2::OutputPin;
use crate::pixel_ops::convert_pixel;

pub const SCREEN_WIDTH: usize = 320usize;
pub const SCREEN_HEIGHT: usize = 240usize;

#[repr(u8)]
pub enum instr {
    NOP = 0x00,
    SWRESET = 0x01,
    RDDID = 0x04,
    RDDST = 0x09,
    SLPIN = 0x10,
    SLPOUT = 0x11,
    PTLON = 0x12,
    NORON = 0x13,
    INVOFF = 0x20,
    INVON = 0x21,
    DISPOFF = 0x28,
    DISPON = 0x29,
    CASET = 0x2A,
    RASET = 0x2B,
    RAMWR = 0x2C,
    RAMRD = 0x2E,
    PTLAR = 0x30,
    VSCRDER = 0x33,
    TEOFF = 0x34,
    TEON = 0x35,
    MADCTL = 0x36,
    VSCAD = 0x37,
    COLMOD = 0x3A,
    VCMOFSET = 0xC5,
}

pub fn  Init<DI: WriteOnlyDataCommand, RP: OutputPin> ( spi_interface: &mut DI, delay_source: &mut impl DelayUs<u32> , reset_pin: &mut RP)
{
	// hard reset
	reset_pin.set_high();
    delay_source.delay_us(10); // ensure the pin change will get registered
    reset_pin.set_low();
    delay_source.delay_us(10); // ensure the pin change will get registered
    reset_pin.set_high();
    delay_source.delay_us(10); // ensure the pin change will get registered

	// rest of the stuff
    write_command(spi_interface, instr::SWRESET); // reset display
    delay_source.delay_us(150_000);
    write_command(spi_interface, instr::SLPOUT); // turn off sleep
    delay_source.delay_us(10_000);
    write_command(spi_interface, instr::INVOFF); // turn off invert
    write_command(spi_interface, instr::VSCRDER); // vertical scroll definition
    write_data(spi_interface,&[0u8, 0u8, 0x14u8, 0u8, 0u8, 0u8]); // 0 TSA, 320 VSA, 0 BSA
    write_command(spi_interface, instr::MADCTL); // left -> right, bottom -> top RGB
    write_data(spi_interface,&[0b0000_0000]);
    write_command(spi_interface, instr::COLMOD); // 16bit 65k colors
    write_data(spi_interface,&[0b0101_0101]);
    write_command(spi_interface, instr::INVON); // hack?
    delay_source.delay_us(10_000);
    write_command(spi_interface, instr::NORON); // turn on display
    delay_source.delay_us(10_000);
    write_command(spi_interface, instr::DISPON); // turn on display
    delay_source.delay_us(10_000);
}

pub fn  write_command<DI: WriteOnlyDataCommand> ( spi_interface: &mut DI, command: instr)
{
	spi_interface.send_commands(U8Iter(&mut once(command as u8))).unwrap();
}

pub fn  write_data<DI: WriteOnlyDataCommand> ( spi_interface: &mut DI, data: &[u8])
{
	spi_interface.send_data(U8Iter(&mut data.iter().cloned())).unwrap();
}

pub fn set_address_window<DI: WriteOnlyDataCommand> (spi_interface: &mut DI, sx: u16, sy: u16, ex: u16, ey: u16)
{
	write_command(spi_interface, instr::CASET);
	write_data(spi_interface,&sx.to_be_bytes());
	write_data(spi_interface,&ex.to_be_bytes());
	write_command(spi_interface, instr::RASET);
	write_data(spi_interface,&sy.to_be_bytes());
	write_data(spi_interface,&ey.to_be_bytes());
}

pub fn set_pixels<DI: WriteOnlyDataCommand, T: IntoIterator<Item = u16>>( spi_interface: &mut DI,sx: u16,sy: u16,ex: u16,ey: u16, colors: T)
{
	set_address_window(spi_interface, sx, sy, ex, ey);
	write_command(spi_interface, instr::RAMWR);
	spi_interface.send_data(U16BEIter(&mut colors.into_iter()));
}

pub fn set_pixels2<DI: WriteOnlyDataCommand, T: IntoIterator<Item = u16>>( spi_interface: &mut DI, colors: T)
{
	spi_interface.send_data(U16BEIter(&mut colors.into_iter()));
}

pub fn set_pixel<DI: WriteOnlyDataCommand>( spi_interface: &mut DI,x: u16,y: u16, color: u16)
{
	set_address_window(spi_interface, x, y, x, y);
	write_command(spi_interface, instr::RAMWR);
    spi_interface.send_data(U16BEIter(&mut once(color)));
}

pub fn set_pixel2<DI: WriteOnlyDataCommand>( spi_interface: &mut DI, color: u16)
{
    spi_interface.send_data(U16BEIter(&mut once(color)));
}

pub fn draw_image<DI: WriteOnlyDataCommand>( spi_interface: &mut DI, framebuffer: &[u8], width: usize)
{
	let height: usize = ((framebuffer.len() as u32) / (width as u32)) as usize;
	let base_x = 0u16;
	let base_y = 0u16;

	let mut counter: u64 = 0u64;

	for i in 0..height
	{
		for j in 0..width
		{
			let current_x = base_x + (i as u16);
			let current_y = base_y +(j as u16);
			//set_pixel(spi_interface, current_x, current_y, convert_pixel(framebuffer[((i * j) + j) as usize]));
			set_pixel(spi_interface, current_x, current_y, convert_pixel(framebuffer[counter as usize]));
			counter += 1;
		}
	}
}

pub fn clear<DI: WriteOnlyDataCommand>( spi_interface: &mut DI, width: usize, height: usize)
{
	set_address_window(spi_interface, 0, 0, height as u16, width as u16);
	write_command(spi_interface, instr::RAMWR);	

	for i in 0..height
	{
		for j in 0..width
		{
			set_pixel2(spi_interface, 0u16);
		}
	}
}