use battery::Manager;
use crate::debug_util::dmsg;

pub struct BatteryMonitor {
	manager: Manager,
}

impl BatteryMonitor {
	pub fn new() -> Result<Self, String> {
		// Create a battery manager
		let manager = match battery::Manager::new() {
			Ok(manager) => manager,
			Err(e) => return Err(format!("Failed to create battery manager: {:?}", e))
		};

		Ok(BatteryMonitor {
			manager,
		})
	}

	pub fn get_battery_info(&self) -> Result<(i32, bool), String> {
		match self.manager.batteries() {
			Err(e) => { Err(format!("Failed to retrieve batteries: {:?}", e)) }
			Ok(mut batteries) => {
				match batteries.next() {
					None => { Err(format!("No batteries found.")) }
					Some(Err(e)) => { Err(format!("Failed to get battery info: {:?}", e)) }
					Some(Ok(bat)) => {
						dmsg!("{:?}", bat);
						let soc = bat.state_of_charge().value;
						let percentage = (soc * 100.0).round() as i32;
						let is_charging = matches!(bat.state(), battery::State::Charging);
						Ok((percentage, is_charging))
					}
				}
			}
		}
	}
}
