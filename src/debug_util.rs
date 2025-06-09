pub fn string_take_end(s: &str, n: usize) -> &str {
	if n == 0 || s.is_empty() {
		return "";
	}
	
	// Count total characters
	let total_chars = s.chars().count();
	
	// If n exceeds total chars, return the whole string
	if n >= total_chars {
		return s;
	}
	
	// Skip to the position where the last n chars start
	let chars_to_skip = total_chars - n;
	
	// Find the byte position where to start the slice
	let byte_pos = s
			.char_indices()
			.skip(chars_to_skip)
			.next()
			.map(|(i, _)| i)
			.unwrap_or(0);
	
	&s[byte_pos..]
}

pub fn debug_println(filename: &str, line: u32, msg: &str) {
	let now = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap();
	let secs = now.as_secs();
	let millis = now.subsec_millis();

	// Extract hours, minutes, seconds
	let hours = (secs % 86400) / 3600;
	let minutes = (secs % 3600) / 60;
	let seconds = secs % 60;

	let f = string_take_end(filename, 20);
	println!("[{:>20}:{:<3}] {:02}:{:02}:{:02}.{:03}: {}", f, line, hours, minutes, seconds, millis, msg);
}

#[macro_export]
macro_rules! debug_value {
	($val:expr) => {
		#[cfg(feature = "debug-logging")]
		{
			match $val {
				tmp => {
					crate::debug_util::debug_println(
						file!(),
						line!(),
						&format!("{} = {:?}", stringify!($val), &tmp)
					);
					tmp
				}
			}
		}
		#[cfg(not(feature = "debug-logging"))]
		{
			$val
		}
	};
	// Handle multiple values
	($($val:expr),+ $(,)?) => {
		($(debug_value!($val)),+,)
	};
}
pub use debug_value as dbgt; // Re-export it as dbgt

#[macro_export]
macro_rules! debug_message {
	($($arg:tt)*) => {
		#[cfg(feature = "debug-logging")]
		{
			crate::debug_util::debug_println(
				file!(),
				line!(),
				&format!($($arg)*)
			);
		}
	};
}
pub use debug_message as dmsg;
