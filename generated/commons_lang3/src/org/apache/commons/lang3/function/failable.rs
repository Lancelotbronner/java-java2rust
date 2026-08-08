use java::io::IOException;
use java::io::UncheckedIOException;
use java::lang::reflect::UndeclaredThrowableException;
use java::util::Collection;
use java::util::Objects;
use java::util::Optional;
use java::util::concurrent::Callable;
use java::util::function::BiConsumer;
use java::util::function::BiFunction;
use java::util::function::BiPredicate;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::function::Supplier;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;
use crate::org::apache::commons::lang3::stream::Streams;
use crate::org::apache::commons::lang3::stream::Streams::FailableStream;

pub struct Failable;

impl Failable {
	pub fn accept<T, U, E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, object1: &T, object2: &U) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::function::failable::Failable::run(consumer, |()|consumer.accept(object1, object2))?;
	}

	pub fn accept<T, E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer, object: &T) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::function::failable::Failable::run(consumer, |()|consumer.accept(object))?;
	}

	pub fn accept<E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_double_consumer::FailableDoubleConsumer, value: f64) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::function::failable::Failable::run(consumer, |()|consumer.accept(value))?;
	}

	pub fn accept<E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_int_consumer::FailableIntConsumer, value: i32) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::function::failable::Failable::run(consumer, |()|consumer.accept(value))?;
	}

	pub fn accept<E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_long_consumer::FailableLongConsumer, value: i64) /* thrown(java.lang.RuntimeException) */ {
		org::apache::commons::lang3::function::failable::Failable::run(consumer, |()|consumer.accept(value))?;
	}

	pub fn apply<T, U, R, E: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::function::failable_bi_function::FailableBiFunction, input1: &T, input2: &U) /* thrown(java.lang.RuntimeException) */ -> R {
		return org::apache::commons::lang3::function::failable::Failable::get(|()|function.apply(input1, input2))?;
	}

	pub fn apply<T, R, E: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::function::failable_function::FailableFunction, input: &T) /* thrown(java.lang.RuntimeException) */ -> R {
		return org::apache::commons::lang3::function::failable::Failable::get(|()|function.apply(input))?;
	}

	pub fn apply_as_double<E: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::function::failable_double_binary_operator::FailableDoubleBinaryOperator, left: f64, right: f64) /* thrown(java.lang.RuntimeException) */ -> f64 {
		return org::apache::commons::lang3::function::failable::Failable::get_as_double(|()|function.apply_as_double(left, right))?;
	}

	pub fn apply_non_null<T, R, E: /* Java */ java::lang::Throwable /**/>(&self, value: &T, mapper: &org::apache::commons::lang3::function::failable_function::FailableFunction) /* thrown(E) */ -> R {
		return  if value != null { Objects::requireNonNull(mapper, "mapper").apply(value) } else { null };
	}

	pub fn apply_non_null<T, U, R, E1: /* Java */ java::lang::Throwable /**/, E2: /* Java */ java::lang::Throwable /**/>(&self, value1: &T, mapper1: &org::apache::commons::lang3::function::failable_function::FailableFunction, mapper2: &org::apache::commons::lang3::function::failable_function::FailableFunction) /* thrown(E | E1 | E2) */ -> R {
		return org::apache::commons::lang3::function::failable::Failable::apply_non_null(&org::apache::commons::lang3::function::failable::Failable::apply_non_null(value1, mapper1)?, mapper2)?;
	}

	pub fn apply_non_null<T, U, V, R, E1: /* Java */ java::lang::Throwable /**/, E2: /* Java */ java::lang::Throwable /**/, E3: /* Java */ java::lang::Throwable /**/>(&self, value1: &T, mapper1: &org::apache::commons::lang3::function::failable_function::FailableFunction, mapper2: &org::apache::commons::lang3::function::failable_function::FailableFunction, mapper3: &org::apache::commons::lang3::function::failable_function::FailableFunction) /* thrown(E | E1 | E2 | E3) */ -> R {
		return org::apache::commons::lang3::function::failable::Failable::apply_non_null(&org::apache::commons::lang3::function::failable::Failable::apply_non_null(&org::apache::commons::lang3::function::failable::Failable::apply_non_null(value1, mapper1)?, mapper2)?, mapper3)?;
	}

	pub fn as_bi_consumer<T, U>(&self, consumer: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer) -> /* Java */ java::util::function::BiConsumer /**/ {
		return |(input1, input2)|org::apache::commons::lang3::function::failable::Failable::accept(consumer, input1, input2)?;
	}

	pub fn as_bi_function<T, U, R>(&self, function: &org::apache::commons::lang3::function::failable_bi_function::FailableBiFunction) -> /* Java */ java::util::function::BiFunction /**/ {
		return |(input1, input2)|org::apache::commons::lang3::function::failable::Failable::apply(function, input1, input2)?;
	}

	pub fn as_bi_predicate<T, U>(&self, predicate: &org::apache::commons::lang3::function::failable_bi_predicate::FailableBiPredicate) -> /* Java */ java::util::function::BiPredicate /**/ {
		return |(input1, input2)|org::apache::commons::lang3::function::failable::Failable::test(predicate, input1, input2);
	}

	pub fn as_callable<V>(&self, callable: &org::apache::commons::lang3::function::failable_callable::FailableCallable) -> /* Java */ java::util::concurrent::Callable /**/ {
		return |()|org::apache::commons::lang3::function::failable::Failable::call(callable);
	}

	pub fn as_consumer<T>(&self, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) -> /* Java */ java::util::function::Consumer /**/ {
		return |input|org::apache::commons::lang3::function::failable::Failable::accept(consumer, input)?;
	}

	pub fn as_function<T, R>(&self, function: &org::apache::commons::lang3::function::failable_function::FailableFunction) -> /* Java */ java::util::function::Function /**/ {
		return |input|org::apache::commons::lang3::function::failable::Failable::apply(function, input)?;
	}

	pub fn as_predicate<T>(&self, predicate: &org::apache::commons::lang3::function::failable_predicate::FailablePredicate) -> /* Java */ java::util::function::Predicate /**/ {
		return |input|org::apache::commons::lang3::function::failable::Failable::test(predicate, input);
	}

	pub fn as_runnable(&self, runnable: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) -> /* Java */ java::lang::Runnable /**/ {
		return |()|org::apache::commons::lang3::function::failable::Failable::run(runnable)?;
	}

	pub fn as_supplier<T>(&self, supplier: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier) -> /* Java */ java::util::function::Supplier /**/ {
		return |()|org::apache::commons::lang3::function::failable::Failable::get(supplier)?;
	}

	pub fn call<V, E: /* Java */ java::lang::Throwable /**/>(&self, callable: &org::apache::commons::lang3::function::failable_callable::FailableCallable) /* thrown(java.lang.RuntimeException) */ -> V {
		return org::apache::commons::lang3::function::failable::Failable::get(callable::call)?;
	}

	pub fn get<T, E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> T {
		let r0 = 'try0: {
			return supplier.get();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_as_boolean<E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_boolean_supplier::FailableBooleanSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> bool {
		let r0 = 'try0: {
			return supplier.get_as_boolean();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_as_double<E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_double_supplier::FailableDoubleSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> f64 {
		let r0 = 'try0: {
			return supplier.get_as_double();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_as_int<E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_int_supplier::FailableIntSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> i32 {
		let r0 = 'try0: {
			return supplier.get_as_int();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_as_long<E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_long_supplier::FailableLongSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> i64 {
		let r0 = 'try0: {
			return supplier.get_as_long();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_as_short<E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_short_supplier::FailableShortSupplier) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> i16 {
		let r0 = 'try0: {
			return supplier.get_as_short();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn rethrow(&self, throwable: &/* Java */ java::lang::Throwable /**/) /* thrown(T | java.io.UncheckedIOException | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ -> /* Java */ java::lang::RuntimeException /**/ {
		Objects::requireNonNull(throwable, "throwable");
		ExceptionUtils::throw_unchecked(throwable)?;
		if throwable instanceof IOException {
			return Err(UncheckedIOException::new(throwable as IOException));
		}
		return Err(UndeclaredThrowableException::new(throwable));
	}

	pub fn run<E: /* Java */ java::lang::Throwable /**/>(&self, runnable: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(T | java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ {
		if runnable != null {
			let r0 = 'try0: {
				runnable.run();
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	fn run<E: /* Java */ java::lang::Throwable /**/>(&self, test: &/* Java */ java::lang::Object /**/, runnable: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(T | java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ {
		if runnable != null && test != null {
			let r0 = 'try0: {
				runnable.run();
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Throwable) => {
					break 'try0 Err(org::apache::commons::lang3::function::failable::Failable::rethrow(t)?);
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	pub fn stream<E>(&self, collection: &/* Java */ java::util::Collection /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return FailableStream<>::new(&collection.stream());
	}

	pub fn stream<T>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return FailableStream<>::new(stream);
	}

	pub fn test<T, U, E: /* Java */ java::lang::Throwable /**/>(&self, predicate: &org::apache::commons::lang3::function::failable_bi_predicate::FailableBiPredicate, object1: &T, object2: &U) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> bool {
		return org::apache::commons::lang3::function::failable::Failable::get_as_boolean(|()|predicate.test(object1, object2))?;
	}

	pub fn test<T, E: /* Java */ java::lang::Throwable /**/>(&self, predicate: &org::apache::commons::lang3::function::failable_predicate::FailablePredicate, object: &T) /* thrown(java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.reflect.UndeclaredThrowableException) */ -> bool {
		return org::apache::commons::lang3::function::failable::Failable::get_as_boolean(|()|predicate.test(object))?;
	}

	pub fn try_with_resources(&self, action: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable, error_handler: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer, resources: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(T | java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ {
		/* final */ let actual_error_handler: FailableConsumer<Throwable, ? extends Throwable>;
		if error_handler == null {
			actual_error_handler = Failable::rethrow;
		} else {
			actual_error_handler = error_handler;
		}
		Streams::of(resources).forEach(|r|Objects::requireNonNull(r, "runnable"));
		let th: Throwable = null;
		let r0 = 'try0: {
			action.run();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ Throwable) => {
				th = t;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		if resources != null {
			for /* final */ runnable in resources {
				let r1 = 'try1: {
					runnable.run();
					break 'try1 Ok(());
				};
				match r1 {
					Err(e @ Throwable) => {
						if th == null {
							th = t;
						}
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
		if th != null {
			let r2 = 'try2: {
				actual_error_handler.accept(th);
				break 'try2 Ok(());
			};
			match r2 {
				Err(e @ Throwable) => {
					break 'try2 Err(match org::apache::commons::lang3::function::failable::Failable::rethrow(t) {
						Err(e) => break 'try1 Err(e),
						Ok(s) => s,
					});
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
	}

	pub fn try_with_resources(&self, action: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable, resources: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(T | java.io.UncheckedIOException | java.lang.RuntimeException | java.lang.Throwable | java.lang.reflect.UndeclaredThrowableException) */ {
		org::apache::commons::lang3::function::failable::Failable::try_with_resources(action, null, resources)?;
	}

	fn new() -> org::apache::commons::lang3::function::failable::Failable {
	// empty
	}
}