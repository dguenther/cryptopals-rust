// Detect AES in ECB mode
// In this file (8.txt) are a bunch of hex-encoded ciphertexts.

// One of them has been encrypted with ECB.

// Detect it.

// Remember that the problem with ECB is that it is stateless and
// deterministic; the same 16 byte plaintext block will always
// produce the same 16 byte ciphertext.


#![feature(old_io)]
#![feature(old_path)]

use std::collections::HashSet;

#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::old_io::BufferedReader;
#[cfg(not(test))]
use std::old_io::File;

fn main() {

	println!("Finding ECB...");

	if env::args().count() < 2 {
		panic!("Must pass a file to detect")
	}

	let arg = match env::args().nth(1) {
		Some(s) => s,
		None => panic!("No input argument given")
	};

	let path = Path::new(arg);
	let mut file = BufferedReader::new(File::open(&path));
	let lines: Vec<_> = file.lines()
		.map(|x| x.unwrap().trim().to_string())
		.collect();

	let mut block_set = HashSet::new();
	for line in lines {
		block_set.clear();
		let blocks = line.len() / 32;
		let mut found = false;
		for index in 0..blocks - 1 {
			let block = line[index * 32..(index + 1) * 32].to_string();
			if block_set.contains(&(block)) {
				println!("duplicate block: {:?}", block);
				found = true;
			} else {
				block_set.insert(block);
			}
		}
		if found {
			println!("in line: {:?}", line);
		}
	}
}