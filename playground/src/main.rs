use chrono::{Local, TimeZone,DateTime};

fn main() {
	let toolchain = if cfg!(target_env = "msvc") {
		"MSVC"
	} else if cfg!(target_env = "gnu") {
		"GNU"
	} else {
		"Unknown"
	};

	println!("Compiled with the {} toolchain.", toolchain);
	println!("Hello, world!");
}
