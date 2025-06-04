use std::time::{Duration, Instant};
use tray_icon::{TrayIcon, TrayIconEvent};
use tray_icon::menu::MenuEvent;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::WindowId;
use crate::battery_monitor::BatteryMonitor;
use crate::icon_builder::IconBuilder;
use crate::tray_util::TrayBuilder;
use crate::debug_util::dmsg;

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

	fn process_all_events(&mut self, event_loop: &ActiveEventLoop) {
		// Process tray icon events
		while let Ok(event) = TrayIconEvent::receiver().try_recv() {
			dmsg!("Tray event: {:?}", event);
		}

		// Process menu events
		while let Ok(event) = MenuEvent::receiver().try_recv() {
			dmsg!("Menu event: {:?}", event);
			// Handle quit menu item
			match event.id.0.as_str() {
				"Quit" => {
					dmsg!("Quit selected, exiting...");
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

impl ApplicationHandler for BatteryTrayApp {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
	}

	fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
		self.process_all_events(event_loop)
	}
}
