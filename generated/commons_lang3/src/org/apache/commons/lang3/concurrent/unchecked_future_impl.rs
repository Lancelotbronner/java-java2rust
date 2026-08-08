use java::util::concurrent::ExecutionException;
use java::util::concurrent::Future;
use java::util::concurrent::TimeUnit;
use java::util::concurrent::TimeoutException;
use crate::org::apache::commons::lang3::exception::UncheckedInterruptedException;

struct UncheckedFutureImpl<V>;

impl<V> UncheckedFutureImpl {
	fn new(future: &/* Java */ java::util::concurrent::Future /**/) -> org::apache::commons::lang3::concurrent::unchecked_future_impl::UncheckedFutureImpl {
		super(future);
	}

	pub fn get(&self) /* thrown(java.lang.InterruptedException | java.util.concurrent.ExecutionException | org.apache.commons.lang3.concurrent.UncheckedExecutionException | org.apache.commons.lang3.exception.UncheckedInterruptedException) */ -> V {
		let r0 = 'try0: {
			return match super.get() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ InterruptedException) => {
				break 'try0 Err(UncheckedInterruptedException::new(e));
			},
			Err(e @ ExecutionException) => {
				return Err(UncheckedExecutionException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get(&self, timeout: i64, unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.InterruptedException | java.util.concurrent.ExecutionException | java.util.concurrent.TimeoutException | org.apache.commons.lang3.concurrent.UncheckedExecutionException | org.apache.commons.lang3.concurrent.UncheckedTimeoutException | org.apache.commons.lang3.exception.UncheckedInterruptedException) */ -> V {
		let r0 = 'try0: {
			return match super.get(timeout, unit) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ InterruptedException) => {
				break 'try0 Err(UncheckedInterruptedException::new(e));
			},
			Err(e @ ExecutionException) => {
				return Err(UncheckedExecutionException::new(e));
			},
			Err(e @ TimeoutException) => {
				return Err(UncheckedTimeoutException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}
}

impl<V> org::apache::commons::lang3::concurrent::unchecked_future::UncheckedFuture for UncheckedFutureImpl<V> {}

impl<V> /* Java */ java::util::concurrent::Future /**/ for UncheckedFutureImpl<V> {}

impl<V> /* Java */ java::util::concurrent::Future /**/ for UncheckedFutureImpl<V> {}