use image::{imageops, ImageBuffer, RgbaImage};

const ICON_WIDTH: u32 = 64;
const ICON_HEIGHT: u32 = 64;

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
	green_gradient_overlay: RgbaImage,
	red_gradient_overlay: RgbaImage,
}

fn image_overlay_multiply(img: &mut RgbaImage, overlay: &RgbaImage, x_offset: i32, y_offset: i32) {
	let img_width = img.width() as i32;
	let img_height = img.height() as i32;
	let overlay_width = overlay.width() as i32;
	let overlay_height = overlay.height() as i32;

	// Calculate intersection bounds
	let start_x = 0.max(x_offset);
	let start_y = 0.max(y_offset);
	let end_x = img_width.min(x_offset + overlay_width);
	let end_y = img_height.min(y_offset + overlay_height);

	// Loop over the intersection area
	for img_y in start_y..end_y {
		for img_x in start_x..end_x {
			let overlay_x = img_x - x_offset;
			let overlay_y = img_y - y_offset;

			let img_px = img.get_pixel_mut(img_x as u32, img_y as u32);
			let overlay_px = overlay.get_pixel(overlay_x as u32, overlay_y as u32);

			// Multiply blend: result = (original * overlay) / 255
			img_px[0] = ((img_px[0] as u16 * overlay_px[0] as u16) / 255) as u8;  // Red
			img_px[1] = ((img_px[1] as u16 * overlay_px[1] as u16) / 255) as u8;  // Green
			img_px[2] = ((img_px[2] as u16 * overlay_px[2] as u16) / 255) as u8;  // Blue
			// Alpha channel remains unchanged
		}
	}
}

impl IconBuilder {
	pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let digit_images = std::array::from_fn(|i| {
			image::load_from_memory(DIGIT_BYTES[i]).unwrap().to_rgba8()
		});

		// Create green overlay used when charging
		let mut green_gradient_overlay = ImageBuffer::new(ICON_WIDTH, ICON_HEIGHT);
		let start_color = image::Rgba([0, 255, 0, 255]);
		let end_color = image::Rgba([255, 255, 255, 255]);
		imageops::vertical_gradient(&mut green_gradient_overlay, &start_color, &end_color);

		// Create red overlay used when discharging
		let mut red_gradient_overlay = ImageBuffer::new(ICON_WIDTH, ICON_HEIGHT);
		let start_color = image::Rgba([255, 200, 200, 255]);
		let end_color = image::Rgba([255, 0, 0, 255]);
		imageops::vertical_gradient(&mut red_gradient_overlay, &start_color, &end_color);
		
		Ok(IconBuilder { digit_images, green_gradient_overlay, red_gradient_overlay })
	}
	
	pub fn create_percentage_icon(&self, percentage: i32, discharge_rate_percent: i32, is_charging: bool) -> Result<RgbaImage, Box<dyn std::error::Error>> {
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
		let mut scaled_img = imageops::resize(&scaled_img, 64, 64, imageops::FilterType::Lanczos3);
		
		// Apply green gradient if charging
		if is_charging {
			if (discharge_rate_percent > 0) {
				// If we are discharging despite being plugged in it must be a weak usb-c charger so dim the green overlay
				image_overlay_multiply(&mut scaled_img, &self.green_gradient_overlay, 0, -15);
			}
			else {
				image_overlay_multiply(&mut scaled_img, &self.green_gradient_overlay, 0, 0);
			}
		}

		// Apply red overlay based on discharge rate (fills from bottom to top)
		if discharge_rate_percent > 0 {
			let fill_height =
				(discharge_rate_percent as f32 / 100.0 * ICON_HEIGHT as f32)
				.round().clamp(0.0, ICON_HEIGHT as f32) as u32;

			image_overlay_multiply(&mut scaled_img, &self.red_gradient_overlay, 0, (ICON_HEIGHT - fill_height) as i32);
		}
		
		Ok(scaled_img)
	}
}
