use java::util::concurrent::ExecutionException;

pub struct ConcurrentException;

impl ConcurrentException {
	static serialVersionUID: i64 = 6622707671812226130;

	fn new() -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		super(message);
	}

	pub fn new(msg: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		super(msg, &ConcurrentUtils::checked_exception(cause));
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		super(&ConcurrentUtils::checked_exception(cause));
	}
}

impl /* Java */ java::io::Serializable /**/ for ConcurrentException {}