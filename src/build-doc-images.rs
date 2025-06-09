mod icon_builder;
use icon_builder::IconBuilder;

#[derive(Debug)]
pub enum UserEvent {
	TrayIconEvent(tray_icon::TrayIconEvent),
	MenuEvent(tray_icon::menu::MenuEvent),
}

fn build_image(icon_builder: &IconBuilder, percent: i32, discharge_rate_percent: i32, is_charging: bool) {
	let icon = icon_builder.create_percentage_icon(percent, discharge_rate_percent, is_charging).unwrap();
	let file_name = format!(
		"doc-images/icon_p{}_dr{}_c{}.png",
		percent,
		discharge_rate_percent,
		if is_charging { 1 } else { 0 },
	);
	icon.save(file_name).unwrap();
}

fn main() -> Result<(), String> {
	let icon_builder = IconBuilder::new().unwrap();

	// Appearance
	build_image(&icon_builder, 78, 0, false);

	// Charging Status
	build_image(&icon_builder, 78, 0, true);

	// Battery Discharge Rate
	build_image(&icon_builder, 78, 20, false);
	build_image(&icon_builder, 78, 50, false);
	build_image(&icon_builder, 78, 100, false);

	// Discharging While Plugged In
	build_image(&icon_builder, 78, 30, true);
	build_image(&icon_builder, 78, 100, true);

	// Battery Remaining
	// 30 mins
	build_image(&icon_builder, 50, 100, false);
	build_image(&icon_builder, 25, 50, false);
	// 1 hour
	build_image(&icon_builder, 100, 100, false);
	build_image(&icon_builder, 50, 50, false);
	build_image(&icon_builder, 25, 25, false);
	build_image(&icon_builder, 10, 10, false);
	// 2 hours
	build_image(&icon_builder, 100, 50, false);
	build_image(&icon_builder, 50, 25, false);
	build_image(&icon_builder, 25, 12, false);
	// 2 hours despite charging
	build_image(&icon_builder, 100, 50, true);
	build_image(&icon_builder, 50, 25, true);
	build_image(&icon_builder, 25, 12, true);
	
	Ok(())
}
