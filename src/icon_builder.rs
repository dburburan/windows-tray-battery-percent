use image::{ImageBuffer, RgbaImage, imageops};

// Embed digit images at compile time
const DIGIT_BYTES: [&[u8]; 10] = [
	include_bytes!("../assets/0.bmp"),
	include_bytes!("../assets/1.bmp"),
	include_bytes!("../assets/2.bmp"),
	include_bytes!("../assets/3.bmp"),
	include_bytes!("../assets/4.bmp"),
	include_bytes!("../assets/5.bmp"),
	include_bytes!("../assets/6.bmp"),
	include_bytes!("../assets/7.bmp"),
	include_bytes!("../assets/8.bmp"),
	include_bytes!("../assets/9.bmp"),
];

pub struct IconBuilder {
	digit_images: [RgbaImage; 10],
}

impl IconBuilder {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let digit_images = std::array::from_fn(|i| {
			image::load_from_memory(DIGIT_BYTES[i]).unwrap().to_rgba8()
		});
		
		Ok(IconBuilder { digit_images })
	}
	
	pub fn create_percentage_icon(&self, percentage: i32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
		let p = percentage.clamp(0, 99);
		let tens = p / 10;
		let ones = p % 10;
		
		// Get the two digit images. My images are 3x5
		let tens_img = &self.digit_images[tens as usize];
		let ones_img = &self.digit_images[ones as usize];
		
		// A typical icon in the tray is 16x16
		let mut combined_img: RgbaImage = ImageBuffer::new(7, 6);
		
		// Copy tens digit to left side using built-in overlay function
		imageops::overlay(&mut combined_img, tens_img, 0, 1);
		
		// Copy ones digit to right side using built-in overlay function
		imageops::overlay(&mut combined_img, ones_img, 4, 1);
		
		// Scale up the image using nearest neighbor interpolation to avoid fuzziness
		let scale_factor = 8; // Scale up by 8x
		let scaled_img = imageops::resize(&combined_img,
			combined_img.width() * scale_factor,
			combined_img.height() * scale_factor,
			imageops::FilterType::Nearest);

		// Then scale the image to the desired square 64x64
		let scaled_img = imageops::resize(&scaled_img, 64, 64, imageops::FilterType::Lanczos3);
		
		Ok(scaled_img)
	}
}
