use java::util::concurrent::ScheduledExecutorService;
use java::util::concurrent::ScheduledFuture;
use java::util::concurrent::ScheduledThreadPoolExecutor;
use java::util::concurrent::TimeUnit;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::Validate;

pub struct TimedSemaphore {
	executor_service: /* Java */ java::util::concurrent::ScheduledExecutorService /**/,
	period: i64,
	unit: /* Java */ java::util::concurrent::TimeUnit /**/,
	own_executor: bool,
	task: /* Java */ java::util::concurrent::ScheduledFuture /**/,
	total_acquire_count: i64,
	period_count: i64,
	limit: i32,
	acquire_count: i32,
	last_calls_per_period: i32,
	shutdown: bool,
}

impl TimedSemaphore {
	pub static NO_LIMIT: i32 = 0;

	static THREAD_POOL_SIZE: i32 = 1;

	pub fn builder(&self) -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
		return Builder::new();
	}

	fn new(builder: &org::apache::commons::lang3::concurrent::timed_semaphore::Builder) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::concurrent::timed_semaphore::TimedSemaphore {
		Validate::inclusive_between(1, Long::MAX_VALUE, builder.period, "Time period must be greater than 0.")?;
		self.period = builder.period;
		self.unit = builder.timeUnit;
		if builder.service != null {
			self.executor_service = builder.service;
			self.own_executor = false;
		} else {
			/* final */ let stpe: ScheduledThreadPoolExecutor = ScheduledThreadPoolExecutor::new(self.THREAD_POOL_SIZE);
			stpe.setContinueExistingPeriodicTasksAfterShutdownPolicy(false);
			stpe.setExecuteExistingDelayedTasksAfterShutdownPolicy(false);
			self.executor_service = stpe;
			self.own_executor = true;
		}
		self.set_limit(builder.limit);
	}

	pub fn new(time_period: i64, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/, limit: i32) -> org::apache::commons::lang3::concurrent::timed_semaphore::TimedSemaphore {
		this(null, time_period, time_unit, limit);
	}

	pub fn new(service: &/* Java */ java::util::concurrent::ScheduledExecutorService /**/, time_period: i64, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/, limit: i32) -> org::apache::commons::lang3::concurrent::timed_semaphore::TimedSemaphore {
		this(&org::apache::commons::lang3::concurrent::timed_semaphore::TimedSemaphore::builder().set_service(service).set_period(time_period).set_time_unit(time_unit).set_limit(limit));
	}

	pub fn acquire(&self) /* thrown(java.lang.InterruptedException | java.lang.IllegalStateException) */ {
		self.prepare_acquire()?;
		let can_pass: bool;
		loop { {
			can_pass = self.acquire_permit();
			if !can_pass {
				self.wait();
			}
		}if !(!can_pass) break;}
	}

	fn acquire_permit(&self) -> bool {
		if self.get_limit() <= self.NO_LIMIT || self.acquire_count < self.get_limit() {
			self.acquire_count += 1;
			return true;
		}
		return false;
	}

	fn end_of_period(&mut self) {
		self.last_calls_per_period = self.acquire_count;
		self.total_acquire_count += self.acquire_count;
		self.period_count += 1;
		self.acquire_count = 0;
		self.notifyAll();
	}

	pub fn get_acquire_count(&self) -> i32 {
		return self.acquire_count;
	}

	pub fn get_available_permits(&self) -> i32 {
		return self.get_limit() - self.get_acquire_count();
	}

	pub fn get_average_calls_per_period(&self) -> f64 {
		return  if self.period_count == 0 { 0 } else { self.total_acquire_count as f64 / self.period_count as f64 };
	}

	fn get_executor_service(&self) -> /* Java */ java::util::concurrent::ScheduledExecutorService /**/ {
		return self.executor_service;
	}

	pub fn get_last_acquires_per_period(&self) -> i32 {
		return self.last_calls_per_period;
	}

	pub fn get_limit(&self) -> i32 {
		return self.limit;
	}

	pub fn get_period(&self) -> i64 {
		return self.period;
	}

	pub fn get_unit(&self) -> /* Java */ java::util::concurrent::TimeUnit /**/ {
		return self.unit;
	}

	pub fn is_shutdown(&self) -> bool {
		return self.shutdown;
	}

	fn prepare_acquire(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.is_shutdown() {
			return Err(IllegalStateException::new("TimedSemaphore is shut down!"));
		}
		if self.task == null {
			self.task = self.start_timer();
		}
	}

	pub fn set_limit(&mut self, limit: i32) {
		self.limit = limit;
	}

	pub fn shutdown(&mut self) {
		if !self.shutdown {
			if self.own_executor {
				// if the executor was created by this instance, it has
				// to be shutdown
				self.get_executor_service().shutdownNow();
			}
			if self.task != null {
				self.task.cancel(false);
			}
			self.shutdown = true;
		}
	}

	fn start_timer(&self) -> /* Java */ java::util::concurrent::ScheduledFuture /**/ {
		return self.get_executor_service().scheduleAtFixedRate(self::endOfPeriod, &self.get_period(), &self.get_period(), &self.get_unit());
	}

	pub fn try_acquire(&self) /* thrown(java.lang.IllegalStateException) */ -> bool {
		self.prepare_acquire()?;
		return self.acquire_permit();
	}
}

pub struct Builder {
	service: /* Java */ java::util::concurrent::ScheduledExecutorService /**/,
	period: i64,
	time_unit: /* Java */ java::util::concurrent::TimeUnit /**/,
	limit: i32,
}

impl Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::concurrent::timed_semaphore::TimedSemaphore {
		return TimedSemaphore::new(self);
	}

	pub fn set_limit(&mut self, limit: i32) -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
		self.limit = limit;
		return self;
	}

	pub fn set_period(&mut self, period: i64) -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
		self.period = period;
		return self;
	}

	pub fn set_service(&mut self, service: &/* Java */ java::util::concurrent::ScheduledExecutorService /**/) -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
		self.service = service;
		return self;
	}

	pub fn set_time_unit(&mut self, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) -> org::apache::commons::lang3::concurrent::timed_semaphore::Builder {
		self.timeUnit = time_unit;
		return self;
	}
}

impl /* Java */ java::util::function::Supplier /**/ for Builder {}