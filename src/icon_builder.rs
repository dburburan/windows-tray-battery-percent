use image::{ImageBuffer, RgbaImage, imageops};

fn load_digit_image(digit: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
	let path = format!("assets/{}.png", digit);
	let img = image::open(&path)?.to_rgba8();
	Ok(img)
}

pub fn create_percentage_icon(percentage: u8) -> Result<RgbaImage, Box<dyn std::error::Error>> {
	let tens = percentage / 10;
	let ones = percentage % 10;
	
	// Load the two digit images. My images are 3x5
	let tens_img = load_digit_image(tens)?;
	let ones_img = load_digit_image(ones)?;
	
	// A typical icon in the tray is 16x16
	let mut combined_img: RgbaImage = ImageBuffer::new(7, 7);
	
	// Copy tens digit to left side using built-in overlay function
	imageops::overlay(&mut combined_img, &tens_img, 0, 1);
	
	// Copy ones digit to right side using built-in overlay function
	imageops::overlay(&mut combined_img, &ones_img, 4, 1);
	
	// Scale up the image using nearest neighbor interpolation to avoid fuzziness
	let scale_factor = 2; // Scale up by 2x
	let scaled_img = imageops::resize(&combined_img, 
		combined_img.width() * scale_factor, 
		combined_img.height() * scale_factor, 
		imageops::FilterType::Nearest);

	// Then scale the image to the desired square 16x16
	let scaled_img = imageops::resize(&scaled_img, 16, 16, imageops::FilterType::Lanczos3);
	
	Ok(scaled_img)
}
