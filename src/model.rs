use crate::vector::Vec2D;
use crate::framebuffer::{self, draw_line};
use crate::pixel_ops::colors;

pub struct Cube2D
{
	pub Points: [Vec2D; 4],
	pub Color: u8
}

pub struct Triangle
{
	pub points: [Vec2D; 3]
}


impl Cube2D
{
	pub fn New() -> Cube2D
	{	
		Cube2D{
			Points: [
				Vec2D{x: 25f32, y: 25f32},
				Vec2D{x: -25f32, y: 25f32},
				Vec2D{x: -25f32, y: -25f32},
				Vec2D{x: 25f32, y: -25f32}
			],
			Color: 255u8
		}
	}

	pub fn Rotate(&self, degrees: f32) -> Cube2D
	{
		Cube2D{
			Points: [
				self.Points[0].rotate(degrees),
				self.Points[1].rotate(degrees),
				self.Points[2].rotate(degrees),
				self.Points[3].rotate(degrees)
			],
			Color: 255u8
		}
	}

	pub fn Translate(&self, t: Vec2D) -> Cube2D
	{
		Cube2D{
			Points: [
				Vec2D{x: self.Points[0].x + t.x, y: self.Points[0].y + t.y},
				Vec2D{x: self.Points[1].x + t.x, y: self.Points[1].y + t.y},
				Vec2D{x: self.Points[2].x + t.x, y: self.Points[2].y + t.y},
				Vec2D{x: self.Points[3].x + t.x, y: self.Points[3].y + t.y}
			],
			Color: 255u8
		}
	}

	pub fn Scale(&self, factor: f32) -> Cube2D
	{
		Cube2D{
			Points: [
				Vec2D{x: self.Points[0].x * factor, y: self.Points[0].y * factor},
				Vec2D{x: self.Points[1].x * factor, y: self.Points[1].y * factor},
				Vec2D{x: self.Points[2].x * factor, y: self.Points[2].y * factor},
				Vec2D{x: self.Points[3].x * factor, y: self.Points[3].y * factor}
			],
			Color: 255u8
		}
	}

	pub fn Draw(&self, framebuffer: &mut [u8])
	{
		framebuffer::draw_line_vertex(framebuffer, &self.Points[0], &self.Points[1], 38u8); // g
		framebuffer::draw_line_vertex(framebuffer, &self.Points[1], &self.Points[2], 86u8); // b
		framebuffer::draw_line_vertex(framebuffer, &self.Points[2], &self.Points[3], 133u8); // r
		framebuffer::draw_line_vertex(framebuffer, &self.Points[3], &self.Points[0], 255u8); // w
	}

	pub fn Draw_Fill(&self, framebuffer: &mut [u8])
	{
		
	}
}

impl Triangle
{
	pub fn New() -> Triangle
	{
		Triangle {
			points: [
				Vec2D{x: 10.0f32, y: -5.0f32},
				Vec2D{x: 0.0f32, y: 5.0f32},
				Vec2D{x: -10.0f32, y: -5.0f32}
			]
		}
	}

	// get the triangle points, sort them, translate them to the center of the coordinate system
	pub fn Draw(&self, framebuffer: &mut [u8], color: u8)
	{
		let mut points = self.Get_Sorted_Verts();

		// draw_line(framebuffer, &points[0], &points[1], colors::WHITE as u8);
		// draw_line(framebuffer, &points[1], &points[2], colors::RED as u8);
		// draw_line(framebuffer, &points[0], &points[2], colors::GREEN as u8);

		self.Rasterize(framebuffer, &points[1], &points[0], &points[2], color);
		self.Rasterize(framebuffer, &points[1], &points[2], &points[0], color);
	}

	// tip is a common point between the two lines 
	fn Rasterize(&self, framebuffer: &mut [u8], origin: &Vec2D, tip: &Vec2D, end: &Vec2D, color: u8)
	{
		let mut points: [Vec2D; 2] = [
			origin - tip,
			end - tip
		];

		// slope will be x/y since we iterate & draw by y
		let slope1 = points[0].x / points[0].y;
		let slope2 = points[1].x / points[1].y;

		if points[0].y > 0.0f32
		{
			for y in 0..(points[0].y as i32 + 1)
			{
				framebuffer::draw_line_horizontal(
					framebuffer,
					(y as f32 + tip.y + 0.5f32), 
					(y as f32 * slope1) + tip.x,
					(y as f32 * slope2) + tip.x,
					color);
			}
		}
		else 
		{
			for y in (points[0].y as i32 + 1)..0
			{
				framebuffer::draw_line_horizontal(
					framebuffer,
					(y as f32 + tip.y - 0.5f32), 
					(y as f32 * slope1) + tip.x,
					(y as f32 * slope2) + tip.x,
					color);
			}
		}
	}

	fn Get_Sorted_Verts(&self) -> [Vec2D; 3]
	{
		let mut points: [Vec2D; 3] = [
			self.points[0],
			self.points[1],
			self.points[2]
		];

		Compare_Swap_Verts(&mut points, 0, 1);
		Compare_Swap_Verts(&mut points, 1, 2);
		Compare_Swap_Verts(&mut points, 0, 1);

		points
	}

	pub fn Rotate(&self, degrees: f32) -> Triangle
	{
		Triangle{
			points: [
				self.points[0].rotate(degrees),
				self.points[1].rotate(degrees),
				self.points[2].rotate(degrees)
			]
		}
	}

	pub fn Translate(&self, t: Vec2D) -> Triangle
	{
		Triangle{
			points: [
				Vec2D{x: self.points[0].x + t.x, y: self.points[0].y + t.y},
				Vec2D{x: self.points[1].x + t.x, y: self.points[1].y + t.y},
				Vec2D{x: self.points[2].x + t.x, y: self.points[2].y + t.y}
			]
		}
	}

	pub fn Scale(&self, factor: f32) -> Triangle
	{
		Triangle{
			points: [
				Vec2D{x: self.points[0].x * factor, y: self.points[0].y * factor},
				Vec2D{x: self.points[1].x * factor, y: self.points[1].y * factor},
				Vec2D{x: self.points[2].x * factor, y: self.points[2].y * factor}
			]
		}
	}
}

// sort by y, v1 should have higher y value
fn Compare_Swap_Verts(points: &mut [Vec2D; 3], v1: usize, v2: usize)
{
	if points[v1].y < points[v2].y
	{
		let temp = points[v1].clone();
		points[v1] = points[v2].clone();
		points[v2] = temp;
	}
}