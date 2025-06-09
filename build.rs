const WINDOWS_MANIFEST: &str = r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
	<application xmlns="urn:schemas-microsoft-com:asm.v3">
		<windowsSettings>
			<dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true</dpiAware>
			<dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
		</windowsSettings>
	</application>
</assembly>"#;

fn main() {
	// Embed Windows manifest for DPI awareness
	#[cfg(target_os = "windows")] {
		let mut res = winres::WindowsResource::new();
		res.set_manifest(WINDOWS_MANIFEST);
		res.compile().unwrap();
	}
}
