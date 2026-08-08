use java::util::concurrent::atomic::AtomicReference;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct AtomicSafeInitializer<T> {
	factory: /* Java */ java::util::concurrent::atomic::AtomicReference /**/ = AtomicReference<>::new(),
	reference: /* Java */ java::util::concurrent::atomic::AtomicReference /**/ = AtomicReference<>::new(&self.get_no_init()),
}

impl<T> AtomicSafeInitializer {
	static NO_INIT: /* Java */ java::lang::Object /**/ = Object::new();

	pub fn builder<T>(&self) -> org::apache::commons::lang3::concurrent::atomic_safe_initializer::Builder {
		return Builder<>::new();
	}

	pub fn new() -> org::apache::commons::lang3::concurrent::atomic_safe_initializer::AtomicSafeInitializer {
	// empty
	}

	fn new(initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> org::apache::commons::lang3::concurrent::atomic_safe_initializer::AtomicSafeInitializer {
		super(initializer, closer);
	}

	pub fn get(&self) /* thrown(java.lang.Throwable | E | org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		let result: T;
		while (result = self.reference.get()) == self.get_no_init() {
			if self.factory.compareAndSet(null, self) {
				let r0 = 'try0: {
					self.reference.set(&match self.initialize() {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					});
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ Throwable) => {
						// Allow retry on failure; otherwise callers spin forever.
						self.factory.set(null);
						// Rethrow preserving original semantics: unchecked as-is, checked wrapped.
						/* final */ let checked: Throwable = match ExceptionUtils::throw_unchecked(t) {
							Err(e) => break 'try0 Err(e),
							Ok(s) => s,
						};
						break 'try0 Err( if checked instanceof ConcurrentException { checked as ConcurrentException } else { ConcurrentException::new(checked) });
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		return result;
	}

	fn get_no_init(&self) -> T {
		return self.NO_INIT as T;
	}

	fn get_typed_exception(&self, e: &/* Java */ java::lang::Exception /**/) -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		return ConcurrentException::new(e);
	}

	pub fn is_initialized(&self) -> bool {
		return self.reference.get() != self.NO_INIT;
	}
}

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for AtomicSafeInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for AtomicSafeInitializer<T> {}

pub struct Builder<I: org::apache::commons::lang3::concurrent::atomic_safe_initializer::AtomicSafeInitializer, T>;

impl<I: org::apache::commons::lang3::concurrent::atomic_safe_initializer::AtomicSafeInitializer, T> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::atomic_safe_initializer::Builder {
	// empty
	}

	pub fn get(&self) -> I {
		return AtomicSafeInitializer::new(&self.get_initializer(), &self.get_closer()) as I;
	}
}

impl<I: org::apache::commons::lang3::concurrent::atomic_safe_initializer::AtomicSafeInitializer, T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<I, T> {}