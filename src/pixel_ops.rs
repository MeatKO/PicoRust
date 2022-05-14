
#[repr(u8)]
pub enum colors {
    WHITE = 255u8,
	RED = 133u8,
	GREEN = 86u8,
	BLUE = 38u8,
	BLACK = 0u8
}

// convert RGB 888 to RGB 332
pub fn make_pixel(r: u8, g: u8, b: u8) -> u8
{
	let mut out_pix: u8 = 0u8;

	out_pix |= (r / (255 / 7)) << 5;
	out_pix |= (g / (255 / 7)) << 2;
	out_pix |= b / (255 / 3);

	out_pix
}

// convert RGB 888 to RGB 565
pub fn make_pixel3(r: u8, g: u8, b: u8) -> u16
{
	let mut out_pix: u16 = 0u16;

	out_pix |= (r as u16 / (255 / 31)) << 11;
	out_pix |= (g as u16 / (255 / 63)) << 5;
	out_pix |= b as u16 / (255 / 31);

	out_pix
}

// convert u16 RGB 565 to u8 RGB 332
pub fn make_pixel2(b1: u8, b2: u8) -> u8
{
	let mut out_pix: u8 = 0u8;

	let base_red: u8 = (b1 >> 3) / (31 / 7);
	let base_green: u8 = (((b1 & 0b0000_0111) << 3) & ((b2 & 0b1110_0000) >> 2)) / (63/7);
	let base_blue: u8 = (b2 & 0b0001_1111) / (31/3);

	out_pix |= (base_red << 5) & 0b1110_0000;
	out_pix |= (base_green << 2) & 0b0001_1100;
	out_pix |= base_blue & 0b0000_0011;

	out_pix
}

// convert u8 RGB 332 to u16 RGB 565
pub fn convert_pixel(pix: u8) -> u16
{
	let mut out_pix: u16 = 0u16;

	let base_red: u16 	= (((pix as u16) & 0b1110_0000) >> 5) * 31 / 7;
	let base_green: u16  = (((pix as u16)  & 0b0001_1100) >> 2) * 63 / 7;
	let base_blue: u16   = ((pix as u16)  & 0b0000_0011) * 31 / 3;

	out_pix |= base_red << 11;
	out_pix |= base_green << 5;
	out_pix |= base_blue;

	out_pix
}

// in : RGB 332
// out : RGB 332
pub fn red_pixel_kernel(pix: u8) -> u8
{
	pix & 0b1110_0000
}
pub fn green_pixel_kernel(pix: u8) -> u8
{
	pix & 0b0001_1100
}
pub fn blue_pixel_kernel(pix: u8) -> u8
{
	pix & 0b0000_0011
}

pub fn red_grayscale_pixel_kernel(pix: u8) -> u8
{
	let scale: u8 = ((pix & 0b1110_0000) >> 5) * (255 / 7);
	make_pixel(scale, scale, scale)
}
pub fn green_grayscale_pixel_kernel(pix: u8) -> u8
{
	let scale: u8 = ((pix & 0b0001_1100) >> 2) * (255 / 7);
	make_pixel(scale, scale, scale)
}
pub fn blue_grayscale_pixel_kernel(pix: u8) -> u8
{
	let scale: u8 = (pix & 0b0000_0011) * (255 / 3);
	make_pixel(scale, scale, scale)
}