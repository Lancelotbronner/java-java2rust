use java::util::Arrays;
use java::util::Comparator;

pub struct ArraySorter;

impl ArraySorter {
	pub fn sort(&self, array: &&[i8]) -> &[i8] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[u16]) -> &[u16] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[f64]) -> &[f64] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[f32]) -> &[f32] {
		if array != null {
			Arrays.sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[i32]) -> &[i32] {
		if array != null {
			Arrays.sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[i64]) -> &[i64] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort(&self, array: &&[i16]) -> &[i16] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort<T>(&self, array: &&[T]) -> &[T] {
		if array != null {
			Arrays::sort(array);
		}
		return array;
	}

	pub fn sort<T>(&self, array: &&[T], comparator: &/* Java */ java::util::Comparator /**/) -> &[T] {
		if array != null {
			Arrays::sort(array, comparator);
		}
		return array;
	}

	pub fn new() -> org::apache::commons::lang3::array_sorter::ArraySorter {
	// empty
	}
}