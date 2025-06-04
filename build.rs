fn main() {
	// Check if the build is a release build
	if std::env::var("PROFILE").unwrap() == "release" {
		// Disable debug output for release builds
		println!("cargo:rustc-cfg=feature=\"no-debug-output\"");
	}
}
