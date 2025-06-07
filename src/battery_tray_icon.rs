use tray_icon::{TrayIcon, TrayIconBuilder, Icon};
use tray_icon::menu::{Menu, MenuItem};

use crate::battery_monitor::{BatteryMonitor, BatteryInfo};
use crate::icon_builder::IconBuilder;
use crate::debug_util::dmsg;

pub struct BatteryTrayIcon {
	tray_icon: Option<TrayIcon>,
	battery_monitor: BatteryMonitor,
	icon_builder: IconBuilder,
	cached_battery_info: Option<BatteryInfo>,
}

fn create_tray_icon(icon: Icon) -> Result<TrayIcon, String> {
	// Create a simple menu
	let menu = Menu::new();
	let quit_item = MenuItem::with_id("quit", "Quit", true, None);
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

impl BatteryTrayIcon {
	pub fn new(battery_monitor: BatteryMonitor, icon_builder: IconBuilder) -> Self {
		Self {
			tray_icon: None,
			battery_monitor,
			icon_builder,
			cached_battery_info: None, // None to force initial update
		}
	}

	pub fn sync_tray_icon(&mut self) -> Result<(), String> {
		// Get current battery info (percentage and charging status)
		let battery_info = self.battery_monitor.get_battery_info()?;

		// Only update tray icon if battery state changed
		if Some(&battery_info) == self.cached_battery_info.as_ref() {
			Ok(())
		}
		else {
			self.cached_battery_info = Some(battery_info.clone());

			// Create new icon image
			let Ok(icon_image) = self.icon_builder.create_percentage_icon(battery_info.percentage, battery_info.is_charging) else {
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
						dmsg!("Updated tray icon to {}%{}",
							battery_info.percentage,
							if battery_info.is_charging { " (charging)" } else { "" }
						);
						Ok(())
					}
				}
			}
		}
	}
}
