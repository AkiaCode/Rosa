use rosa::Downloader::download;
use std::io::Write;
use std::fs;
use std::io;
use std::io::Read;
use std::convert::TryInto;

fn main() {
	let mut img = download("https://torchedsammy.github.io/assets/img/profile.png", ".");
	let mut file = fs::File::create("test.png").unwrap();
 
    io::copy(&mut img, &mut file);
}
