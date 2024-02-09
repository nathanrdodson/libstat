use std::{fs::metadata, path::Path};
use walkdir::{DirEntryExt, WalkDir as wd};

// start with basic scanner
// dont store data, don't watch for file changes
// get some simple piece of information like file size

pub fn scan_library(lib_root: &Path) -> () {
	let walker = wd::new(lib_root);

	for result in walker {
		let _ = match result {
			Ok(entry) => {
				let metadata = metadata(entry.path());
				let _ = match metadata {
					Ok(m) => {
						println!("{}, {:?}, {:?}", entry.path().display(), entry.ino(), m.len());
					},
					Err(err) => {
						println!("ERROR: {}", err);
					}
				};
			},
			Err(err) => {
				println!("ERROR: {}", err);
				continue;
			}
		};
	}
}
