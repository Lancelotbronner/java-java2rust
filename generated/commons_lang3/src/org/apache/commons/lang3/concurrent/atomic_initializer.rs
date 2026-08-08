use java::util::concurrent::atomic::AtomicReference;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct AtomicInitializer<T> {
	reference: /* Java */ java::util::concurrent::atomic::AtomicReference /**/ = AtomicReference<>::new(&self.get_no_init()),
}

impl<T> AtomicInitializer {
	static NO_INIT: /* Java */ java::lang::Object /**/ = Object::new();

	pub fn builder<T>(&self) -> org::apache::commons::lang3::concurrent::atomic_initializer::Builder {
		return Builder<>::new();
	}

	pub fn new() -> org::apache::commons::lang3::concurrent::atomic_initializer::AtomicInitializer {
	// empty
	}

	fn new(initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> org::apache::commons::lang3::concurrent::atomic_initializer::AtomicInitializer {
		super(initializer, closer);
	}

	pub fn get(&self) /* thrown(E | java.lang.Throwable | org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		let result: T = self.reference.get();
		if result == self.get_no_init() {
			result = self.initialize()?;
			if !self.reference.compareAndSet(&self.get_no_init(), result) {
				// another thread has initialized the reference
				result = self.reference.get();
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

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for AtomicInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for AtomicInitializer<T> {}

pub struct Builder<I: org::apache::commons::lang3::concurrent::atomic_initializer::AtomicInitializer, T>;

impl<I: org::apache::commons::lang3::concurrent::atomic_initializer::AtomicInitializer, T> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::atomic_initializer::Builder {
	// empty
	}

	pub fn get(&self) -> I {
		return AtomicInitializer::new(&self.get_initializer(), &self.get_closer()) as I;
	}
}

impl<I: org::apache::commons::lang3::concurrent::atomic_initializer::AtomicInitializer, T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<I, T> {}