use std::ascii::AsciiExt;
use std::str;

pub fn hex_to_base64(hex_string: &str) -> String {
	if hex_string.len() % 2 != 0 {
		panic!("Hex string length should be even.");
	}

	let hex_lower = hex_string.to_ascii_lowercase();

	let bytes = hex_lower.as_bytes();

	let mut shift: i8 = 20;
	let mut string_group = 0;
	let mut output_vec = vec!();

	for &x in bytes.iter() {

		// if x is in the ascii range for numbers, subtract 48,
		// which is '0' in ascii. Otherwise, it should be a lowercase letter.
		// 'a' is 97, so subtract 87.
		let byte = match x < 58 {
			true => x - 48,
			false => x - 87
		};

		let test = (byte as usize) << shift;
		debug!("byte: {:?}, test: {:?}, shift: {:?}", byte, test, shift);
		string_group += test;

		shift -= 4;

		if shift < 0 {
			output_vec.append(&mut convert_string_group(string_group, 3));
			shift = 20;
			string_group = 0;
		}

	}

	if string_group != 0 {
		// assuming an even number of hex chars, shift will either be 12 or 4.
		// if 12, we have 1 significant byte, and if 4 we have 2.
		let significant_bytes: usize = (2 - (shift / 8)) as usize;
		output_vec.append(&mut convert_string_group(string_group, significant_bytes));
	}

	// turn the byte vector into a string
	match str::from_utf8(&output_vec) {
	    Ok(v) => {
	        debug!("output string: {:?}", v);
	        return v.to_string();
	    }
	    Err(e) => {
	        panic!("error parsing string: {:?}", e);
    	}
	}
}

pub fn base64_to_hex(input: &str) -> String {
	let bytes = input.as_bytes();
	let mut nums = vec!();
	for index in (0..bytes.len()).step_by(4) {
		let slice = &bytes[index..index+4];

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

	let hex: Vec<u8> = nums.iter().map(|&x| decimal_to_hex_char(x)).collect();

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

		_ => panic!("Not in base64 range")
	}
}

pub fn hex_string_to_decimal_pairs(string: &String) -> Vec<u8> {
	let string_lower = string.to_ascii_lowercase();
	let bytes = string_lower.as_bytes();

	let mut decimal_values = vec!();
	let mut tick = 1;
	let mut current_byte = 0;

	for &x in bytes.iter() {
		current_byte += hex_char_to_decimal(x) * 16u8.pow(tick);
		tick = tick ^ 1;
		if tick == 1 {
			decimal_values.push(current_byte);
			current_byte = 0;
		}
	}

	decimal_values
}

pub fn decimals_to_hex_string(decimals: Vec<u8>) -> String {
	let mut converted_string: Vec<u8> = vec!();
	for num in decimals {
		if num < 16 {
			converted_string.push(decimal_to_hex_char(0));
			converted_string.push(decimal_to_hex_char(num));
		} else {
			converted_string.push(decimal_to_hex_char(num / 16));
			converted_string.push(decimal_to_hex_char(num % 16));
		}
	}

	// turn the byte vector into a string
	match str::from_utf8(&converted_string) {
	    Ok(v) => {
	        return v.to_string();
	    }
	    Err(_) => { panic!("Error converting numbers to string") }
	}
}

pub fn hex_char_to_decimal(character: u8) -> u8 {
	// if x is in the ascii range for numbers, subtract 48,
	// which is '0' in ascii. Otherwise, it should be a lowercase letter.
	// 'a' is 97, so subtract 87.
	match character < 58 {
		true => character - 48,
		false => character - 87
	}
}

pub fn decimal_to_hex_char(decimal: u8) -> u8 {
	match decimal {
		0...9 => decimal + 48,
		10...15 => decimal + 87,
		_ => panic!("Not a valid hex char")
	}
}

fn convert_string_group(group: usize, significant_bytes: usize) -> Vec<u8> {
	let mut converted_vec = vec!();
	let octets = vec!(
		((group & (63 * 64usize.pow(3))) >> 18) as u8,
		((group & (63 * 64usize.pow(2))) >> 12) as u8,
		((group & (63 * 64)) >> 6) as u8,
		(group & 63) as u8,
	);
	debug!("group: {:?}", group);
	debug!("octets: {:?}", octets);
	for &octet in octets.iter() {
		converted_vec.push(
			match octet {
				0...25 => octet + 65, // uppercase
				26...51 => octet + 71, // lowercase
				52...61 => octet - 4, // numbers
				62 => 43, // +
				63 => 47, // /
				_ => panic!("Not a valid octet")
			}
		);
	}

	// if we have less than 3 significant bytes, truncate insignificant chars
	// and replace them with '=' characters
	if significant_bytes < 3 {
		converted_vec.truncate(significant_bytes + 1);
		converted_vec.resize(4, 61);
	}

	converted_vec
}

#[cfg(test)]
mod test {
	#[test]
	fn ff_to_base64() {
		let output = super::hex_to_base64("ff");
		assert_eq!(output, "/w==");
	}

	#[test]
	fn ffff_to_base64() {
		let output = super::hex_to_base64("ffff");
		assert_eq!(output, "//8=");
	}

	#[test]
	fn uppercase_to_base64() {
		let output = super::hex_to_base64("4D616E");
		assert_eq!(output, "TWFu");
	}

	#[test]
	fn base64_one_equals_to_hex() {
		let output = super::base64_to_hex("oSNFZ4k=");
		assert_eq!(output, "a123456789");
	}

	#[test]
	fn base64_two_equals_to_hex() {
		let output = super::base64_to_hex("EjRWeJq83g==");
		assert_eq!(output, "123456789abcde");
	}

	#[test]
	fn man_to_hex() {
		let output = super::base64_to_hex("TWFu");
		assert_eq!(output, "4d616e");
	}
}