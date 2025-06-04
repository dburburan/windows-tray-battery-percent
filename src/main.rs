#![allow(unused)]

use std::time::{Duration, Instant};
use std::thread;
use tray_icon::{TrayIcon, TrayIconBuilder, Icon, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuEvent};
use image::RgbaImage;
use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::WindowId;

mod battery_monitor;
mod icon_builder;
mod tray_util;
mod battery_tray_app;

fn main() -> Result<(), String> {
	// Create event loop
	let event_loop = EventLoop::new().map_err(|e| format!("Failed to create event loop: {:?}", e))?;
	
	// Create battery monitor
	let battery_monitor = battery_monitor::BatteryMonitor::new()?;
	
	// Create application
	let mut app = battery_tray_app::BatteryTrayApp {
		battery_monitor,
		current_percentage: 255, // Invalid value to force initial update
		tray_icon: None,
	};

	// Run the event loop
	event_loop.run_app(&mut app).map_err(|e| format!("Event loop error: {:?}", e))?;

	Ok(())
}
