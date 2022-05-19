use crate::{vector::vec2f, framebuffer::{draw_line, draw_pixel}};
use crate::pixel_ops::colors;

// Letters are represented as lines, pairs of vec2fs

#[derive(Clone, Copy)]
pub struct Letter
{
	pub points: [vec2f; 10],
	pub line_count: usize
}

impl Letter
{
	pub fn scale(&self, factor: f32) -> Letter
	{
		let mut out_letter = self.clone();

		for i in 0..10
		{
			out_letter.points[i] = out_letter.points[i].scale(factor);
		}

		out_letter
	}

	pub fn translate(&self, trans_vec: vec2f) -> Letter
	{
		let mut out_letter = self.clone();

		for i in 0..10
		{
			out_letter.points[i] = out_letter.points[i].translate(trans_vec);
		}

		out_letter
	}

	pub fn flip_x(&self) -> Letter
	{
		let mut out_letter = self.clone();

		for i in 0..10
		{
			out_letter.points[i].x = out_letter.points[i].x * -1.0f32;
		}

		out_letter
	}

	pub fn flip_y(&self) -> Letter
	{
		let mut out_letter = self.clone();

		for i in 0..10
		{
			out_letter.points[i].y = out_letter.points[i].y * -1.0f32;
		}

		out_letter
	}
}

pub fn print_text(framebuffer: &mut [u8], str_in: &[u8], trans_vec: vec2f, scale: f32, color: u8)
{
	let mut counter: f32 = 0.0f32;
	for char in str_in
	{
		let current_letter = get_letter_for_char(char).flip_y().scale(scale * 0.8).translate(&trans_vec + &vec2f { x: counter * scale, y: 0.0f32 });

		for i in 0..current_letter.line_count
		{
			draw_line(framebuffer, &current_letter.points[i * 2 + 0], &current_letter.points[i * 2 + 1], color);
		}

		counter += 1.0f32;
	}
}

fn get_letter_for_char(char: &u8) -> Letter
{
	match char{
		0u8..=47u8 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},
					vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 0},
		58..=64 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},
					vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 0},
		91..=127 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},
					vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 0},
		// 0
		48 => Letter{ points: [vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.5f32},
					vec2f{x: 0.5f32, y: 0.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 4},
		// 1
		49 => Letter{ points: [vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 1},
		// 2
		50 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.75f32},	vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.75f32},		vec2f{x: 0.0f32, y: 0.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 3},
		// 3
		51 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.75f32},	vec2f{x: 1.0f32, y: 0.75f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32}], line_count: 4},
		// 4
		52 => Letter{ points: [vec2f{x: 0.25f32, y: 1.0f32},vec2f{x: 0.0f32, y: 0.5f32},	vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32}], line_count: 3},
		// 5
		53 => Letter{ points: [vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.5f32},
					vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32}], line_count: 3},
		// 6
		54 => Letter{ points: [vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},
					vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32}], line_count: 3},
		// 7
		54 => Letter{ points: [vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},
					vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32}], line_count: 2},
		// 8
		55 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 4},
		// 9
		56 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},
					vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 3},



		// A
		65 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// B
		66 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.75f32},		vec2f{x: 1.0f32, y: 0.75f32},
					vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 5},
		// C
		67 => Letter{ points: [vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.75f32},
					vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 2},
		// D
		68 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.5f32},
					vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// E
		69 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 4},
		// F
		70 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// G
		71 => Letter{ points: [vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},
					vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// H
		72 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// I
		73 => Letter{ points: [vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 1},
		// J
		74 => Letter{ points: [vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.5f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 1.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 2},
		// K
		75 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// L
		76 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 2},
		// M
		77 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 0.5f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 4},
		// N
		78 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},
					vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// O
		79 => Letter{ points: [vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.5f32},
					vec2f{x: 0.5f32, y: 0.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 4},
		// P
		80 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.75f32, y: 0.75f32},		vec2f{x: 0.75f32, y: 0.75f32},
					vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// Q
		81 => Letter{ points: [vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.5f32},
					vec2f{x: 0.5f32, y: 0.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.5f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.0f32},], line_count: 5},
		// R
		82 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.75f32, y: 0.75f32},		vec2f{x: 0.75f32, y: 0.75f32},
					vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.5f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 4},
		// S
		83 => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.25f32},	vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.75f32},		vec2f{x: 0.0f32, y: 0.75f32},
					vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 3},
		// T
		84 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.5f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.75f32},
					vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.5f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 2},
		// U
		85 => Letter{ points: [vec2f{x: 0.0f32, y: 0.25f32},vec2f{x: 0.0f32, y: 1.0f32},	vec2f{x: 0.0f32, y: 0.25f32},vec2f{x: 0.25f32, y: 0.0f32},		vec2f{x: 0.25f32, y: 0.0f32},
					vec2f{x: 0.75f32, y: 0.0f32},		vec2f{x: 0.75f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 5},
		// V
		86 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.5f32, y: 0.0f32},		vec2f{x: 0.5f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.25f32, y: 0.0f32},
					vec2f{x: 0.75f32, y: 0.0f32},		vec2f{x: 0.75f32, y: 0.0f32},vec2f{x: 1.0f32, y: 0.25f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 2},
		// W
		87 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 0.5f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 4},
		// X
		88 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 2},
		// Y
		89 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 0.5f32, y: 0.5f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.5f32, y: 0.5f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 2},
		// Z
		90 => Letter{ points: [vec2f{x: 0.0f32, y: 1.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 0.0f32, y: 0.0f32},
					vec2f{x: 1.0f32, y: 0.0f32},		vec2f{x: 1.0f32, y: 0.0f32},vec2f{x: 1.0f32, y: 1.0f32},		vec2f{x: 1.0f32, y: 0.25f32},vec2f{x: 1.0f32, y: 1.0f32},], line_count: 3},
		
			
			
							
		_ => Letter{ points: [vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},
		vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},vec2f{x: 0.0f32, y: 0.0f32},], line_count: 0}
	}
}