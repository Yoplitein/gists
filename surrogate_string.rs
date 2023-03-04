#[derive(Clone, Debug)]
pub struct SurrogateString(String);

impl SurrogateString {
	unsafe fn inner(&self) -> &String {
		&self.0
	}
	
	unsafe fn inner_mut(&mut self) -> &mut String {
		&mut self.0
	}
	
	unsafe fn into_inner(self) -> String {
		self.0
	}
}

impl From<&[u8]> for SurrogateString {
	fn from(mut bytes: &[u8]) -> Self {
		let mut res = String::new();
		loop {
			match std::str::from_utf8(bytes) {
				Ok(str) => {
					res.push_str(str);
					break;
				},
				Err(err) => {
					let (head, tail) = bytes.split_at(err.valid_up_to());
					res.push_str(unsafe { std::str::from_utf8_unchecked(head) });

					let mut done = false;
					let invalidBytes = if let Some(invalidLen) = err.error_len() {
						bytes = &bytes[head.len() + invalidLen ..];
						&tail[.. invalidLen]
					} else {
						done = true;
						tail
					};
					res.extend(
						invalidBytes
							.iter()
							.map(|&byte| unsafe { char::from_u32_unchecked(0xDC00 | byte as u32) }),
					);

					if done {
						break;
					}
				},
			}
		}
		Self(res)
	}
}

impl From<SurrogateString> for Vec<u8> {
	fn from(str: SurrogateString) -> Self {
		let mut res = Vec::new();
		let mut tmp = [0u8; 4];
		for char in str.0.chars() {
			let charVal = char as u32;
			if charVal & 0xFFFF_DC00 == 0xDC00 {
				res.push((charVal & 0xFF) as u8);
			} else {
				let len = char.encode_utf8(&mut tmp).len();
				res.extend(&tmp[0 .. len]);
			}
		}
		res
	}
}

#[test]
fn test_surrogate_string() {
	let bstr = b"test\xFF \xCE\x91".as_slice();
	let sstr = SurrogateString::from(bstr);
	assert_eq!(sstr.0.as_bytes(), b"test\xED\xB3\xBF \xCE\x91");
	assert_eq!(Vec::from(sstr), bstr);
}
