use std::collections::HashMap;
use crate::conversion;

const ENG_FREQ: [f32; 26] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,  // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,  // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,  // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074                     // V-Z
];

pub fn get_letter_count(s: &[u8], len: &mut u32) -> Result<HashMap<u8, u32>, &'static str> {
    let mut map = HashMap::new();
    *len = s.len() as u32;

    for i in 0..=25 {
        map.entry(i).or_insert(0);
    }
    for i in s {
        if i >= &b'A' && i <= &b'Z' {
            let count = map.entry(i-&b'A').or_insert(0);
            *count += 1;
        }
        else if i >= &b'a' && i <= &b'z' {
            let count = map.entry(i-&b'a').or_insert(0);
            *count += 1;
        } else if i >= &32 && i <= &126 { *len -= 1; }
        else if i == &9 || i == &10 || i == &13 { *len -= 1; }
        else {
            return Err("Impossible Charactere sequence");
        }

    }
    Ok(map)
}

pub fn xor(v: &[u8], v2: &[u8]) -> Result<Vec<u8>, &'static str> {
	if v.len() != v2.len() {
		return Err("length not equal of params");
	}

    let mut s = Vec::new();
    for i in 0..v.len() {
        s.push(v[i] ^ v2[i])
    }
    Ok(s)
}

pub fn single_xor(v: &[u8], x: u8) -> Vec<u8> {
	v.into_iter().map(|y| y ^ x).collect()
}

pub fn repeating_key_xor(v: &[u8], key: &[u8]) -> Vec<u8> {
	let mut res = Vec::new();

	let mut j = 0;
	for i in v {
		res.push(i ^ key[j % key.len()]);
		j += 1;
	}
	res
}

pub fn get_chi2(v: &[u8]) -> Result<f32, &'static str> {
    let mut len = 0;
    let count = get_letter_count(v, &mut len)?;
    let mut chi2 = 0.;

    for i in 0..=25 {
        let expected = len as f32 * ENG_FREQ[i as usize];
        let diff = count[&i] as f32 - expected;
        chi2 += diff*diff / expected;
    }

    Ok(chi2)
}

pub fn get_bhattacharyya(s: &[u8]) -> Result<f32, &'static str> {
    let mut len = 0;
    let c = get_letter_count(s, &mut len)?;

    Ok(c.into_iter().map(|x| f32::sqrt(ENG_FREQ[x.0 as usize]*x.1 as f32/len as f32)).sum())
}

pub fn hamming_distance(s: &[u8], s2: &[u8]) -> u32 {
	s.iter().zip(s2.iter()).fold(0, |acc, (a, b)| (a ^ b).count_ones() + acc)
}


#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn distance() {
		let s = "this is a test";
		let s2 = "wokka wokka!!!";

		let result = 37;
		assert_eq!(result, hamming_distance(s.as_bytes(), s2.as_bytes()));
	}

	#[test]
	fn xor_len() {
		let v1 = vec![0x58, 0x65, 0xde, 0xa5, 0xff];
		let v2 = vec![0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x73, 0x6f,
					  0x6d, 0x65, 0x20, 0x74, 0x65, 0x78, 0x74, 0x21];
		let result = Err("length not equal of params");

		assert_eq!(result, xor(&v1, &v2));
	}

	#[test]
	fn xor_pass() {
		let v1 = vec![0x58, 0x65, 0xde, 0xa5, 0xff];
		let v2 = vec![0x74, 0x68, 0x69, 0x73, 0x20];
		let result = vec![0x2c, 0xd, 0xb7, 0xd6, 0xdf];

		assert_eq!(Ok(result), xor(&v1, &v2));
	}

    #[test]
    fn letter_count() -> Result<(), &'static str> {
        let text = "most of these exercises should take only a couple minutes";
        let mut len = 0;
        let exp_len: u32 = (text.len() - 9) as u32;
        let mut result: HashMap<u8, u32> = HashMap::new();
        result.insert(0, 2);
        result.insert(1, 0);
        result.insert(2, 2);
        result.insert(3, 1);
        result.insert(4, 8);
        result.insert(5, 1);
        result.insert(6, 0);
        result.insert(7, 2);
        result.insert(8, 2);
        result.insert(9, 0);
        result.insert(10, 1);
        result.insert(11, 3);
        result.insert(12, 2);
        result.insert(13, 2);
        result.insert(14, 5);
        result.insert(15, 1);
        result.insert(16, 0);
        result.insert(17, 1);
        result.insert(18, 6);
        result.insert(19, 4);
        result.insert(20, 3);
        result.insert(21, 0);
        result.insert(22, 0);
        result.insert(23, 1);
        result.insert(24, 1);
        result.insert(25, 0);

        assert_eq!(result, get_letter_count(text.as_bytes(), &mut len)?);
        assert_eq!(exp_len, len);
        Ok(())
	}
	
	#[test]
	fn repeat_xor() {

		let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
		let key = "ICE";

		let result = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

		assert_eq!(result, conversion::hex_encode(&repeating_key_xor(text.as_bytes(), key.as_bytes())));
	}

	
    
}

