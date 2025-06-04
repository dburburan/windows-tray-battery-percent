#![allow(unused)]

use std::thread;
use std::time::Duration;
use tray_icon::{TrayIcon, TrayIconBuilder, Icon, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuEvent};
use image::RgbaImage;

mod battery;
mod icon_builder;

fn create_tray_icon(rgba_image: &RgbaImage) -> Result<TrayIcon, String> {
	// Create icon from image data
	let icon = Icon::from_rgba(rgba_image.clone().into_raw(), rgba_image.width(), rgba_image.height())
		.map_err(|e| format!("Failed to create icon: {:?}", e))?;
	
	// Create a simple menu
	let menu = Menu::new();
	let quit_item = MenuItem::new("Quit", true, None);
	menu.append(&quit_item).map_err(|e| format!("Failed to add menu item: {:?}", e))?;
	
	// Create the tray icon
	let tray_icon = TrayIconBuilder::new()
		.with_icon(icon)
		.with_menu(Box::new(menu))
		.with_tooltip("Battery Percentage")
		.build()
		.map_err(|e| format!("Failed to create tray icon: {:?}", e))?;
	
	Ok(tray_icon)
}

fn main() -> Result<(), String> {
	// Create battery monitor
	let battery_monitor = battery::BatteryMonitor::new()?;
	let mut current_percentage = 255; // Invalid value to force initial update
	
	// Create initial tray icon
	let initial_image = icon_builder::create_percentage_icon(0)
		.map_err(|e| format!("Failed to create initial icon: {:?}", e))?;
	let tray_icon = create_tray_icon(&initial_image)?;

	loop {
		// Process tray icon events
		if let Ok(event) = TrayIconEvent::receiver().try_recv() {
			println!("Tray event: {:?}", event);
		}

		// Process menu events
		if let Ok(event) = MenuEvent::receiver().try_recv() {
			println!("Menu event: {:?}", event);
			// Handle quit menu item
			match event.id.0.as_str() {
				"Quit" => {
					println!("Quit selected, exiting...");
					return Ok(());
				}
				_ => {}
			}
		}

		// Get current battery percentage
		let battery_percent = battery_monitor.get_percentage()?;
		
		// Only update tray icon if percentage changed
		if battery_percent != current_percentage {
			current_percentage = battery_percent;
			dbg!(battery_percent);
						
			// Create new icon image and update the existing tray icon
			match icon_builder::create_percentage_icon(battery_percent as u8) {
				Ok(rgba_image) => {
					let icon = Icon::from_rgba(rgba_image.clone().into_raw(), rgba_image.width(), rgba_image.height())
						.map_err(|e| format!("Failed to create icon: {:?}", e))?;
					
					if let Err(e) = tray_icon.set_icon(Some(icon)) {
						eprintln!("Failed to update tray icon: {:?}", e);
					}
					else {
						println!("Updated tray icon to {}%", battery_percent);
					}
				}
				Err(e) => eprintln!("Failed to create icon image: {:?}", e),
			}
		}

		// Check for update every 1 second
		thread::sleep(Duration::from_secs(1));
	}
}
