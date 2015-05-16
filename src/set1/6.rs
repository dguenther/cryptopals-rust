// There's a file here. (6.txt) It's been base64'd after being encrypted with
// repeating-key XOR.

// Decrypt it.

// Here's how:

// 1. Let KEYSIZE be the guessed length of the key;
// try values from 2 to (say) 40.
// 2. Write a function to compute the edit distance/Hamming distance between
// two strings. The Hamming distance is just the number of differing bits.
// The distance between:

// this is a test

// and

// wokka wokka!!!

// is 37. Make sure your code agrees before you proceed.

// 3. For each KEYSIZE, take the first KEYSIZE worth of bytes,
// and the second KEYSIZE worth of bytes, and find the edit distance
// between them. Normalize this result by dividing by KEYSIZE.

// 4. The KEYSIZE with the smallest normalized edit distance is probably
// the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values.
// Or take 4 KEYSIZE blocks instead of 2 and average the distances.

// 5. Now that you probably know the KEYSIZE: break the ciphertext into blocks
// of KEYSIZE length.

// 6. Now transpose the blocks: make a block that is the first byte of every
// block, and a block that is the second byte of every block, and so on.

// 7. Solve each block as if it was single-character XOR. You already have code
// to do this.

// 8. For each block, the single-byte XOR key that produces the best looking
// histogram is the repeating-key XOR key byte for that block. Put them
// together and you have the key.

// This code is going to turn out to be surprisingly useful later on.
// Breaking repeating-key XOR ("Vigenere") statistically is obviously an
// academic exercise, a "Crypto 101" thing. But more people "know how" to break
// it than can actually break it, and a similar technique breaks something much
// more important.


#[macro_use]
extern crate log;
extern crate cryptopalslib;

use std::cmp;
use std::cmp::Ordering;
use std::str;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::io::prelude::*;
#[cfg(not(test))]
use std::io::BufReader;
#[cfg(not(test))]
use std::fs::File;
#[cfg(not(test))]
use std::path::Path;


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

	let (key, output) = break_repeating_key_xor_in_file(&arg);
	println!("key: {:?}", key);
	println!("output: {:?}", output);
}

#[cfg(not(test))]
fn break_repeating_key_xor_in_file(path: &str) -> (String, String) {
	let path = Path::new(path);
	let file = BufReader::new(File::open(&path).unwrap());
	let lines = file.lines().map(|x| x.unwrap()).collect();

	break_repeating_key_xor_in_lines(lines)
}

fn break_repeating_key_xor_in_lines(lines: Vec<String>) -> (String, String) {
	let input = cryptopalslib::convert::base64_lines_to_hex(&lines);
	let nums = cryptopalslib::convert::hex_string_to_decimal_pairs(&input);
	break_repeating_key_xor(nums)
}

fn break_repeating_key_xor(bytes: Vec<u8>) -> (String, String) {

	debug!("{:?}", bytes.len());
	let results = rank_keylengths(&bytes);
	debug!("{:?}", results);

	// loop over the vector of keysizes
	for &(_, keysize) in results.iter() {
		debug!("{:?}", keysize);

		// find the best key and the corresponding decoded column for
		// each byte of the keysize.
		let (best_keys, decoded_columns) = find_best_key_and_columns(keysize, &bytes);


		// if we've got something in the output string list,
		// we might have figured out the key, so merge the strings
		// into something readable
		if decoded_columns.len() != 0 {
			let mut output_string = String::new();
			let string_len = decoded_columns[0].len();
			for index in 0..string_len {
				for str_index in 0..decoded_columns.len() {

					if index < decoded_columns[str_index].len() {
						output_string.push(decoded_columns[str_index].chars().nth(index).unwrap());
					}
				}
			}

			let key = match str::from_utf8(&best_keys) {
			    Ok(v) => v.to_string(),
			    Err(_) => { format!("{:?}", best_keys) }
			};
			return (key.to_string(), output_string);
		}
	}
	("".to_string(), "".to_string())
}

fn find_best_key_and_columns(keysize: usize, bytes: &Vec<u8>) -> (Vec<u8>, Vec<String>) {
	let mut decoded_columns = vec!();
	let mut best_keys = vec!();
	for index in 0..keysize {
		let filtered_bytes: Vec<_> = bytes.iter().enumerate()
			.filter(
				|&(x, _)| {
					if index > x {
						return false
					} else {
						return (x - index) % keysize == 0
					}
				})
			.map(|(_, &y)| y)
			.collect();

		let (_, best_key, string) = cryptopalslib::xor::score_and_xor(filtered_bytes);

		// if we get an empty string back, we don't have any valid text,
		// so throw out this keysize
		if string == "" {
			best_keys = vec!();
			decoded_columns = vec!();
			break;
		}

		best_keys.push(best_key);
		decoded_columns.push(string);
	}
	(best_keys, decoded_columns)
}

fn rank_keylengths(bytes: &Vec<u8>) -> Vec<(f32, usize)> {
	let mut results: Vec<(f32, usize)> = Vec::new();
	for keysize in 2..cmp::min(40, (bytes.len() / 3)) {
		// take first keysize of bytes
		let first = &bytes[0..keysize];
		// take second keysize of bytes
		let second = &bytes[keysize..keysize*2];
		// take third keysize of bytes
		let third = &bytes[keysize*2..keysize*3];
		// average the hamming distance between them
		let distance = (cryptopalslib::score::hamming_distance(first, second) + cryptopalslib::score::hamming_distance(second, third) + cryptopalslib::score::hamming_distance(first, third)) / 3;
		// normalize the average distance by the keysize
		let normalized = distance as f32 / keysize as f32;
		results.push((normalized, keysize));
	}

	// sort the vector from least to greatest average distance.
	results.sort_by(|&(x1, _), &(x2, _)| x1.partial_cmp(&x2).unwrap_or(Ordering::Equal));
	results
}

#[cfg(test)]
mod set1challenge6 {

	#[test]
	fn decode() {
		// taken from https://picoctf.com/crypto_mats/index.html
		let input = vec!("mIdwJYSyjmxxt7uZfnGVv4F6OIS/mDU4ifqffTTHvIp2Jceug3Qly/qeeyWOtstzMI6oh2xxlb+IcD+TtpI5cY6uy2IwlPqbZz6Fu4l5KMeYmXwlhrOFMiLHuI5mJcexjmUlx6mOdiOCrsU1BY+zmDU4lPqJcDKGr5hwcYi8y2E5gvqYcDKVv4hscZSvmWc+krSPfD+A+op5Pceug3BxhrmffCeOroJwIse5imcjjr+PNT6J+oNwI4L6j2AjjrSMNQaIqIdxcbC7mTUFkLXLYjCU+oRzcZGzn3Q9x7OGZT6Vrop7MoL6n3pxiK+ZNT+GroJ6P4a2y2Y0hK+ZfCWe+op7Ncevh2E4irufcHGRs4hhPpWjxQ==".to_string());
		let (_, output) = super::break_repeating_key_xor_in_lines(input);
		assert_eq!(output, "Bletchey Park rejoices in the fact that, until fairly recently, it was probably Britain's best kept secret. This is because of the secrecy surrounding all the activities carried on here during World War Two was of vital importance to our national security and ultimate victory.");
	}
}


