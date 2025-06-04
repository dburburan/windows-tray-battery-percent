use std::time::{Duration, Instant};
use std::thread;
use tray_icon::{TrayIcon, TrayIconBuilder, Icon, TrayIconEvent};
use tray_icon::menu::{Menu, MenuItem, MenuEvent};
use image::RgbaImage;
use winit::application::ApplicationHandler;
use winit::event_loop::{EventLoop, ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::WindowId;

use crate::battery_monitor::BatteryMonitor;
use crate::icon_builder::IconBuilder;
use crate::tray_util::TrayBuilder;

pub struct BatteryTrayApp {
	pub battery_monitor: BatteryMonitor,
	pub icon_builder: IconBuilder,
	pub current_percentage: i32,
	pub tray_icon: Option<TrayIcon>,
}

impl BatteryTrayApp {
	pub fn new(battery_monitor: BatteryMonitor, icon_builder : IconBuilder) -> Self {
		Self {
			battery_monitor,
			icon_builder,
			current_percentage: 255, // Invalid value to force initial update
			tray_icon: None,
		}
	}
}

impl ApplicationHandler for BatteryTrayApp {
	fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
		// Application resumed
	}

	fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {
		// Handle window events (we don't have windows, so this is empty)
	}

	fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
		// Process tray icon events
		while let Ok(event) = TrayIconEvent::receiver().try_recv() {
			println!("Tray event: {:?}", event);
		}

		// Process menu events
		while let Ok(event) = MenuEvent::receiver().try_recv() {
			println!("Menu event: {:?}", event);
			// Handle quit menu item
			match event.id.0.as_str() {
				"Quit" => {
					println!("Quit selected, exiting...");
					std::process::exit(0);
				}
				_ => {}
			}
		}

		// Update tray icon with current battery percentage
		if let Err(e) = self.creset_tray_icon() {
			eprintln!("Failed to update tray icon: {}", e);
		}

		// Set control flow to wake up after 1 second
		event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1)));
	}
}
