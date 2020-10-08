use ureq;
use std::io::{Read, copy};
use std::fs::File;

pub fn get_file(url: &str) -> impl Read {
	let response: ureq::Response = ureq::get(url).call();
	
	response.into_reader()
}

pub fn download_files(filenames: Vec<&str>, path: &str, urls: Vec<&str>) {
	let mut b = 0;
	for i in urls {
		let responses: ureq::Response = ureq::get(i).call();
		let mut files = responses.into_reader();
		let len = i.len();
		let extension = &i[len - 4..];
		let file = filenames.clone()[b];
		let filename: &str = &(file.to_string() + extension);
		let destpath = &[&path,"/", filename].join("");
		let mut dest =  File::create(destpath).unwrap();
		match copy(&mut files, &mut dest) {
			Ok(a) => a,
			_ => unreachable!(),
		};
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