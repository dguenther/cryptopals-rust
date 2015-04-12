// Detect AES in ECB mode
// In this file (8.txt) are a bunch of hex-encoded ciphertexts.

// One of them has been encrypted with ECB.

// Detect it.

// Remember that the problem with ECB is that it is stateless and
// deterministic; the same 16 byte plaintext block will always
// produce the same 16 byte ciphertext.


use std::collections::HashSet;

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

	println!("Finding ECB...");

	if env::args().count() < 2 {
		panic!("Must pass a file to detect")
	}

	let arg = match env::args().nth(1) {
		Some(s) => s,
		None => panic!("No input argument given")
	};

	let path = Path::new(&arg);
	let file = BufReader::new(File::open(&path).unwrap());
	let lines: Vec<String> = file.lines()
		.map(|x| x.unwrap().trim().to_string())
		.collect();

	for line in lines {
		let duplicates = detect_ecb_in_line(&line);
		if duplicates.len() > 0 {
			for block in duplicates {
				println!("duplicate block: {:?}", block);
			}
			println!("in line: {:?}", line);
		}
	}
}

fn detect_ecb_in_line(line: &str) -> Vec<String> {
	let blocks = line.len() / 32;
	let mut block_set = HashSet::new();
	let mut duplicates = vec!();
	for index in 0..blocks - 1 {
		let block = line[index * 32..(index + 1) * 32].to_string();
		if block_set.contains(&block) {
			duplicates.push(block);
		} else {
			block_set.insert(block);
		}
	}
	duplicates
}

#[cfg(test)]
mod set1challenge8 {

	#[test]
	fn detect_ecb_in_line() {
		let line = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
		let duplicates = super::detect_ecb_in_line(&line);
		assert_eq!(duplicates.len(), 3);
		assert_eq!(duplicates[0], "08649af70dc06f4fd5d2d69c744cd283");
		assert_eq!(duplicates[1], "08649af70dc06f4fd5d2d69c744cd283");
		assert_eq!(duplicates[2], "08649af70dc06f4fd5d2d69c744cd283");
	}

	#[test]
	fn dont_detect_ecb_in_line() {
		let line = "b148a13d9a04ba6ef17afb0e25a6c91a454ec0eded513a567a9824dd3cd16770f4c1dae48854c2cf557139640c1cd121cac974f74f7001aa4927f6bdb4e0fa73676855df520e2af6ac785a420e43e829fa4e77e5de386d58404d42aa57bf56467f98322275df9f1a72fbb03fa8ea8b84356bbcd7159c59ef283a1aec240ef5d25df6e2aaaea36826beb03b0826d4abc8f22837812dafe6c9623517471fc653b9";
		let duplicates = super::detect_ecb_in_line(&line);
		assert_eq!(duplicates.len(), 0);

	}
}