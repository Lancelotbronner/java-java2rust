use java::util::Objects;

pub struct ImmutableTriple<L, M, R> {
	left: L,
	middle: M,
	right: R,
}

impl<L, M, R> ImmutableTriple {
	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple] = ;

	static NULL: org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple = ImmutableTriple<>::new(null, null, null);

	static serialVersionUID: i64 = 1;

	pub fn empty_array<L, M, R>(&self) -> &[org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple] {
		return self.EMPTY_ARRAY as Vec<ImmutableTriple<L, M, R>>;
	}

	pub fn null_triple<L, M, R>(&self) -> org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple {
		return self.NULL;
	}

	pub fn of<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple {
		return  if left != null | middle != null || right != null { ImmutableTriple<>::new(left, middle, right) } else { org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple::null_triple() };
	}

	pub fn of_non_null<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple {
		return org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple::of(&Objects::requireNonNull(left, "left"), &Objects::requireNonNull(middle, "middle"), &Objects::requireNonNull(right, "right"));
	}

	pub fn new(left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::immutable_triple::ImmutableTriple {
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
}

impl<L, M, R> /* Java */ java::lang::Comparable /**/ for ImmutableTriple<L, M, R> {}

impl<L, M, R> /* Java */ java::io::Serializable /**/ for ImmutableTriple<L, M, R> {}