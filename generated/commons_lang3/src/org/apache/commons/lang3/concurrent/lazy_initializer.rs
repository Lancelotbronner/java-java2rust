use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct LazyInitializer<T> {
	object: T = NO_INIT as T,
}

impl<T> LazyInitializer {
	static NO_INIT: /* Java */ java::lang::Object /**/ = Object::new();

	pub fn builder<T>(&self) -> org::apache::commons::lang3::concurrent::lazy_initializer::Builder {
		return Builder<>::new();
	}

	pub fn new() -> org::apache::commons::lang3::concurrent::lazy_initializer::LazyInitializer {
	// empty
	}

	fn new(initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> org::apache::commons::lang3::concurrent::lazy_initializer::LazyInitializer {
		super(initializer, closer);
	}

	pub fn get(&mut self) /* thrown(java.lang.Throwable | E | org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		// use a temporary variable to reduce the number of reads of the
		// volatile field
		let result: T = self.object;
		if result == self.NO_INIT {
			synchronized (self) {
				result = self.object;
				if result == self.NO_INIT {
					self.object = result = self.initialize()?;
				}
			}
		}
		return result;
	}

	fn get_typed_exception(&self, e: &/* Java */ java::lang::Exception /**/) -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		return ConcurrentException::new(e);
	}

	pub fn is_initialized(&self) -> bool {
		return self.object != self.NO_INIT;
	}
}

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for LazyInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for LazyInitializer<T> {}

pub struct Builder<I: org::apache::commons::lang3::concurrent::lazy_initializer::LazyInitializer, T>;

impl<I: org::apache::commons::lang3::concurrent::lazy_initializer::LazyInitializer, T> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::lazy_initializer::Builder {
	// empty
	}

	pub fn get(&self) -> I {
		return LazyInitializer::new(&self.get_initializer(), &self.get_closer()) as I;
	}
}

impl<I: org::apache::commons::lang3::concurrent::lazy_initializer::LazyInitializer, T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<I, T> {}