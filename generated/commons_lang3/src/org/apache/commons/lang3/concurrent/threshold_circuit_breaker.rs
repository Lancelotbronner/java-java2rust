use java::util::concurrent::atomic::AtomicLong;

pub struct ThresholdCircuitBreaker {
	threshold: i64,
	used: /* Java */ java::util::concurrent::atomic::AtomicLong /**/,
}

impl ThresholdCircuitBreaker {
	static INITIAL_COUNT: i64 = 0;

	pub fn new(threshold: i64) -> org::apache::commons::lang3::concurrent::threshold_circuit_breaker::ThresholdCircuitBreaker {
		self.used = AtomicLong::new(self.INITIAL_COUNT);
		self.threshold = threshold;
	}

	pub fn check_state(&self) -> bool {
		return !self.is_open();
	}

	pub fn close(&self) {
		super.close();
		self.used.set(self.INITIAL_COUNT);
	}

	pub fn get_threshold(&self) -> i64 {
		return self.threshold;
	}

	pub fn increment_and_check_state(&self, increment: &/* Java */ java::lang::Long /**/) -> bool {
		if self.threshold == 0 {
			self.open();
		}
		/* final */ let used: i64 = self.used.addAndGet(increment);
		if used > self.threshold {
			self.open();
		}
		return self.check_state();
	}
}

impl org::apache::commons::lang3::concurrent::circuit_breaker::CircuitBreaker for ThresholdCircuitBreaker {}