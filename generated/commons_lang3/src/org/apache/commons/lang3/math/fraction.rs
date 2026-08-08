use java::io::Serializable;
use java::math::BigInteger;
use java::util::Objects;

pub struct Fraction {
	numerator: i32,
	denominator: i32,
	hash_code: i32,
	to_string: /* Java */ java::lang::String /**/,
	to_proper_string: /* Java */ java::lang::String /**/,
}

impl Fraction {
	static serialVersionUID: i64 = 65382027393090;

	pub static ZERO: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(0, 1);

	pub static ONE: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(1, 1);

	pub static ONE_HALF: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(1, 2);

	pub static ONE_THIRD: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(1, 3);

	pub static TWO_THIRDS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(2, 3);

	pub static ONE_QUARTER: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(1, 4);

	pub static TWO_QUARTERS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(2, 4);

	pub static THREE_QUARTERS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(3, 4);

	pub static ONE_FIFTH: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(1, 5);

	pub static TWO_FIFTHS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(2, 5);

	pub static THREE_FIFTHS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(3, 5);

	pub static FOUR_FIFTHS: org::apache::commons::lang3::math::fraction::Fraction = Fraction::new(4, 5);

	fn add_and_check(&self, x: i32, y: i32) /* thrown(java.lang.ArithmeticException) */ -> i32 {
		/* final */ let s: i64 = x as i64 + y as i64;
		if s < Integer::MIN_VALUE || s > Integer::MAX_VALUE {
			return Err(ArithmeticException::new("overflow: add"));
		}
		return s as i32;
	}

	pub fn get_fraction(&self, mut value: f64) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		/* final */ let sign: i32 =  if value < 0 { -1 } else { 1 };
		value = Math::abs(value);
		if value > Integer::MAX_VALUE || Double::isNaN(value) {
			return Err(ArithmeticException::new("The value must not be greater than Integer.MAX_VALUE or NaN"));
		}
		/* final */ let whole_number: i32 = value as i32;
		value -= whole_number;
		// the pre-previous
		let numer0: i32 = 0;
		// the pre-previous
		let denom0: i32 = 1;
		// the previous
		let numer1: i32 = 1;
		// the previous
		let denom1: i32 = 0;
		// the current, setup in calculation
		let numer2: i32;
		// the current, setup in calculation
		let denom2: i32;
		let a1: i32 = value as i32;
		let a2: i32;
		let x1: f64 = 1;
		let x2: f64;
		let y1: f64 = value - a1;
		let y2: f64;
		let delta1: f64;
		let delta2: f64 = Double::MAX_VALUE;
		let fraction: f64;
		let i: i32 = 1;
		loop { {
			delta1 = delta2;
			a2 = (x1 / y1) as i32;
			x2 = y1;
			y2 = x1 - a2 * y1;
			numer2 = a1 * numer1 + numer0;
			denom2 = a1 * denom1 + denom0;
			fraction = numer2 as f64 / denom2 as f64;
			delta2 = Math::abs(value - fraction);
			a1 = a2;
			x1 = x2;
			y1 = y2;
			numer0 = numer1;
			denom0 = denom1;
			numer1 = numer2;
			denom1 = denom2;
			i += 1;
		}if !(delta1 > delta2 && denom2 <= 10000 && denom2 > 0 && i < 25) break;}
		if i == 25 {
			return Err(ArithmeticException::new("Unable to convert double to fraction"));
		}
		return org::apache::commons::lang3::math::fraction::Fraction::get_reduced_fraction((numer0 + whole_number * denom0) * sign, denom0)?;
	}

	pub fn get_fraction(&self, mut numerator: i32, mut denominator: i32) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if denominator == 0 {
			return Err(ArithmeticException::new("The denominator must not be zero"));
		}
		if denominator < 0 {
			if numerator == Integer::MIN_VALUE || denominator == Integer::MIN_VALUE {
				return Err(ArithmeticException::new("overflow: can't negate"));
			}
			numerator = -numerator;
			denominator = -denominator;
		}
		return Fraction::new(numerator, denominator);
	}

	pub fn get_fraction(&self, whole: i32, numerator: i32, denominator: i32) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if denominator == 0 {
			return Err(ArithmeticException::new("The denominator must not be zero"));
		}
		if denominator < 0 {
			return Err(ArithmeticException::new("The denominator must not be negative"));
		}
		if numerator < 0 {
			return Err(ArithmeticException::new("The numerator must not be negative"));
		}
		/* final */ let numerator_value: i64;
		if whole < 0 {
			numerator_value = whole * denominator as i64 - numerator;
		} else {
			numerator_value = whole * denominator as i64 + numerator;
		}
		if numerator_value < Integer::MIN_VALUE || numerator_value > Integer::MAX_VALUE {
			return Err(ArithmeticException::new("Numerator too large to represent as an Integer."));
		}
		return Fraction::new(numerator_value as i32, denominator);
	}

	pub fn get_fraction(&self, mut str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.NumberFormatException | java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		Objects::requireNonNull(str, "str");
		// parse double format
		let pos: i32 = str.indexOf('.');
		if pos >= 0 {
			return org::apache::commons::lang3::math::fraction::Fraction::get_fraction(&Double::parseDouble(str))?;
		}
		// parse X Y/Z format
		pos = str.indexOf(' ');
		if pos > 0 {
			/* final */ let whole: i32 = Integer::parseInt(&str.substring(0, pos));
			str = str.substring(pos + 1);
			pos = str.indexOf('/');
			if pos < 0 {
				return Err(NumberFormatException::new("The fraction could not be parsed as the format X Y/Z"));
			}
			/* final */ let numer: i32 = Integer::parseInt(&str.substring(0, pos));
			/* final */ let denom: i32 = Integer::parseInt(&str.substring(pos + 1));
			return org::apache::commons::lang3::math::fraction::Fraction::get_fraction(whole, numer, denom)?;
		}
		// parse Y/Z format
		pos = str.indexOf('/');
		if pos < 0 {
			// simple whole number
			return org::apache::commons::lang3::math::fraction::Fraction::get_fraction(&Integer::parseInt(str), 1)?;
		}
		/* final */ let numer: i32 = Integer::parseInt(&str.substring(0, pos));
		/* final */ let denom: i32 = Integer::parseInt(&str.substring(pos + 1));
		return org::apache::commons::lang3::math::fraction::Fraction::get_fraction(numer, denom)?;
	}

	pub fn get_reduced_fraction(&self, mut numerator: i32, mut denominator: i32) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if denominator == 0 {
			return Err(ArithmeticException::new("The denominator must not be zero"));
		}
		if numerator == 0 {
			// normalize zero.
			return self.ZERO;
		}
		// allow 2^k/-2^31 as a valid fraction (where k>0)
		if denominator == Integer::MIN_VALUE && (numerator & 1) == 0 {
			numerator /= 2;
			denominator /= 2;
		}
		if denominator < 0 {
			if numerator == Integer::MIN_VALUE || denominator == Integer::MIN_VALUE {
				return Err(ArithmeticException::new("overflow: can't negate"));
			}
			numerator = -numerator;
			denominator = -denominator;
		}
		// simplify fraction.
		/* final */ let gcd: i32 = org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(numerator, denominator)?;
		numerator /= gcd;
		denominator /= gcd;
		return Fraction::new(numerator, denominator);
	}

	fn greatest_common_divisor(&self, mut u: i32, mut v: i32) /* thrown(java.lang.ArithmeticException) */ -> i32 {
		// From Commons Math:
		if u == 0 || v == 0 {
			if u == Integer::MIN_VALUE || v == Integer::MIN_VALUE {
				return Err(ArithmeticException::new("overflow: gcd is 2^31"));
			}
			return Math::abs(u) + Math::abs(v);
		}
		// if either operand is abs 1, return 1:
		if Math::abs(u) == 1 || Math::abs(v) == 1 {
			return 1;
		}
		// overflow)
		if u > 0 {
			u = -u;
		}
		// make u negative
		if v > 0 {
			v = -v;
		}
		// make v negative
		// B1. [Find power of 2]
		let k: i32 = 0;
		while (u & 1) == 0 && (v & 1) == 0 && k < 31 {
			// while u and v are both even...
			u /= 2;
			v /= 2;
			// cast out twos.
			k += 1;
		}
		if k == 31 {
			return Err(ArithmeticException::new("overflow: gcd is 2^31"));
		}
		// B2. Initialize: u and v have been divided by 2^k and at least
		// one is odd.
		let t: i32 =  if (u & 1) == 1 { v } else { -(u / 2) };
		// t positive: u was even, v is odd (t replaces u)
		loop { {
			// B4/B3: cast out twos from t.
			while (t & 1) == 0 {
				// while t is even.
				// cast out twos
				t /= 2;
			}
			// B5 [reset max(u,v)]
			if t > 0 {
				u = -t;
			} else {
				v = t;
			}
			// B6/B3. at this point both u and v should be odd.
			t = (v - u) / 2;
		// |u| larger: t positive (replace u)
		// |v| larger: t negative (replace v)
		}if !(t != 0) break;}
		// gcd is u*2^k
		return -u * (1 << k);
	}

	fn mul_and_check(&self, x: i32, y: i32) /* thrown(java.lang.ArithmeticException) */ -> i32 {
		/* final */ let m: i64 = x as i64 * y as i64;
		if m < Integer::MIN_VALUE || m > Integer::MAX_VALUE {
			return Err(ArithmeticException::new("overflow: mul"));
		}
		return m as i32;
	}

	fn mul_pos_and_check(&self, x: i32, y: i32) /* thrown(java.lang.ArithmeticException) */ -> i32 {
		/*  assert x>=0 && y>=0; */ 
		/* final */ let m: i64 = x as i64 * y as i64;
		if m > Integer::MAX_VALUE {
			return Err(ArithmeticException::new("overflow: mulPos"));
		}
		return m as i32;
	}

	fn sub_and_check(&self, x: i32, y: i32) /* thrown(java.lang.ArithmeticException) */ -> i32 {
		/* final */ let s: i64 = x as i64 - y as i64;
		if s < Integer::MIN_VALUE || s > Integer::MAX_VALUE {
			return Err(ArithmeticException::new("overflow: add"));
		}
		return s as i32;
	}

	fn new(numerator: i32, denominator: i32) -> org::apache::commons::lang3::math::fraction::Fraction {
		self.numerator = numerator;
		self.denominator = denominator;
	}

	pub fn abs(&self) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if self.numerator >= 0 {
			return self;
		}
		return self.negate()?;
	}

	pub fn add(&self, fraction: &org::apache::commons::lang3::math::fraction::Fraction) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		return self.add_sub(fraction, true)?;
	}

	fn add_sub(&self, fraction: &org::apache::commons::lang3::math::fraction::Fraction, is_add: bool) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		Objects::requireNonNull(fraction, "fraction");
		// zero is identity for addition.
		if self.numerator == 0 {
			return  if is_add { fraction } else { fraction.negate()? };
		}
		if fraction.numerator == 0 {
			return self;
		}
		// if denominators are randomly distributed, d1 will be 1 about 61%
		// of the time.
		/* final */ let d1: i32 = org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(self.denominator, fraction.denominator)?;
		if d1 == 1 {
			// result is ((u*v' +/- u'v) / u'v')
			/* final */ let uvp: i32 = org::apache::commons::lang3::math::fraction::Fraction::mul_and_check(self.numerator, fraction.denominator)?;
			/* final */ let upv: i32 = org::apache::commons::lang3::math::fraction::Fraction::mul_and_check(fraction.numerator, self.denominator)?;
			return Fraction::new( if is_add { org::apache::commons::lang3::math::fraction::Fraction::add_and_check(uvp, upv)? } else { org::apache::commons::lang3::math::fraction::Fraction::sub_and_check(uvp, upv)? }, &org::apache::commons::lang3::math::fraction::Fraction::mul_pos_and_check(self.denominator, fraction.denominator)?);
		}
		// the quantity 't' requires 65 bits of precision; see knuth 4.5.1
		// exercise 7. we're going to use a BigInteger.
		// t = u(v'/d1) +/- v(u'/d1)
		/* final */ let uvp: BigInteger = BigInteger::valueOf(self.numerator).multiply(&BigInteger::valueOf(fraction.denominator / d1));
		/* final */ let upv: BigInteger = BigInteger::valueOf(fraction.numerator).multiply(&BigInteger::valueOf(self.denominator / d1));
		/* final */ let t: BigInteger =  if is_add { uvp.add(upv) } else { uvp.subtract(upv) };
		// but d2 doesn't need extra precision because
		// d2 = gcd(t,d1) = gcd(t mod d1, d1)
		/* final */ let tmodd1: i32 = t.mod(&BigInteger::valueOf(d1)).intValue();
		/* final */ let d2: i32 =  if tmodd1 == 0 { d1 } else { org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(tmodd1, d1)? };
		// result is (t/d2) / (u'/d1)(v'/d2)
		/* final */ let w: BigInteger = t.divide(&BigInteger::valueOf(d2));
		if w.bitLength() > 31 {
			return Err(ArithmeticException::new("overflow: numerator too large after multiply"));
		}
		return Fraction::new(&w.intValue(), &org::apache::commons::lang3::math::fraction::Fraction::mul_pos_and_check(self.denominator / d1, fraction.denominator / d2)?);
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::math::fraction::Fraction) -> i32 {
		if self == other {
			return 0;
		}
		if self.numerator == other.numerator && self.denominator == other.denominator {
			return 0;
		}
		// otherwise see which is less
		/* final */ let first: i64 = self.numerator as i64 * other.denominator as i64;
		/* final */ let second: i64 = other.numerator as i64 * self.denominator as i64;
		return Long::compare(first, second);
	}

	pub fn divide_by(&self, fraction: &org::apache::commons::lang3::math::fraction::Fraction) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		Objects::requireNonNull(fraction, "fraction");
		if fraction.numerator == 0 {
			return Err(ArithmeticException::new("The fraction to divide by must not be zero"));
		}
		return self.multiply_by(&fraction.invert()?);
	}

	pub fn double_value(&self) -> f64 {
		return self.numerator as f64 / self.denominator as f64;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if !(obj instanceof Fraction) {
			return false;
		}
		/* final */ let other: Fraction = obj as Fraction;
		return self.get_numerator() == other.get_numerator() && self.get_denominator() == other.get_denominator();
	}

	pub fn float_value(&self) -> f32 {
		return self.numerator as f32 / self.denominator as f32;
	}

	pub fn get_denominator(&self) -> i32 {
		return self.denominator;
	}

	pub fn get_numerator(&self) -> i32 {
		return self.numerator;
	}

	pub fn get_proper_numerator(&self) -> i32 {
		return Math::abs(self.numerator % self.denominator);
	}

	pub fn get_proper_whole(&self) -> i32 {
		return self.numerator / self.denominator;
	}

	pub fn hash_code(&mut self) -> i32 {
		if self.hash_code == 0 {
			// hash code update should be atomic.
			self.hash_code = Objects::hash(self.denominator, self.numerator);
		}
		return self.hash_code;
	}

	pub fn int_value(&self) -> i32 {
		return self.numerator / self.denominator;
	}

	pub fn invert(&self) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if self.numerator == 0 {
			return Err(ArithmeticException::new("Unable to invert zero."));
		}
		if self.numerator == Integer::MIN_VALUE {
			return Err(ArithmeticException::new("overflow: can't negate numerator"));
		}
		if self.numerator < 0 {
			return Fraction::new(-self.denominator, -self.numerator);
		}
		return Fraction::new(self.denominator, self.numerator);
	}

	pub fn long_value(&self) -> i64 {
		return self.numerator as i64 / self.denominator;
	}

	pub fn multiply_by(&self, fraction: &org::apache::commons::lang3::math::fraction::Fraction) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		Objects::requireNonNull(fraction, "fraction");
		if self.numerator == 0 || fraction.numerator == 0 {
			return self.ZERO;
		}
		// knuth 4.5.1
		// make sure we don't overflow unless the result *must* overflow.
		/* final */ let d1: i32 = org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(self.numerator, fraction.denominator)?;
		/* final */ let d2: i32 = org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(fraction.numerator, self.denominator)?;
		return org::apache::commons::lang3::math::fraction::Fraction::get_reduced_fraction(&org::apache::commons::lang3::math::fraction::Fraction::mul_and_check(self.numerator / d1, fraction.numerator / d2)?, &org::apache::commons::lang3::math::fraction::Fraction::mul_pos_and_check(self.denominator / d2, fraction.denominator / d1)?)?;
	}

	pub fn negate(&self) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		// the positive range is one smaller than the negative range of an int.
		if self.numerator == Integer::MIN_VALUE {
			return Err(ArithmeticException::new("overflow: too large to negate"));
		}
		return Fraction::new(-self.numerator, self.denominator);
	}

	pub fn pow(&self, power: i32) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if power == 1 {
			return self;
		}
		if power == 0 {
			return self.ONE;
		}
		if power < 0 {
			if power == Integer::MIN_VALUE {
				// MIN_VALUE can't be negated.
				return self.invert()?.pow(2)?.pow(-(power / 2))?;
			}
			return self.invert()?.pow(-power)?;
		}
		/* final */ let f: Fraction = self.multiply_by(self)?;
		if power % 2 == 0 {
			// if even...
			return f.pow(power / 2)?;
		}
		return f.pow(power / 2)?.multiply_by(self)?;
	}

	pub fn reduce(&self) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		if self.numerator == 0 {
			return  if self.equals(self.ZERO) { self } else { self.ZERO };
		}
		/* final */ let gcd: i32 = org::apache::commons::lang3::math::fraction::Fraction::greatest_common_divisor(&Math::abs(self.numerator), self.denominator)?;
		if gcd == 1 {
			return self;
		}
		return org::apache::commons::lang3::math::fraction::Fraction::get_fraction(self.numerator / gcd, self.denominator / gcd)?;
	}

	pub fn subtract(&self, fraction: &org::apache::commons::lang3::math::fraction::Fraction) /* thrown(java.lang.ArithmeticException) */ -> org::apache::commons::lang3::math::fraction::Fraction {
		return self.add_sub(fraction, false)?;
	}

	pub fn to_proper_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.to_proper_string == null {
			if self.numerator == 0 {
				self.to_proper_string = "0";
			} else if self.numerator == self.denominator {
				self.to_proper_string = "1";
			} else if self.numerator == -1 * self.denominator {
				self.to_proper_string = "-1";
			} else if ( if self.numerator > 0 { -self.numerator } else { self.numerator }) < -self.denominator {
				// note that we do the magnitude comparison test above with
				// NEGATIVE (not positive) numbers, since negative numbers
				// have a larger range. otherwise numerator == Integer.MIN_VALUE
				// is handled incorrectly.
				/* final */ let proper_numerator: i32 = self.get_proper_numerator();
				if proper_numerator == 0 {
					self.to_proper_string = Integer::toString(&self.get_proper_whole());
				} else {
					self.to_proper_string = self.get_proper_whole() + " " + proper_numerator + "/" + self.get_denominator();
				}
			} else {
				self.to_proper_string = self.get_numerator() + "/" + self.get_denominator();
			}
		}
		return self.to_proper_string;
	}

	pub fn to_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.to_string == null {
			self.to_string = self.get_numerator() + "/" + self.get_denominator();
		}
		return self.to_string;
	}
}

impl /* Java */ java::lang::Comparable /**/ for Fraction {}

impl /* Java */ java::io::Serializable /**/ for Fraction {}