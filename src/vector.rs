use core::ops::{self, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy)]
pub struct vec2f
{
	pub x: f32,
	pub y: f32
}

#[derive(Clone, Copy)]
pub struct vec3f
{
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(Clone, Copy)]
pub struct vec4f
{
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32
}

impl vec3f
{
	pub fn new() -> vec3f
	{
		vec3f{
			x: 0.0f32,
			y: 0.0f32,
			z: 0.0f32
		}
	}

	pub fn scale(&self, factor: f32) -> vec3f
	{
		vec3f{
			x: self.x * factor,
			y: self.y * factor,
			z: self.z * factor
		}
	}

	pub fn add(&self, v3: &vec3f) -> vec3f
	{
		vec3f{
			x: self.x + v3.x,
			y: self.y + v3.y,
			z: self.z + v3.z
		}
	}
}

impl ops::Add<&vec3f> for vec3f 
{
	type Output = vec3f;

    fn add(self, _rhs: &vec3f) -> vec3f 
	{
        vec3f{
			x: self.x + _rhs.x,
			y: self.y + _rhs.y,
			z: self.z + _rhs.z
		}
    }
}

impl vec4f
{
	pub fn new() -> vec4f
	{
		vec4f{
			x: 0.0f32,
			y: 0.0f32,
			z: 0.0f32,
			w: 0.0f32
		}
	}
}

impl vec2f
{
	pub fn new() -> vec2f
	{
		vec2f{
			x: 0.0f32,
			y: 0.0f32
		}
	}

	pub fn add(&self, v2: &vec2f) -> vec2f
	{
		vec2f{
			x: self.x + v2.x,
			y: self.y + v2.y
		}
	}

	pub fn sub(&self, v2: &vec2f) -> vec2f
	{
		vec2f{
			x: self.x - v2.x,
			y: self.y - v2.y
		}
	}

	pub fn scale(&self, factor: f32) -> vec2f
	{
		vec2f{
			x: self.x * factor,
			y: self.y * factor
		}
	}

	pub fn rotate(&self, degrees: f32) -> vec2f
	{
		let s = libm::sin(degrees as f64) as f32;
		let c = libm::cos(degrees as f64) as f32;

		vec2f{
			x: (self.x * c - self.y * s), 
			y: (self.x * s + self.y * c)
		}
	}

	pub fn translate(&self, vec: vec2f) -> vec2f
	{
		vec2f{
			x: self.x + vec.x, 
			y: self.y + vec.y
		}
	}
}

impl Default for vec2f
{
	fn default() -> Self {
		vec2f::new()
	}
}

impl ops::Add<&vec2f> for &vec2f 
{
    type Output = vec2f;

    fn add(self, _rhs: &vec2f) -> vec2f 
	{
        vec2f{
			x: self.x + _rhs.x,
			y: self.y + _rhs.y
		}
    }
}

impl ops::Sub<&vec2f> for &vec2f 
{
    type Output = vec2f;

    fn sub(self, _rhs: &vec2f) -> vec2f 
	{
        vec2f{
			x: self.x - _rhs.x,
			y: self.y - _rhs.y
		}
    }
}

impl ops::Mul<f32> for vec2f 
{
    type Output = vec2f;

    fn mul(self, factor: f32) -> vec2f 
	{
        vec2f{
			x: self.x * factor,
			y: self.y * factor
		}
    }
}

impl ops::Div<f32> for vec2f 
{
    type Output = vec2f;

    fn div(self, factor: f32) -> vec2f 
	{
        vec2f{
			x: self.x / factor,
			y: self.y / factor
		}
    }
}

// assign 
impl ops::AddAssign<&vec2f> for vec2f 
{

    fn add_assign(&mut self, _rhs: &vec2f) 
	{
        self.x += _rhs.x;
		self.y += _rhs.y;
    }
}

impl ops::SubAssign<&vec2f> for vec2f 
{

    fn sub_assign(&mut self, _rhs: &vec2f) 
	{
        self.x -= _rhs.x;
		self.y -= _rhs.y;
    }
}

impl ops::MulAssign<f32> for vec2f 
{

    fn mul_assign(&mut self, factor: f32) 
	{
        self.x *= factor;
		self.y *= factor;
    }
}

impl ops::DivAssign<f32> for vec2f 
{
    fn div_assign(&mut self, factor: f32) 
	{
        self.x /= factor;
		self.y /= factor;
    }
}