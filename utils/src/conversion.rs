pub fn to_b64(bytes: &[u8]) -> String {
	let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

	let mut b64 = String::new();
	let bytes_len = bytes.len();
	let paddings = bytes_len % 3;

	let mut reste = 0;
	let mut r: usize = 0;
	for i in bytes {
		let p: usize;
		
		reste += 2;
		p = r + (i >> reste) as usize;
		r = ((i & (u8::pow(2,reste)-1)) << (6 - reste)) as usize;
		b64.push_str(&charset[p..(p+1)]);
		if reste == 6 {
			reste = 0;
			b64.push_str(&charset[r..(r+1)]);
			r = 0;
		}
	}
	if r != 0 {
		b64.push_str(&charset[r..(r+1)]);
	}
	if paddings != 0 {
	
		for _ in paddings..3 {
			b64.push('=');
		}
	}
	b64
}

pub fn from_b64(s: &str) -> Result<Vec<u8>, &'static str> {
	let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	let mut v = Vec::new();

	let mut reste = 0;
	let mut p = 0;
	for i in s.chars().filter(|&x| x != '=') {
		match charset.find(i) {
			None =>	return Err("Invalid base64 sequence"),
			Some(x) => {
				if reste == 0 {
					p = x as u8;
				} else {
					v.push(p << reste | x as u8 >> (6-reste));
					p = x as u8 & (u8::pow(2, 6-reste)-1);
				}
				reste = (reste + 2) % 8;
			}
		}
	}

	Ok(v)
}

pub fn hex_decode(s: &str) -> Option<Vec<u8>> {
	let mut v: Vec<u8> = Vec::new();
	if s.len() % 2 != 0 {
		return None;
	}
	let mut id = 0;
	let mut nb = 0;
	for i in s.chars() {
		let n = i.to_digit(16);
		id += 1;
		match n {
			Some(x) => {
				if id % 2 != 0 {
					nb = x * 16;
				} else {
					nb += x;
					v.push(nb as u8);
				}
			},
			None => return None,
		}
	}
	Some(v)
}

pub fn hex_encode(s: &[u8]) -> String {
    let mut st = String::new();

    for i in s {
        st.push_str(&format!("{:02x?}", i));
    }
    st
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn b64_encode() {
		let bytes = "test strings";
		let b64 = "dGVzdCBzdHJpbmdz";

		assert_eq!(b64.to_string(), to_b64(bytes.as_bytes()));

		let bytes = "another strings";
		let b64 = "YW5vdGhlciBzdHJpbmdz";

		assert_eq!(b64.to_string(), to_b64(bytes.as_bytes()));
	
		let bytes = "the last strings";
		let b64 = "dGhlIGxhc3Qgc3RyaW5ncw==";

		assert_eq!(b64.to_string(), to_b64(bytes.as_bytes()));
	}

	#[test]
	fn b64_decode() {
		let bytes = "test strings";
		let b64 = "dGVzdCBzdHJpbmdz";

		assert_eq!(Ok(bytes.as_bytes().to_vec()), from_b64(b64));
	
		let bytes = "another strings";
		let b64 = "YW5vdGhlciBzdHJpbmdz";

		assert_eq!(Ok(bytes.as_bytes().to_vec()), from_b64(b64));
	
		let bytes = "the last strings";
		let b64 = "dGhlIGxhc3Qgc3RyaW5ncw==";

		assert_eq!(Ok(bytes.as_bytes().to_vec()), from_b64(b64));
	}

	#[test]
	fn decode_hex() {
		let hex = "54f2a2df15";
		let result = vec![0x54, 0xf2, 0xa2, 0xdf, 0x15];

		assert_eq!(Some(result), hex_decode(hex));
	
		let hex = "654d8da45d7d9a3ef";
		let result = None;

		assert_eq!(result, hex_decode(hex));
	
		let hex = "3215thisissometext";
		let result = None;

		assert_eq!(result, hex_decode(hex));
	}

	#[test]
	fn encode_hex() {
		let bytes = "this is some text!".as_bytes();
		let result = "7468697320697320736f6d65207465787421";

		assert_eq!(result.to_string(), hex_encode(bytes));
	
		let bytes = vec![0x58, 0x65, 0xde, 0xa5, 0xff, 0x0a];
		let result = "5865dea5ff0a";

		assert_eq!(result.to_string(), hex_encode(&bytes));
	}
}