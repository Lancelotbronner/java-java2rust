use java::lang::reflect::Array;
use java::math::BigDecimal;
use java::math::BigInteger;
use java::math::RoundingMode;
use java::util::Objects;
use crate::org::apache::commons::lang3::CharUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Validate;

pub struct NumberUtils;

impl NumberUtils {
	pub static LONG_ZERO: /* Java */ java::lang::Long /**/ = Long::valueOf(0);

	pub static LONG_ONE: /* Java */ java::lang::Long /**/ = Long::valueOf(1);

	pub static LONG_MINUS_ONE: /* Java */ java::lang::Long /**/ = Long::valueOf(-1);

	pub static INTEGER_ZERO: /* Java */ java::lang::Integer /**/ = Integer::valueOf(0);

	pub static INTEGER_ONE: /* Java */ java::lang::Integer /**/ = Integer::valueOf(1);

	pub static INTEGER_TWO: /* Java */ java::lang::Integer /**/ = Integer::valueOf(2);

	pub static INTEGER_MINUS_ONE: /* Java */ java::lang::Integer /**/ = Integer::valueOf(-1);

	pub static SHORT_ZERO: /* Java */ java::lang::Short /**/ = Short::valueOf(0 as i16);

	pub static SHORT_ONE: /* Java */ java::lang::Short /**/ = Short::valueOf(1 as i16);

	pub static SHORT_MINUS_ONE: /* Java */ java::lang::Short /**/ = Short::valueOf(-1 as i16);

	pub static BYTE_ZERO: /* Java */ java::lang::Byte /**/ = Byte::valueOf(0 as i8);

	pub static BYTE_ONE: /* Java */ java::lang::Byte /**/ = Byte::valueOf(1 as i8);

	pub static BYTE_MINUS_ONE: /* Java */ java::lang::Byte /**/ = Byte::valueOf(-1 as i8);

	pub static DOUBLE_ZERO: /* Java */ java::lang::Double /**/ = Double::valueOf(0.0);

	pub static DOUBLE_ONE: /* Java */ java::lang::Double /**/ = Double::valueOf(1.0);

	pub static DOUBLE_MINUS_ONE: /* Java */ java::lang::Double /**/ = Double::valueOf(-1.0);

	pub static FLOAT_ZERO: /* Java */ java::lang::Float /**/ = Float::valueOf(0.0f);

	pub static FLOAT_ONE: /* Java */ java::lang::Float /**/ = Float::valueOf(1.0f);

	pub static FLOAT_MINUS_ONE: /* Java */ java::lang::Float /**/ = Float::valueOf(-1.0f);

	pub static LONG_INT_MAX_VALUE: /* Java */ java::lang::Long /**/ = Long::valueOf(Integer::MAX_VALUE);

	pub static LONG_INT_MIN_VALUE: /* Java */ java::lang::Long /**/ = Long::valueOf(Integer::MIN_VALUE);

	pub fn compare(&self, x: i8, y: i8) -> i32 {
		return Byte::compare(x, y);
	}

	pub fn compare(&self, x: i32, y: i32) -> i32 {
		return Integer::compare(x, y);
	}

	pub fn compare(&self, x: i64, y: i64) -> i32 {
		return Long::compare(x, y);
	}

	pub fn compare(&self, x: i16, y: i16) -> i32 {
		return Short::compare(x, y);
	}

	pub fn create_big_decimal(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.NumberFormatException) */ -> /* Java */ java::math::BigDecimal /**/ {
		if str == null {
			return null;
		}
		// handle JDK1.3.1 bug where "" throws IndexOutOfBoundsException
		if StringUtils::is_blank(str) {
			return Err(NumberFormatException::new("A blank string is not a valid number"));
		}
		return BigDecimal::new(str);
	}

	pub fn create_big_integer(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.NumberFormatException) */ -> /* Java */ java::math::BigInteger /**/ {
		if str == null {
			return null;
		}
		if str.isEmpty() {
			return Err(NumberFormatException::new("An empty string is not a valid number"));
		}
		// offset within string
		let pos: i32 = 0;
		let radix: i32 = 10;
		// need to negate later?
		let negate: bool = false;
		/* final */ let char0: char = str.charAt(0);
		if char0 == '-' {
			negate = true;
			pos = 1;
		} else if char0 == '+' {
			pos = 1;
		}
		if str.startsWith("0x", pos) || str.startsWith("0X", pos) {
			// hex
			radix = 16;
			pos += 2;
		} else if str.startsWith("#", pos) {
			// alternative hex (allowed by Long/Integer)
			radix = 16;
			pos += 1;
		} else if str.startsWith("0", pos) && str.length() > pos + 1 {
			// octal; so long as there are additional digits
			radix = 8;
			pos += 1;
		}
		// default is to treat as decimal
		/* final */ let value: BigInteger = BigInteger::new(&str.substring(pos), radix);
		return  if negate { value.negate() } else { value };
	}

	pub fn create_double(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Double /**/ {
		if str == null {
			return null;
		}
		return Double::valueOf(str);
	}

	pub fn create_float(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Float /**/ {
		if str == null {
			return null;
		}
		return Float::valueOf(str);
	}

	pub fn create_integer(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Integer /**/ {
		if str == null {
			return null;
		}
		// decode() handles 0xAABD and 0777 (hex and octal) as well.
		return Integer::decode(str);
	}

	pub fn create_long(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Long /**/ {
		if str == null {
			return null;
		}
		return Long::decode(str);
	}

	pub fn create_number(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.NumberFormatException) */ -> /* Java */ java::lang::Number /**/ {
		if str == null {
			return null;
		}
		if StringUtils::is_blank(str) {
			return Err(NumberFormatException::new("A blank string is not a valid number"));
		}
		// Need to deal with all possible hex prefixes here
		/* final */ let hex_prefixes: vec![Vec<String>; 3] = vec!["0x", "0X", "#", ]
		;
		/* final */ let length: i32 = str.length();
		/* final */ let offset: i32 =  if org::apache::commons::lang3::math::number_utils::NumberUtils::is_sign(&str.charAt(0)) { 1 } else { 0 };
		let pfx_len: i32 = 0;
		for /* final */ pfx in hex_prefixes {
			if str.startsWith(pfx, offset) {
				pfx_len += pfx.length() + offset;
				break;
			}
		}
		if pfx_len > 0 {
			// we have a hex number
			// strip leading zeroes
			let first_sig_digit: char = 0;
			 {
				let i: i32 = pfx_len;
				while i < length {
					{
						first_sig_digit = str.charAt(i);
						if first_sig_digit != '0' {
							break;
						}
						pfx_len += 1;
					}
					i += 1;
				 }
			 }
	
			/* final */ let hex_digits: i32 = length - pfx_len;
			if hex_digits > 16 || hex_digits == 16 && first_sig_digit > '7' {
				// too many for Long
				return org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_integer(str)?;
			}
			if hex_digits > 8 || hex_digits == 8 && first_sig_digit > '7' {
				// too many for an int
				return org::apache::commons::lang3::math::number_utils::NumberUtils::create_long(str);
			}
			return org::apache::commons::lang3::math::number_utils::NumberUtils::create_integer(str);
		}
		/* final */ let last_char: char = str.charAt(length - 1);
		/* final */ let mant: String;
		/* final */ let dec: String;
		/* final */ let exp: String;
		/* final */ let dec_pos: i32 = str.indexOf('.');
		// assumes both not present
		/* final */ let exp_pos: i32 = str.indexOf('e') + str.indexOf('E') + 1;
		// if both e and E are present, this is caught by the checks on expPos (which prevent IOOBE)
		// and the parsing which will detect if e or E appear in a number due to using the wrong offset
		// Detect if the return type has been requested
		/* final */ let request_type: bool = !Character::isDigit(last_char) && last_char != '.';
		if dec_pos > -1 {
			// there is a decimal point
			if exp_pos > -1 {
				// there is an exponent
				if exp_pos <= dec_pos || exp_pos > length {
					// prevents double exponent causing IOOBE
					return Err(NumberFormatException::new(str + " is not a valid number."));
				}
				dec = str.substring(dec_pos + 1, exp_pos);
			} else {
				// No exponent, but there may be a type character to remove
				dec = str.substring(dec_pos + 1,  if request_type { length - 1 } else { length });
			}
			mant = org::apache::commons::lang3::math::number_utils::NumberUtils::get_mantissa(str, dec_pos)?;
		} else {
			if exp_pos > -1 {
				if exp_pos > length {
					// prevents double exponent causing IOOBE
					return Err(NumberFormatException::new(str + " is not a valid number."));
				}
				mant = org::apache::commons::lang3::math::number_utils::NumberUtils::get_mantissa(str, exp_pos)?;
			} else {
				// No decimal, no exponent, but there may be a type character to remove
				mant = org::apache::commons::lang3::math::number_utils::NumberUtils::get_mantissa(str,  if request_type { length - 1 } else { length })?;
			}
			dec = null;
		}
		if request_type {
			if exp_pos > -1 && exp_pos < length - 1 {
				exp = str.substring(exp_pos + 1, length - 1);
			} else {
				exp = null;
			}
			// Requesting a specific type.
			/* final */ let numeric: String = str.substring(0, length - 1);
			match last_char {
				'l' =>  {
				}
				'L' =>  {
					if dec == null && exp == null && (!numeric.isEmpty() && numeric.charAt(0) == '-' && org::apache::commons::lang3::math::number_utils::NumberUtils::is_digits(&numeric.substring(1)) || org::apache::commons::lang3::math::number_utils::NumberUtils::is_digits(numeric)) {
						let r0 = 'try0: {
							return org::apache::commons::lang3::math::number_utils::NumberUtils::create_long(numeric);
							break 'try0 Ok(());
						};
						match r0 {
							Err(e @ NumberFormatException) => {
							// Too big for a long
							},
							Err(e) => Err(e)?,
							Ok => (),
						}
						return match org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_integer(numeric) {
							Err(e) => break 'try0 Err(e),
							Ok(s) => s,
						};
					}
					break 'try0 Err(NumberFormatException::new(str + " is not a valid number."));
				}
				'f' =>  {
				}
				'F' =>  {
					let r0 = 'try0: {
						/* final */ let f: Float = org::apache::commons::lang3::math::number_utils::NumberUtils::create_float(str);
						if !(f.isInfinite() || f.floatValue() == 0.0F && !org::apache::commons::lang3::math::number_utils::NumberUtils::is_zero(mant, dec)) {
							// has non-zeros in it, then float does not have the precision we want
							return f;
						}
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ NumberFormatException) => {
						// ignore the bad number
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				// falls-through
				'd' =>  {
				}
				'D' =>  {
					let r1 = 'try1: {
						/* final */ let d: Double = org::apache::commons::lang3::math::number_utils::NumberUtils::create_double(str);
						if !(d.isInfinite() || d.doubleValue() == 0.0 && !org::apache::commons::lang3::math::number_utils::NumberUtils::is_zero(mant, dec)) {
							return d;
						}
						break 'try1 Ok(());
					};
					match r1 {
						Err(e @ NumberFormatException) => {
						// ignore the bad number
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
					let r2 = 'try2: {
						return match org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_decimal(numeric) {
							Err(e) => break 'try2 Err(e),
							Ok(s) => s,
						};
						break 'try2 Ok(());
					};
					match r2 {
						Err(e @ NumberFormatException) => {
						// ignore the bad number
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				// falls-through
				_ =>  {
					break 'try2 Err(NumberFormatException::new(str + " is not a valid number."));
				}
			}
		}
		// small and go from there...
		if exp_pos > -1 && exp_pos < length - 1 {
			exp = str.substring(exp_pos + 1);
		} else {
			exp = null;
		}
		if dec == null && exp == null {
			// Must be an Integer, Long, Biginteger
			let r2 = 'try2: {
				return org::apache::commons::lang3::math::number_utils::NumberUtils::create_integer(str);
				break 'try2 Ok(());
			};
			match r2 {
				Err(e @ NumberFormatException) => {
				// ignore the bad number
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			let r3 = 'try3: {
				return org::apache::commons::lang3::math::number_utils::NumberUtils::create_long(str);
				break 'try3 Ok(());
			};
			match r3 {
				Err(e @ NumberFormatException) => {
				// ignore the bad number
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			return match org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_integer(str) {
				Err(e) => break 'try3 Err(e),
				Ok(s) => s,
			};
		}
		// Must be a Float, Double, BigDecimal
		let r4 = 'try4: {
			/* final */ let f: Float = org::apache::commons::lang3::math::number_utils::NumberUtils::create_float(str);
			/* final */ let d: Double = org::apache::commons::lang3::math::number_utils::NumberUtils::create_double(str);
			if !f.isInfinite() && !(f.floatValue() == 0.0F && !org::apache::commons::lang3::math::number_utils::NumberUtils::is_zero(mant, dec)) && f.toString().equals(&d.toString()) {
				return f;
			}
			if !d.isInfinite() && !(d.doubleValue() == 0.0 && !org::apache::commons::lang3::math::number_utils::NumberUtils::is_zero(mant, dec)) {
				/* final */ let b: BigDecimal = match org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_decimal(str) {
					Err(e) => break 'try4 Err(e),
					Ok(s) => s,
				};
				if b.compareTo(&BigDecimal::valueOf(&d.doubleValue())) == 0 {
					return d;
				}
				return b;
			}
			break 'try4 Ok(());
		};
		match r4 {
			Err(e @ NumberFormatException) => {
			// ignore the bad number
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return match org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_decimal(str) {
			Err(e) => break 'try4 Err(e),
			Ok(s) => s,
		};
	}

	fn get_mantissa(&self, str: &/* Java */ java::lang::String /**/, stop_pos: i32) /* thrown(java.lang.NumberFormatException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let first_char: char = str.charAt(0);
		/* final */ let has_sign: bool = org::apache::commons::lang3::math::number_utils::NumberUtils::is_sign(first_char);
		/* final */ let length: i32 = str.length();
		if length <= ( if has_sign { 1 } else { 0 }) || length < stop_pos {
			return Err(NumberFormatException::new(str + " is not a valid number."));
		}
		return  if has_sign { str.substring(1, stop_pos) } else { str.substring(0, stop_pos) };
	}

	fn is_all_zeros(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		if str == null {
			return true;
		}
		 {
			let i: i32 = str.length() - 1;
			while i >= 0 {
				{
					if str.charAt(i) != '0' {
						return false;
					}
				}
				i -= 1;
			 }
		 }
	
		return true;
	}

	pub fn is_creatable(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		if StringUtils::is_empty(str) {
			return false;
		}
		/* final */ let chars: Vec<char> = str.toCharArray();
		let sz: i32 = chars.length;
		let has_exp: bool = false;
		let has_dec_point: bool = false;
		let allow_signs: bool = false;
		let found_digit: bool = false;
		// deal with any possible sign up front
		/* final */ let start: i32 =  if org::apache::commons::lang3::math::number_utils::NumberUtils::is_sign(chars[0]) { 1 } else { 0 };
		if sz > start + 1 && chars[start] == '0' && !StringUtils::contains(str, '.') {
			// leading 0, skip if is a decimal number
			if chars[start + 1] == 'x' || chars[start + 1] == 'X' {
				// leading 0x/0X
				let i: i32 = start + 2;
				if i == sz {
					// str == "0x"
					return false;
				}
				// checking hex (it can't be anything else)
				while i < chars.length {
					{
						if !CharUtils::is_hex(chars[i]) {
							return false;
						}
					}
					i += 1;
				 }
	
				return true;
			}
			if Character::isDigit(chars[start + 1]) {
				// leading 0, but not hex, must be octal
				let i: i32 = start + 1;
				while i < chars.length {
					{
						if !CharUtils::is_octal(chars[i]) {
							return false;
						}
					}
					i += 1;
				 }
	
				return true;
			}
		}
		// don't want to loop to the last char, check it afterwards
		sz -= 1;
		// for type qualifiers
		let i: i32 = start;
		// make a valid number (e.g. chars[0..5] = "1234E")
		while i < sz || i < sz + 1 && allow_signs && !found_digit {
			if CharUtils::is_ascii_numeric(chars[i]) {
				found_digit = true;
				allow_signs = false;
			} else if chars[i] == '.' {
				if has_dec_point || has_exp {
					// two decimal points or dec in exponent
					return false;
				}
				has_dec_point = true;
			} else if chars[i] == 'e' || chars[i] == 'E' {
				// we've already taken care of hex.
				if has_exp {
					// two E's
					return false;
				}
				if !found_digit {
					return false;
				}
				has_exp = true;
				allow_signs = true;
			} else if org::apache::commons::lang3::math::number_utils::NumberUtils::is_sign(chars[i]) {
				if !allow_signs {
					return false;
				}
				allow_signs = false;
				// we need a digit after the E
				found_digit = false;
			} else {
				return false;
			}
			i += 1;
		}
		if i < chars.length {
			if CharUtils::is_ascii_numeric(chars[i]) {
				// no type qualifier, OK
				return true;
			}
			if chars[i] == 'e' || chars[i] == 'E' {
				// can't have an E at the last byte
				return false;
			}
			if chars[i] == '.' {
				if has_dec_point || has_exp {
					// two decimal points or dec in exponent
					return false;
				}
				// single trailing decimal point after non-exponent is ok
				return found_digit;
			}
			if !allow_signs && (chars[i] == 'd' || chars[i] == 'D' || chars[i] == 'f' || chars[i] == 'F') {
				return found_digit;
			}
			if chars[i] == 'l' || chars[i] == 'L' {
				// not allowing L with an exponent or decimal point
				return found_digit && !has_exp && !has_dec_point;
			}
			// last character is illegal
			return false;
		}
		// found digit it to make sure weird stuff like '.' and '1E-' doesn't pass
		return !allow_signs && found_digit;
	}

	pub fn is_digits(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return StringUtils::is_numeric(str);
	}

	pub fn is_number(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::is_creatable(str);
	}

	pub fn is_parsable(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		if StringUtils::is_empty(str) {
			return false;
		}
		if str.charAt(str.length() - 1) == '.' {
			return false;
		}
		if str.charAt(0) == '-' {
			if str.length() == 1 {
				return false;
			}
			return org::apache::commons::lang3::math::number_utils::NumberUtils::with_decimals_parsing(str, 1);
		}
		return org::apache::commons::lang3::math::number_utils::NumberUtils::with_decimals_parsing(str, 0);
	}

	fn is_sign(&self, ch: u16) -> bool {
		return ch == '-' || ch == '+';
	}

	fn is_zero(&self, mant: &/* Java */ java::lang::String /**/, dec: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::is_all_zeros(mant) && org::apache::commons::lang3::math::number_utils::NumberUtils::is_all_zeros(dec);
	}

	pub fn max(&self, array: i8) -> i8 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: i8 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if array[i] > max {
						max = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, mut a: i8, b: i8, c: i8) -> i8 {
		if b > a {
			a = b;
		}
		if c > a {
			a = c;
		}
		return a;
	}

	pub fn max(&self, array: f64) -> f64 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: f64 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					if Double::isNaN(array[j]) {
						return Double::NaN;
					}
					if array[j] > max {
						max = array[j];
					}
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, a: f64, b: f64, c: f64) -> f64 {
		return Math::max(&Math::max(a, b), c);
	}

	pub fn max(&self, array: f32) -> f32 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: f32 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					if Float::isNaN(array[j]) {
						return Float::NaN;
					}
					if array[j] > max {
						max = array[j];
					}
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, a: f32, b: f32, c: f32) -> f32 {
		return Math::max(&Math::max(a, b), c);
	}

	pub fn max(&self, array: i32) -> i32 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: i32 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					if array[j] > max {
						max = array[j];
					}
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, mut a: i32, b: i32, c: i32) -> i32 {
		if b > a {
			a = b;
		}
		if c > a {
			a = c;
		}
		return a;
	}

	pub fn max(&self, array: i64) -> i64 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: i64 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					if array[j] > max {
						max = array[j];
					}
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, mut a: i64, b: i64, c: i64) -> i64 {
		if b > a {
			a = b;
		}
		if c > a {
			a = c;
		}
		return a;
	}

	pub fn max(&self, array: i16) -> i16 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns max
		let max: i16 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if array[i] > max {
						max = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, mut a: i16, b: i16, c: i16) -> i16 {
		if b > a {
			a = b;
		}
		if c > a {
			a = c;
		}
		return a;
	}

	pub fn min(&self, array: i8) -> i8 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: i8 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if array[i] < min {
						min = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, mut a: i8, b: i8, c: i8) -> i8 {
		if b < a {
			a = b;
		}
		if c < a {
			a = c;
		}
		return a;
	}

	pub fn min(&self, array: f64) -> f64 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: f64 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if Double::isNaN(array[i]) {
						return Double::NaN;
					}
					if array[i] < min {
						min = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, a: f64, b: f64, c: f64) -> f64 {
		return Math::min(&Math::min(a, b), c);
	}

	pub fn min(&self, array: f32) -> f32 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: f32 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if Float::isNaN(array[i]) {
						return Float::NaN;
					}
					if array[i] < min {
						min = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, a: f32, b: f32, c: f32) -> f32 {
		return Math::min(&Math::min(a, b), c);
	}

	pub fn min(&self, array: i32) -> i32 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: i32 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					if array[j] < min {
						min = array[j];
					}
				}
				j += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, mut a: i32, b: i32, c: i32) -> i32 {
		if b < a {
			a = b;
		}
		if c < a {
			a = c;
		}
		return a;
	}

	pub fn min(&self, array: i64) -> i64 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: i64 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if array[i] < min {
						min = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, mut a: i64, b: i64, c: i64) -> i64 {
		if b < a {
			a = b;
		}
		if c < a {
			a = c;
		}
		return a;
	}

	pub fn min(&self, array: i16) -> i16 {
		// Validates input
		org::apache::commons::lang3::math::number_utils::NumberUtils::validate_array(array);
		// Finds and returns min
		let min: i16 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					if array[i] < min {
						min = array[i];
					}
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, mut a: i16, b: i16, c: i16) -> i16 {
		if b < a {
			a = b;
		}
		if c < a {
			a = c;
		}
		return a;
	}

	pub fn to_byte(&self, str: &/* Java */ java::lang::String /**/) -> i8 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_byte(str, 0 as i8);
	}

	pub fn to_byte(&self, str: &/* Java */ java::lang::String /**/, default_value: i8) -> i8 {
		let r0 = 'try0: {
			return Byte::parseByte(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn to_double(&self, value: &/* Java */ java::math::BigDecimal /**/) -> f64 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_double(value, 0.0);
	}

	pub fn to_double(&self, value: &/* Java */ java::math::BigDecimal /**/, default_value: f64) -> f64 {
		return  if value == null { default_value } else { value.doubleValue() };
	}

	pub fn to_double(&self, str: &/* Java */ java::lang::String /**/) -> f64 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_double(str, 0.0);
	}

	pub fn to_double(&self, str: &/* Java */ java::lang::String /**/, default_value: f64) -> f64 {
		let r0 = 'try0: {
			return Double::parseDouble(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn to_float(&self, str: &/* Java */ java::lang::String /**/) -> f32 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_float(str, 0.0f);
	}

	pub fn to_float(&self, str: &/* Java */ java::lang::String /**/, default_value: f32) -> f32 {
		let r0 = 'try0: {
			return Float::parseFloat(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn to_int(&self, str: &/* Java */ java::lang::String /**/) -> i32 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_int(str, 0);
	}

	pub fn to_int(&self, str: &/* Java */ java::lang::String /**/, default_value: i32) -> i32 {
		let r0 = 'try0: {
			return Integer::parseInt(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn to_long(&self, str: &/* Java */ java::lang::String /**/) -> i64 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_long(str, 0);
	}

	pub fn to_long(&self, str: &/* Java */ java::lang::String /**/, default_value: i64) -> i64 {
		let r0 = 'try0: {
			return Long::parseLong(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::math::BigDecimal /**/) -> /* Java */ java::math::BigDecimal /**/ {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(value, self.INTEGER_TWO, RoundingMode::HALF_EVEN);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::math::BigDecimal /**/, scale: i32, rounding_mode: &/* Java */ java::math::RoundingMode /**/) -> /* Java */ java::math::BigDecimal /**/ {
		if value == null {
			return BigDecimal::ZERO;
		}
		return value.setScale(scale,  if rounding_mode == null { RoundingMode::HALF_EVEN } else { rounding_mode });
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::Double /**/) -> /* Java */ java::math::BigDecimal /**/ {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(value, self.INTEGER_TWO, RoundingMode::HALF_EVEN);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::Double /**/, scale: i32, rounding_mode: &/* Java */ java::math::RoundingMode /**/) -> /* Java */ java::math::BigDecimal /**/ {
		if value == null {
			return BigDecimal::ZERO;
		}
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(&BigDecimal::valueOf(value), scale, rounding_mode);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::Float /**/) -> /* Java */ java::math::BigDecimal /**/ {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(value, self.INTEGER_TWO, RoundingMode::HALF_EVEN);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::Float /**/, scale: i32, rounding_mode: &/* Java */ java::math::RoundingMode /**/) -> /* Java */ java::math::BigDecimal /**/ {
		if value == null {
			return BigDecimal::ZERO;
		}
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(&BigDecimal::valueOf(value), scale, rounding_mode);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::String /**/) -> /* Java */ java::math::BigDecimal /**/ {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(value, self.INTEGER_TWO, RoundingMode::HALF_EVEN);
	}

	pub fn to_scaled_big_decimal(&self, value: &/* Java */ java::lang::String /**/, scale: i32, rounding_mode: &/* Java */ java::math::RoundingMode /**/) /* thrown(java.lang.NumberFormatException) */ -> /* Java */ java::math::BigDecimal /**/ {
		if value == null {
			return BigDecimal::ZERO;
		}
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_scaled_big_decimal(&org::apache::commons::lang3::math::number_utils::NumberUtils::create_big_decimal(value)?, scale, rounding_mode);
	}

	pub fn to_short(&self, str: &/* Java */ java::lang::String /**/) -> i16 {
		return org::apache::commons::lang3::math::number_utils::NumberUtils::to_short(str, 0 as i16);
	}

	pub fn to_short(&self, str: &/* Java */ java::lang::String /**/, default_value: i16) -> i16 {
		let r0 = 'try0: {
			return Short::parseShort(str);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ RuntimeException) => {
				return default_value;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn validate_array(&self, array: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		Objects::requireNonNull(array, "array");
		Validate::is_true(Array::getLength(array) != 0, "Array cannot be empty.")?;
	}

	fn with_decimals_parsing(&self, str: &/* Java */ java::lang::String /**/, begin_idx: i32) -> bool {
		let decimal_points: i32 = 0;
		 {
			let i: i32 = begin_idx;
			while i < str.length() {
				{
					/* final */ let ch: char = str.charAt(i);
					/* final */ let is_decimal_point: bool = ch == '.';
					if is_decimal_point {
						decimal_points += 1;
					}
					if decimal_points > 1 {
						return false;
					}
					if !is_decimal_point && !Character::isDigit(ch) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn new() -> org::apache::commons::lang3::math::number_utils::NumberUtils {
	// empty
	}
}