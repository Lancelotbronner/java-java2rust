use java::util::Objects;

pub struct MutableTriple<L, M, R> {
	left: L,
	middle: M,
	right: R,
}

impl<L, M, R> MutableTriple {
	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::mutable_triple::MutableTriple] = ;

	static serialVersionUID: i64 = 1;

	pub fn empty_array<L, M, R>(&self) -> &[org::apache::commons::lang3::tuple::mutable_triple::MutableTriple] {
		return self.EMPTY_ARRAY as Vec<MutableTriple<L, M, R>>;
	}

	pub fn of<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::mutable_triple::MutableTriple {
		return MutableTriple<>::new(left, middle, right);
	}

	pub fn of_non_null<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::mutable_triple::MutableTriple {
		return org::apache::commons::lang3::tuple::mutable_triple::MutableTriple::of(&Objects::requireNonNull(left, "left"), &Objects::requireNonNull(middle, "middle"), &Objects::requireNonNull(right, "right"));
	}

	pub fn new() -> org::apache::commons::lang3::tuple::mutable_triple::MutableTriple {
	}

	pub fn new(left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::mutable_triple::MutableTriple {
		self.left = left;
		self.middle = middle;
		self.right = right;
	}

	pub fn get_left(&self) -> L {
		return self.left;
	}

	pub fn get_middle(&self) -> M {
		return self.middle;
	}

	pub fn get_right(&self) -> R {
		return self.right;
	}

	pub fn set_left(&mut self, left: &L) {
		self.left = left;
	}

	pub fn set_middle(&mut self, middle: &M) {
		self.middle = middle;
	}

	pub fn set_right(&mut self, right: &R) {
		self.right = right;
	}
}

impl<L, M, R> /* Java */ java::lang::Comparable /**/ for MutableTriple<L, M, R> {}

impl<L, M, R> /* Java */ java::io::Serializable /**/ for MutableTriple<L, M, R> {}