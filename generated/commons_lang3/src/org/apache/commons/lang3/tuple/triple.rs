use java::io::Serializable;
use java::util::Objects;
use crate::org::apache::commons::lang3::builder::CompareToBuilder;

pub struct Triple<L, M, R>;

impl<L, M, R> Triple {
	static serialVersionUID: i64 = 1;

	pub static EMPTY_ARRAY: &[org::apache::commons::lang3::tuple::triple::Triple] = ;

	pub fn empty_array<L, M, R>(&self) -> &[org::apache::commons::lang3::tuple::triple::Triple] {
		return self.EMPTY_ARRAY as Vec<Triple<L, M, R>>;
	}

	pub fn of<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::triple::Triple {
		return ImmutableTriple::of(left, middle, right);
	}

	pub fn of_non_null<L, M, R>(&self, left: &L, middle: &M, right: &R) -> org::apache::commons::lang3::tuple::triple::Triple {
		return ImmutableTriple::of_non_null(left, middle, right);
	}

	pub fn new() -> org::apache::commons::lang3::tuple::triple::Triple {
	// empty
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::tuple::triple::Triple) -> i32 {
		return CompareToBuilder::new().append(&self.get_left(), &other.get_left()).append(&self.get_middle(), &other.get_middle()).append(&self.get_right(), &other.get_right()).to_comparison();
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if obj instanceof Triple<?, ?, ?> {
			/* final */ let other: Triple<?, ?, ?> = obj as Triple<?, ?, ?>;
			return Objects::equals(&self.get_left(), &other.get_left()) && Objects::equals(&self.get_middle(), &other.get_middle()) && Objects::equals(&self.get_right(), &other.get_right());
		}
		return false;
	}

	pub fn get_left(&self) -> L ;

	pub fn get_middle(&self) -> M ;

	pub fn get_right(&self) -> R ;

	pub fn hash_code(&self) -> i32 {
		// See Map.Entry API specification
		return Objects::hashCode(&self.get_left()) ^ Objects::hashCode(&self.get_middle()) ^ Objects::hashCode(&self.get_right());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "(" + self.get_left() + "," + self.get_middle() + "," + self.get_right() + ")";
	}

	pub fn to_string(&self, format: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return String::format(format, &self.get_left(), &self.get_middle(), &self.get_right());
	}
}

impl<L, M, R> /* Java */ java::lang::Comparable /**/ for Triple<L, M, R> {}

impl<L, M, R> /* Java */ java::io::Serializable /**/ for Triple<L, M, R> {}