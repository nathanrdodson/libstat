use std::path::Path;

mod libparse;

fn main() {
	libparse::scan_library(Path::new(".workspace/Library"))
}