use java::util::concurrent::Callable;
use java::util::concurrent::CancellationException;
use java::util::concurrent::ExecutionException;
use java::util::concurrent::ExecutorService;
use java::util::concurrent::Executors;
use java::util::concurrent::Future;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct BackgroundInitializer<T> {
	external_executor: /* Java */ java::util::concurrent::ExecutorService /**/,
	executor: /* Java */ java::util::concurrent::ExecutorService /**/,
	future: /* Java */ java::util::concurrent::Future /**/,
}

impl<T> BackgroundInitializer {
	pub fn builder<T>(&self) -> org::apache::commons::lang3::concurrent::background_initializer::Builder {
		return Builder<>::new();
	}

	fn new() -> org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer {
		this(null);
	}

	fn new(exec: &/* Java */ java::util::concurrent::ExecutorService /**/) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer {
		self.set_external_executor(exec)?;
	}

	fn new(initializer: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier, closer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer, exec: &/* Java */ java::util::concurrent::ExecutorService /**/) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer {
		super(initializer, closer);
		self.set_external_executor(exec)?;
	}

	fn create_executor(&self) -> /* Java */ java::util::concurrent::ExecutorService /**/ {
		return Executors::newFixedThreadPool(&self.get_task_count());
	}

	fn create_task(&self, exec_destroy: &/* Java */ java::util::concurrent::ExecutorService /**/) -> /* Java */ java::util::concurrent::Callable /**/ {
		return InitializationTask::new(exec_destroy);
	}

	pub fn get(&self) /* thrown(java.lang.IllegalStateException | org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		let r0 = 'try0: {
			return match self.get_future() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			}.get();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ExecutionException) => {
				if let Err(e) = ConcurrentUtils::handle_cause(execex) {
					return Err(e);
				};
				// should not be reached
				return null;
			},
			Err(e @ InterruptedException) => {
				// reset interrupted state
				Thread::currentThread().interrupt();
				break 'try0 Err(ConcurrentException::new(iex));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn get_active_executor(&self) -> /* Java */ java::util::concurrent::ExecutorService /**/ {
		return self.executor;
	}

	pub fn get_external_executor(&self) -> /* Java */ java::util::concurrent::ExecutorService /**/ {
		return self.external_executor;
	}

	pub fn get_future(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::concurrent::Future /**/ {
		if self.future == null {
			return Err(IllegalStateException::new("start() must be called first!"));
		}
		return self.future;
	}

	fn get_task_count(&self) -> i32 {
		return 1;
	}

	fn get_typed_exception(&self, e: &/* Java */ java::lang::Exception /**/) -> /* Java */ java::lang::Exception /**/ {
		//This Exception object will be used for type comparison in AbstractConcurrentInitializer.initialize but not thrown
		return Exception::new(e);
	}

	pub fn is_initialized(&self) -> bool {
		if self.future == null || !self.future.isDone() {
			return false;
		}
		let r0 = 'try0: {
			self.future.get();
			return true;
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ CancellationExceptionExecutionException | InterruptedException | ) => {
				return false;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn is_started(&self) -> bool {
		return self.future != null;
	}

	pub fn set_external_executor(&mut self, external_executor: &/* Java */ java::util::concurrent::ExecutorService /**/) /* thrown(java.lang.IllegalStateException) */ {
		if self.is_started() {
			return Err(IllegalStateException::new("Cannot set ExecutorService after start()!"));
		}
		self.externalExecutor = external_executor;
	}

	pub fn start(&mut self) -> bool {
		// Not yet started?
		if !self.is_started() {
			// Determine the executor to use and whether a temporary one has to be created.
			/* final */ let temp_exec: ExecutorService;
			self.executor = self.get_external_executor();
			if self.executor == null {
				self.executor = temp_exec = self.create_executor();
			} else {
				temp_exec = null;
			}
			self.future = self.executor.submit(&self.create_task(temp_exec));
			return true;
		}
		return false;
	}
}

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for BackgroundInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for BackgroundInitializer<T> {}

pub struct Builder<I: org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer, T> {
	external_executor: /* Java */ java::util::concurrent::ExecutorService /**/,
}

impl<I: org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer, T> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::background_initializer::Builder {
	// empty
	}

	pub fn get(&self) -> I {
		return BackgroundInitializer::new(&self.get_initializer(), &self.get_closer(), self.external_executor) as I;
	}

	pub fn set_external_executor(&mut self, external_executor: &/* Java */ java::util::concurrent::ExecutorService /**/) -> org::apache::commons::lang3::concurrent::background_initializer::Builder {
		self.externalExecutor = external_executor;
		return self.as_this();
	}
}

impl<I: org::apache::commons::lang3::concurrent::background_initializer::BackgroundInitializer, T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<I, T> {}

struct InitializationTask {
	exec_finally: /* Java */ java::util::concurrent::ExecutorService /**/,
}

impl InitializationTask {
	fn new(exec: &/* Java */ java::util::concurrent::ExecutorService /**/) -> org::apache::commons::lang3::concurrent::background_initializer::InitializationTask {
		self.exec_finally = exec;
	}

	pub fn call(&self) /* thrown(java.lang.Throwable | E | java.lang.Exception) */ -> T {
		let r0 = 'try0: {
			return match self.initialize() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		if self.exec_finally != null {
			self.exec_finally.shutdown();
		}
	
	}
}

impl /* Java */ java::util::concurrent::Callable /**/ for InitializationTask {}