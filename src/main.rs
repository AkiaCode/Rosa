use rosadl::*;

fn main() {
	let mut img = get_file("https://torchedsammy.github.io/assets/img/profile.png").unwrap();
	let mut _img2 = download_multi(["test1", "test2"].to_vec(), ".", ["https://torchedsammy.github.io/assets/img/background.png", "https://torchedsammy.github.io/assets/img/profile.png"].to_vec());
	download(&mut img, ".", "test.png");
	println!("done");
}
