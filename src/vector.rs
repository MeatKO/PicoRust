use core::ops::{self, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Copy)]
pub struct Vec2D
{
	pub x: f32,
	pub y: f32
}

impl Vec2D
{
	pub fn New() -> Vec2D
	{
		Vec2D{
			x: 0.0f32,
			y: 0.0f32
		}
	}

	pub fn add(&self, v2: &Vec2D) -> Vec2D
	{
		Vec2D{
			x: self.x + v2.x,
			y: self.y + v2.y
		}
	}

	pub fn sub(&self, v2: &Vec2D) -> Vec2D
	{
		Vec2D{
			x: self.x - v2.x,
			y: self.y - v2.y
		}
	}

	pub fn scale(&self, factor: f32) -> Vec2D
	{
		Vec2D{
			x: self.x * factor,
			y: self.y * factor
		}
	}

	pub fn rotate(&self, degrees: f32) -> Vec2D
	{
		let s = libm::sin(degrees as f64) as f32;
		let c = libm::cos(degrees as f64) as f32;

		Vec2D{
			x: (self.x * c - self.y * s), 
			y: (self.x * s + self.y * c)
		}
	}

	pub fn translate(&self, vec: Vec2D) -> Vec2D
	{
		Vec2D{
			x: self.x + vec.x, 
			y: self.y + vec.y
		}
	}
}

impl Default for Vec2D
{
	fn default() -> Self {
		Vec2D::New()
	}
}

impl ops::Add<&Vec2D> for &Vec2D 
{
    type Output = Vec2D;

    fn add(self, _rhs: &Vec2D) -> Vec2D 
	{
        Vec2D{
			x: self.x + _rhs.x,
			y: self.y + _rhs.y
		}
    }
}

impl ops::Sub<&Vec2D> for &Vec2D 
{
    type Output = Vec2D;

    fn sub(self, _rhs: &Vec2D) -> Vec2D 
	{
        Vec2D{
			x: self.x - _rhs.x,
			y: self.y - _rhs.y
		}
    }
}

impl ops::Mul<f32> for Vec2D 
{
    type Output = Vec2D;

    fn mul(self, factor: f32) -> Vec2D 
	{
        Vec2D{
			x: self.x * factor,
			y: self.y * factor
		}
    }
}

impl ops::Div<f32> for Vec2D 
{
    type Output = Vec2D;

    fn div(self, factor: f32) -> Vec2D 
	{
        Vec2D{
			x: self.x / factor,
			y: self.y / factor
		}
    }
}

// assign 
impl ops::AddAssign<&Vec2D> for Vec2D 
{

    fn add_assign(&mut self, _rhs: &Vec2D) 
	{
        self.x += _rhs.x;
		self.y += _rhs.y;
    }
}

impl ops::SubAssign<&Vec2D> for Vec2D 
{

    fn sub_assign(&mut self, _rhs: &Vec2D) 
	{
        self.x -= _rhs.x;
		self.y -= _rhs.y;
    }
}

impl ops::MulAssign<f32> for Vec2D 
{

    fn mul_assign(&mut self, factor: f32) 
	{
        self.x *= factor;
		self.y *= factor;
    }
}

impl ops::DivAssign<f32> for Vec2D 
{
    fn div_assign(&mut self, factor: f32) 
	{
        self.x /= factor;
		self.y /= factor;
    }
}