use java::security::NoSuchAlgorithmException;
use java::security::SecureRandom;
use java::security::Security;
use java::util::Random;
use java::util::concurrent::ThreadLocalRandom;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::exception::UncheckedException;

pub struct RandomUtils {
	random: /* Java */ java::util::function::Supplier /**/,
}

impl RandomUtils {
	static INSECURE: org::apache::commons::lang3::random_utils::RandomUtils = RandomUtils::new(ThreadLocalRandom::current);

	static SECURE: org::apache::commons::lang3::random_utils::RandomUtils = RandomUtils::new(SecureRandom::new);

	static SECURE_STRONG_SUPPLIER: /* Java */ java::util::function::Supplier /**/ = |()|RandomUtils::SECURE_RANDOM_STRONG.get();

	static SECURE_STRONG: org::apache::commons::lang3::random_utils::RandomUtils = RandomUtils::new(SECURE_STRONG_SUPPLIER);

	static SECURE_RANDOM_STRONG: /* Java */ java::lang::ThreadLocal /**/ = ThreadLocal::withInitial(|()|{
		let r0 = 'try0: {
			return SecureRandom::getInstanceStrong();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchAlgorithmException) => {
				break 'try0 Err(UncheckedException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	});

	pub fn insecure(&self) -> org::apache::commons::lang3::random_utils::RandomUtils {
		return self.INSECURE;
	}

	pub fn next_boolean(&self) -> bool {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_boolean();
	}

	pub fn next_bytes(&self, count: i32) -> &[i8] {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_bytes(count);
	}

	pub fn next_double(&self) -> f64 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_double();
	}

	pub fn next_double(&self, start_inclusive: f64, end_exclusive: f64) -> f64 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_double(start_inclusive, end_exclusive);
	}

	pub fn next_float(&self) -> f32 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_float();
	}

	pub fn next_float(&self, start_inclusive: f32, end_exclusive: f32) -> f32 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_float(start_inclusive, end_exclusive);
	}

	pub fn next_int(&self) -> i32 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_int();
	}

	pub fn next_int(&self, start_inclusive: i32, end_exclusive: i32) -> i32 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_int(start_inclusive, end_exclusive);
	}

	pub fn next_long(&self) -> i64 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_long();
	}

	pub fn next_long(&self, start_inclusive: i64, end_exclusive: i64) -> i64 {
		return org::apache::commons::lang3::random_utils::RandomUtils::secure().random_long(start_inclusive, end_exclusive);
	}

	pub fn secure(&self) -> org::apache::commons::lang3::random_utils::RandomUtils {
		return self.SECURE;
	}

	fn secure_random(&self) -> /* Java */ java::security::SecureRandom /**/ {
		return self.SECURE_RANDOM_STRONG.get();
	}

	pub fn secure_strong(&self) -> org::apache::commons::lang3::random_utils::RandomUtils {
		return self.SECURE_STRONG;
	}

	pub fn new() -> org::apache::commons::lang3::random_utils::RandomUtils {
		this(self.SECURE_STRONG_SUPPLIER);
	}

	fn new(random: &/* Java */ java::util::function::Supplier /**/) -> org::apache::commons::lang3::random_utils::RandomUtils {
		self.random = random;
	}

	fn random(&self) -> /* Java */ java::util::Random /**/ {
		return self.random.get();
	}

	pub fn random_boolean(&self) -> bool {
		return self.random().nextBoolean();
	}

	pub fn random_bytes(&self, count: i32) /* thrown(java.lang.IllegalArgumentException) */ -> &[i8] {
		Validate::is_true(count >= 0, "Count cannot be negative.")?;
		/* final */ let result: [i8; count] = [0; count];
		self.random().nextBytes(result);
		return result;
	}

	pub fn random_double(&self) -> f64 {
		return self.random_double(0, Double::MAX_VALUE);
	}

	pub fn random_double(&self, start_inclusive: f64, end_exclusive: f64) /* thrown(java.lang.IllegalArgumentException) */ -> f64 {
		Validate::is_true(end_exclusive >= start_inclusive, "Start value must be smaller or equal to end value.")?;
		Validate::is_true(start_inclusive >= 0, "Both range values must be non-negative.")?;
		if start_inclusive == end_exclusive {
			return start_inclusive;
		}
		return start_inclusive + (end_exclusive - start_inclusive) * self.random().nextDouble();
	}

	pub fn random_float(&self) -> f32 {
		return self.random_float(0, Float::MAX_VALUE);
	}

	pub fn random_float(&self, start_inclusive: f32, end_exclusive: f32) /* thrown(java.lang.IllegalArgumentException) */ -> f32 {
		Validate::is_true(end_exclusive >= start_inclusive, "Start value must be smaller or equal to end value.")?;
		Validate::is_true(start_inclusive >= 0, "Both range values must be non-negative.")?;
		if start_inclusive == end_exclusive {
			return start_inclusive;
		}
		return start_inclusive + (end_exclusive - start_inclusive) * self.random().nextFloat();
	}

	pub fn random_int(&self) -> i32 {
		return self.random_int(0, Integer::MAX_VALUE);
	}

	pub fn random_int(&self, start_inclusive: i32, end_exclusive: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		Validate::is_true(end_exclusive >= start_inclusive, "Start value must be smaller or equal to end value.")?;
		Validate::is_true(start_inclusive >= 0, "Both range values must be non-negative.")?;
		if start_inclusive == end_exclusive {
			return start_inclusive;
		}
		return start_inclusive + self.random().nextInt(end_exclusive - start_inclusive);
	}

	pub fn random_long(&self) -> i64 {
		return self.random_long(Long::MAX_VALUE);
	}

	fn random_long(&self, n: i64) -> i64 {
		// Extracted from o.a.c.rng.core.BaseProvider.nextLong(long)
		let bits: i64;
		let val: i64;
		loop { {
			bits = self.random().nextLong() /* unsigned */ >> 1;
			val = bits % n;
		}if !(bits - val + n - 1 < 0) break;}
		return val;
	}

	pub fn random_long(&self, start_inclusive: i64, end_exclusive: i64) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		Validate::is_true(end_exclusive >= start_inclusive, "Start value must be smaller or equal to end value.")?;
		Validate::is_true(start_inclusive >= 0, "Both range values must be non-negative.")?;
		if start_inclusive == end_exclusive {
			return start_inclusive;
		}
		return start_inclusive + self.random_long(end_exclusive - start_inclusive);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "RandomUtils [random=" + self.random() + "]";
	}
}