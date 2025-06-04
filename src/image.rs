use image::{ImageBuffer, RgbaImage};

fn load_digit_image(digit: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
	let path = format!("assets/{}.png", digit);
	let img = image::open(&path)?.to_rgba8();
	Ok(img)
}

pub fn create_percentage_icon(percentage: u8) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let tens = percentage / 10;
	let ones = percentage % 10;
	
	// Load the two digit images
	let tens_img = load_digit_image(tens)?;
	let ones_img = load_digit_image(ones)?;
	
	// Create a 6x5 image (two 3x5 digits side by side)
	let mut combined_img: RgbaImage = ImageBuffer::new(6, 5);
	
	// Copy tens digit to left side
	for y in 0..5 {
		for x in 0..3 {
			let pixel = tens_img.get_pixel(x, y);
			combined_img.put_pixel(x, y, *pixel);
		}
	}
	
	// Copy ones digit to right side
	for y in 0..5 {
		for x in 0..3 {
			let pixel = ones_img.get_pixel(x, y);
			combined_img.put_pixel(x + 3, y, *pixel);
		}
	}
	
	// Convert to raw RGBA bytes
	let raw_data = combined_img.into_raw();
	Ok(raw_data)
}

