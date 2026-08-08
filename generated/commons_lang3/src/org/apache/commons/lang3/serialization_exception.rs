pub struct SerializationException;

impl SerializationException {
	static serialVersionUID: i64 = 4029025366392702726;

	pub fn new() -> org::apache::commons::lang3::serialization_exception::SerializationException {
	}

	pub fn new(msg: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::serialization_exception::SerializationException {
		super(msg);
	}

	pub fn new(msg: &/* Java */ java::lang::String /**/, cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::serialization_exception::SerializationException {
		super(msg, cause);
	}

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::serialization_exception::SerializationException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for SerializationException {}