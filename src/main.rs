use rosadl::*;

fn main() {
	let mut img = get_file("https://torchedsammy.github.io/assets/img/profile.png");
	let mut _img2 = download_files(["test1", "test2"].to_vec(), ".", ["https://torchedsammy.github.io/assets/img/background.png", "https://torchedsammy.github.io/assets/img/profile.png"].to_vec());
	download(&mut img, ".", "test.png");
	println!("done");
}
