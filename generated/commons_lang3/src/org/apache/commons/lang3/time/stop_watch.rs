use java::time::Duration;
use java::time::Instant;
use java::util::ArrayList;
use java::util::Collections;
use java::util::List;
use java::util::Objects;
use java::util::concurrent::TimeUnit;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableRunnable;
use crate::org::apache::commons::lang3::function::FailableSupplier;
use crate::org::apache::commons::lang3::tuple::ImmutablePair;

pub struct StopWatch {
	message: /* Java */ java::lang::String /**/,
	running_state: org::apache::commons::lang3::time::stop_watch::State = State::UNSTARTED,
	split_state: org::apache::commons::lang3::time::stop_watch::SplitState = SplitState::UNSPLIT,
	start_time_nanos: i64,
	start_instant: /* Java */ java::time::Instant /**/,
	stop_instant: /* Java */ java::time::Instant /**/,
	stop_time_nanos: i64,
	splits: /* Java */ java::util::List /**/ = ArrayList<>::new(),
}

impl StopWatch {
	static NANO_2_MILLIS: i64 = 1_000_000;

	pub fn create(&self) -> org::apache::commons::lang3::time::stop_watch::StopWatch {
		return StopWatch::new();
	}

	pub fn create_started(&self) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::time::stop_watch::StopWatch {
		/* final */ let sw: StopWatch = StopWatch::new();
		sw.start()?;
		return sw;
	}

	pub fn new() -> org::apache::commons::lang3::time::stop_watch::StopWatch {
		this(null);
	}

	pub fn new(message: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::time::stop_watch::StopWatch {
		self.message = message;
	}

	pub fn format_split_time(&self) -> /* Java */ java::lang::String /**/ {
		return DurationFormatUtils::format_durationhms(&self.get_split_duration().toMillis());
	}

	pub fn format_time(&self) -> /* Java */ java::lang::String /**/ {
		return DurationFormatUtils::format_durationhms(&self.get_time());
	}

	pub fn get<T>(&self, supplier: &/* Java */ java::util::function::Supplier /**/) /* thrown(java.lang.IllegalStateException) */ -> T {
		self.start_resume();
		let r0 = 'try0: {
			return supplier.get();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		if let Err(e) = self.suspend() {
			return Err(e);
		};
	
	}

	pub fn get_duration(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::time::Duration /**/ {
		return Duration::ofNanos(&self.get_nano_time()?);
	}

	pub fn get_message(&self) -> /* Java */ java::lang::String /**/ {
		return self.message;
	}

	pub fn get_splits(&self) -> /* Java */ java::util::List /**/ {
		return Collections::unmodifiableList(self.splits);
	}

	pub fn get_nano_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		match self.running_state {
			STOPPED =>  {
			}
			SUSPENDED =>  {
				return self.stop_time_nanos - self.start_time_nanos;
			}
			UNSTARTED =>  {
				return 0;
			}
			RUNNING =>  {
				return System::nanoTime() - self.start_time_nanos;
			}
			_ =>  {
				break;
			}
		}
		return Err(IllegalStateException::new("Illegal running state has occurred."));
	}

	pub fn get_split_duration(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::time::Duration /**/ {
		return Duration::ofNanos(&self.get_split_nano_time()?);
	}

	pub fn get_split_nano_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		if self.split_state != SplitState::SPLIT {
			return Err(IllegalStateException::new("Stopwatch must be split to get the split time."));
		}
		return self.splits.get(self.splits.size() - 1).get_right().toNanos();
	}

	pub fn get_split_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		return self.nanos_to_millis(&self.get_split_nano_time()?);
	}

	pub fn get_start_instant(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::time::Instant /**/ {
		if self.running_state == State::UNSTARTED {
			return Err(IllegalStateException::new("Stopwatch has not been started"));
		}
		return self.start_instant;
	}

	pub fn get_start_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		return self.get_start_instant()?.toEpochMilli();
	}

	pub fn get_stop_instant(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::time::Instant /**/ {
		if self.running_state == State::UNSTARTED {
			return Err(IllegalStateException::new("Stopwatch has not been started"));
		}
		return self.stop_instant;
	}

	pub fn get_stop_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		// stopTimeNanos stores System.nanoTime() for elapsed time
		return self.get_stop_instant()?.toEpochMilli();
	}

	pub fn gett<T, E: /* Java */ java::lang::Throwable /**/>(&self, supplier: &org::apache::commons::lang3::function::failable_supplier::FailableSupplier) /* thrown(E | java.lang.IllegalStateException | java.lang.Throwable) */ -> T {
		self.start_resume();
		let r0 = 'try0: {
			return match supplier.get() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		if let Err(e) = self.suspend() {
			return Err(e);
		};
	
	}

	pub fn get_time(&self) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		return self.nanos_to_millis(&self.get_nano_time()?);
	}

	pub fn get_time(&self, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.IllegalStateException) */ -> i64 {
		return time_unit.convert(&self.get_nano_time()?, TimeUnit::NANOSECONDS);
	}

	pub fn is_started(&self) -> bool {
		return self.running_state.is_started();
	}

	pub fn is_stopped(&self) -> bool {
		return self.running_state.is_stopped();
	}

	pub fn is_suspended(&self) -> bool {
		return self.running_state.is_suspended();
	}

	fn nanos_to_millis(&self, nanos: i64) -> i64 {
		return nanos / self.NANO_2_MILLIS;
	}

	pub fn reset(&mut self) {
		self.running_state = State::UNSTARTED;
		self.split_state = SplitState::UNSPLIT;
		self.splits.clear();
	}

	pub fn resume(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state != State::SUSPENDED {
			return Err(IllegalStateException::new("Stopwatch must be suspended to resume."));
		}
		self.start_time_nanos += System::nanoTime() - self.stop_time_nanos;
		self.running_state = State::RUNNING;
	}

	pub fn run(&self, runnable: &/* Java */ java::lang::Runnable /**/) /* thrown(java.lang.IllegalStateException) */ {
		self.start_resume();
		let r0 = 'try0: {
			runnable.run();
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		if let Err(e) = self.suspend() {
			return Err(e);
		};
	
	}

	pub fn runt<E: /* Java */ java::lang::Throwable /**/>(&self, runnable: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(E | java.lang.IllegalStateException | java.lang.Throwable) */ {
		self.start_resume();
		let r0 = 'try0: {
			if let Err(e) = runnable.run() {
				return Err(e);
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		if let Err(e) = self.suspend() {
			return Err(e);
		};
	
	}

	pub fn split(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state != State::RUNNING {
			return Err(IllegalStateException::new("Stopwatch is not running."));
		}
		self.stop_time_nanos = System::nanoTime();
		self.split_state = SplitState::SPLIT;
		self.splits.add(Split::new(&String::valueOf(&self.splits.size()), &Duration::ofNanos(self.stop_time_nanos - self.start_time_nanos)));
	}

	pub fn split(&mut self, label: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state != State::RUNNING {
			return Err(IllegalStateException::new("Stopwatch is not running."));
		}
		self.stop_time_nanos = System::nanoTime();
		self.split_state = SplitState::SPLIT;
		self.splits.add(Split::new(label, &Duration::ofNanos(self.stop_time_nanos - self.start_time_nanos)));
	}

	pub fn start(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state == State::STOPPED {
			return Err(IllegalStateException::new("Stopwatch must be reset before being restarted."));
		}
		if self.running_state != State::UNSTARTED {
			return Err(IllegalStateException::new("Stopwatch already started."));
		}
		self.start_time_nanos = System::nanoTime();
		self.start_instant = Instant::now();
		self.running_state = State::RUNNING;
		self.splits.clear();
	}

	fn start_resume(&self) /* thrown(java.lang.IllegalStateException) */ {
		if self.is_stopped() {
			self.start()?;
		} else if self.is_suspended() {
			self.resume()?;
		}
	}

	pub fn stop(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state != State::RUNNING && self.running_state != State::SUSPENDED {
			return Err(IllegalStateException::new("Stopwatch is not running."));
		}
		if self.running_state == State::RUNNING {
			self.stop_time_nanos = System::nanoTime();
			self.stop_instant = Instant::now();
		}
		self.running_state = State::STOPPED;
	}

	pub fn suspend(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.running_state != State::RUNNING {
			return Err(IllegalStateException::new("Stopwatch must be running to suspend."));
		}
		self.stop_time_nanos = System::nanoTime();
		self.stop_instant = Instant::now();
		self.running_state = State::SUSPENDED;
	}

	pub fn to_split_string(&self) -> /* Java */ java::lang::String /**/ {
		/* final */ let msg_str: String = Objects::toString(self.message, StringUtils::EMPTY);
		/* final */ let formatted_time: String = self.format_split_time();
		return  if msg_str.isEmpty() { formatted_time } else { msg_str + StringUtils::SPACE + formatted_time };
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		/* final */ let msg_str: String = Objects::toString(self.message, StringUtils::EMPTY);
		/* final */ let formatted_time: String = self.format_time();
		return  if msg_str.isEmpty() { formatted_time } else { msg_str + StringUtils::SPACE + formatted_time };
	}

	pub fn unsplit(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		if self.split_state != SplitState::SPLIT {
			return Err(IllegalStateException::new("Stopwatch has not been split."));
		}
		self.split_state = SplitState::UNSPLIT;
		self.splits.remove(self.splits.size() - 1);
	}
}

enum SplitState;

enum State;

pub struct Split;

impl Split {
	pub fn new(label: &/* Java */ java::lang::String /**/, duration: &/* Java */ java::time::Duration /**/) -> org::apache::commons::lang3::time::stop_watch::Split {
		super(label, duration);
	}

	pub fn get_label(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_left();
	}

	pub fn get_duration(&self) -> /* Java */ java::time::Duration /**/ {
		return self.get_right();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("Split [%s, %s])", &self.get_label(), &self.get_duration());
	}
}

impl /* Java */ java::util::Map::Entry /**/ for Split {}

impl /* Java */ java::lang::Comparable /**/ for Split {}

impl /* Java */ java::io::Serializable /**/ for Split {}