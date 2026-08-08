use java::util::concurrent::ConcurrentMap;
use java::util::concurrent::ExecutionException;
use java::util::concurrent::Future;
use java::util::concurrent::TimeUnit;
use crate::org::apache::commons::lang3::Validate;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;

pub struct ConcurrentUtils;

impl ConcurrentUtils {
	fn checked_exception(&self, ex: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Throwable /**/ {
		Validate::is_true(&ExceptionUtils::is_checked(ex), "Not a checked exception: %s", ex)?;
		return ex;
	}

	pub fn constant_future<T>(&self, value: &T) -> /* Java */ java::util::concurrent::Future /**/ {
		return ConstantFuture<>::new(value);
	}

	pub fn create_if_absent<K, V>(&self, map: &/* Java */ java::util::concurrent::ConcurrentMap /**/, key: &K, init: &org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException) */ -> V {
		if map == null || init == null {
			return null;
		}
		/* final */ let value: V = map.get(key);
		if value == null {
			return org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils::put_if_absent(map, key, &init.get());
		}
		return value;
	}

	pub fn create_if_absent_unchecked<K, V>(&self, map: &/* Java */ java::util::concurrent::ConcurrentMap /**/, key: &K, init: &org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException | org.apache.commons.lang3.concurrent.ConcurrentRuntimeException) */ -> V {
		let r0 = 'try0: {
			return match org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils::create_if_absent(map, key, init) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ConcurrentException) => {
				break 'try0 Err(ConcurrentRuntimeException::new(&cex.getCause()));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn extract_cause(&self, ex: &/* Java */ java::util::concurrent::ExecutionException /**/) /* thrown(java.lang.Throwable) */ -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		if ex == null || ex.getCause() == null {
			return null;
		}
		ExceptionUtils::throw_unchecked(&ex.getCause())?;
		return ConcurrentException::new(&ex.getMessage(), &ex.getCause());
	}

	pub fn extract_cause_unchecked(&self, ex: &/* Java */ java::util::concurrent::ExecutionException /**/) /* thrown(java.lang.Throwable) */ -> org::apache::commons::lang3::concurrent::concurrent_runtime_exception::ConcurrentRuntimeException {
		if ex == null || ex.getCause() == null {
			return null;
		}
		ExceptionUtils::throw_unchecked(&ex.getCause())?;
		return ConcurrentRuntimeException::new(&ex.getMessage(), &ex.getCause());
	}

	pub fn handle_cause(&self, ex: &/* Java */ java::util::concurrent::ExecutionException /**/) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException) */ {
		/* final */ let cause: ConcurrentException = org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils::extract_cause(ex)?;
		if cause != null {
			return Err(cause);
		}
	}

	pub fn handle_cause_unchecked(&self, ex: &/* Java */ java::util::concurrent::ExecutionException /**/) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentRuntimeException) */ {
		/* final */ let cause: ConcurrentRuntimeException = org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils::extract_cause_unchecked(ex)?;
		if cause != null {
			return Err(cause);
		}
	}

	pub fn initialize<T>(&self, initializer: &org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		return  if initializer != null { initializer.get() } else { null };
	}

	pub fn initialize_unchecked<T>(&self, initializer: &org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException | org.apache.commons.lang3.concurrent.ConcurrentRuntimeException) */ -> T {
		let r0 = 'try0: {
			return match org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils::initialize(initializer) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ConcurrentException) => {
				break 'try0 Err(ConcurrentRuntimeException::new(&cex.getCause()));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn put_if_absent<K, V>(&self, map: &/* Java */ java::util::concurrent::ConcurrentMap /**/, key: &K, value: &V) -> V {
		if map == null {
			return null;
		}
		/* final */ let result: V = map.putIfAbsent(key, value);
		return  if result != null { result } else { value };
	}

	fn new() -> org::apache::commons::lang3::concurrent::concurrent_utils::ConcurrentUtils {
	}
}

struct ConstantFuture<T> {
	value: T,
}

impl<T> ConstantFuture {
	fn new(value: &T) -> org::apache::commons::lang3::concurrent::concurrent_utils::ConstantFuture {
		self.value = value;
	}

	pub fn cancel(&self, may_interrupt_if_running: bool) -> bool {
		return false;
	}

	pub fn get(&self) -> T {
		return self.value;
	}

	pub fn get(&self, timeout: i64, unit: &/* Java */ java::util::concurrent::TimeUnit /**/) -> T {
		return self.value;
	}

	pub fn is_cancelled(&self) -> bool {
		return false;
	}

	pub fn is_done(&self) -> bool {
		return true;
	}
}

impl<T> /* Java */ java::util::concurrent::Future /**/ for ConstantFuture<T> {}