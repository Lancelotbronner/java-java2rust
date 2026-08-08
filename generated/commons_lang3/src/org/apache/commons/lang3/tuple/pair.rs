use java::io::Serializable;
use java::util::Map;
use java::util::Objects;
use crate::org::apache::commons::lang3::builder::CompareToBuilder;
use crate::org::apache::commons::lang3::function::FailableBiConsumer;
use crate::org::apache::commons::lang3::function::FailableBiFunction;

pub struct Pair<L, R>;

impl<L, R> Pair {
	static serialVersionUID: i64 = 4954918890077093841;

	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::pair::Pair] = ;

	pub fn empty_array<L, R>(&self) -> &[org::apache::commons::lang3::tuple::pair::Pair] {
		return self.EMPTY_ARRAY as Vec<Pair<L, R>>;
	}

	pub fn of<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::pair::Pair {
		return ImmutablePair::of(left, right);
	}

	pub fn of<L, R>(&self, pair: &/* Java */ java::util::Map::Entry /**/) -> org::apache::commons::lang3::tuple::pair::Pair {
		return ImmutablePair::of(pair);
	}

	pub fn of_non_null<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::pair::Pair {
		return ImmutablePair::of_non_null(left, right);
	}

	pub fn new() -> org::apache::commons::lang3::tuple::pair::Pair {
	// empty
	}

	pub fn accept<E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer) /* thrown(E | E) */ {
		consumer.accept(&self.get_key(), &self.get_value())?;
	}

	pub fn apply<V, E: /* Java */ java::lang::Throwable /**/>(&self, function: &org::apache::commons::lang3::function::failable_bi_function::FailableBiFunction) /* thrown(E | E) */ -> V {
		return function.apply(&self.get_key(), &self.get_value())?;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::tuple::pair::Pair) -> i32 {
		// @formatter:off
		return CompareToBuilder::new().append(&self.get_left(), &other.get_left()).append(&self.get_right(), &other.get_right()).to_comparison();
	// @formatter:on
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if obj instanceof Map.Entry<?, ?> {
			/* final */ let other: Map.Entry<?, ?> = obj as Map.Entry<?, ?>;
			return Objects::equals(&self.get_key(), &other.getKey()) && Objects::equals(&self.get_value(), &other.getValue());
		}
		return false;
	}

	pub fn get_key(&self) -> L {
		return self.get_left();
	}

	pub fn get_left(&self) -> L ;

	pub fn get_right(&self) -> R ;

	pub fn get_value(&self) -> R {
		return self.get_right();
	}

	pub fn hash_code(&self) -> i32 {
		// See Map.Entry API specification
		return Objects::hashCode(&self.get_key()) ^ Objects::hashCode(&self.get_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "(" + self.get_left() + ',' + self.get_right() + ')';
	}

	pub fn to_string(&self, format: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return String::format(format, &self.get_left(), &self.get_right());
	}
}

impl<L, R> /* Java */ java::util::Map::Entry /**/ for Pair<L, R> {}

impl<L, R> /* Java */ java::lang::Comparable /**/ for Pair<L, R> {}

impl<L, R> /* Java */ java::io::Serializable /**/ for Pair<L, R> {}