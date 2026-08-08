use java::util::Map;
use java::util::Objects;

pub struct MutablePair<L, R> {
	left: L,
	right: R,
}

impl<L, R> MutablePair {
	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::mutable_pair::MutablePair] = ;

	static serialVersionUID: i64 = 4954918890077093841;

	pub fn empty_array<L, R>(&self) -> &[org::apache::commons::lang3::tuple::mutable_pair::MutablePair] {
		return self.EMPTY_ARRAY as Vec<MutablePair<L, R>>;
	}

	pub fn of<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
		return MutablePair<>::new(left, right);
	}

	pub fn of<L, R>(&self, pair: &/* Java */ java::util::Map::Entry /**/) -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
		/* final */ let left: L;
		/* final */ let right: R;
		if pair != null {
			left = pair.getKey();
			right = pair.getValue();
		} else {
			left = null;
			right = null;
		}
		return MutablePair<>::new(left, right);
	}

	pub fn of_non_null<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
		return org::apache::commons::lang3::tuple::mutable_pair::MutablePair::of(&Objects::requireNonNull(left, "left"), &Objects::requireNonNull(right, "right"));
	}

	pub fn of_non_null<L, R>(&self, pair: &/* Java */ java::util::Map::Entry /**/) -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
		return org::apache::commons::lang3::tuple::mutable_pair::MutablePair::of(&Objects::requireNonNull(pair, "pair"));
	}

	pub fn new() -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
	}

	pub fn new(left: &L, right: &R) -> org::apache::commons::lang3::tuple::mutable_pair::MutablePair {
		self.left = left;
		self.right = right;
	}

	pub fn get_left(&self) -> L {
		return self.left;
	}

	pub fn get_right(&self) -> R {
		return self.right;
	}

	pub fn set_left(&mut self, left: &L) {
		self.left = left;
	}

	pub fn set_right(&mut self, right: &R) {
		self.right = right;
	}

	pub fn set_value(&self, value: &R) -> R {
		/* final */ let result: R = self.get_right();
		self.set_right(value);
		return result;
	}
}

impl<L, R> /* Java */ java::util::Map::Entry /**/ for MutablePair<L, R> {}

impl<L, R> /* Java */ java::lang::Comparable /**/ for MutablePair<L, R> {}

impl<L, R> /* Java */ java::io::Serializable /**/ for MutablePair<L, R> {}