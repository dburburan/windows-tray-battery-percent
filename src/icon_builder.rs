use image::{imageops, ImageBuffer, RgbaImage};

const ICON_WIDTH: u32 = 64;
const ICON_HEIGHT: u32 = 64;
const ICON_MARGIN_W: u32 = 0;
const ICON_MARGIN_H: u32 = 4;
const ICON_DIGIT_GAP_W: u32 = 8;

// Embed digit images at compile time
const ICON_DIGIT_BYTES: [&[u8]; 10] = [
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

const ICON_100_BYTES: &[u8] = include_bytes!("../assets/100.bmp");

pub struct IconBuilder {
	icon_digit_images: [RgbaImage; 10],
	icon_100_image: RgbaImage,
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
		// We load and cache all the digit images here
		// Also scale up the image using nearest neighbor interpolation to avoid fuzziness later
		let icon_digit_images = std::array::from_fn(|i| {
			let img = image::load_from_memory(ICON_DIGIT_BYTES[i]).unwrap().to_rgba8();
			let img = {
				let w = 8*img.width();
				let h = 8*img.height();
				imageops::resize(&img, w, h, imageops::FilterType::Nearest)
			};
			{
				let w = (ICON_WIDTH - ICON_DIGIT_GAP_W) / 2 - ICON_MARGIN_W;
				let h = ICON_HEIGHT - 2*ICON_MARGIN_H;
				imageops::resize(&img, w, h, imageops::FilterType::Lanczos3)
			}
		});

		// Load and scale the 100% icon
		let icon_100_image = {
			let img = image::load_from_memory(ICON_100_BYTES).unwrap().to_rgba8();
			let img = {
				// This is scaled less than all the digits as the resource is already at double resolution,
				// so that the 100 icon can use "half" pixels to squish into the required space
				let w = 4*img.width();
				let h = 4*img.height();
				imageops::resize(&img, w, h, imageops::FilterType::Nearest)
			};
			{
				let w = ICON_WIDTH  - 2*ICON_MARGIN_W;
				let h = ICON_HEIGHT - 2*ICON_MARGIN_H;
				imageops::resize(&img, w, h, imageops::FilterType::Lanczos3)
			}
		};

		// Create green overlay used when charging
		let mut green_gradient_overlay = {
			let mut img = ImageBuffer::new(ICON_WIDTH, ICON_HEIGHT);
			let start_color = image::Rgba([0, 255, 0, 255]);
			let end_color = image::Rgba([255, 255, 255, 255]);
			imageops::vertical_gradient(&mut img, &start_color, &end_color);
			img
		};

		// Create red overlay used when discharging
		let mut red_gradient_overlay = {
			let mut img = ImageBuffer::new(ICON_WIDTH, ICON_HEIGHT);
			let start_color = image::Rgba([255, 200, 200, 255]);
			let end_color = image::Rgba([255, 0, 0, 255]);
			imageops::vertical_gradient(&mut img, &start_color, &end_color);
			img
		};
		
		Ok(IconBuilder {
			icon_digit_images,
			icon_100_image,
			green_gradient_overlay,
			red_gradient_overlay,
		})
	}
	
	pub fn create_percentage_icon(&self, percentage: i32, discharge_rate_percent: i32, is_charging: bool)
			-> Result<RgbaImage, Box<dyn std::error::Error>> {
		let p = percentage.clamp(0, 100);

		let mut icon_image = ImageBuffer::new(ICON_WIDTH, ICON_HEIGHT);
		if p == 100 {
			imageops::overlay(&mut icon_image, &self.icon_100_image, ICON_MARGIN_W as i64, ICON_MARGIN_H as i64);
		}
		else {
			let tens = p / 10;
			let ones = p % 10;
			
			// Get the digit images
			let tens_img = &self.icon_digit_images[tens as usize];
			let ones_img = &self.icon_digit_images[ones as usize];
			
			// Copy digits onto icon_image
			let x1 = ICON_MARGIN_W;
			let x2 = ICON_WIDTH - ones_img.width() - ICON_MARGIN_W;
			imageops::overlay(&mut icon_image, tens_img, x1 as i64, ICON_MARGIN_H as i64);
			imageops::overlay(&mut icon_image, ones_img, x2 as i64, ICON_MARGIN_H as i64);
		}
		
		// Apply green gradient if charging
		if is_charging {
			if discharge_rate_percent > 0 {
				// If we are discharging despite being plugged in it must be a weak usb-c charger so dim the green overlay
				image_overlay_multiply(&mut icon_image, &self.green_gradient_overlay, 0, -15);
			}
			else {
				image_overlay_multiply(&mut icon_image, &self.green_gradient_overlay, 0, 0);
			}
		}

		// Apply red overlay based on discharge rate (fills from bottom to top)
		if discharge_rate_percent > 0 {
			let fill_height =
				(discharge_rate_percent as f32 / 100.0 * ICON_HEIGHT as f32)
				.round().clamp(0.0, ICON_HEIGHT as f32) as u32;

			image_overlay_multiply(&mut icon_image, &self.red_gradient_overlay, 0, (ICON_HEIGHT - fill_height) as i32);
		}
		
		#[cfg(feature = "debug_image_icon")]
		let icon_image = {
			let mut img = ImageBuffer::from_pixel(icon_image.width(), icon_image.height(), image::Rgba([255, 0, 255, 255]));
			imageops::overlay(&mut img, &icon_image, 0, 0);
			img
		};

		Ok(icon_image)
	}
}
