// The Base64-encoded content in this file (7.txt)has been encrypted
// via AES-128 in ECB mode under the key

// "YELLOW SUBMARINE".
// (case-sensitive, without the quotes; exactly 16 characters;
// I like "YELLOW SUBMARINE" because it's exactly 16 bytes long,
// and now you do too).

// Decrypt it. You know the key, after all.

// Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.


#![feature(step_by)]

#[macro_use]
extern crate log;
extern crate openssl;

use std::ascii::AsciiExt;
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
	let base64_decoded = base64_lines_to_hex(&lines);
	let nums = convert_hex_string_to_decimal_pairs(&base64_decoded);

	let t = openssl::crypto::symm::decrypt(openssl::crypto::symm::Type::AES_128_ECB, key.as_bytes(), vec!(), &nums);

	match str::from_utf8(&t) {
		Ok(s) => return s.to_string(),
		Err(_) => panic!("Result couldn't be converted to u8")
	}
}

fn convert_hex_string_to_decimal_pairs(string: &String) -> Vec<u8> {
	let string_lower = string.to_ascii_lowercase();
	let bytes = string_lower.as_bytes();

	let mut decimal_values = vec!();
	let mut tick = 1;
	let mut current_byte = 0;

	for &x in bytes.iter() {
		current_byte += convert_hex_char_to_decimal(x) * 16u8.pow(tick);
		tick = tick ^ 1;
		if tick == 1 {
			decimal_values.push(current_byte);
			current_byte = 0;
		}
	}

	decimal_values
}

fn convert_hex_char_to_decimal(character: u8) -> u8 {
	// if x is in the ascii range for numbers, subtract 48,
	// which is '0' in ascii. Otherwise, it should be a lowercase letter.
	// 'a' is 97, so subtract 87.
	match character < 58 {
		true => character - 48,
		false => character - 87
	}
}

// BASE 64 TO HEX

fn base64_lines_to_hex(lines: &Vec<String>) -> String {
	let mut output = String::new();
	for line in lines {
		output.push_str(&base64_to_hex(&line.trim()));
	}
	output
}

fn base64_to_hex(input: &str) -> String {
	let bytes = input.as_bytes();
	let mut nums = vec!();
	for index in (0..bytes.len()).step_by(4) {
		let slice = &bytes[index..index+4];
		debug!("{:?}", index);
		let byte0 = base64_ascii_to_index(slice[0]);
		let byte1 = base64_ascii_to_index(slice[1]);
		let byte2 = base64_ascii_to_index(slice[2]);
		let byte3 = base64_ascii_to_index(slice[3]);

		match (byte0, byte1, byte2, byte3) {
			(Some(b0), Some(b1), Some(b2), Some(b3)) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
				nums.push(b1 & 15);
				nums.push((b2 & 60) >> 2);
				nums.push(((b2 & 3) << 2) + (b3 >> 4));
				nums.push(b3 & 15);
			},
			(Some(b0), Some(b1), Some(b2), None) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
				nums.push(b1 & 15);
				nums.push((b2 & 60) >> 2);
			},
			(Some(b0), Some(b1), None, None) => {
				nums.push(b0 >> 2);
				nums.push(((b0 & 3) << 2) + (b1 >> 4));
			},
			_ => panic!("Only the last two bytes may be unused")
		};

	}

	let hex: Vec<u8> = nums.iter().map(|&x| convert_num_to_hex_char(x)).collect();

	match str::from_utf8(&hex) {
	    Ok(v) => {
	        debug!("output string: {:?}", v);
	        return v.to_string();
	    }
	    Err(e) => {
	        panic!("error parsing string: {:?}", e);
    	}
	}

}

fn base64_ascii_to_index(ascii: u8) -> Option<u8> {
	match ascii {
		65...90 => Some(ascii - 65), // uppercase
		97...122 => Some(ascii - 71), // lowercase
		48...57 => Some(ascii + 4), // numbers
		43 => Some(ascii + 19), // +
		47 => Some(ascii + 16), // /
		// equals means empty byte. need some way of
		// differentiating from 'A'.
		61 => None, // =

		_ => panic!("{:?}: Not in base64 range", ascii)
	}
}

fn convert_num_to_hex_char(num: u8) -> u8 {
	match num {
		0...9 => num + 48,
		10...15 => num + 87,
		_ => panic!("Not a valid hex char")
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