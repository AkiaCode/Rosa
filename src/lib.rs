use ureq;
use std::result::Result;
use std::io::{Read, copy};
use std::fs::File;

pub fn get_file(url: &str) -> Result<impl Read, &ureq::Error> {
	let response = ureq::get(url).call();
	if let Some(err) = response.synthetic_error() {
		return Err(err)
	}

	Ok(response.into_reader())
}

pub fn download<F: ?Sized>(mut file: &mut F, path: &str, name: &str) where F: Read {
	let destpath = &[&path, "/", &name].join("");
	let mut dest = File::create(destpath).unwrap();
	match copy(&mut file, &mut dest) {
    	Ok(a) => a,
    	_ => unreachable!(),
	};
}