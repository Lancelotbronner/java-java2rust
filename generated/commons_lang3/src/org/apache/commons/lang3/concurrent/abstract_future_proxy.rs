use java::util::Objects;
use java::util::concurrent::ExecutionException;
use java::util::concurrent::Future;
use java::util::concurrent::TimeUnit;
use java::util::concurrent::TimeoutException;

pub struct AbstractFutureProxy<V> {
	future: /* Java */ java::util::concurrent::Future /**/,
}

impl<V> AbstractFutureProxy {
	pub fn new(future: &/* Java */ java::util::concurrent::Future /**/) -> org::apache::commons::lang3::concurrent::abstract_future_proxy::AbstractFutureProxy {
		self.future = Objects::requireNonNull(future, "future");
	}

	pub fn cancel(&self, may_interrupt_if_running: bool) -> bool {
		return self.future.cancel(may_interrupt_if_running);
	}

	pub fn get(&self) /* thrown(java.lang.InterruptedException | java.util.concurrent.ExecutionException) */ -> V {
		return self.future.get();
	}

	pub fn get(&self, timeout: i64, unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.InterruptedException | java.util.concurrent.ExecutionException | java.util.concurrent.TimeoutException) */ -> V {
		return self.future.get(timeout, unit);
	}

	pub fn get_future(&self) -> /* Java */ java::util::concurrent::Future /**/ {
		return self.future;
	}

	pub fn is_cancelled(&self) -> bool {
		return self.future.isCancelled();
	}

	pub fn is_done(&self) -> bool {
		return self.future.isDone();
	}
}

impl<V> /* Java */ java::util::concurrent::Future /**/ for AbstractFutureProxy<V> {}