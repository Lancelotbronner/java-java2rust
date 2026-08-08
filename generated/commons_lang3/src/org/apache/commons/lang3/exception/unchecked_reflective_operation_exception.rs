pub struct UncheckedReflectiveOperationException;

impl UncheckedReflectiveOperationException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::unchecked_reflective_operation_exception::UncheckedReflectiveOperationException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedReflectiveOperationException {}