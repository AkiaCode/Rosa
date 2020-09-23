use rosa::*;

fn main() {
	let mut img = get_file("https://torchedsammy.github.io/assets/img/profile.png");
	download(&mut img, ".", "test.png");
}
