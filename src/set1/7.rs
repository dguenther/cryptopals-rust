// The Base64-encoded content in this file (7.txt)has been encrypted
// via AES-128 in ECB mode under the key

// "YELLOW SUBMARINE".
// (case-sensitive, without the quotes; exactly 16 characters;
// I like "YELLOW SUBMARINE" because it's exactly 16 bytes long,
// and now you do too).

// Decrypt it. You know the key, after all.

// Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.

#[macro_use]
extern crate log;
extern crate openssl;
extern crate cryptopalslib;

use std::str;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::io::BufReader;
#[cfg(not(test))]
use std::fs::File;
#[cfg(not(test))]
use std::path::Path;
#[cfg(not(test))]
use std::io::prelude::*;

#[cfg(not(test))]
fn main() {

	println!("Decoding...");

	if env::args().count() < 2 {
		panic!("Must pass a file to decode")
	}

	let arg = match env::args().nth(1) {
		Some(s) => s,
		None => panic!("No input argument given")
	};

	let output = decrypt_aes_ecb_128_file("YELLOW SUBMARINE", &arg);
	println!("output: {:?}", output);
}

#[cfg(not(test))]
fn decrypt_aes_ecb_128_file(key: &str, path: &str) -> String {
	let path = Path::new(path);
	let file = BufReader::new(File::open(&path).unwrap());
	let lines: Vec<_> = file.lines()
		.map(|x| x.unwrap())
		.collect();

	decrypt_base64_aes_ecb_128(key, lines)
}

fn decrypt_base64_aes_ecb_128(key: &str, lines: Vec<String>) -> String {
	let base64_decoded = cryptopalslib::convert::base64_lines_to_hex(&lines);
	let nums = cryptopalslib::convert::hex_string_to_decimal_pairs(&base64_decoded);

	let t = openssl::crypto::symm::decrypt(openssl::crypto::symm::Type::AES_128_ECB, key.as_bytes(), vec!(), &nums);

	match str::from_utf8(&t) {
		Ok(s) => return s.to_string(),
		Err(_) => panic!("Result couldn't be converted to u8")
	}
}

#[cfg(test)]
mod set1challenge7 {

	#[test]
	fn decrypt_aes_ecb_128() {
		let input = "o3VBEciqmzUQswmiEMLdfPuhlv1XK0i0ww26jHAiaeY=".to_string();
		let original = "This is encrypt.";
		let key = "TESTTESTTESTTEST";

		let output = super::decrypt_base64_aes_ecb_128(key, vec!(input));
		assert_eq!(output, original);
	}

}