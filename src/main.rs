#![allow(unused)]

use winit::event_loop::EventLoop;

mod battery_monitor;
mod icon_builder;
mod tray_util;
mod battery_tray_app;
mod debug_util;

fn main() -> Result<(), String> {
	// Create battery monitor
	let battery_monitor = battery_monitor::BatteryMonitor::new()?;

	// The icon builder holds the image resources for drawing digits
	let icon_builder = icon_builder::IconBuilder::new().unwrap();
	
	// Create application
	let mut app = battery_tray_app::BatteryTrayApp::new(battery_monitor, icon_builder);

	// Start the event loop
	let event_loop = EventLoop::new().map_err(|e| format!("Failed to create event loop: {:?}", e))?;
	event_loop.run_app(&mut app).map_err(|e| format!("Event loop error: {:?}", e))?;

	Ok(())
}
