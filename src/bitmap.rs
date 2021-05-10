extern "C" {
	fn view_bitmap(ptr: *const Color, width: usize, len: usize);
}

// Since the default integral value is 0, the default color will be transparent black - as intended.
#[repr(packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
impl Color {}

#[derive(Debug)]
pub struct Bitmap {
	pub width: usize,
	pub data: Box<[Color]>,
}
impl Bitmap {
	pub fn new(width: usize, height: usize) -> Self {
		Self::new_filled(Color::default(), width, height)
	}
	pub fn new_filled(color: Color, width: usize, height: usize) -> Self {
		Self::new_data(vec![color; width * height], width)
	}
	pub fn new_data(data: impl Into<Box<[Color]>>, width: usize) -> Self {
		let data = data.into();
		// Ensure that the data is a multiple of width
		assert_eq!(data.len() % width, 0);
		Self { width, data }
	}
	pub fn height(&self) -> usize {
		self.data.len() / self.width
	}
	pub fn view(&self) {
		let ptr = self.data.as_ptr();
		unsafe {
			view_bitmap(
				ptr,
				self.width,
				self.data.len() * std::mem::size_of::<Color>(),
			);
		}
	}
}
