use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
	pub origin: Vector,
	pub direction: Vector,
	// TODO: Min + Max t ?
}
impl Ray {
	pub fn at(&self, t: f32) -> Vector {
		self.origin + self.direction * t
	}
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
// Vector functions:
impl Vector {
	pub fn length_squared(&self) -> f32 {
		Self::dot(self, self)
	}
	pub fn length(&self) -> f32 {
		self.length_squared().sqrt()
	}
	pub fn dot(u: &Self, v: &Self) -> f32 {
		u.x * v.x + u.y * v.y + u.z * v.z
	}
	pub fn cross(u: &Self, v: &Self) -> Self {
		Vector::new(
			u.y * v.z - u.z * v.y,
			u.z * v.x - u.x * v.z,
			u.x * v.y - u.y * v.x,
		)
	}
	pub fn unit(self) -> Self {
		let length = self.length();
		self / length
	}
}
// Operators:
impl Vector {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}
	pub fn mult_parts(v1: Self, v2: Self) -> Self {
		Self {
			x: v1.x * v2.x,
			y: v1.y * v2.y,
			z: v1.z * v2.z,
		}
	}
	pub fn reflect(v: Self, n: Self) -> Self {
		v - n * 2.0 * Self::dot(&v, &n)
	}
}
impl Add for Vector {
	type Output = Self;
	fn add(mut self, other: Self) -> Self::Output {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self
	}
}
impl Sub for Vector {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		self + (other * -1.0)
	}
}
impl Mul<f32> for Vector {
	type Output = Self;
	fn mul(mut self, other: f32) -> Self::Output {
		self.x *= other;
		self.y *= other;
		self.z *= other;
		self
	}
}
impl Div<f32> for Vector {
	type Output = Self;
	fn div(mut self, other: f32) -> Self::Output {
		self.x /= other;
		self.y /= other;
		self.z /= other;
		self
	}
}
