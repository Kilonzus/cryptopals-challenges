use std::collections::HashMap;
use std::convert::TryInto;


fn main() {
    let tc_one = hexDecode("48656c6c6f");

    for val in &tc_one {
    	println!("{}", val);
    }
}

fn hexDecode(s: &str) -> Vec<i32> {
	let hex_digits = "0123456789abcdef";
	let mut numArr = Vec::new();

	let mut hm: HashMap<char, usize> = HashMap::new();
	for (i,c) in hex_digits.chars().enumerate() {
		hm.insert(c, i);
	}

	for (i,c) in s.chars().enumerate() {
		if i % 2 != 0 {
			let tmpByte: u8 = s.as_bytes()[i-1];
			let tmpChar: char = tmpByte as char;
			numArr.push((hm[&tmpChar] << 4 | hm[&c]).try_into().unwrap());
		}
	}

	if s.len() % 2 != 0 {
		let tmpByte: u8 = s.as_bytes()[s.len()-1];
		let tmpChar: char = tmpByte as char;
		numArr.push((hm[&tmpChar] << 4).try_into().unwrap());
	}

	return numArr;
}
/*
fn base64Encode(arr: Vec<i32>) -> String {
	let b64_digits = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

	let mut hm: HashMap<char, usize> = HashMap::new();
	for (i,c) in b64_digits.chars().enumerate() {
		hm.insert(c, i);
	}


}
*/
fn chunk(s: &str, n: i32) -> Vec<char> {
	let mut charChunk = Vec::new();
	for c in s.chars() {
		charChunk.push(c);
		if charChunk.len() == (n).try_into().unwrap() {
			return charChunk;
		}
	}
	return charChunk;
}