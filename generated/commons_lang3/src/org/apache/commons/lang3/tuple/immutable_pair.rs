use java::util::Map;
use java::util::Objects;

pub struct ImmutablePair<L, R> {
	left: L,
	right: R,
}

impl<L, R> ImmutablePair {
	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair] = ;

	static NULL: org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair = ImmutablePair<>::new(null, null);

	static serialVersionUID: i64 = 4954918890077093841;

	pub fn empty_array<L, R>(&self) -> &[org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair] {
		return self.EMPTY_ARRAY as Vec<ImmutablePair<L, R>>;
	}

	pub fn left<L, R>(&self, left: &L) -> org::apache::commons::lang3::tuple::pair::Pair {
		return org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair::of(left, null);
	}

	pub fn null_pair<L, R>(&self) -> org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair {
		return self.NULL;
	}

	pub fn of<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair {
		return  if left != null || right != null { ImmutablePair<>::new(left, right) } else { org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair::null_pair() };
	}

	pub fn of<L, R>(&self, pair: &/* Java */ java::util::Map::Entry /**/) -> org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair {
		return  if pair != null { ImmutablePair<>::new(&pair.getKey(), &pair.getValue()) } else { org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair::null_pair() };
	}

	pub fn of_non_null<L, R>(&self, left: &L, right: &R) -> org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair {
		return org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair::of(&Objects::requireNonNull(left, "left"), &Objects::requireNonNull(right, "right"));
	}

	pub fn right<L, R>(&self, right: &R) -> org::apache::commons::lang3::tuple::pair::Pair {
		return org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair::of(null, right);
	}

	pub fn new(left: &L, right: &R) -> org::apache::commons::lang3::tuple::immutable_pair::ImmutablePair {
		self.left = left;
		self.right = right;
	}

	pub fn get_left(&self) -> L {
		return self.left;
	}

	pub fn get_right(&self) -> R {
		return self.right;
	}

	pub fn set_value(&self, value: &R) /* thrown(java.lang.UnsupportedOperationException) */ -> R {
		return Err(UnsupportedOperationException::new());
	}
}

impl<L, R> /* Java */ java::util::Map::Entry /**/ for ImmutablePair<L, R> {}

impl<L, R> /* Java */ java::lang::Comparable /**/ for ImmutablePair<L, R> {}

impl<L, R> /* Java */ java::io::Serializable /**/ for ImmutablePair<L, R> {}