use java::lang::Thread::UncaughtExceptionHandler;
use java::util::Objects;
use java::util::concurrent::ExecutorService;
use java::util::concurrent::Executors;
use java::util::concurrent::ThreadFactory;
use java::util::concurrent::atomic::AtomicLong;

pub struct BasicThreadFactory {
	thread_counter: /* Java */ java::util::concurrent::atomic::AtomicLong /**/,
	wrapped_factory: /* Java */ java::util::concurrent::ThreadFactory /**/,
	uncaught_exception_handler: /* Java */ java::lang::Thread::UncaughtExceptionHandler /**/,
	naming_pattern: /* Java */ java::lang::String /**/,
	priority: /* Java */ java::lang::Integer /**/,
	daemon: /* Java */ java::lang::Boolean /**/,
}

impl BasicThreadFactory {
	pub fn builder(&self) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		return Builder::new();
	}

	fn new(builder: &org::apache::commons::lang3::concurrent::basic_thread_factory::Builder) -> org::apache::commons::lang3::concurrent::basic_thread_factory::BasicThreadFactory {
		self.wrapped_factory =  if builder.factory != null { builder.factory } else { Executors::defaultThreadFactory() };
		self.naming_pattern = builder.namingPattern;
		self.priority = builder.priority;
		self.daemon = builder.daemon;
		self.uncaught_exception_handler = builder.exceptionHandler;
		self.thread_counter = AtomicLong::new();
	}

	pub fn get_daemon_flag(&self) -> /* Java */ java::lang::Boolean /**/ {
		return self.daemon;
	}

	pub fn get_naming_pattern(&self) -> /* Java */ java::lang::String /**/ {
		return self.naming_pattern;
	}

	pub fn get_priority(&self) -> /* Java */ java::lang::Integer /**/ {
		return self.priority;
	}

	pub fn get_thread_count(&self) -> i64 {
		return self.thread_counter.get();
	}

	pub fn get_uncaught_exception_handler(&self) -> /* Java */ java::lang::Thread::UncaughtExceptionHandler /**/ {
		return self.uncaught_exception_handler;
	}

	pub fn get_wrapped_factory(&self) -> /* Java */ java::util::concurrent::ThreadFactory /**/ {
		return self.wrapped_factory;
	}

	fn initialize_thread(&self, thread: &/* Java */ java::lang::Thread /**/) {
		if self.get_naming_pattern() != null {
			/* final */ let count: Long = Long::valueOf(&self.thread_counter.incrementAndGet());
			thread.setName(&String::format(&self.get_naming_pattern(), count));
		}
		if self.get_uncaught_exception_handler() != null {
			thread.setUncaughtExceptionHandler(&self.get_uncaught_exception_handler());
		}
		if self.get_priority() != null {
			thread.setPriority(&self.get_priority().intValue());
		}
		if self.get_daemon_flag() != null {
			thread.setDaemon(&self.get_daemon_flag().booleanValue());
		}
	}

	pub fn new_thread(&self, runnable: &/* Java */ java::lang::Runnable /**/) -> /* Java */ java::lang::Thread /**/ {
		/* final */ let thread: Thread = self.get_wrapped_factory().newThread(runnable);
		self.initialize_thread(thread);
		return thread;
	}
}

impl /* Java */ java::util::concurrent::ThreadFactory /**/ for BasicThreadFactory {}

pub struct Builder {
	factory: /* Java */ java::util::concurrent::ThreadFactory /**/,
	exception_handler: /* Java */ java::lang::Thread::UncaughtExceptionHandler /**/,
	naming_pattern: /* Java */ java::lang::String /**/,
	priority: /* Java */ java::lang::Integer /**/,
	daemon: /* Java */ java::lang::Boolean /**/,
}

impl Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
	// empty
	}

	pub fn build(&self) -> org::apache::commons::lang3::concurrent::basic_thread_factory::BasicThreadFactory {
		/* final */ let factory: BasicThreadFactory = BasicThreadFactory::new(self);
		self.reset();
		return factory;
	}

	pub fn daemon(&self) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		return self.daemon(true);
	}

	pub fn daemon(&mut self, daemon: bool) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		self.daemon = Boolean::valueOf(daemon);
		return self;
	}

	pub fn naming_pattern(&mut self, naming_pattern: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		self.namingPattern = Objects::requireNonNull(naming_pattern, "pattern");
		return self;
	}

	pub fn priority(&mut self, priority: i32) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		self.priority = Integer::valueOf(priority);
		return self;
	}

	pub fn reset(&mut self) {
		self.factory = null;
		self.exception_handler = null;
		self.naming_pattern = null;
		self.priority = null;
		self.daemon = null;
	}

	pub fn uncaught_exception_handler(&mut self, exception_handler: &/* Java */ java::lang::Thread::UncaughtExceptionHandler /**/) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		self.exceptionHandler = Objects::requireNonNull(exception_handler, "handler");
		return self;
	}

	pub fn wrapped_factory(&mut self, factory: &/* Java */ java::util::concurrent::ThreadFactory /**/) -> org::apache::commons::lang3::concurrent::basic_thread_factory::Builder {
		self.factory = Objects::requireNonNull(factory, "factory");
		return self;
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for Builder {}