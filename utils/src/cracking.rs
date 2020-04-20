use crate::{conversion, basics::*};

pub struct Solver<T>
    where T: Fn(&[u8]) -> Result<f32, &'static str>
{
    pub correct: String,
	pub proba: f32,
	pub key: u8,
	calcule: T
}

impl<T> Solver<T>
    where T: Fn(&[u8]) -> Result<f32, &'static str>
{
    pub fn guess_single_xor(v: &[u8], calcule: T) -> Solver<T> {
		use conversion::to_b64;

        let mut correct = Vec::new();
		let mut proba = 0.;
		let mut key = 0;
        for i in 0..=255 {
            let xored = single_xor(v, i);
            if let Ok(x) = calcule(&xored) {
                if x > proba {
					proba = x;
					key = i;
                    correct = xored;
                }
            }
        }
        
        Solver {
            correct:to_b64(&correct),
            proba,
			calcule,
			key,
        }
    }

}

pub fn find_repeated_xor_key(m: &[u8], key_size: usize) -> Vec<u8> {


	let mut blocks: Vec<Vec<u8>> = vec!();
	let chunks = m.chunks(key_size);
	for chunk in chunks {
		for (i, &b) in chunk.iter().enumerate() {
			if blocks.len() <= i {
				blocks.push(vec!(b));
			} else if let Some(block) = blocks.get_mut(i) {
				block.push(b);
			}
		}
	}
	blocks.iter().map(|i| Solver::guess_single_xor(i, get_bhattacharyya).key).collect()
}

pub fn guess_key_size(v: &[u8]) -> Vec<usize> {

	//guessing the key size (the smallest distance is probably the key size)
	let mut best = (vec!(8.0), vec!(v.len()));
	let threshold = 0.1;

	for key_size in 2..=40 {
		let mut chunks = v.chunks(key_size);
		let mut total_diff = 0.0;
		let mut pairs = 0;
		while let (Some(a), Some(b)) = (chunks.next(), chunks.next()) {
			pairs += 1;
			total_diff += hamming_distance(a, b)as f32;
		}

		let diff = (total_diff / pairs as f32) / key_size as f32;
		let bestavg = best.0.iter().fold(0.0, |a, &x| a + x) / best.0.len() as f32;
		let percdiff = ((bestavg - diff) / bestavg).abs();

		if percdiff < threshold {
			best.0.push(diff);
			best.1.push(key_size);
		} else if diff < bestavg {
			best = (vec!(diff), vec!(key_size));
		}
	}

	best.1
}


#[test]
fn guess_repeated_xor() {
    let quote = "Alice was beginning to get very tired of sitting by her sister on the bank, and
    of having nothing to do: once or twice she had peeped into the book her sister was reading,
    but it had no pictures or conversations in it, `and what is the use of a book,' thought
    Alice `without pictures or conversation?' So she was considering in her own mind (as well
    as she could, for the hot day made her feel very sleepy and stupid), whether the pleasure
    of making a daisy-chain would be worth the trouble of getting up and picking the daisies,
    when suddenly a White Rabbit with pink eyes ran close by her.  There was nothing so very
    remarkable in that; nor did Alice think it so very much out of the way to hear the Rabbit
    say to itself, `Oh dear! Oh dear! I shall be late!' (when she thought it over afterwards,
    it occurred to her that she ought to have wondered at this, but at the time it all seemed
    quite natural); but when the Rabbit actually took a watch out of its waistcoat-pocket, and
    looked at it, and then hurried on, Alice started to her feet, for it flashed across her
    mind that she had never before seen a rabbit with either a waistcoat-pocket, or a watch to
    take out of it, and burning with curiosity, she ran across the field after it, and
    fortunately was just in time to see it pop down a large rabbit-hole under the hedge.";
    let msg = quote.as_bytes();

    let key = [0x11, 0x23, 0x3f, 0xf9, 0x82, 0x12, 0x99, 0x22];
    let cipher = repeating_key_xor(&msg, &key);
    assert!(guess_key_size(&cipher).contains(&key.len()));
}