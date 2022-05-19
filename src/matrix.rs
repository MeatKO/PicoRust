use core::ops::{self, Mul};
use crate::vector::{vec3f, vec4f};
use libm;

#[derive(Clone, Copy)]
pub struct mat4x4
{
	pub data: [[f32; 4]; 4] //this is two-dimensional array of 32bit floats
}

impl mat4x4
{
	// identity matrix
	pub fn new() -> mat4x4
	{
		let mut out_mat = mat4x4{
			data: [[0f32; 4]; 4]
		};

		out_mat.data[0][0] = 1.0f32;
		out_mat.data[1][1] = 1.0f32;
		out_mat.data[2][2] = 1.0f32;
		out_mat.data[3][3] = 1.0f32;

		out_mat
	}

	// some mathemagic I stole from the interwebz
	pub fn new_projection(fov: f32, near: f32, far: f32) -> mat4x4
	{
		let mut out_mat = mat4x4::new();

		let scale: f32 = 1.0f32 / libm::tanf(fov * 0.5f32 * 3.14159f32 / 180.0f32);
		out_mat.data[0][0] = scale;
		out_mat.data[1][1] = scale;
		out_mat.data[2][2] = -far / (far - near);
		out_mat.data[3][2] = -far * near / (far - near);
		out_mat.data[2][3] = -1.0f32;
		out_mat.data[3][3] = 0.0f32;

		out_mat
	}

	// some mathemagic I stole from the interwebz again
	pub fn new_rot_x(degrees: f32) -> mat4x4
	{
		let mut out_mat = mat4x4::new();

		out_mat.data[1][1] = libm::cos(degrees as f64) as f32;
		out_mat.data[2][2] = libm::cos(degrees as f64) as f32;
		out_mat.data[1][2] = libm::sin(degrees as f64) as f32;
		out_mat.data[2][1] = -libm::sin(degrees as f64) as f32;


		out_mat
	}

	pub fn new_rot_y(degrees: f32) -> mat4x4
	{
		let mut out_mat = mat4x4::new();

		out_mat.data[0][0] = libm::cos(degrees as f64) as f32;
		out_mat.data[2][2] = libm::cos(degrees as f64) as f32;
		out_mat.data[2][0] = libm::sin(degrees as f64) as f32;
		out_mat.data[0][2] = -libm::sin(degrees as f64) as f32;


		out_mat
	}

	pub fn new_rot_z(degrees: f32) -> mat4x4
	{
		let mut out_mat = mat4x4::new();

		out_mat.data[0][0] = libm::cos(degrees as f64) as f32;
		out_mat.data[1][1] = libm::cos(degrees as f64) as f32;
		out_mat.data[0][1] = libm::sin(degrees as f64) as f32;
		out_mat.data[1][0] = -libm::sin(degrees as f64) as f32;


		out_mat
	}
}

impl ops::Mul<&vec3f> for mat4x4 
{
    type Output = vec3f;

    fn mul(self, vec: &vec3f) -> vec3f 
	{
        let mut out_vec = vec3f{
			x: (self.data[0][0] * vec.x) + (self.data[1][0] * vec.y) + (self.data[2][0] * vec.z) + self.data[3][0],
			y: (self.data[0][1] * vec.x) + (self.data[1][1] * vec.y) + (self.data[2][1] * vec.z) + self.data[3][1],
			z: (self.data[0][2] * vec.x) + (self.data[1][2] * vec.y) + (self.data[2][2] * vec.z) + self.data[3][2],
			
		};

		let w = (self.data[0][3] * vec.x) + (self.data[1][3] * vec.y) + (self.data[2][3] * vec.z) + self.data[3][3];

		if w != 1.0f32
		{
			out_vec.x /= w;
			out_vec.y /= w;
			out_vec.z /= w;
		}

		out_vec
    }
}

impl ops::Mul<&mat4x4> for mat4x4 
{
    type Output = mat4x4;

    fn mul(self, mat_in: &mat4x4) -> mat4x4 
	{
        let mut out_mat: mat4x4 = mat4x4::new();
		out_mat.data = [[1.0f32; 4]; 4];

		out_mat.data[0][0] = (self.data[0][0] * mat_in.data[0][0]) + (self.data[1][0] * mat_in.data[0][1]) + (self.data[2][0] * mat_in.data[0][2]) + (self.data[3][0] * mat_in.data[0][3]);
		out_mat.data[0][1] = (self.data[0][1] * mat_in.data[0][0]) + (self.data[1][1] * mat_in.data[0][1]) + (self.data[2][1] * mat_in.data[0][2]) + (self.data[3][1] * mat_in.data[0][3]);
		out_mat.data[0][2] = (self.data[0][2] * mat_in.data[0][0]) + (self.data[1][2] * mat_in.data[0][1]) + (self.data[2][2] * mat_in.data[0][2]) + (self.data[3][2] * mat_in.data[0][3]);
		out_mat.data[0][3] = (self.data[0][3] * mat_in.data[0][0]) + (self.data[1][3] * mat_in.data[0][1]) + (self.data[2][3] * mat_in.data[0][2]) + (self.data[3][3] * mat_in.data[0][3]);

		out_mat.data[1][0] = (self.data[0][0] * mat_in.data[1][0]) + (self.data[1][0] * mat_in.data[1][1]) + (self.data[2][0] * mat_in.data[1][2]) + (self.data[3][0] * mat_in.data[1][3]);
		out_mat.data[1][1] = (self.data[0][1] * mat_in.data[1][0]) + (self.data[1][1] * mat_in.data[1][1]) + (self.data[2][1] * mat_in.data[1][2]) + (self.data[3][1] * mat_in.data[1][3]);
		out_mat.data[1][2] = (self.data[0][2] * mat_in.data[1][0]) + (self.data[1][2] * mat_in.data[1][1]) + (self.data[2][2] * mat_in.data[1][2]) + (self.data[3][2] * mat_in.data[1][3]);
		out_mat.data[1][3] = (self.data[0][3] * mat_in.data[1][0]) + (self.data[1][3] * mat_in.data[1][1]) + (self.data[2][3] * mat_in.data[1][2]) + (self.data[3][3] * mat_in.data[1][3]);
		
		out_mat.data[2][0] = (self.data[0][0] * mat_in.data[2][0]) + (self.data[1][0] * mat_in.data[2][1]) + (self.data[2][0] * mat_in.data[2][2]) + (self.data[3][0] * mat_in.data[2][3]);
		out_mat.data[2][1] = (self.data[0][1] * mat_in.data[2][0]) + (self.data[1][1] * mat_in.data[2][1]) + (self.data[2][1] * mat_in.data[2][2]) + (self.data[3][1] * mat_in.data[2][3]);
		out_mat.data[2][2] = (self.data[0][2] * mat_in.data[2][0]) + (self.data[1][2] * mat_in.data[2][1]) + (self.data[2][2] * mat_in.data[2][2]) + (self.data[3][2] * mat_in.data[2][3]);
		out_mat.data[2][3] = (self.data[0][3] * mat_in.data[2][0]) + (self.data[1][3] * mat_in.data[2][1]) + (self.data[2][3] * mat_in.data[2][2]) + (self.data[3][3] * mat_in.data[2][3]);

		out_mat.data[3][0] = (self.data[0][0] * mat_in.data[3][0]) + (self.data[1][0] * mat_in.data[3][1]) + (self.data[2][0] * mat_in.data[3][2]) + (self.data[3][0] * mat_in.data[3][3]);
		out_mat.data[3][1] = (self.data[0][1] * mat_in.data[3][0]) + (self.data[1][1] * mat_in.data[3][1]) + (self.data[2][1] * mat_in.data[3][2]) + (self.data[3][1] * mat_in.data[3][3]);
		out_mat.data[3][2] = (self.data[0][2] * mat_in.data[3][0]) + (self.data[1][2] * mat_in.data[3][1]) + (self.data[2][2] * mat_in.data[3][2]) + (self.data[3][2] * mat_in.data[3][3]);
		out_mat.data[3][3] = (self.data[0][3] * mat_in.data[3][0]) + (self.data[1][3] * mat_in.data[3][1]) + (self.data[2][3] * mat_in.data[3][2]) + (self.data[3][3] * mat_in.data[3][3]);

		out_mat
    }
}