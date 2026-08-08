pub struct CircuitBreakingException;

impl CircuitBreakingException {
	static serialVersionUID: i64 = 1408176654686913340;

	pub fn new() -> org::apache::commons::lang3::concurrent::circuit_breaking_exception::CircuitBreakingException {
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::concurrent::circuit_breaking_exception::CircuitBreakingException {
		super(message);
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::circuit_breaking_exception::CircuitBreakingException {
		super(message, cause);
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::circuit_breaking_exception::CircuitBreakingException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for CircuitBreakingException {}