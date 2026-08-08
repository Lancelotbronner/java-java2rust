use java::util::UUID;

pub struct Conversion;

impl Conversion {
	static TTTT: &[bool] = vec![true, true, true, true, ]
	;

	static FTTT: &[bool] = vec![false, true, true, true, ]
	;

	static TFTT: &[bool] = vec![true, false, true, true, ]
	;

	static FFTT: &[bool] = vec![false, false, true, true, ]
	;

	static TTFT: &[bool] = vec![true, true, false, true, ]
	;

	static FTFT: &[bool] = vec![false, true, false, true, ]
	;

	static TFFT: &[bool] = vec![true, false, false, true, ]
	;

	static FFFT: &[bool] = vec![false, false, false, true, ]
	;

	static TTTF: &[bool] = vec![true, true, true, false, ]
	;

	static FTTF: &[bool] = vec![false, true, true, false, ]
	;

	static TFTF: &[bool] = vec![true, false, true, false, ]
	;

	static FFTF: &[bool] = vec![false, false, true, false, ]
	;

	static TTFF: &[bool] = vec![true, true, false, false, ]
	;

	static FTFF: &[bool] = vec![false, true, false, false, ]
	;

	static TFFF: &[bool] = vec![true, false, false, false, ]
	;

	static FFFF: &[bool] = vec![false, false, false, false, ]
	;

	pub fn binary_be_msb0_to_hex_digit(&self, src: &&[bool]) /* thrown(java.lang.IndexOutOfBoundsException | java.lang.IllegalArgumentException) */ -> u16 {
		return org::apache::commons::lang3::conversion::Conversion::binary_be_msb0_to_hex_digit(src, 0)?;
	}

	pub fn binary_be_msb0_to_hex_digit(&self, src: &&[bool], src_pos: i32) /* thrown(java.lang.IndexOutOfBoundsException | java.lang.IllegalArgumentException) */ -> u16 {
		// JDK 9: Objects.checkIndex(int index, int length)
		if Integer::compareUnsigned(src_pos, src.length) >= 0 {
			// Throw the correct exception
			if src.length == 0 {
				return Err(IllegalArgumentException::new("Cannot convert an empty array."));
			}
			return Err(IndexOutOfBoundsException::new(src_pos + " is not within array length " + src.length));
		}
		// Little-endian bit 0 position
		/* final */ let pos: i32 = src.length - 1 - src_pos;
		if 3 <= pos && src[pos - 3] {
			if src[pos - 2] {
				if src[pos - 1] {
					return  if src[pos] { 'f' } else { 'e' };
				}
				return  if src[pos] { 'd' } else { 'c' };
			}
			if src[pos - 1] {
				return  if src[pos] { 'b' } else { 'a' };
			}
			return  if src[pos] { '9' } else { '8' };
		}
		if 2 <= pos && src[pos - 2] {
			if src[pos - 1] {
				return  if src[pos] { '7' } else { '6' };
			}
			return  if src[pos] { '5' } else { '4' };
		}
		if 1 <= pos && src[pos - 1] {
			return  if src[pos] { '3' } else { '2' };
		}
		return  if src[pos] { '1' } else { '0' };
	}

	pub fn binary_to_byte(&self, src: &&[bool], src_pos: i32, dst_init: i8, dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i8 {
		if src.length == 0 && src_pos == 0 || 0 == n_bools {
			return dst_init;
		}
		if n_bools - 1 + dst_pos >= Byte::SIZE {
			return Err(IllegalArgumentException::new("nBools - 1 + dstPos >= 8"));
		}
		let out: i8 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + dst_pos;
					/* final */ let bits: i32 = ( if src[i + src_pos] { 1 } else { 0 }) << shift;
					/* final */ let mask: i32 = 0x1 << shift;
					out = (out & ~mask | bits) as i8;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn binary_to_hex_digit(&self, src: &&[bool]) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		return org::apache::commons::lang3::conversion::Conversion::binary_to_hex_digit(src, 0)?;
	}

	pub fn binary_to_hex_digit(&self, src: &&[bool], src_pos: i32) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		if src.length == 0 {
			return Err(IllegalArgumentException::new("Cannot convert an empty array."));
		}
		if src.length > src_pos + 3 && src[src_pos + 3] {
			if src[src_pos + 2] {
				if src[src_pos + 1] {
					return  if src[src_pos] { 'f' } else { 'e' };
				}
				return  if src[src_pos] { 'd' } else { 'c' };
			}
			if src[src_pos + 1] {
				return  if src[src_pos] { 'b' } else { 'a' };
			}
			return  if src[src_pos] { '9' } else { '8' };
		}
		if src.length > src_pos + 2 && src[src_pos + 2] {
			if src[src_pos + 1] {
				return  if src[src_pos] { '7' } else { '6' };
			}
			return  if src[src_pos] { '5' } else { '4' };
		}
		if src.length > src_pos + 1 && src[src_pos + 1] {
			return  if src[src_pos] { '3' } else { '2' };
		}
		return  if src[src_pos] { '1' } else { '0' };
	}

	pub fn binary_to_hex_digit_msb0_4bits(&self, src: &&[bool]) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		return org::apache::commons::lang3::conversion::Conversion::binary_to_hex_digit_msb0_4bits(src, 0)?;
	}

	pub fn binary_to_hex_digit_msb0_4bits(&self, src: &&[bool], src_pos: i32) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		if src.length > Byte::SIZE {
			return Err(IllegalArgumentException::new("src.length > 8: src.length=" + src.length));
		}
		if src.length - src_pos < 4 {
			return Err(IllegalArgumentException::new("src.length - srcPos < 4: src.length=" + src.length + ", srcPos=" + src_pos));
		}
		if src[src_pos + 3] {
			if src[src_pos + 2] {
				if src[src_pos + 1] {
					return  if src[src_pos] { 'f' } else { '7' };
				}
				return  if src[src_pos] { 'b' } else { '3' };
			}
			if src[src_pos + 1] {
				return  if src[src_pos] { 'd' } else { '5' };
			}
			return  if src[src_pos] { '9' } else { '1' };
		}
		if src[src_pos + 2] {
			if src[src_pos + 1] {
				return  if src[src_pos] { 'e' } else { '6' };
			}
			return  if src[src_pos] { 'a' } else { '2' };
		}
		if src[src_pos + 1] {
			return  if src[src_pos] { 'c' } else { '4' };
		}
		return  if src[src_pos] { '8' } else { '0' };
	}

	pub fn binary_to_int(&self, src: &&[bool], src_pos: i32, dst_init: i32, dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if src.length == 0 && src_pos == 0 || 0 == n_bools {
			return dst_init;
		}
		if n_bools - 1 + dst_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("nBools - 1 + dstPos >= 32"));
		}
		let out: i32 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + dst_pos;
					/* final */ let bits: i32 = ( if src[i + src_pos] { 1 } else { 0 }) << shift;
					/* final */ let mask: i32 = 0x1 << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn binary_to_long(&self, src: &&[bool], src_pos: i32, dst_init: i64, dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		if src.length == 0 && src_pos == 0 || 0 == n_bools {
			return dst_init;
		}
		if n_bools - 1 + dst_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("nBools - 1 + dstPos >= 64"));
		}
		let out: i64 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + dst_pos;
					/* final */ let bits: i64 = ( if src[i + src_pos] { 1 } else { 0 }) << shift;
					/* final */ let mask: i64 = 0x1 << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn binary_to_short(&self, src: &&[bool], src_pos: i32, dst_init: i16, dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i16 {
		if src.length == 0 && src_pos == 0 || 0 == n_bools {
			return dst_init;
		}
		if n_bools - 1 + dst_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("nBools - 1 + dstPos >= 16"));
		}
		let out: i16 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + dst_pos;
					/* final */ let bits: i32 = ( if src[i + src_pos] { 1 } else { 0 }) << shift;
					/* final */ let mask: i32 = 0x1 << shift;
					out = (out & ~mask | bits) as i16;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn byte_array_to_int(&self, src: &&[i8], src_pos: i32, dst_init: i32, dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if src.length == 0 && src_pos == 0 || 0 == n_bytes {
			return dst_init;
		}
		if (n_bytes - 1) * Byte::SIZE + dst_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + dstPos >= 32"));
		}
		let out: i32 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + dst_pos;
					/* final */ let bits: i32 = (0xff & src[i + src_pos]) << shift;
					/* final */ let mask: i32 = 0xff << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn byte_array_to_long(&self, src: &&[i8], src_pos: i32, dst_init: i64, dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		if src.length == 0 && src_pos == 0 || 0 == n_bytes {
			return dst_init;
		}
		if (n_bytes - 1) * Byte::SIZE + dst_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + dstPos >= 64"));
		}
		let out: i64 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + dst_pos;
					/* final */ let bits: i64 = (0xff & src[i + src_pos]) << shift;
					/* final */ let mask: i64 = 0xff << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn byte_array_to_short(&self, src: &&[i8], src_pos: i32, dst_init: i16, dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i16 {
		if src.length == 0 && src_pos == 0 || 0 == n_bytes {
			return dst_init;
		}
		if (n_bytes - 1) * Byte::SIZE + dst_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + dstPos >= 16"));
		}
		let out: i16 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + dst_pos;
					/* final */ let bits: i32 = (0xff & src[i + src_pos]) << shift;
					/* final */ let mask: i32 = 0xff << shift;
					out = (out & ~mask | bits) as i16;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn byte_array_to_uuid(&self, src: &&[i8], src_pos: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::UUID /**/ {
		if src.length - src_pos < 16 {
			return Err(IllegalArgumentException::new("Need at least 16 bytes for UUID"));
		}
		return UUID::new(&org::apache::commons::lang3::conversion::Conversion::byte_array_to_long(src, src_pos, 0, 0, Byte::SIZE)?, &org::apache::commons::lang3::conversion::Conversion::byte_array_to_long(src, src_pos + 8, 0, 0, Byte::SIZE)?);
	}

	pub fn byte_to_binary(&self, src: i8, src_pos: i32, mut dst: &&[bool], mut dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		if 0 == n_bools {
			return dst;
		}
		if n_bools - 1 + src_pos >= Byte::SIZE {
			return Err(IllegalArgumentException::new("nBools -  1 + srcPos >= 8"));
		}
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + src_pos;
					dst[dst_pos + i] = (0x1 & src /* signed */ >> shift) != 0;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn byte_to_hex(&self, src: i8, src_pos: i32, dst_init: &/* Java */ java::lang::String /**/, dst_pos: i32, n_hexs: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if 0 == n_hexs {
			return dst_init;
		}
		if (n_hexs - 1) * 4 + src_pos >= Byte::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + srcPos >= 8"));
		}
		/* final */ let sb: StringBuilder = StringBuilder::new(dst_init);
		let append: i32 = sb.length();
		 {
			let i: i32 = 0;
			while i < n_hexs {
				{
					/* final */ let shift: i32 = i * 4 + src_pos;
					/* final */ let bits: i32 = 0xF & src /* signed */ >> shift;
					if dst_pos + i == append {
						append += 1;
						sb.append(&org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					} else {
						sb.setCharAt(dst_pos + i, &org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					}
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	pub fn hex_digit_msb0_to_binary(&self, hex_char: u16) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		match hex_char {
			'0' =>  {
				return self.FFFF.clone();
			}
			'1' =>  {
				return self.FFFT.clone();
			}
			'2' =>  {
				return self.FFTF.clone();
			}
			'3' =>  {
				return self.FFTT.clone();
			}
			'4' =>  {
				return self.FTFF.clone();
			}
			'5' =>  {
				return self.FTFT.clone();
			}
			'6' =>  {
				return self.FTTF.clone();
			}
			'7' =>  {
				return self.FTTT.clone();
			}
			'8' =>  {
				return self.TFFF.clone();
			}
			'9' =>  {
				return self.TFFT.clone();
			}
			// fall through
			'a' =>  {
			}
			'A' =>  {
				return self.TFTF.clone();
			}
			// fall through
			'b' =>  {
			}
			'B' =>  {
				return self.TFTT.clone();
			}
			// fall through
			'c' =>  {
			}
			'C' =>  {
				return self.TTFF.clone();
			}
			// fall through
			'd' =>  {
			}
			'D' =>  {
				return self.TTFT.clone();
			}
			// fall through
			'e' =>  {
			}
			'E' =>  {
				return self.TTTF.clone();
			}
			// fall through
			'f' =>  {
			}
			'F' =>  {
				return self.TTTT.clone();
			}
			_ =>  {
				return Err(IllegalArgumentException::new("Cannot convert '" + hex_char + "' to a hexadecimal digit"));
			}
		}
	}

	pub fn hex_digit_msb0_to_int(&self, hex_char: u16) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		match hex_char {
			'0' =>  {
				return 0x0;
			}
			'1' =>  {
				return 0x8;
			}
			'2' =>  {
				return 0x4;
			}
			'3' =>  {
				return 0xC;
			}
			'4' =>  {
				return 0x2;
			}
			'5' =>  {
				return 0xA;
			}
			'6' =>  {
				return 0x6;
			}
			'7' =>  {
				return 0xE;
			}
			'8' =>  {
				return 0x1;
			}
			'9' =>  {
				return 0x9;
			}
			// fall through
			'a' =>  {
			}
			'A' =>  {
				return 0x5;
			}
			// fall through
			'b' =>  {
			}
			'B' =>  {
				return 0xD;
			}
			// fall through
			'c' =>  {
			}
			'C' =>  {
				return 0x3;
			}
			// fall through
			'd' =>  {
			}
			'D' =>  {
				return 0xB;
			}
			// fall through
			'e' =>  {
			}
			'E' =>  {
				return 0x7;
			}
			// fall through
			'f' =>  {
			}
			'F' =>  {
				return 0xF;
			}
			_ =>  {
				return Err(IllegalArgumentException::new("Cannot convert '" + hex_char + "' to a hexadecimal digit"));
			}
		}
	}

	pub fn hex_digit_to_binary(&self, hex_char: u16) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		match hex_char {
			'0' =>  {
				return self.FFFF.clone();
			}
			'1' =>  {
				return self.TFFF.clone();
			}
			'2' =>  {
				return self.FTFF.clone();
			}
			'3' =>  {
				return self.TTFF.clone();
			}
			'4' =>  {
				return self.FFTF.clone();
			}
			'5' =>  {
				return self.TFTF.clone();
			}
			'6' =>  {
				return self.FTTF.clone();
			}
			'7' =>  {
				return self.TTTF.clone();
			}
			'8' =>  {
				return self.FFFT.clone();
			}
			'9' =>  {
				return self.TFFT.clone();
			}
			// fall through
			'a' =>  {
			}
			'A' =>  {
				return self.FTFT.clone();
			}
			// fall through
			'b' =>  {
			}
			'B' =>  {
				return self.TTFT.clone();
			}
			// fall through
			'c' =>  {
			}
			'C' =>  {
				return self.FFTT.clone();
			}
			// fall through
			'd' =>  {
			}
			'D' =>  {
				return self.TFTT.clone();
			}
			// fall through
			'e' =>  {
			}
			'E' =>  {
				return self.FTTT.clone();
			}
			// fall through
			'f' =>  {
			}
			'F' =>  {
				return self.TTTT.clone();
			}
			_ =>  {
				return Err(IllegalArgumentException::new("Cannot convert '" + hex_char + "' to a hexadecimal digit"));
			}
		}
	}

	pub fn hex_digit_to_int(&self, hex_char: u16) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		/* final */ let digit: i32 = Character::digit(hex_char, 16);
		if digit < 0 {
			return Err(IllegalArgumentException::new("Cannot convert '" + hex_char + "' to a hexadecimal digit"));
		}
		return digit;
	}

	pub fn hex_to_byte(&self, src: &/* Java */ java::lang::String /**/, src_pos: i32, dst_init: i8, dst_pos: i32, n_hex: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i8 {
		if 0 == n_hex {
			return dst_init;
		}
		if (n_hex - 1) * 4 + dst_pos >= Byte::SIZE {
			return Err(IllegalArgumentException::new("(nHex - 1) * 4 + dstPos >= 8"));
		}
		let out: i8 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_hex {
				{
					/* final */ let shift: i32 = i * 4 + dst_pos;
					/* final */ let bits: i32 = (0xf & org::apache::commons::lang3::conversion::Conversion::hex_digit_to_int(&src.charAt(i + src_pos))?) << shift;
					/* final */ let mask: i32 = 0xf << shift;
					out = (out & ~mask | bits) as i8;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn hex_to_int(&self, src: &/* Java */ java::lang::String /**/, src_pos: i32, dst_init: i32, dst_pos: i32, n_hex: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if 0 == n_hex {
			return dst_init;
		}
		if (n_hex - 1) * 4 + dst_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + dstPos >= 32"));
		}
		let out: i32 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_hex {
				{
					/* final */ let shift: i32 = i * 4 + dst_pos;
					/* final */ let bits: i32 = (0xf & org::apache::commons::lang3::conversion::Conversion::hex_digit_to_int(&src.charAt(i + src_pos))?) << shift;
					/* final */ let mask: i32 = 0xf << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn hex_to_long(&self, src: &/* Java */ java::lang::String /**/, src_pos: i32, dst_init: i64, dst_pos: i32, n_hex: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		if 0 == n_hex {
			return dst_init;
		}
		if (n_hex - 1) * 4 + dst_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + dstPos >= 64"));
		}
		let out: i64 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_hex {
				{
					/* final */ let shift: i32 = i * 4 + dst_pos;
					/* final */ let bits: i64 = (0xf & org::apache::commons::lang3::conversion::Conversion::hex_digit_to_int(&src.charAt(i + src_pos))?) << shift;
					/* final */ let mask: i64 = 0xf << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn hex_to_short(&self, src: &/* Java */ java::lang::String /**/, src_pos: i32, dst_init: i16, dst_pos: i32, n_hex: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i16 {
		if 0 == n_hex {
			return dst_init;
		}
		if (n_hex - 1) * 4 + dst_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + dstPos >= 16"));
		}
		let out: i16 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_hex {
				{
					/* final */ let shift: i32 = i * 4 + dst_pos;
					/* final */ let bits: i32 = (0xf & org::apache::commons::lang3::conversion::Conversion::hex_digit_to_int(&src.charAt(i + src_pos))?) << shift;
					/* final */ let mask: i32 = 0xf << shift;
					out = (out & ~mask | bits) as i16;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn int_array_to_long(&self, src: &&[i32], src_pos: i32, dst_init: i64, dst_pos: i32, n_ints: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		if src.length == 0 && src_pos == 0 || 0 == n_ints {
			return dst_init;
		}
		if (n_ints - 1) * Integer::SIZE + dst_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nInts - 1) * 32 + dstPos >= 64"));
		}
		let out: i64 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_ints {
				{
					/* final */ let shift: i32 = i * Integer::SIZE + dst_pos;
					/* final */ let bits: i64 = (0xffffffff & src[i + src_pos]) << shift;
					/* final */ let mask: i64 = 0xffffffff << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn int_to_binary(&self, src: i32, src_pos: i32, mut dst: &&[bool], mut dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		if 0 == n_bools {
			return dst;
		}
		if n_bools - 1 + src_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("nBools -  1 + srcPos >= 32"));
		}
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + src_pos;
					dst[dst_pos + i] = (0x1 & src /* signed */ >> shift) != 0;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn int_to_byte_array(&self, src: i32, src_pos: i32, mut dst: &&[i8], mut dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i8] {
		if 0 == n_bytes {
			return dst;
		}
		if (n_bytes - 1) * Byte::SIZE + src_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + srcPos >= 32"));
		}
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + src_pos;
					dst[dst_pos + i] = (0xff & src /* signed */ >> shift) as i8;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn int_to_hex(&self, src: i32, src_pos: i32, dst_init: &/* Java */ java::lang::String /**/, dst_pos: i32, n_hexs: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if 0 == n_hexs {
			return dst_init;
		}
		if (n_hexs - 1) * 4 + src_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + srcPos >= 32"));
		}
		/* final */ let sb: StringBuilder = StringBuilder::new(dst_init);
		let append: i32 = sb.length();
		 {
			let i: i32 = 0;
			while i < n_hexs {
				{
					/* final */ let shift: i32 = i * 4 + src_pos;
					/* final */ let bits: i32 = 0xF & src /* signed */ >> shift;
					if dst_pos + i == append {
						append += 1;
						sb.append(&org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					} else {
						sb.setCharAt(dst_pos + i, &org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					}
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	pub fn int_to_hex_digit(&self, nibble: i32) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		/* final */ let c: char = Character::forDigit(nibble, 16);
		if c == Character::MIN_VALUE {
			return Err(IllegalArgumentException::new("nibble value not between 0 and 15: " + nibble));
		}
		return c;
	}

	pub fn int_to_hex_digit_msb0(&self, nibble: i32) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		match nibble {
			0x0 =>  {
				return '0';
			}
			0x1 =>  {
				return '8';
			}
			0x2 =>  {
				return '4';
			}
			0x3 =>  {
				return 'c';
			}
			0x4 =>  {
				return '2';
			}
			0x5 =>  {
				return 'a';
			}
			0x6 =>  {
				return '6';
			}
			0x7 =>  {
				return 'e';
			}
			0x8 =>  {
				return '1';
			}
			0x9 =>  {
				return '9';
			}
			0xA =>  {
				return '5';
			}
			0xB =>  {
				return 'd';
			}
			0xC =>  {
				return '3';
			}
			0xD =>  {
				return 'b';
			}
			0xE =>  {
				return '7';
			}
			0xF =>  {
				return 'f';
			}
			_ =>  {
				return Err(IllegalArgumentException::new("nibble value not between 0 and 15: " + nibble));
			}
		}
	}

	pub fn int_to_short_array(&self, src: i32, src_pos: i32, mut dst: &&[i16], mut dst_pos: i32, n_shorts: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i16] {
		if 0 == n_shorts {
			return dst;
		}
		if (n_shorts - 1) * Short::SIZE + src_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nShorts - 1) * 16 + srcPos >= 32"));
		}
		 {
			let i: i32 = 0;
			while i < n_shorts {
				{
					/* final */ let shift: i32 = i * Short::SIZE + src_pos;
					dst[dst_pos + i] = (0xffff & src /* signed */ >> shift) as i16;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn long_to_binary(&self, src: i64, src_pos: i32, mut dst: &&[bool], mut dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		if 0 == n_bools {
			return dst;
		}
		if n_bools - 1 + src_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("nBools -  1 + srcPos >= 64"));
		}
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + src_pos;
					dst[dst_pos + i] = (0x1 & src /* signed */ >> shift) != 0;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn long_to_byte_array(&self, src: i64, src_pos: i32, mut dst: &&[i8], mut dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i8] {
		if 0 == n_bytes {
			return dst;
		}
		if (n_bytes - 1) * Byte::SIZE + src_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + srcPos >= 64"));
		}
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + src_pos;
					dst[dst_pos + i] = (0xff & src /* signed */ >> shift) as i8;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn long_to_hex(&self, src: i64, src_pos: i32, dst_init: &/* Java */ java::lang::String /**/, dst_pos: i32, n_hexs: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if 0 == n_hexs {
			return dst_init;
		}
		if (n_hexs - 1) * 4 + src_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + srcPos >= 64"));
		}
		/* final */ let sb: StringBuilder = StringBuilder::new(dst_init);
		let append: i32 = sb.length();
		 {
			let i: i32 = 0;
			while i < n_hexs {
				{
					/* final */ let shift: i32 = i * 4 + src_pos;
					/* final */ let bits: i32 = (0xF & src /* signed */ >> shift) as i32;
					if dst_pos + i == append {
						append += 1;
						sb.append(&org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					} else {
						sb.setCharAt(dst_pos + i, &org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					}
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	pub fn long_to_int_array(&self, src: i64, src_pos: i32, mut dst: &&[i32], mut dst_pos: i32, n_ints: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i32] {
		if 0 == n_ints {
			return dst;
		}
		if (n_ints - 1) * Integer::SIZE + src_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nInts - 1) * 32 + srcPos >= 64"));
		}
		 {
			let i: i32 = 0;
			while i < n_ints {
				{
					/* final */ let shift: i32 = i * Integer::SIZE + src_pos;
					dst[dst_pos + i] = (0xffffffff & src /* signed */ >> shift) as i32;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn long_to_short_array(&self, src: i64, src_pos: i32, mut dst: &&[i16], mut dst_pos: i32, n_shorts: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i16] {
		if 0 == n_shorts {
			return dst;
		}
		if (n_shorts - 1) * Short::SIZE + src_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nShorts - 1) * 16 + srcPos >= 64"));
		}
		 {
			let i: i32 = 0;
			while i < n_shorts {
				{
					/* final */ let shift: i32 = i * Short::SIZE + src_pos;
					dst[dst_pos + i] = (0xffff & src /* signed */ >> shift) as i16;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn short_array_to_int(&self, src: &&[i16], src_pos: i32, dst_init: i32, dst_pos: i32, n_shorts: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if src.length == 0 && src_pos == 0 || 0 == n_shorts {
			return dst_init;
		}
		if (n_shorts - 1) * Short::SIZE + dst_pos >= Integer::SIZE {
			return Err(IllegalArgumentException::new("(nShorts - 1) * 16 + dstPos >= 32"));
		}
		let out: i32 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_shorts {
				{
					/* final */ let shift: i32 = i * Short::SIZE + dst_pos;
					/* final */ let bits: i32 = (0xffff & src[i + src_pos]) << shift;
					/* final */ let mask: i32 = 0xffff << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn short_array_to_long(&self, src: &&[i16], src_pos: i32, dst_init: i64, dst_pos: i32, n_shorts: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		if src.length == 0 && src_pos == 0 || 0 == n_shorts {
			return dst_init;
		}
		if (n_shorts - 1) * Short::SIZE + dst_pos >= Long::SIZE {
			return Err(IllegalArgumentException::new("(nShorts - 1) * 16 + dstPos >= 64"));
		}
		let out: i64 = dst_init;
		 {
			let i: i32 = 0;
			while i < n_shorts {
				{
					/* final */ let shift: i32 = i * Short::SIZE + dst_pos;
					/* final */ let bits: i64 = (0xffff & src[i + src_pos]) << shift;
					/* final */ let mask: i64 = 0xffff << shift;
					out = out & ~mask | bits;
				}
				i += 1;
			 }
		 }
	
		return out;
	}

	pub fn short_to_binary(&self, src: i16, src_pos: i32, mut dst: &&[bool], mut dst_pos: i32, n_bools: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[bool] {
		if 0 == n_bools {
			return dst;
		}
		if n_bools - 1 + src_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("nBools -  1 + srcPos >= 16"));
		}
		assert!( n_bools - 1 < Short::SIZE - src_pos);
		 {
			let i: i32 = 0;
			while i < n_bools {
				{
					/* final */ let shift: i32 = i + src_pos;
					dst[dst_pos + i] = (0x1 & src /* signed */ >> shift) != 0;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn short_to_byte_array(&self, src: i16, src_pos: i32, mut dst: &&[i8], mut dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i8] {
		if 0 == n_bytes {
			return dst;
		}
		if (n_bytes - 1) * Byte::SIZE + src_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("(nBytes - 1) * 8 + srcPos >= 16"));
		}
		 {
			let i: i32 = 0;
			while i < n_bytes {
				{
					/* final */ let shift: i32 = i * Byte::SIZE + src_pos;
					dst[dst_pos + i] = (0xff & src /* signed */ >> shift) as i8;
				}
				i += 1;
			 }
		 }
	
		return dst;
	}

	pub fn short_to_hex(&self, src: i16, src_pos: i32, dst_init: &/* Java */ java::lang::String /**/, dst_pos: i32, n_hexs: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if 0 == n_hexs {
			return dst_init;
		}
		if (n_hexs - 1) * 4 + src_pos >= Short::SIZE {
			return Err(IllegalArgumentException::new("(nHexs - 1) * 4 + srcPos >= 16"));
		}
		/* final */ let sb: StringBuilder = StringBuilder::new(dst_init);
		let append: i32 = sb.length();
		 {
			let i: i32 = 0;
			while i < n_hexs {
				{
					/* final */ let shift: i32 = i * 4 + src_pos;
					/* final */ let bits: i32 = 0xF & src /* signed */ >> shift;
					if dst_pos + i == append {
						append += 1;
						sb.append(&org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					} else {
						sb.setCharAt(dst_pos + i, &org::apache::commons::lang3::conversion::Conversion::int_to_hex_digit(bits)?);
					}
				}
				i += 1;
			 }
		 }
	
		return sb.toString();
	}

	pub fn uuid_to_byte_array(&self, src: &/* Java */ java::util::UUID /**/, dst: &&[i8], dst_pos: i32, n_bytes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i8] {
		if 0 == n_bytes {
			return dst;
		}
		if n_bytes > 16 {
			return Err(IllegalArgumentException::new("nBytes > 16"));
		}
		org::apache::commons::lang3::conversion::Conversion::long_to_byte_array(&src.getMostSignificantBits(), 0, dst, dst_pos, &Math::min(n_bytes, 8))?;
		if n_bytes >= 8 {
			org::apache::commons::lang3::conversion::Conversion::long_to_byte_array(&src.getLeastSignificantBits(), 0, dst, dst_pos + 8, n_bytes - 8)?;
		}
		return dst;
	}

	pub fn new() -> org::apache::commons::lang3::conversion::Conversion {
	// empty
	}
}