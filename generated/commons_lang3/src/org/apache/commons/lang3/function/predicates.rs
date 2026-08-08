use java::util::function::Predicate;

pub struct Predicates;

impl Predicates {
	static TRUE: /* Java */ java::util::function::Predicate /**/ = |t|true;

	static FALSE: /* Java */ java::util::function::Predicate /**/ = |t|false;

	pub fn false_predicate<T>(&self) -> /* Java */ java::util::function::Predicate /**/ {
		return self.FALSE as Predicate<T>;
	}

	pub fn true_predicate<T>(&self) -> /* Java */ java::util::function::Predicate /**/ {
		return self.TRUE as Predicate<T>;
	}

	fn new() -> org::apache::commons::lang3::function::predicates::Predicates {
	// empty
	}
}