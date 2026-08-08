pub struct NotImplementedException {
	code: /* Java */ java::lang::String /**/,
}

impl NotImplementedException {
	static serialVersionUID: i64 = 20131021;

	pub fn new() -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		self.code = null;
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		this(message, null as String);
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, code: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		super(message);
		self.code = code;
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		this(message, cause, null);
	}

	pub fn new(message: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/, code: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		super(message, cause);
		self.code = code;
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		this(cause, null);
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/, code: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::not_implemented_exception::NotImplementedException {
		super(cause);
		self.code = code;
	}

	pub fn get_code(&self) -> /* Java */ java::lang::String /**/ {
		return self.code;
	}
}

impl /* Java */ java::io::Serializable /**/ for NotImplementedException {}