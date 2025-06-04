#![allow(unused)]

use std::thread;
use std::time::Duration;
use tray_icon::{TrayIcon, TrayIconBuilder, Icon, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuEvent};

mod battery;
mod image;

fn create_tray_icon(image_data: Vec<u8>) -> Result<TrayIcon, String> {
	// Create icon from image data
	let icon = Icon::from_rgba(image_data, 6, 5)
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
	let mut _tray_icon: Option<TrayIcon> = None;

	loop {
		// Get current battery percentage
		match battery_monitor.get_percentage() {
			Ok(percentage) => {
				if percentage != current_percentage {
					current_percentage = percentage;
					
					// Create new icon image
					match image::create_percentage_icon(percentage as u8) {
						Ok(image_data) => {
							match create_tray_icon(image_data) {
								Ok(tray) => {
									_tray_icon = Some(tray);
									println!("Updated tray icon to {}%", percentage);
								}
								Err(e) => eprintln!("Failed to create tray icon: {}", e),
							}
						}
						Err(e) => eprintln!("Failed to create icon image: {:?}", e),
					}
				}
			}
			Err(e) => eprintln!("Failed to get battery percentage: {}", e),
		}

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

		// Update every 30 seconds
		thread::sleep(Duration::from_secs(30));
	}
}
