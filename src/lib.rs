use ureq;

pub mod Downloader {
	pub fn download(url: &str, path: &str) -> impl std::io::Read {
		let mut request = ureq::get(url);

		let response = request
			.call();

		response.into_reader()
	}
}