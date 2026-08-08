use java::util::concurrent::Callable;
use java::util::concurrent::FutureTask;

pub struct FutureTasks;

impl FutureTasks {
	pub fn run<V>(&self, callable: &/* Java */ java::util::concurrent::Callable /**/) -> /* Java */ java::util::concurrent::FutureTask /**/ {
		/* final */ let future_task: FutureTask<V> = FutureTask<>::new(callable);
		future_task.run();
		return future_task;
	}

	fn new() -> org::apache::commons::lang3::concurrent::future_tasks::FutureTasks {
	// No instances needed.
	}
}