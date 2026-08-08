use java::io::IOException;
use java::io::UncheckedIOException;
use java::lang::reflect::UndeclaredThrowableException;
use java::util::Arrays;
use java::util::Collection;
use java::util::Objects;
use java::util::concurrent::Callable;
use java::util::function::BiConsumer;
use java::util::function::BiFunction;
use java::util::function::BiPredicate;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::function::Supplier;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::Streams::FailableStream;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;
use crate::org::apache::commons::lang3::function::Failable;
use crate::org::apache::commons::lang3::function::FailableBooleanSupplier;

pub struct Functions;

impl Functions {
	pub fn accept<O1, O2, T: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::functions::FailableBiConsumer, object1: &O1, object2: &O2) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::functions::Functions::run(|()|consumer.accept(object1, object2))?;
	}

	pub fn accept<O, T: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::functions::FailableConsumer, object: &O) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::functions::Functions::run(|()|consumer.accept(object))?;
	}

	pub fn apply<O1, O2, O, T: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::functions::FailableBiFunction, input1: &O1, input2: &O2) /* thrown(java.lang.RuntimeException) */ -> O {
		return org::apache::commons::lang3::functions::Functions::get(|()|function.apply(input1, input2))?;
	}

	pub fn apply<I, O, T: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::functions::FailableFunction, input: &I) /* thrown(java.lang.RuntimeException) */ -> O {
		return org::apache::commons::lang3::functions::Functions::get(|()|function.apply(input))?;
	}

	pub fn as_bi_consumer<O1, O2>(&self, consumer: &org::apache::commons::lang3::functions::FailableBiConsumer) -> /* Java */ java::util::function::BiConsumer /**/ {
		return |(input1, input2)|org::apache::commons::lang3::functions::Functions::accept(consumer, input1, input2)?;
	}

	pub fn as_bi_function<O1, O2, O>(&self, function: &org::apache::commons::lang3::functions::FailableBiFunction) -> /* Java */ java::util::function::BiFunction /**/ {
		return |(input1, input2)|org::apache::commons::lang3::functions::Functions::apply(function, input1, input2)?;
	}

	pub fn as_bi_predicate<O1, O2>(&self, predicate: &org::apache::commons::lang3::functions::FailableBiPredicate) -> /* Java */ java::util::function::BiPredicate /**/ {
		return |(input1, input2)|org::apache::commons::lang3::functions::Functions::test(predicate, input1, input2);
	}

	pub fn as_callable<O>(&self, callable: &org::apache::commons::lang3::functions::FailableCallable) -> /* Java */ java::util::concurrent::Callable /**/ {
		return |()|org::apache::commons::lang3::functions::Functions::call(callable);
	}

	pub fn as_consumer<I>(&self, consumer: &org::apache::commons::lang3::functions::FailableConsumer) -> /* Java */ java::util::function::Consumer /**/ {
		return |input|org::apache::commons::lang3::functions::Functions::accept(consumer, input)?;
	}

	pub fn as_function<I, O>(&self, function: &org::apache::commons::lang3::functions::FailableFunction) -> /* Java */ java::util::function::Function /**/ {
		return |input|org::apache::commons::lang3::functions::Functions::apply(function, input)?;
	}

	pub fn as_predicate<I>(&self, predicate: &org::apache::commons::lang3::functions::FailablePredicate) -> /* Java */ java::util::function::Predicate /**/ {
		return |input|org::apache::commons::lang3::functions::Functions::test(predicate, input);
	}

	pub fn as_runnable(&self, runnable: &org::apache::commons::lang3::functions::FailableRunnable) -> /* Java */ java::lang::Runnable /**/ {
		return |()|org::apache::commons::lang3::functions::Functions::run(runnable)?;
	}

	pub fn as_supplier<O>(&self, supplier: &org::apache::commons::lang3::functions::FailableSupplier) -> /* Java */ java::util::function::Supplier /**/ {
		return |()|org::apache::commons::lang3::functions::Functions::get(supplier)?;
	}

	pub fn call<O, T: /* Java */ java::lang::Throwable /**/>(&self, callable: &org::apache::commons::lang3::functions::FailableCallable) /* thrown(java.lang.RuntimeException) */ -> O {
		return org::apache::commons::lang3::functions::Functions::get(callable::call)?;
	}

	pub fn get<O, T: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::functions::FailableSupplier) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.RuntimeException | java.io.UncheckedIOException) */ -> O {
		let r0 = 'try0: {
			return supplier.get();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::functions::Functions::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn get_as_boolean<T: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_boolean_supplier::FailableBooleanSupplier) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.RuntimeException | java.io.UncheckedIOException) */ -> bool {
		let r0 = 'try0: {
			return supplier.get_as_boolean();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::functions::Functions::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn rethrow(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.Throwable | java.io.UncheckedIOException) */ -> /* Java */ java::lang::RuntimeException /**/ {
		Objects::requireNonNull(throwable, "throwable");
		ExceptionUtils::throw_unchecked(throwable)?;
		if throwable instanceof IOException {
			return Err(UncheckedIOException::new(throwable as IOException));
		}
		return Err(UndeclaredThrowableException::new(throwable));
	}

	pub fn run<T: /* Java */ java::lang::Throwable /**/>(&self, runnable: &org::apache::commons::lang3::functions::FailableRunnable) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.Throwable | java.lang.RuntimeException | java.io.UncheckedIOException) */ {
		let r0 = 'try0: {
			runnable.run();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::functions::Functions::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn stream<O>(&self, collection: &/* Java */ java::util::Collection /**/) -> org::apache::commons::lang3::streams::FailableStream {
		return FailableStream<>::new(&collection.stream());
	}

	pub fn stream<O>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::streams::FailableStream {
		return FailableStream<>::new(stream);
	}

	pub fn test<O1, O2, T: /* Java */ java::lang::Throwable /**/>(&self, predicate: &org::apache::commons::lang3::functions::FailableBiPredicate, object1: &O1, object2: &O2) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.RuntimeException | java.io.UncheckedIOException) */ -> bool {
		return org::apache::commons::lang3::functions::Functions::get_as_boolean(|()|predicate.test(object1, object2))?;
	}

	pub fn test<O, T: /* Java */ java::lang::Throwable /**/>(&self, predicate: &org::apache::commons::lang3::functions::FailablePredicate, object: &O) /* thrown(java.lang.reflect.UndeclaredThrowableException | java.lang.RuntimeException | java.io.UncheckedIOException) */ -> bool {
		return org::apache::commons::lang3::functions::Functions::get_as_boolean(|()|predicate.test(object))?;
	}

	pub fn try_with_resources(&self, action: &org::apache::commons::lang3::functions::FailableRunnable, error_handler: &org::apache::commons::lang3::functions::FailableConsumer, resources: &org::apache::commons::lang3::functions::FailableRunnable) {
		/* final */ let fr: [Option<org.apache.commons.lang3.function.FailableRunnable>; resources.length] = [None; resources.length];
		Arrays::setAll(fr, |i||()|resources[i].run());
		Failable.tryWithResources(action::run,  if error_handler != null { errorHandler::accept } else { null }, fr);
	}

	pub fn try_with_resources(&self, action: &org::apache::commons::lang3::functions::FailableRunnable, resources: &org::apache::commons::lang3::functions::FailableRunnable) {
		org::apache::commons::lang3::functions::Functions::try_with_resources(action, null, resources);
	}

	pub fn new() -> org::apache::commons::lang3::functions::Functions {
	// empty
	}
}

pub trait FailableBiConsumer<O1, O2, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableBiFunction<O1, O2, R, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableBiPredicate<O1, O2, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableCallable<R, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableConsumer<O, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableFunction<I, R, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailablePredicate<I, T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableRunnable<T: /* Java */ java::lang::Throwable /**/>;

pub trait FailableSupplier<R, T: /* Java */ java::lang::Throwable /**/>;