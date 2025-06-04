use image::{ImageBuffer, RgbaImage, imageops};

fn load_digit_image(digit: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
	let path = format!("assets/{}.png", digit);
	let img = image::open(&path)?.to_rgba8();
	Ok(img)
}

pub fn create_percentage_icon(percentage: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
	let tens = percentage / 10;
	let ones = percentage % 10;
	
	// Load the two digit images
	let tens_img = load_digit_image(tens)?;
	let ones_img = load_digit_image(ones)?;
	
	// Create a 7x5 image (two 3x5 digits side by side, with a gap in the middle)
	let mut combined_img: RgbaImage = ImageBuffer::new(7, 5);
	
	// Copy tens digit to left side using built-in overlay function
	imageops::overlay(&mut combined_img, &tens_img, 0, 0);
	
	// Copy ones digit to right side using built-in overlay function
	imageops::overlay(&mut combined_img, &ones_img, 4, 0);
	
	// Scale up the image using nearest neighbor interpolation to avoid fuzziness
	let scale_factor = 4; // Scale up by 4x (6x5 becomes 24x20)
	let scaled_img = imageops::resize(&combined_img, 
		combined_img.width() * scale_factor, 
		combined_img.height() * scale_factor, 
		imageops::FilterType::Nearest);
	
	Ok(scaled_img)
}
