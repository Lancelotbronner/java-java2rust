use java::beans::PropertyChangeListener;
use java::util::EnumMap;
use java::util::Map;
use java::util::concurrent::TimeUnit;
use java::util::concurrent::atomic::AtomicReference;

pub struct EventCountCircuitBreaker {
	check_interval_data: /* Java */ java::util::concurrent::atomic::AtomicReference /**/,
	opening_threshold: i32,
	opening_interval: i64,
	closing_threshold: i32,
	closing_interval: i64,
}

impl EventCountCircuitBreaker {
	static STRATEGY_MAP: /* Java */ java::util::Map /**/ = org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker::create_strategy_map();

	fn create_strategy_map(&self) -> /* Java */ java::util::Map /**/ {
		/* final */ let map: Map<State, AbstractStateStrategy> = EnumMap<>::new(State.class);
		map.put(State::CLOSED, StateStrategyClosed::new());
		map.put(State::OPEN, StateStrategyOpen::new());
		return map;
	}

	fn state_strategy(&self, state: &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::State) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::AbstractStateStrategy {
		return self.STRATEGY_MAP.get(state);
	}

	pub fn new(threshold: i32, check_interval: i64, check_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker {
		this(threshold, check_interval, check_unit, threshold);
	}

	pub fn new(opening_threshold: i32, check_interval: i64, check_unit: &/* Java */ java::util::concurrent::TimeUnit /**/, closing_threshold: i32) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker {
		this(opening_threshold, check_interval, check_unit, closing_threshold, check_interval, check_unit);
	}

	pub fn new(opening_threshold: i32, opening_interval: i64, opening_unit: &/* Java */ java::util::concurrent::TimeUnit /**/, closing_threshold: i32, closing_interval: i64, closing_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker {
		self.check_interval_data = AtomicReference<>::new(CheckIntervalData::new(0, 0));
		self.openingThreshold = opening_threshold;
		self.openingInterval = opening_unit.toNanos(opening_interval);
		self.closingThreshold = closing_threshold;
		self.closingInterval = closing_unit.toNanos(closing_interval);
	}

	fn change_state_and_start_new_check_interval(&self, new_state: &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::State) {
		self.change_state(new_state);
		self.check_interval_data.set(CheckIntervalData::new(0, &self.nano_time()));
	}

	pub fn check_state(&self) -> bool {
		return self.perform_state_check(0);
	}

	pub fn close(&self) {
		super.close();
		self.check_interval_data.set(CheckIntervalData::new(0, &self.nano_time()));
	}

	pub fn get_closing_interval(&self) -> i64 {
		return self.closing_interval;
	}

	pub fn get_closing_threshold(&self) -> i32 {
		return self.closing_threshold;
	}

	pub fn get_opening_interval(&self) -> i64 {
		return self.opening_interval;
	}

	pub fn get_opening_threshold(&self) -> i32 {
		return self.opening_threshold;
	}

	pub fn increment_and_check_state(&self) -> bool {
		return self.increment_and_check_state(1);
	}

	pub fn increment_and_check_state(&self, increment: &/* Java */ java::lang::Integer /**/) -> bool {
		return self.perform_state_check(increment);
	}

	fn nano_time(&self) -> i64 {
		return System::nanoTime();
	}

	fn next_check_interval_data(&self, increment: i32, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, current_state: &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::State, time: i64) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData {
		/* final */ let next_data: CheckIntervalData;
		if org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker::state_strategy(current_state).is_check_interval_finished(self, current_data, time) {
			next_data = CheckIntervalData::new(increment, time);
		} else {
			next_data = current_data.increment(increment);
		}
		return next_data;
	}

	pub fn open(&self) {
		super.open();
		self.check_interval_data.set(CheckIntervalData::new(0, &self.nano_time()));
	}

	fn perform_state_check(&self, increment: i32) -> bool {
		let current_data: CheckIntervalData;
		let next_data: CheckIntervalData;
		let current_state: State;
		loop { {
			/* final */ let time: i64 = self.nano_time();
			current_state = .get();
			current_data = self.check_interval_data.get();
			next_data = self.next_check_interval_data(increment, current_data, current_state, time);
		}if !(!self.update_check_interval_data(current_data, next_data)) break;}
		// Refer to the header comment!
		if org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker::state_strategy(current_state).is_state_transition(self, current_data, next_data) {
			current_state = current_state.opposite_state();
			self.change_state_and_start_new_check_interval(current_state);
		}
		return !org::apache::commons::lang3::concurrent::abstract_circuit_breaker::AbstractCircuitBreaker::is_open(current_state);
	}

	fn update_check_interval_data(&self, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, next_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData) -> bool {
		return current_data == next_data || self.check_interval_data.compareAndSet(current_data, next_data);
	}
}

impl org::apache::commons::lang3::concurrent::circuit_breaker::CircuitBreaker for EventCountCircuitBreaker {}

struct AbstractStateStrategy;

impl AbstractStateStrategy {
	fn fetch_check_interval(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker) -> i64 ;

	pub fn is_check_interval_finished(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, now: i64) -> bool {
		return now - current_data.get_check_interval_start() > self.fetch_check_interval(breaker);
	}

	pub fn is_state_transition(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, next_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData) -> bool ;
}

struct CheckIntervalData {
	event_count: i32,
	check_interval_start: i64,
}

impl CheckIntervalData {
	fn new(count: i32, interval_start: i64) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData {
		self.event_count = count;
		self.check_interval_start = interval_start;
	}

	pub fn get_check_interval_start(&self) -> i64 {
		return self.check_interval_start;
	}

	pub fn get_event_count(&self) -> i32 {
		return self.event_count;
	}

	pub fn increment(&self, delta: i32) -> org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData {
		return  if delta == 0 { self } else { CheckIntervalData::new(self.get_event_count() + delta, &self.get_check_interval_start()) };
	}
}

struct StateStrategyClosed;

impl StateStrategyClosed {
	fn fetch_check_interval(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker) -> i64 {
		return breaker.get_opening_interval();
	}

	pub fn is_state_transition(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, next_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData) -> bool {
		return next_data.get_event_count() > breaker.get_opening_threshold();
	}
}

struct StateStrategyOpen;

impl StateStrategyOpen {
	fn fetch_check_interval(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker) -> i64 {
		return breaker.get_closing_interval();
	}

	pub fn is_state_transition(&self, breaker: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::EventCountCircuitBreaker, current_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData, next_data: &org::apache::commons::lang3::concurrent::event_count_circuit_breaker::CheckIntervalData) -> bool {
		return next_data.get_check_interval_start() != current_data.get_check_interval_start() && current_data.get_event_count() < breaker.get_closing_threshold();
	}
}