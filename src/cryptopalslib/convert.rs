use std::ascii::AsciiExt;
use std::str;

/// Given a string containing hexadecimal digits, returns a string containing
/// the Base64 representation of those hex digits.
///
/// # Panics
///
/// Panics when given a hex string of odd length.
/// Panics when the resulting bytes can't be converted to a string.
///
/// # Examples
///
/// ```
/// let hex = "4D616E";
/// let base64 = cryptopalslib::convert::hex_to_base64(hex);
/// assert_eq!(base64, "TWFu");
/// ```
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

/// Given a string containing Base64-encoded data, returns a string containing
/// the hexadecimal representation of that data.
///
/// # Panics
///
/// Panics when the resulting bytes can't be converted to a string.
/// Panics when given a string ending with more than two equals signs.
///
/// # Examples
///
/// ```
/// let base64 = "TWFu";
/// let hex = cryptopalslib::convert::base64_to_hex(base64);
/// assert_eq!(hex, "4d616e");
/// ```
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

/// Converts a vector of Base64-encoded strings into a single string of
/// hexadecimal digits.
///
/// Useful when converting a file containing Base64 data with newlines in it.
///
/// # Panics
///
/// Panics when the resulting bytes can't be converted to a string.
/// Panics when given a string ending with more than two equals signs.
///
/// # Examples
///
/// ```
/// let base64 = vec!("TWFu".to_string(), "TWFu".to_string());
/// let hex = cryptopalslib::convert::base64_lines_to_hex(&base64);
/// println!("{}", hex);
/// ```
pub fn base64_lines_to_hex(lines: &Vec<String>) -> String {
	let mut output = String::new();
	for line in lines {
		output.push_str(&base64_to_hex(&line.trim()));
	}
	output
}

/// Converts a Base64 ASCII character to the value it represents.
///
/// For example, when given 65, representing "A", returns 0.
/// Returns None when given 61 (which represents an equals sign in ASCII).
///
/// # Panics
///
/// Panics when given an ASCII character that isn't in the Base64 range.
fn base64_ascii_to_index(ascii: u8) -> Option<u8> {
	match ascii {
		65...90 => Some(ascii - 65), // uppercase
		97...122 => Some(ascii - 71), // lowercase
		48...57 => Some(ascii + 4), // numbers
		43 => Some(ascii + 19), // +
		47 => Some(ascii + 16), // /
		// equals means empty byte. need some way of
		// differentiating from 'A', so we'll use the Option type
		61 => None, // =

		_ => panic!("Not in base64 range")
	}
}

/// Converts a string of hex digits into a vector of 8-bit integers
/// representing characters.
///
/// Every two hex digits will be converted to one 8-bit integer.
///
/// # Examples
///
/// ```
/// let hex = "4D616E";
/// let nums = cryptopalslib::convert::hex_string_to_decimal_pairs(hex);
/// assert_eq!(nums, vec!(77, 97, 110));
/// ```
pub fn hex_string_to_decimal_pairs(string: &str) -> Vec<u8> {
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

/// Converts a vector of 8-bit integers into a string containing hex digits.
///
/// Every 8-bit integer will be split into two hex digits.
///
/// # Examples
///
/// ```
/// let nums = vec!(77, 97, 110);
/// let hex = cryptopalslib::convert::decimals_to_hex_string(nums);
/// assert_eq!(hex, "4d616e");
/// ```
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

/// Converts the ASCII representation of a hexadecimal character into the
/// number it represents.
///
/// # Examples
/// ```
/// let hex = 48;
/// let decimal = cryptopalslib::convert::hex_char_to_decimal(hex);
/// assert_eq!(0, decimal);
/// ```
pub fn hex_char_to_decimal(character: u8) -> u8 {
	// if x is in the ascii range for numbers, subtract 48,
	// which is '0' in ascii. Otherwise, it should be a lowercase letter.
	// 'a' is 97, so subtract 87.
	match character < 58 {
		true => character - 48,
		false => character - 87
	}
}

/// Converts a number from 0-16 to an ASCII-encoded hex character.
///
/// # Panics
///
/// Panics if the number given isn't in the range 0-16.
///
/// # Examples
/// ```
/// let num = 0;
/// let hex = cryptopalslib::convert::decimal_to_hex_char(num);
/// assert_eq!(48, hex);
/// ```
pub fn decimal_to_hex_char(decimal: u8) -> u8 {
	match decimal {
		0...9 => decimal + 48,
		10...15 => decimal + 87,
		_ => panic!("Not a valid hex char")
	}
}

/// Converts a 24-bit number into a vector of 4 base64-indexed values.
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