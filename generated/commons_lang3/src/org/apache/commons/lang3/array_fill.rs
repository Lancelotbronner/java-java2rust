use java::util::Arrays;
use java::util::function::IntFunction;
use crate::org::apache::commons::lang3::function::FailableIntFunction;

pub struct ArrayFill;

impl ArrayFill {
	pub fn fill(&self, a: &&[bool], val: bool) -> &[bool] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[i8], val: i8) -> &[i8] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[u16], val: u16) -> &[u16] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[f64], val: f64) -> &[f64] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[f32], val: f32) -> &[f32] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[i32], val: i32) -> &[i32] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[i64], val: i64) -> &[i64] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill(&self, a: &&[i16], val: i16) -> &[i16] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	pub fn fill<T, E: /* Java */ java::lang::Throwable /**/>(&self, mut array: &&[T], generator: &org::apache::commons::lang3::function::failable_int_function::FailableIntFunction) /* thrown(E) */ -> &[T] {
		if array != null && generator != null {
			 {
				let i: i32 = 0;
				while i < array.length {
					{
						array[i] = generator.apply(i);
					}
					i += 1;
				 }
			 }
	
		}
		return array;
	}

	pub fn fill<T>(&self, a: &&[T], val: &T) -> &[T] {
		if a != null {
			Arrays::fill(a, val);
		}
		return a;
	}

	fn new() -> org::apache::commons::lang3::array_fill::ArrayFill {
	// no instances
	}
}