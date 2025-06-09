use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::event::WindowEvent;
use winit::window::WindowId;
use crate::battery_monitor::BatteryMonitor;
use crate::icon_builder::IconBuilder;
use crate::battery_tray_icon::BatteryTrayIcon;
use crate::debug_util::dmsg;
use crate::UserEvent;

const UPDATE_SLEEP_SECONDS: u64 = 10;

pub struct BatteryTrayApp {
	pub tray_icon: BatteryTrayIcon,
}

impl BatteryTrayApp {
	pub fn new(battery_monitor: BatteryMonitor, icon_builder : IconBuilder) -> Self {
		Self {
			tray_icon: BatteryTrayIcon::new(battery_monitor, icon_builder),
		}
	}

	fn check_battery(&mut self, event_loop: &ActiveEventLoop) {
		if let Err(_e) = self.tray_icon.sync_tray_icon() {
			dmsg!("Failed to update tray icon: {}", _e);
		}
		
		// Make sure we check again soon
		event_loop.set_control_flow(ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(UPDATE_SLEEP_SECONDS)));
	}
}

impl ApplicationHandler<UserEvent> for BatteryTrayApp {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		dmsg!("Resume event");
		self.check_battery(event_loop);
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {
		dmsg!("Window event: {:?}", _event);
		self.check_battery(event_loop);
	}

	fn new_events(&mut self, event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
		dmsg!("New event: {:?}", _cause);
		self.check_battery(event_loop);
	}

	fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
		dmsg!("User event: {:?}", event);
		self.check_battery(event_loop);

		match event {
			UserEvent::TrayIconEvent(_tray_event) => { }
			UserEvent::MenuEvent(menu_event) => {
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
