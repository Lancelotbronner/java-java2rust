pub struct ConcurrentRuntimeException;

impl ConcurrentRuntimeException {
	static serialVersionUID: i64 = -6582182735562919670;

	fn new() -> org::apache::commons::lang3::concurrent::concurrent_runtime_exception::ConcurrentRuntimeException {
	}

	pub fn new(msg: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::concurrent_runtime_exception::ConcurrentRuntimeException {
		super(msg, &ConcurrentUtils::checked_exception(cause));
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::concurrent_runtime_exception::ConcurrentRuntimeException {
		super(&ConcurrentUtils::checked_exception(cause));
	}
}

impl /* Java */ java::io::Serializable /**/ for ConcurrentRuntimeException {}