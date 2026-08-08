use java::util::function::Predicate;
use crate::org::apache::commons::lang3::ObjectUtils;

pub struct ComparableUtils;

impl ComparableUtils {
	pub fn between<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A, c: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).between(b, c);
	}

	pub fn between_exclusive<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A, c: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).between_exclusive(b, c);
	}

	pub fn ge<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).greater_than_or_equal_to(b);
	}

	pub fn gt<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).greater_than(b);
	}

	pub fn is<A: /* Java */ java::lang::Comparable /**/>(&self, a: &A) -> org::apache::commons::lang3::compare::comparable_utils::ComparableCheckBuilder {
		return ComparableCheckBuilder<>::new(a);
	}

	pub fn le<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).less_than_or_equal_to(b);
	}

	pub fn lt<A: /* Java */ java::lang::Comparable /**/>(&self, b: &A) -> /* Java */ java::util::function::Predicate /**/ {
		return |a|org::apache::commons::lang3::compare::comparable_utils::ComparableUtils::is(a).less_than(b);
	}

	pub fn max<A: /* Java */ java::lang::Comparable /**/>(&self, comparable1: &A, comparable2: &A) -> A {
		return  if ObjectUtils::compare(comparable1, comparable2, false) > 0 { comparable1 } else { comparable2 };
	}

	pub fn min<A: /* Java */ java::lang::Comparable /**/>(&self, comparable1: &A, comparable2: &A) -> A {
		return  if ObjectUtils::compare(comparable1, comparable2, true) < 0 { comparable1 } else { comparable2 };
	}

	fn new() -> org::apache::commons::lang3::compare::comparable_utils::ComparableUtils {
	// empty
	}
}

pub struct ComparableCheckBuilder<A: /* Java */ java::lang::Comparable /**/> {
	a: A,
}

impl<A: /* Java */ java::lang::Comparable /**/> ComparableCheckBuilder {
	fn new(a: &A) -> org::apache::commons::lang3::compare::comparable_utils::ComparableCheckBuilder {
		self.a = a;
	}

	pub fn between(&self, b: &A, c: &A) -> bool {
		return self.between_ordered(b, c) || self.between_ordered(c, b);
	}

	pub fn between_exclusive(&self, b: &A, c: &A) -> bool {
		return self.between_ordered_exclusive(b, c) || self.between_ordered_exclusive(c, b);
	}

	fn between_ordered(&self, b: &A, c: &A) -> bool {
		return self.greater_than_or_equal_to(b) && self.less_than_or_equal_to(c);
	}

	fn between_ordered_exclusive(&self, b: &A, c: &A) -> bool {
		return self.greater_than(b) && self.less_than(c);
	}

	pub fn equal_to(&self, b: &A) -> bool {
		return self.a != null && self.a.compareTo(b) == 0;
	}

	pub fn greater_than(&self, b: &A) -> bool {
		return self.a != null && self.a.compareTo(b) > 0;
	}

	pub fn greater_than_or_equal_to(&self, b: &A) -> bool {
		return self.a != null && self.a.compareTo(b) >= 0;
	}

	pub fn less_than(&self, b: &A) -> bool {
		return self.a != null && self.a.compareTo(b) < 0;
	}

	pub fn less_than_or_equal_to(&self, b: &A) -> bool {
		return self.a != null && self.a.compareTo(b) <= 0;
	}
}