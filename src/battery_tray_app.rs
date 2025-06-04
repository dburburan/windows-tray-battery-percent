use std::time::{Duration, Instant};
use tray_icon::TrayIcon;
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::WindowId;
use crate::battery_monitor::BatteryMonitor;
use crate::icon_builder::IconBuilder;
use crate::tray_util::TrayBuilder;
use crate::debug_util::dmsg;
use crate::UserEvent;

const UPDATE_SLEEP_SECONDS: u64 = 30;

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
			current_percentage: -1, // Invalid value to force initial update
			tray_icon: None,
		}
	}
}

impl ApplicationHandler<UserEvent> for BatteryTrayApp {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		// Set up periodic battery updates
		event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now()));
	}

	fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {
	}

	fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
		// Only update battery on timer events, not on every event
		match cause {
			winit::event::StartCause::ResumeTimeReached { .. } => {
				// Update tray icon with current battery percentage
				if let Err(e) = self.creset_tray_icon() {
					eprintln!("Failed to update tray icon: {}", e);
				}
				// Schedule next update
				event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(UPDATE_SLEEP_SECONDS)));
			}
			_ => {
				// Don't spam logs for other events
			}
		}
	}

	fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
		match event {
			UserEvent::TrayIconEvent(tray_event) => {
				dmsg!("Tray event: {:?}", tray_event);
			}
			UserEvent::MenuEvent(menu_event) => {
				dmsg!("Menu event: {:?}", menu_event);
				// Handle quit menu item
				match menu_event.id.0.as_str() {
					"quit" => {
						dmsg!("Quit selected, exiting...");
						std::process::exit(0);
					}
					_ => {}
				}
			}
		}
	}
}
