use java::util::concurrent::ExecutionException;
use crate::org::apache::commons::lang3::exception::UncheckedException;

pub struct UncheckedExecutionException;

impl UncheckedExecutionException {
	static serialVersionUID: i64 = 1;

	pub fn new(cause: &/* Java */ java::lang::Throwable /**/) -> org::apache::commons::lang3::concurrent::unchecked_execution_exception::UncheckedExecutionException {
		super(cause);
	}
}

impl /* Java */ java::io::Serializable /**/ for UncheckedExecutionException {}