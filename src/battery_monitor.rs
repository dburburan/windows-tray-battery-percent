use battery::Manager;

pub struct BatteryMonitor {
    manager: Manager,
}

impl BatteryMonitor {
	pub fn new() -> Result<Self, String> {
		// Create a battery manager
		match battery::Manager::new() {
			Ok(manager) => Ok(BatteryMonitor{manager}),
			Err(e) => Err(format!("Failed to create battery manager: {:?}", e))
		}
	}

	pub fn get_percentage(&self) -> Result<i32, String> {
		match self.manager.batteries() {
			Err(e) => { Err(format!("Failed to retrieve batteries: {:?}", e)) }
			Ok(mut batteries) => {
				match batteries.next() {
					None => { Err(format!("No batteries found.")) }
					Some(Err(e)) => { Err(format!("Failed to get battery info: {:?}", e)) }
					Some(Ok(bat)) => {
						dbg!(bat.state_of_charge());
						let percentage = (bat.state_of_charge().value * 100.0).round() as i32;
						dbg!(percentage);
						Ok(percentage)
					}
				}
			}
		}
	}
}
