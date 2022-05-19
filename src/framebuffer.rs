use crate::vector::vec2f;
use crate::pixel_ops::colors;

pub fn draw_iter<T: Iterator<Item = u8>>(framebuffer: &mut [u8], x_begin: usize, y_begin: usize, x_end: usize, y_end: usize, mut pixels: T)
{
	//let mut iter = pixels.into_iter();

	for x in x_begin..x_end 
	{
		for y in y_begin..y_end
		{
			match pixels.next()
			{
				None => { return },
				Some(pix) => { framebuffer[y * 240 + x] = pix; }
			}
		}
	}
}

pub fn draw_array(framebuffer: &mut [u8], x_begin: usize, y_begin: usize, pixels: &[u8], width: usize)
{
	let height = pixels.len() / width;

	for x in 0..width 
	{
		for y in 0..height
		{
			let current_color: u8 = pixels[(y * width) + x];
			draw_pixel(framebuffer, x + x_begin, y + y_begin, current_color);
		}
	}
}

pub fn draw_pixel(framebuffer: &mut [u8], x: usize, y: usize, color: u8)
{
	if (x > 0 && x < 240) &&  (y > 0 && y < 320)
	{
		framebuffer[y * 240 + x] = color;
	}
}

pub fn clear(framebuffer: &mut [u8])
{
	for x in 0..(240 * 320)
	{
		framebuffer[x] = 0u8;
	}
}

pub fn draw_line_vertex(framebuffer: &mut [u8], begin: &vec2f, end: &vec2f, color: u8)
{
	draw_line(framebuffer, &vec2f { x: begin.x, y: begin.y }, &vec2f { x: end.x, y: end.y }, color);
}

pub fn draw_vertex(framebuffer: &mut [u8], vert: &vec2f, pixel: u8)
{
	draw_pixel(framebuffer, vert.x as usize, vert.y as usize, pixel);
}

pub fn draw_line(framebuffer: &mut [u8], begin: &vec2f, end: &vec2f, color: u8)
{
	let line_vec: vec2f = end - begin;
	let slope = line_vec.y / line_vec.x;

	if slope <= 1.0f32 && slope >= -1.0f32
	{
		draw_line_x(framebuffer, begin, &line_vec, slope, color);
	}
	else 
	{
		draw_line_y(framebuffer, begin, &line_vec, slope, color);
	}
}

pub fn draw_line_horizontal(framebuffer: &mut [u8], y: f32, begin_x: f32, end_x: f32, color: u8)
{
	if begin_x < end_x 
	{
		for x in (begin_x as usize)..(end_x as usize)
		{
			draw_pixel(framebuffer, x, y as usize, color)
		}
	}
	else
	{
		for x in (end_x as usize)..(begin_x as usize)
		{
			draw_pixel(framebuffer, x, y as usize, color)
		}
	}
}

fn draw_line_x(framebuffer: &mut [u8], begin: &vec2f, line_vec: &vec2f, slope: f32, color: u8)
{
	if line_vec.x > 0.0f32
	{
		
		for x in 0..(line_vec.x as i32)
		{
			draw_pixel(framebuffer, (x as f32 + begin.x) as usize, ((x as f32 * slope) + begin.y) as usize, color);
		}
	}
	else 
	{
		for x in (line_vec.x as i32)..0
		{
			draw_pixel(framebuffer, (x as f32 + begin.x) as usize, ((x as f32 * slope) + begin.y) as usize, color);
		}
	}
}

fn draw_line_y(framebuffer: &mut [u8], begin: &vec2f, line_vec: &vec2f, slope: f32, color: u8)
{
	if line_vec.y > 0.0f32
	{
		for y in 0..(line_vec.y as i32)
		{
			draw_pixel(framebuffer, ((y as f32 / slope) + begin.x) as usize, (y + begin.y as i32) as usize, color);
		}
	}
	else 
	{
		for y in (line_vec.y as i32)..0
		{
			draw_pixel(framebuffer, ((y as f32 / slope) + begin.x) as usize, (y + begin.y as i32) as usize, color);
		}
	}
}