use java::util::Collections;
use java::util::HashMap;
use java::util::Map;
use java::util::NoSuchElementException;
use java::util::Objects;
use java::util::Set;
use java::util::concurrent::ExecutorService;

pub struct MultiBackgroundInitializer {
	child_initializers: /* Java */ java::util::Map /**/ = HashMap<>::new(),
}

impl MultiBackgroundInitializer {
	pub fn new() -> org::apache::commons::lang3::concurrent::multi_background_initializer::MultiBackgroundInitializer {
	}

	pub fn new(exec: &/* Java */ java::util::concurrent::ExecutorService /**/) -> org::apache::commons::lang3::concurrent::multi_background_initializer::MultiBackgroundInitializer {
		super(exec);
	}

	pub fn add_initializer(&self, name: &/* Java */ java::lang::String /**/, background_initializer: &org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer) /* thrown(java.lang.IllegalStateException) */ {
		Objects::requireNonNull(name, "name");
		Objects::requireNonNull(background_initializer, "backgroundInitializer");
		synchronized (self) {
			if self.is_started() {
				return Err(IllegalStateException::new("addInitializer() must not be called after start()!"));
			}
			self.child_initializers.put(name, background_initializer);
		}
	}

	pub fn close(&self) /* thrown(java.lang.Throwable | org.apache.commons.lang3.concurrent.ConcurrentException) */ {
		let exception: ConcurrentException = null;
		for /* final */ child in self.child_initializers.values() {
			let r0 = 'try0: {
				if let Err(e) = child.close() {
					return Err(e);
				};
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Exception) => {
					if exception == null {
						exception = ConcurrentException::new();
					}
					if e instanceof ConcurrentException {
						// Because ConcurrentException is only created by classes in this package
						// we can safely unwrap it.
						exception.addSuppressed(&e.getCause());
					} else {
						exception.addSuppressed(e);
					}
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		if exception != null {
			break 'try0 Err(exception);
		}
	}

	fn get_task_count(&self) -> i32 {
		return 1 + self.child_initializers.values().stream().mapToInt(BackgroundInitializer::getTaskCount).sum();
	}

	fn initialize(&self) /* thrown(java.lang.Exception) */ -> org::apache::commons::lang3::concurrent::multi_background_initializer::MultiBackgroundInitializerResults {
		/* final */ let inits: Map<String, BackgroundInitializer<?>>;
		synchronized (self) {
			// create a snapshot to operate on
			inits = HashMap<>::new(self.child_initializers);
		}
		// start the child initializers
		/* final */ let exec: ExecutorService = self.get_active_executor();
		inits.values().forEach(|bi|{
			if bi.getExternalExecutor() == null {
				// share the executor service if necessary
				bi.setExternalExecutor(exec);
			}
			bi.start();
		});
		// collect the results
		/* final */ let results: Map<String, Object> = HashMap<>::new();
		/* final */ let excepts: Map<String, ConcurrentException> = HashMap<>::new();
		inits.forEach(|(k, v)|{
			let r0 = 'try0: {
				results.put(k, &match v.get() {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				});
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ ConcurrentException) => {
					excepts.put(k, cex);
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		});
		return MultiBackgroundInitializerResults::new(inits, results, excepts);
	}

	pub fn is_initialized(&self) -> bool {
		if self.child_initializers.isEmpty() {
			return false;
		}
		return self.child_initializers.values().stream().allMatch(BackgroundInitializer::isInitialized);
	}
}

impl org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for MultiBackgroundInitializer {}

impl org::apache::commons::lang3::function::failable_supplier::FailableSupplier for MultiBackgroundInitializer {}

pub struct MultiBackgroundInitializerResults {
	initializers: /* Java */ java::util::Map /**/,
	result_objects: /* Java */ java::util::Map /**/,
	exceptions: /* Java */ java::util::Map /**/,
}

impl MultiBackgroundInitializerResults {
	fn new(initializers: &/* Java */ java::util::Map /**/, result_objects: &/* Java */ java::util::Map /**/, exceptions: &/* Java */ java::util::Map /**/) -> org::apache::commons::lang3::concurrent::multi_background_initializer::MultiBackgroundInitializerResults {
		self.initializers = initializers;
		self.resultObjects = result_objects;
		self.exceptions = exceptions;
	}

	fn check_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.util.NoSuchElementException) */ -> org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer {
		/* final */ let init: BackgroundInitializer<?> = self.initializers.get(name);
		if init == null {
			return Err(NoSuchElementException::new("No child initializer with name " + name));
		}
		return init;
	}

	pub fn get_exception(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.util.NoSuchElementException) */ -> org::apache::commons::lang3::concurrent::concurrent_exception::ConcurrentException {
		self.check_name(name)?;
		return self.exceptions.get(name);
	}

	pub fn get_initializer(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.util.NoSuchElementException) */ -> org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer {
		return self.check_name(name)?;
	}

	pub fn get_result_object(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.util.NoSuchElementException) */ -> /* Java */ java::lang::Object /**/ {
		self.check_name(name)?;
		return self.result_objects.get(name);
	}

	pub fn initializer_names(&self) -> /* Java */ java::util::Set /**/ {
		return Collections::unmodifiableSet(&self.initializers.keySet());
	}

	pub fn is_exception(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.util.NoSuchElementException) */ -> bool {
		self.check_name(name)?;
		return self.exceptions.containsKey(name);
	}

	pub fn is_successful(&self) -> bool {
		return self.exceptions.isEmpty();
	}
}