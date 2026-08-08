use java::io::Serializable;
use java::util::Comparator;
use java::util::Objects;

pub struct Range<T> {
	comparator: /* Java */ java::util::Comparator /**/,
	hash_code: i32,
	maximum: T,
	minimum: T,
	to_string: /* Java */ java::lang::String /**/,
}

impl<T> Range {
	static serialVersionUID: i64 = 1;

	pub fn between<T: /* Java */ java::lang::Comparable /**/>(&self, from_inclusive: &T, to_inclusive: &T) -> org::apache::commons::lang3::range::Range {
		return org::apache::commons::lang3::range::Range::of(from_inclusive, to_inclusive, null);
	}

	pub fn between<T>(&self, from_inclusive: &T, to_inclusive: &T, comparator: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::range::Range {
		return Range<>::new(from_inclusive, to_inclusive, comparator);
	}

	pub fn is<T: /* Java */ java::lang::Comparable /**/>(&self, element: &T) -> org::apache::commons::lang3::range::Range {
		return org::apache::commons::lang3::range::Range::of(element, element, null);
	}

	pub fn is<T>(&self, element: &T, comparator: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::range::Range {
		return org::apache::commons::lang3::range::Range::of(element, element, comparator);
	}

	pub fn of<T: /* Java */ java::lang::Comparable /**/>(&self, from_inclusive: &T, to_inclusive: &T) -> org::apache::commons::lang3::range::Range {
		return org::apache::commons::lang3::range::Range::of(from_inclusive, to_inclusive, null);
	}

	pub fn of<T>(&self, from_inclusive: &T, to_inclusive: &T, comparator: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::range::Range {
		return Range<>::new(from_inclusive, to_inclusive, comparator);
	}

	fn new(element1: &T, element2: &T, comp: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::range::Range {
		Objects::requireNonNull(element1, "element1");
		Objects::requireNonNull(element2, "element2");
		if comp == null {
			self.comparator = ComparableComparator::INSTANCE;
		} else {
			self.comparator = comp;
		}
		if self.comparator.compare(element1, element2) < 1 {
			self.minimum = element1;
			self.maximum = element2;
		} else {
			self.minimum = element2;
			self.maximum = element1;
		}
	}

	pub fn contains(&self, element: &T) -> bool {
		if element == null {
			return false;
		}
		return self.comparator.compare(element, self.minimum) > -1 && self.comparator.compare(element, self.maximum) < 1;
	}

	pub fn contains_range(&self, other_range: &org::apache::commons::lang3::range::Range) -> bool {
		if other_range == null {
			return false;
		}
		return self.contains(other_range.minimum) && self.contains(other_range.maximum);
	}

	pub fn element_compare_to(&self, element: &T) -> i32 {
		// Comparable API says throw NPE on null
		Objects::requireNonNull(element, "element");
		if self.is_after(element) {
			return -1;
		}
		if self.is_before(element) {
			return 1;
		}
		return 0;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if obj == null || obj.getClass() != self.getClass() {
			return false;
		}
		/* final */ let range: Range<T> = obj as Range<T>;
		return self.minimum.equals(range.minimum) && self.maximum.equals(range.maximum);
	}

	pub fn fit(&self, element: &T) -> T {
		// Comparable API says throw NPE on null
		Objects::requireNonNull(element, "element");
		if self.is_after(element) {
			return self.minimum;
		}
		if self.is_before(element) {
			return self.maximum;
		}
		return element;
	}

	pub fn get_comparator(&self) -> /* Java */ java::util::Comparator /**/ {
		return self.comparator;
	}

	pub fn get_maximum(&self) -> T {
		return self.maximum;
	}

	pub fn get_minimum(&self) -> T {
		return self.minimum;
	}

	pub fn hash_code(&mut self) -> i32 {
		let result: i32 = self.hash_code;
		if self.hash_code == 0 {
			result = 17;
			result = 37 * result + self.getClass().hashCode();
			result = 37 * result + self.minimum.hashCode();
			result = 37 * result + self.maximum.hashCode();
			self.hash_code = result;
		}
		return result;
	}

	pub fn intersection_with(&self, other: &org::apache::commons::lang3::range::Range) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::range::Range {
		if !self.is_overlapped_by(other) {
			return Err(IllegalArgumentException::new(&String::format("Cannot calculate intersection with non-overlapping range %s", other)));
		}
		if self.equals(other) {
			return self;
		}
		/* final */ let min: T =  if self.get_comparator().compare(self.minimum, other.minimum) < 0 { other.minimum } else { self.minimum };
		/* final */ let max: T =  if self.get_comparator().compare(self.maximum, other.maximum) < 0 { self.maximum } else { other.maximum };
		return org::apache::commons::lang3::range::Range::of(min, max, &self.get_comparator());
	}

	pub fn is_after(&self, element: &T) -> bool {
		if element == null {
			return false;
		}
		return self.comparator.compare(element, self.minimum) < 0;
	}

	pub fn is_after_range(&self, other_range: &org::apache::commons::lang3::range::Range) -> bool {
		if other_range == null {
			return false;
		}
		return self.is_after(other_range.maximum);
	}

	pub fn is_before(&self, element: &T) -> bool {
		if element == null {
			return false;
		}
		return self.comparator.compare(element, self.maximum) > 0;
	}

	pub fn is_before_range(&self, other_range: &org::apache::commons::lang3::range::Range) -> bool {
		if other_range == null {
			return false;
		}
		return self.is_before(other_range.minimum);
	}

	pub fn is_ended_by(&self, element: &T) -> bool {
		if element == null {
			return false;
		}
		return self.comparator.compare(element, self.maximum) == 0;
	}

	pub fn is_natural_ordering(&self) -> bool {
		return self.comparator == ComparableComparator::INSTANCE;
	}

	pub fn is_overlapped_by(&self, other_range: &org::apache::commons::lang3::range::Range) -> bool {
		if other_range == null {
			return false;
		}
		return other_range.contains(self.minimum) || other_range.contains(self.maximum) || self.contains(other_range.minimum);
	}

	pub fn is_started_by(&self, element: &T) -> bool {
		if element == null {
			return false;
		}
		return self.comparator.compare(element, self.minimum) == 0;
	}

	pub fn to_string(&mut self) -> /* Java */ java::lang::String /**/ {
		if self.to_string == null {
			self.to_string = "[" + self.minimum + ".." + self.maximum + "]";
		}
		return self.to_string;
	}

	pub fn to_string(&self, format: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return String::format(format, self.minimum, self.maximum, self.comparator);
	}
}

impl<T> /* Java */ java::io::Serializable /**/ for Range<T> {}

enum ComparableComparator;