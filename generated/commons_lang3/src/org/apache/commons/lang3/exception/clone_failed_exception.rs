pub struct CloneFailedException;

impl CloneFailedException {
	static serialVersionUID: i64 = 20091223;

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::exception::clone_failed_exception::CloneFailedException {
		super(message);
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::clone_failed_exception::CloneFailedException {
		super(message, cause);
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::clone_failed_exception::CloneFailedException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for CloneFailedException {}