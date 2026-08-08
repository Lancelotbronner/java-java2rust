use java::beans::PropertyChangeListener;
use java::beans::PropertyChangeSupport;
use java::util::concurrent::atomic::AtomicReference;

pub struct AbstractCircuitBreaker<T> {
	state: /* Java */ java::util::concurrent::atomic::AtomicReference /**/ = AtomicReference<>::new(State::CLOSED),
	change_support: /* Java */ java::beans::PropertyChangeSupport /**/,
}

impl<T> AbstractCircuitBreaker {
	pub static PROPERTY_NAME: /* Java */ java::lang::String /**/ = "open";

	fn is_open(&self, state: &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::State) -> bool {
		return state == State::OPEN;
	}

	pub fn new() -> org::apache::commons::lang3::concurrent::abstract_circuit_breaker::AbstractCircuitBreaker {
		self.change_support = PropertyChangeSupport::new(self);
	}

	pub fn add_change_listener(&self, listener: &/* Java */ java::beans::PropertyChangeListener /**/) {
		self.change_support.addPropertyChangeListener(listener);
	}

	fn change_state(&self, new_state: &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::State) {
		if self.state.compareAndSet(&new_state.opposite_state(), new_state) {
			self.change_support.firePropertyChange(self.PROPERTY_NAME, !org::apache::commons::lang3::concurrent::abstract_circuit_breaker::AbstractCircuitBreaker::is_open(new_state), &org::apache::commons::lang3::concurrent::abstract_circuit_breaker::AbstractCircuitBreaker::is_open(new_state));
		}
	}

	pub fn check_state(&self) -> bool ;

	pub fn close(&self) {
		self.change_state(State::CLOSED);
	}

	pub fn increment_and_check_state(&self, increment: &T) -> bool ;

	pub fn is_closed(&self) -> bool {
		return !self.is_open();
	}

	pub fn is_open(&self) -> bool {
		return org::apache::commons::lang3::concurrent::abstract_circuit_breaker::AbstractCircuitBreaker::is_open(&self.state.get());
	}

	pub fn open(&self) {
		self.change_state(State::OPEN);
	}

	pub fn remove_change_listener(&self, listener: &/* Java */ java::beans::PropertyChangeListener /**/) {
		self.change_support.removePropertyChangeListener(listener);
	}
}

impl<T> org::apache::commons::lang3::concurrent::circuit_breaker::CircuitBreaker for AbstractCircuitBreaker<T> {}

enum State;