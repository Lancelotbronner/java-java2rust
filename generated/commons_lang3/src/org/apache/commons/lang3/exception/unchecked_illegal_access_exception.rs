pub struct UncheckedIllegalAccessException;

impl UncheckedIllegalAccessException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::unchecked_illegal_access_exception::UncheckedIllegalAccessException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedIllegalAccessException {}