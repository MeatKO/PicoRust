use crate::vector::{vec2f, vec3f};
use crate::framebuffer::{self, draw_line};
use crate::pixel_ops::colors;
use crate::matrix::mat4x4;
use crate::display;

pub struct triangle
{
	pub points: [vec2f; 3]
}

pub struct triangle3d
{
	pub points: [vec3f; 3]
}

pub struct cube
{
	pub points: [vec3f; 8]
}

impl cube
{
	pub fn new() -> cube
	{	
		cube{
			points: [
				vec3f{x: 1.0f32, y: 1.0f32, z: 1.0f32},
				vec3f{x: 1.0f32, y: -1.0f32, z: 1.0f32},
				vec3f{x: -1.0f32, y: -1.0f32, z: 1.0f32},
				vec3f{x: -1.0f32, y: 1.0f32, z: 1.0f32},
				vec3f{x: 1.0f32, y: 1.0f32, z: -1.0f32},
				vec3f{x: 1.0f32, y: -1.0f32, z: -1.0f32},
				vec3f{x: -1.0f32, y: -1.0f32, z: -1.0f32},
				vec3f{x: -1.0f32, y: 1.0f32, z: -1.0f32}
			]
		}
	}

	pub fn scale(&self, factor: f32) -> cube
	{	
		cube{
			points: [
				self.points[0].scale(factor),
				self.points[1].scale(factor),
				self.points[2].scale(factor),
				self.points[3].scale(factor),
				self.points[4].scale(factor),
				self.points[5].scale(factor),
				self.points[6].scale(factor),
				self.points[7].scale(factor),
			]
		}
	}

	pub fn translate(&self, trans_vec: vec3f) -> cube
	{	
		cube{
			points: [
				self.points[0] + &trans_vec,
				self.points[1] + &trans_vec,
				self.points[2] + &trans_vec,
				self.points[3] + &trans_vec,
				self.points[4] + &trans_vec,
				self.points[5] + &trans_vec,
				self.points[6] + &trans_vec,
				self.points[7] + &trans_vec,
			]
		}
	}

	pub fn project(&self, mvp_matrix: &mat4x4) -> [vec2f; 8]
	{
		let mut out_points = [vec2f::new(); 8];

		for i in 0..8
		{
			let vec_projected = *mvp_matrix * &self.points[i];

			out_points[i].x = f32::min(display::SCREEN_WIDTH as f32 - 1.0f32, (vec_projected.x + 1.0f32) * 0.5f32 * display::SCREEN_WIDTH as f32);
			out_points[i].y = f32::min(display::SCREEN_HEIGHT as f32 - 1.0f32, (1.0f32 - ((vec_projected.y + 1.0f32) * 0.5f32)) * display::SCREEN_HEIGHT as f32);

			out_points[i].x -= 30.0f32;
			out_points[i].y += 30.0f32;
		}

		out_points
	}

	pub fn rasterize_wireframe(&self, framebuffer: &mut [u8], mvp_matrix: &mat4x4, color: u8)
	{
		let projected_points = self.project(mvp_matrix);

		draw_line(framebuffer, &projected_points[0], &projected_points[1], color);
		draw_line(framebuffer, &projected_points[1], &projected_points[2], color);
		draw_line(framebuffer, &projected_points[2], &projected_points[3], color);
		draw_line(framebuffer, &projected_points[3], &projected_points[0], color);

		draw_line(framebuffer, &projected_points[4], &projected_points[5], color);
		draw_line(framebuffer, &projected_points[5], &projected_points[6], color);
		draw_line(framebuffer, &projected_points[6], &projected_points[7], color);
		draw_line(framebuffer, &projected_points[7], &projected_points[4], color);

		draw_line(framebuffer, &projected_points[0], &projected_points[4], color);
		draw_line(framebuffer, &projected_points[5], &projected_points[1], color);
		draw_line(framebuffer, &projected_points[6], &projected_points[2], color);
		draw_line(framebuffer, &projected_points[7], &projected_points[3], color);
	}

	pub fn rasterize(&self, framebuffer: &mut [u8], mvp_matrix: &mat4x4, color: u8)
	{
		let projected_points = self.project(mvp_matrix);

		// right
		triangle::from(&projected_points[0], &projected_points[1], &projected_points[5]).draw(framebuffer, color);
		triangle::from(&projected_points[0], &projected_points[5], &projected_points[4]).draw(framebuffer, color);

		// back
		triangle::from(&projected_points[0], &projected_points[1], &projected_points[2]).draw(framebuffer, color);
		triangle::from(&projected_points[0], &projected_points[3], &projected_points[2]).draw(framebuffer, color);

		// left
		triangle::from(&projected_points[2], &projected_points[3], &projected_points[7]).draw(framebuffer, color);
		triangle::from(&projected_points[2], &projected_points[6], &projected_points[7]).draw(framebuffer, color);

		// front
		triangle::from(&projected_points[6], &projected_points[5], &projected_points[4]).draw(framebuffer, color);
		triangle::from(&projected_points[6], &projected_points[4], &projected_points[7]).draw(framebuffer, color);

		// top
		triangle::from(&projected_points[1], &projected_points[3], &projected_points[4]).draw(framebuffer, color);
		triangle::from(&projected_points[3], &projected_points[4], &projected_points[7]).draw(framebuffer, color);

		// top
		triangle::from(&projected_points[1], &projected_points[2], &projected_points[5]).draw(framebuffer, color);
		triangle::from(&projected_points[2], &projected_points[5], &projected_points[6]).draw(framebuffer, color);
	}
}

impl triangle
{
	pub fn new() -> triangle
	{
		triangle {
			points: [
				vec2f{x: 10.0f32, y: -5.0f32},
				vec2f{x: 0.0f32, y: 5.0f32},
				vec2f{x: -10.0f32, y: -5.0f32}
			]
		}
	}

	pub fn from(p1: &vec2f, p2: &vec2f, p3: &vec2f) -> triangle
	{
		triangle {
			points: [
				vec2f{x: p1.x, y: p1.y},
				vec2f{x: p2.x, y: p2.y},
				vec2f{x: p3.x, y: p3.y},
			]
		}
	}

	// get the triangle points, sort them, translate them to the center of the coordinate system
	pub fn draw(&self, framebuffer: &mut [u8], color: u8)
	{
		let mut points = self.get_sorted_verts();

		self.rasterize(framebuffer, &points[1], &points[0], &points[2], color);
		self.rasterize(framebuffer, &points[1], &points[2], &points[0], color);
	}

	// tip is a common point between the two lines 
	fn rasterize(&self, framebuffer: &mut [u8], origin: &vec2f, tip: &vec2f, end: &vec2f, color: u8)
	{
		let mut points: [vec2f; 2] = [
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

	fn get_sorted_verts(&self) -> [vec2f; 3]
	{
		let mut points: [vec2f; 3] = [
			self.points[0],
			self.points[1],
			self.points[2]
		];

		compare_swap_verts(&mut points, 0, 1);
		compare_swap_verts(&mut points, 1, 2);
		compare_swap_verts(&mut points, 0, 1);

		points
	}

	pub fn rotate(&self, degrees: f32) -> triangle
	{
		triangle{
			points: [
				self.points[0].rotate(degrees),
				self.points[1].rotate(degrees),
				self.points[2].rotate(degrees)
			]
		}
	}

	pub fn translate(&self, t: vec2f) -> triangle
	{
		triangle{
			points: [
				vec2f{x: self.points[0].x + t.x, y: self.points[0].y + t.y},
				vec2f{x: self.points[1].x + t.x, y: self.points[1].y + t.y},
				vec2f{x: self.points[2].x + t.x, y: self.points[2].y + t.y}
			]
		}
	}

	pub fn scale(&self, factor: f32) -> triangle
	{
		triangle{
			points: [
				vec2f{x: self.points[0].x * factor, y: self.points[0].y * factor},
				vec2f{x: self.points[1].x * factor, y: self.points[1].y * factor},
				vec2f{x: self.points[2].x * factor, y: self.points[2].y * factor}
			]
		}
	}
}

// sort by y, v1 should have higher y value
fn compare_swap_verts(points: &mut [vec2f; 3], v1: usize, v2: usize)
{
	if points[v1].y < points[v2].y
	{
		let temp = points[v1].clone();
		points[v1] = points[v2].clone();
		points[v2] = temp;
	}
}