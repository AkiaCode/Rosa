use ureq;
use std::io::{Read, copy};
use std::fs::File;

pub fn get_file(url: &str) -> impl Read {
	let mut request = ureq::get(url);

	let response = request.call();

	response.into_reader()
}

pub fn download<F: ?Sized>(mut file: &mut F, path: &str, name: &str) where F: Read {
	let destpath = &[&path, "/", &name].join("");
	let mut dest = File::create(destpath).unwrap();
	copy(&mut file, &mut dest);
}