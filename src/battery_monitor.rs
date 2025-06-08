use battery::Manager;
use crate::debug_util::dmsg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
	pub percentage: i32,
	pub discharge_rate_percent: i32,
	pub is_charging: bool,
}

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

	pub fn get_battery_info(&self) -> Result<BatteryInfo, String> {
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
						let is_charging =
							matches!(bat.state(), battery::State::Charging) ||
							matches!(bat.time_to_empty(), None);
						let discharge_rate_percent: i32 = match bat.time_to_empty() {
							None => 0,
							Some(time) => {
								(soc / time.get::<battery::units::time::hour>() * 100.0).round() as i32
							},
						};
						Ok(BatteryInfo{percentage, discharge_rate_percent, is_charging})
					}
				}
			}
		}
	}
}
