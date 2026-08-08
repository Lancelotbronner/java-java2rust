use java::security::SecureRandom;
use java::security::Security;
use java::util::Random;
use java::util::concurrent::ThreadLocalRandom;
use java::util::function::Supplier;

pub struct RandomStringUtils {
	random: /* Java */ java::util::function::Supplier /**/,
}

impl RandomStringUtils {
	static SECURE_SUPPLIER: /* Java */ java::util::function::Supplier /**/ = RandomUtils::secure;

	static INSECURE: org::apache::commons::lang3::random_string_utils::RandomStringUtils = RandomStringUtils::new(RandomUtils::insecure);

	static SECURE: org::apache::commons::lang3::random_string_utils::RandomStringUtils = RandomStringUtils::new(SECURE_SUPPLIER);

	static SECURE_STRONG: org::apache::commons::lang3::random_string_utils::RandomStringUtils = RandomStringUtils::new(RandomUtils::secureStrong);

	static ALPHANUMERICAL_CHARS: &[u16] = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ]
	;

	static ASCII_0: i32 = '0';

	static ASCII_9: i32 = '9';

	static ASCII_A: i32 = 'A';

	static ASCII_z: i32 = 'z';

	static CACHE_PADDING_BITS: i32 = 3;

	static BITS_TO_BYTES_DIVISOR: i32 = 5;

	static BASE_CACHE_SIZE_PADDING: i32 = 10;

	pub fn insecure(&self) -> org::apache::commons::lang3::random_string_utils::RandomStringUtils {
		return self.INSECURE;
	}

	pub fn random(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count);
	}

	pub fn random(&self, count: i32, letters: bool, numbers: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count, letters, numbers);
	}

	pub fn random(&self, count: i32, chars: u16) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count, chars);
	}

	pub fn random(&self, count: i32, start: i32, end: i32, letters: bool, numbers: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count, start, end, letters, numbers);
	}

	pub fn random(&self, count: i32, start: i32, end: i32, letters: bool, numbers: bool, chars: u16) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count, start, end, letters, numbers, chars);
	}

	pub fn random(&self, count: i32, mut start: i32, mut end: i32, letters: bool, numbers: bool, chars: &&[u16], random: &/* Java */ java::util::Random /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if count == 0 {
			return StringUtils::EMPTY;
		}
		if count < 0 {
			return Err(IllegalArgumentException::new("Requested random string length " + count + " is less than 0."));
		}
		if chars != null && chars.length == 0 {
			return Err(IllegalArgumentException::new("The chars array must not be empty"));
		}
		if start == 0 && end == 0 {
			if chars != null {
				end = chars.length;
			} else if !letters && !numbers {
				end = Character::MAX_CODE_POINT;
			} else {
				end = 'z' + 1;
				start = ' ';
			}
		} else if end <= start {
			return Err(IllegalArgumentException::new("Parameter end (" + end + ") must be greater than start (" + start + ")"));
		} else if start < 0 || end < 0 {
			return Err(IllegalArgumentException::new("Character positions MUST be >= 0"));
		}
		if end > Character::MAX_CODE_POINT {
			// Technically, it should be `Character.MAX_CODE_POINT+1` as `end` is excluded
			// But the character `Character.MAX_CODE_POINT` is private use, so it would anyway be excluded
			end = Character::MAX_CODE_POINT;
		}
		// Optimizations and tests when chars == null and using ASCII characters (end <= 0x7f)
		if chars == null && end <= 0x7f {
			// picking a 6-bit integer and only rejecting with probability 2 / 64 = 1 / 32
			if letters && numbers && start <= self.ASCII_0 && end >= self.ASCII_z + 1 {
				return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, 0, 0, false, false, self.ALPHANUMERICAL_CHARS, random)?;
			}
			if numbers && end <= self.ASCII_0 || letters && end <= self.ASCII_A {
				return Err(IllegalArgumentException::new("Parameter end (" + end + ") must be greater then (" + self.ASCII_0 + ") for generating digits " + "or greater then (" + self.ASCII_A + ") for generating letters."));
			}
			// even after this optimization.
			if letters && numbers {
				start = Math::max(self.ASCII_0, start);
				end = Math::min(self.ASCII_z + 1, end);
			} else if numbers {
				// just numbers, no letters
				start = Math::max(self.ASCII_0, start);
				end = Math::min(self.ASCII_9 + 1, end);
			} else if letters {
				// just letters, no numbers
				start = Math::max(self.ASCII_A, start);
				end = Math::min(self.ASCII_z + 1, end);
			}
		}
		/* final */ let builder: StringBuilder = StringBuilder::new(count);
		/* final */ let gap: i32 = end - start;
		/* final */ let gap_bits: i32 = Integer::SIZE - Integer::numberOfLeadingZeros(gap);
		// The size of the cache we use is an heuristic:
		// about twice the number of bytes required if no rejection
		// Ideally the cache size depends on multiple factor, including the cost of generating x bytes
		// of randomness as well as the probability of rejection. It is however not easy to know
		// those values programmatically for the general case.
		// Calculate cache size:
		// 1. Multiply count by bits needed per character (gapBits)
		// 2. Add padding bits (3) to handle partial bytes
		// 3. Divide by 5 to convert to bytes (normally this would be by 8, dividing by 5 allows for about 60% extra space)
		// 4. Add base padding (10) to handle small counts efficiently
		// 5. Ensure we don't exceed Integer.MAX_VALUE / 5 + 10 to provide a good balance between overflow prevention and
		//    making the cache extremely large
		/* final */ let desired_cache_size: i64 = (count as i64 * gap_bits + self.CACHE_PADDING_BITS) / self.BITS_TO_BYTES_DIVISOR + self.BASE_CACHE_SIZE_PADDING;
		/* final */ let cache_size: i32 = Math::min(desired_cache_size, Integer::MAX_VALUE / self.BITS_TO_BYTES_DIVISOR + self.BASE_CACHE_SIZE_PADDING) as i32;
		/* final */ let arb: CachedRandomBits = CachedRandomBits::new(cache_size, random);
		while count -= 1 !!!check!!! post decrement != 0 {
			// Generate a random value between start (included) and end (excluded)
			/* final */ let random_value: i32 = arb.next_bits(gap_bits)? + start;
			// Rejection sampling if value too large
			if random_value >= end {
				count += 1;
				continue;
			}
			/* final */ let code_point: i32;
			if chars == null {
				code_point = random_value;
				match Character::getType(code_point) {
					Character::UNASSIGNED =>  {
					}
					Character::PRIVATE_USE =>  {
					}
					Character::SURROGATE =>  {
						count += 1;
						continue;
					}
				}
			} else {
				code_point = chars[random_value];
			}
			/* final */ let number_of_chars: i32 = Character::charCount(code_point);
			if count == 0 && number_of_chars > 1 {
				count += 1;
				continue;
			}
			if letters && Character::isLetter(code_point) || numbers && Character::isDigit(code_point) || !letters && !numbers {
				builder.appendCodePoint(code_point);
				if number_of_chars == 2 {
					count -= 1;
				}
			} else {
				count += 1;
			}
		}
		return builder.toString();
	}

	pub fn random(&self, count: i32, chars: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next(count, chars);
	}

	pub fn random_alphabetic(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_alphabetic(count);
	}

	pub fn random_alphabetic(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_alphabetic(min_length_inclusive, max_length_exclusive);
	}

	pub fn random_alphanumeric(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_alphanumeric(count);
	}

	pub fn random_alphanumeric(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_alphanumeric(min_length_inclusive, max_length_exclusive);
	}

	pub fn random_ascii(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_ascii(count);
	}

	pub fn random_ascii(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_ascii(min_length_inclusive, max_length_exclusive);
	}

	pub fn random_graph(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_graph(count);
	}

	pub fn random_graph(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_graph(min_length_inclusive, max_length_exclusive);
	}

	pub fn random_numeric(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_numeric(count);
	}

	pub fn random_numeric(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_numeric(min_length_inclusive, max_length_exclusive);
	}

	pub fn random_print(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_print(count);
	}

	pub fn random_print(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::secure().next_print(min_length_inclusive, max_length_exclusive);
	}

	pub fn secure(&self) -> org::apache::commons::lang3::random_string_utils::RandomStringUtils {
		return self.SECURE;
	}

	pub fn secure_strong(&self) -> org::apache::commons::lang3::random_string_utils::RandomStringUtils {
		return self.SECURE_STRONG;
	}

	pub fn new() -> org::apache::commons::lang3::random_string_utils::RandomStringUtils {
		this(self.SECURE_SUPPLIER);
	}

	fn new(random: &/* Java */ java::util::function::Supplier /**/) -> org::apache::commons::lang3::random_string_utils::RandomStringUtils {
		self.random = random;
	}

	pub fn next(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return self.next(count, false, false);
	}

	pub fn next(&self, count: i32, letters: bool, numbers: bool) -> /* Java */ java::lang::String /**/ {
		return self.next(count, 0, 0, letters, numbers);
	}

	pub fn next(&self, count: i32, chars: u16) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if chars == null {
			return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, 0, 0, false, false, null, &self.random())?;
		}
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, 0, chars.length, false, false, chars, &self.random())?;
	}

	pub fn next(&self, count: i32, start: i32, end: i32, letters: bool, numbers: bool) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, start, end, letters, numbers, null, &self.random())?;
	}

	pub fn next(&self, count: i32, start: i32, end: i32, letters: bool, numbers: bool, chars: u16) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, start, end, letters, numbers, chars, &self.random())?;
	}

	pub fn next(&self, count: i32, chars: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if chars == null {
			return org::apache::commons::lang3::random_string_utils::RandomStringUtils::random(count, 0, 0, false, false, null, &self.random())?;
		}
		return self.next(count, &chars.toCharArray())?;
	}

	pub fn next_alphabetic(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return self.next(count, true, false);
	}

	pub fn next_alphabetic(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return self.next_alphabetic(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive));
	}

	pub fn next_alphanumeric(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return self.next(count, true, true);
	}

	pub fn next_alphanumeric(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return self.next_alphanumeric(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive));
	}

	pub fn next_ascii(&self, count: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next(count, 32, 127, false, false)?;
	}

	pub fn next_ascii(&self, min_length_inclusive: i32, max_length_exclusive: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next_ascii(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive))?;
	}

	pub fn next_graph(&self, count: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next(count, 33, 126, false, false)?;
	}

	pub fn next_graph(&self, min_length_inclusive: i32, max_length_exclusive: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next_graph(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive))?;
	}

	pub fn next_numeric(&self, count: i32) -> /* Java */ java::lang::String /**/ {
		return self.next(count, false, true);
	}

	pub fn next_numeric(&self, min_length_inclusive: i32, max_length_exclusive: i32) -> /* Java */ java::lang::String /**/ {
		return self.next_numeric(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive));
	}

	pub fn next_print(&self, count: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next(count, 32, 126, false, false)?;
	}

	pub fn next_print(&self, min_length_inclusive: i32, max_length_exclusive: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.next_print(&self.random_utils().random_int(min_length_inclusive, max_length_exclusive))?;
	}

	fn random(&self) -> /* Java */ java::util::Random /**/ {
		return self.random_utils().random();
	}

	fn random_utils(&self) -> org::apache::commons::lang3::random_utils::RandomUtils {
		return self.random.get();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "RandomStringUtils [random=" + self.random() + "]";
	}
}