use java::util::concurrent::TimeoutException;
use crate::org::apache::commons::lang3::exception::UncheckedException;

pub struct UncheckedTimeoutException;

impl UncheckedTimeoutException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::unchecked_timeout_exception::UncheckedTimeoutException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedTimeoutException {}