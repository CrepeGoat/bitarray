use std::convert::{From, Into};


#[derive(Debug, Clone, Copy)]
struct BitArray {
	array: u64,
	left_margin: u64,
	right_margin: u64,
	left_align: bool,
}

impl From<BitArray> for u64 {
	fn from(ba: BitArray) -> u64 {
		(ba.array & (!0u64 >> ba.left_margin)) >> ba.right_margin
	}
}

impl BitArray {
	pub fn length(&self) -> u64 {
		64u64 - (self.left_margin + self.right_margin)
	}

	fn mask(&self) -> u64 {
		(!0u64 >> self.left_margin) & (!0u64 << self.right_margin)
	}
	}

	fn trim_to(self, new_len: u64) -> BitArray {
		if new_len >= self.length() {
			return self;
		}
		
		Self {
			array: self.array,
			left_margin: 
				if self.left_align {self.left_margin}
				else {64-self.right_margin-new_len},
			right_margin:
				if !self.left_align {self.right_margin}
				else {64-self.left_margin-new_len},
			left_align: self.left_align,
		}
	}

	fn apply_binary<F>(func: F, bits1: BitArray, bits2: BitArray) -> BitArray
		where F: Fn(u64, u64) -> u64
	{
		let bits1 = bits1.trim_to(bits2.length());
		let bits2 = bits2.trim_to(bits1.length());

		BitArray{
			array: func(bits1.into(), bits2.into()),
			left_margin: 64-bits1.length(),
			right_margin: 0u64,
			left_align: bits1.left_align || bits2.left_align,
		}
	}

}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn into_u64() {
		let bitarray = BitArray{
			array: 0x0000f0000000ff00,
			left_margin: 24,
			right_margin: 4,
			left_align: false,
		};

		assert_eq!(u64::from(bitarray), 0xff0u64);
	}

	#[test]
	fn mask() {
		let bitarray = BitArray{
			array: 0u64,
			left_margin: 64-6,
			right_margin: 3,
			left_align: false,
		};
		assert_eq!(bitarray.mask(), 0b111000)
	}	

	#[test]
	fn trim_to() {
		assert_eq!(
			u64::from(BitArray{
				array: 0x0ff000000000ff0,
				left_margin: 0,
				right_margin: 0,
				left_align: false,
			}.trim_to(60)),
			0xff000000000ff0,
		);
		assert_eq!(
			u64::from(BitArray{
				array: 0x0ff000000000ff0,
				left_margin: 0,
				right_margin: 0,
				left_align: true,
			}.trim_to(60)),
			0x0ff000000000ff,
		);
	}
}

