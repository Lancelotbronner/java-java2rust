use java::util::Objects;
use crate::org::apache::commons::lang3::builder::AbstractSupplier;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct AbstractConcurrentInitializer<T, E: /* Java */ java::lang::Exception /**/> {
	closer: org::apache::commons::lang3::function::failable_consumer::FailableConsumer,
	initializer: org::apache::commons::lang3::function::failable_supplier::FailableSupplier,
}

impl<T, E: /* Java */ java::lang::Exception /**/> AbstractConcurrentInitializer {
	pub fn new() -> org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractConcurrentInitializer {
		this(&FailableSupplier::nul(), &FailableConsumer::nop());
	}

	fn new(initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractConcurrentInitializer {
		self.closer = Objects::requireNonNull(closer, "closer");
		self.initializer = Objects::requireNonNull(initializer, "initializer");
	}

	pub fn close(&self) /* thrown(java.lang.Throwable | org.apache.commons.lang3.concurrent.ConcurrentException) */ {
		if self.is_initialized() {
			let r0 = 'try0: {
				self.closer.accept(&self.get());
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Exception) => {
					// ConcurrentException
					break 'try0 Err(ConcurrentException::new(&ExceptionUtils::throw_unchecked(e)?));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	fn get_typed_exception(&self, e: &/* Java */ java::lang::Exception /**/) -> E ;

	fn initialize(&self) /* thrown(java.lang.Throwable | E) */ -> T {
		let r0 = 'try0: {
			return self.initializer.get();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Exception) => {
				// Do this first so we don't pass a RuntimeException or Error into an exception constructor
				match ExceptionUtils::throw_unchecked(e) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				// Depending on the subclass of AbstractConcurrentInitializer E can be Exception or ConcurrentException
				// if E is Exception the if statement below will always be true, and the new Exception object created
				// in getTypedException will never be thrown. If E is ConcurrentException and the if statement is false
				// we throw the ConcurrentException returned from getTypedException, which wraps the original exception.
				/* final */ let typed_exception: E = self.get_typed_exception(e);
				if typed_exception.getClass().isAssignableFrom(&e.getClass()) {
					break 'try0 Err(e as E);
				}
				return Err(typed_exception);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn is_initialized(&self) -> bool ;
}

impl<T, E: /* Java */ java::lang::Exception /**/> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for AbstractConcurrentInitializer<T, E> {}

impl<T, E: /* Java */ java::lang::Exception /**/> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for AbstractConcurrentInitializer<T, E> {}

pub struct AbstractBuilder<I: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractConcurrentInitializer, T, B: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractBuilder, E: /* Java */ java::lang::Exception /**/> {
	closer: org::apache::commons::lang3::function::failable_consumer::FailableConsumer = FailableConsumer::nop(),
	initializer: org::apache::commons::lang3::function::failable_supplier::FailableSupplier = FailableSupplier::nul(),
}

impl<I: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractConcurrentInitializer, T, B: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractBuilder, E: /* Java */ java::lang::Exception /**/> AbstractBuilder {
	pub fn new() -> org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractBuilder {
	// empty
	}

	pub fn get_closer(&self) -> org::apache::commons::lang3::function::failable_consumer::FailableConsumer {
		return self.closer;
	}

	pub fn get_initializer(&self) -> org::apache::commons::lang3::function::failable_supplier::FailableSupplier {
		return self.initializer;
	}

	pub fn set_closer(&mut self, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> B {
		self.closer =  if closer != null { closer } else { FailableConsumer::nop() };
		return self.as_this();
	}

	pub fn set_initializer(&mut self, initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier) -> B {
		self.initializer =  if initializer != null { initializer } else { FailableSupplier::nul() };
		return self.as_this();
	}
}

impl<I: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractConcurrentInitializer, T, B: org::apache::commons::lang3::concurrent::abstract_concurrent_initializer::AbstractBuilder, E: /* Java */ java::lang::Exception /**/> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for AbstractBuilder<I, T, B, E> {}