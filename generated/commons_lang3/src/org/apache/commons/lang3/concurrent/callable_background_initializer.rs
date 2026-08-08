use java::util::Objects;
use java::util::concurrent::Callable;
use java::util::concurrent::ExecutorService;

pub struct CallableBackgroundInitializer<T> {
	callable: /* Java */ java::util::concurrent::Callable /**/,
}

impl<T> CallableBackgroundInitializer {
	pub fn new(call: &/* Java */ java::util::concurrent::Callable /**/) -> org::apache::commons::lang3::concurrent::callable_background_initializer::CallableBackgroundInitializer {
		self.check_callable(call);
		self.callable = call;
	}

	pub fn new(call: &/* Java */ java::util::concurrent::Callable /**/, exec: &/* Java */ java::util::concurrent::ExecutorService /**/) -> org::apache::commons::lang3::concurrent::callable_background_initializer::CallableBackgroundInitializer {
		super(exec);
		self.check_callable(call);
		self.callable = call;
	}

	fn check_callable(&self, callable: &/* Java */ java::util::concurrent::Callable /**/) {
		Objects::requireNonNull(callable, "callable");
	}

	fn get_typed_exception(&self, e: &/* Java */ java::lang::Exception /**/) -> /* Java */ java::lang::Exception /**/ {
		//This Exception object will be used for type comparison in AbstractConcurrentInitializer.initialize but not thrown
		return Exception::new(e);
	}

	fn initialize(&self) /* thrown(java.lang.Exception) */ -> T {
		return self.callable.call();
	}
}

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for CallableBackgroundInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for CallableBackgroundInitializer<T> {}