use tray_icon::{TrayIcon, TrayIconBuilder, Icon};
use tray_icon::menu::{Menu, MenuItem};

use crate::battery_tray_app::BatteryTrayApp;

pub trait TrayBuilder {
    fn creset_tray_icon(&mut self) -> Result<(), String>;
}

fn create_tray_icon(icon: Icon) -> Result<TrayIcon, String> {
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

impl TrayBuilder for BatteryTrayApp {
	fn creset_tray_icon(&mut self) -> Result<(), String> {
		// Get current battery percentage
		let battery_percent = self.battery_monitor.get_percentage()?;

		// Only update tray icon if percentage changed
		if battery_percent == self.current_percentage { Ok(()) }
		else {
			self.current_percentage = battery_percent;
			dbg!(battery_percent);

			// Create new icon image
			let Ok(icon_image) = self.icon_builder.create_percentage_icon(battery_percent) else {
				return Err(format!("Couldn't build icon"));
			};
			let icon = Icon::from_rgba(icon_image.clone().into_raw(), icon_image.width(), icon_image.height())
				.map_err(|e| format!("Failed to create icon: {:?}", e))?;

			// Create or update tray icon
			match &self.tray_icon {
				None => {
					match create_tray_icon(icon) {
						Ok(t) => {
							self.tray_icon = Some(t);
							Ok(())
						}
						Err(e) => {
							Err(e)
						}
					}
				}
				Some(t) => {
					if let Err(e) = t.set_icon(Some(icon)) {
						Err(format!("Failed to update tray icon: {:?}", e))
					}
					else {
						println!("Updated tray icon to {}%", battery_percent);
						Ok(())
					}
				}
			}
		}
	}
}