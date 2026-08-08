pub struct UncheckedInterruptedException;

impl UncheckedInterruptedException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::exception::unchecked_interrupted_exception::UncheckedInterruptedException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedInterruptedException {}