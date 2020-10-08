use ureq;
use std::result::Result;
use std::io::{Read, copy};
use std::fs::File;

pub fn get_file(url: &str) -> Result<impl Read, String> {
	let response = ureq::get(url).call();
	if let Some(err) = response.synthetic_error() {
		return Err(err.to_string())
	}

	Ok(response.into_reader())
}


pub fn download_multi(filenames: Vec<&str>, path: &str, urls: Vec<&str>) {
	let mut b = 0;
	for i in urls {
		let responses: ureq::Response = ureq::get(i).call();
		let mut files = responses.into_reader();
		let len = i.len();
		let extension = &i[len - 4..];
		let file = filenames.clone()[b];
		let filename: &str = &(file.to_string() + extension);
		download(&mut files, path, filename);
	  	b += 1;
	}
}

pub fn download<F: ?Sized>(mut file: &mut F, path: &str, name: &str) where F: Read {
	let destpath = &[&path, "/", &name].join("");
	let mut dest = File::create(destpath).unwrap();
	match copy(&mut file, &mut dest) {
    	Ok(a) => a,
    	_ => unreachable!(),
	};
}