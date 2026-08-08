pub struct UncheckedException;

impl UncheckedException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::unchecked_exception::UncheckedException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedException {}