use java::util::concurrent::CancellationException;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ConcurrentMap;
use java::util::concurrent::ExecutionException;
use java::util::concurrent::Future;
use java::util::function::Function;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;

pub struct Memoizer<I, O> {
	cache: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new(),
	mapping_function: /* Java */ java::util::function::Function /**/,
	recalculate: bool,
}

impl<I, O> Memoizer {
	pub fn new(computable: &org::apache::commons::lang3::concurrent::computable::Computable) -> org::apache::commons::lang3::concurrent::memoizer::Memoizer {
		this(computable, false);
	}

	pub fn new(computable: &org::apache::commons::lang3::concurrent::computable::Computable, recalculate: bool) -> org::apache::commons::lang3::concurrent::memoizer::Memoizer {
		self.recalculate = recalculate;
		self.mappingFunction = |k|FutureTasks::run(|()|computable.compute(k)?);
	}

	pub fn new(function: &/* Java */ java::util::function::Function /**/) -> org::apache::commons::lang3::concurrent::memoizer::Memoizer {
		this(function, false);
	}

	pub fn new(function: &/* Java */ java::util::function::Function /**/, recalculate: bool) -> org::apache::commons::lang3::concurrent::memoizer::Memoizer {
		self.recalculate = recalculate;
		self.mappingFunction = |k|FutureTasks::run(|()|function.apply(k));
	}

	pub fn compute(&self, arg: &I) /* thrown(java.lang.IllegalStateException | java.lang.InterruptedException | java.lang.RuntimeException) */ -> O {
		while true {
			/* final */ let future: Future<O> = self.cache.computeIfAbsent(arg, self.mapping_function);
			let r0 = 'try0: {
				return future.get();
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ CancellationException) => {
					self.cache.remove(arg, future);
				},
				Err(e @ ExecutionException) => {
					if self.recalculate {
						self.cache.remove(arg, future);
					}
					break 'try0 Err(self.launder_exception(&e.getCause())?);
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	fn launder_exception(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.IllegalStateException | java.lang.Throwable) */ -> /* Java */ java::lang::RuntimeException /**/ {
		return Err(IllegalStateException::new("Unchecked exception", &ExceptionUtils::throw_unchecked(throwable)?));
	}
}

impl<I, O> org::apache::commons::lang3::concurrent::computable::Computable for Memoizer<I, O> {}