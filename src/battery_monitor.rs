use starship_battery::{Manager, State};
use crate::debug_util::dmsg;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
	pub percentage: i32,
	pub discharge_rate_percent: i32,
	pub is_charging: bool,
}

#[derive(Debug, Clone)]
struct PreviousBatteryState {
	soc: f32,
	timestamp: Instant,
}

pub struct BatteryMonitor {
	manager: Manager,
	previous_state: Option<PreviousBatteryState>,
}

impl BatteryMonitor {
	pub fn new() -> Result<Self, String> {
		// Create a battery manager
		let manager = match Manager::new() {
			Ok(manager) => manager,
			Err(e) => return Err(format!("Failed to create battery manager: {:?}", e))
		};

		Ok(BatteryMonitor {
			manager,
			previous_state: None,
		})
	}

	pub fn get_battery_info(&mut self) -> Result<BatteryInfo, String> {
		match self.manager.batteries() {
			Err(e) => { Err(format!("Failed to retrieve batteries: {:?}", e)) }
			Ok(mut batteries) => {
				match batteries.next() {
					None => { Err(format!("No batteries found.")) }
					Some(Err(e)) => { Err(format!("Failed to get battery info: {:?}", e)) }
					Some(Ok(bat)) => {
						dmsg!("{:?}", bat);
						// Calculate percentage from the reported soc, which we trust
						let soc = bat.state_of_charge().value;
						let percentage = (soc * 100.0).round() as i32;

						// Calculate rate of discharge in battery-soc/hour. We calculate from the soc and
						// not from time_to_empty or energy_rate as sometimes those numbers seem wrong
						let timestamp = Instant::now();
						let discharge_rate_percent: i32 = match &self.previous_state {
							None => 0,
							Some(prev_state) => {
								let time_diff_hours = timestamp.duration_since(prev_state.timestamp).as_secs_f32() / 3600.0;
								if !(time_diff_hours > 0.0) {0} else {
									let soc_diff = prev_state.soc - soc; // Subtraction is 'backwards' as this is discharge rate
									(soc_diff / time_diff_hours * 100.0).ceil() as i32
								}
							},
						};

						// Sometimes the battery state doesn't switch to charging even when plugged in
						// Possibly when the BMS is choosing to just not charge the battery because it's full
						// So we add a couple of other checks
						let is_charging = 
							matches!(bat.state(), State::Charging) ||
							matches!(bat.time_to_empty(), None) ||
							discharge_rate_percent < 0;

						// Update the stored state for next measurement
						self.previous_state = Some(PreviousBatteryState {soc, timestamp});

						Ok(BatteryInfo{percentage, discharge_rate_percent, is_charging})
					}
				}
			}
		}
	}
}
