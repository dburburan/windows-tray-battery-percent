#![allow(unused)]

use winit::event_loop::EventLoop;
use tray_icon::{TrayIconEvent, menu::MenuEvent};

mod battery_monitor;
mod icon_builder;
mod tray_util;
mod battery_tray_app;
mod debug_util;

#[derive(Debug)]
pub enum UserEvent {
	TrayIconEvent(tray_icon::TrayIconEvent),
	MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() -> Result<(), String> {
	// Create battery monitor
	let battery_monitor = battery_monitor::BatteryMonitor::new()?;

	// The icon builder holds the image resources for drawing digits
	let icon_builder = icon_builder::IconBuilder::new().unwrap();
	
	// Create application
	let mut app = battery_tray_app::BatteryTrayApp::new(battery_monitor, icon_builder);

	// Create event loop with user events
	let event_loop = EventLoop::<UserEvent>::with_user_event().build().map_err(|e| format!("Failed to create event loop: {:?}", e))?;

	// Set up tray event handlers to forward events to winit event loop
	let proxy = event_loop.create_proxy();
	TrayIconEvent::set_event_handler(Some(move |event| {
		let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
	}));

	let proxy = event_loop.create_proxy();
	MenuEvent::set_event_handler(Some(move |event| {
		let _ = proxy.send_event(UserEvent::MenuEvent(event));
	}));

	// Start the event loop
	event_loop.run_app(&mut app).map_err(|e| format!("Event loop error: {:?}", e))?;

	Ok(())
}
