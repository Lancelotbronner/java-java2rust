use java::util::Objects;
use crate::org::apache::commons::lang3::Validate;

pub struct IEEE754rUtils;

impl IEEE754rUtils {
	pub fn max(&self, array: f64) /* thrown(java.lang.IllegalArgumentException) */ -> f64 {
		Objects::requireNonNull(array, "array");
		Validate::is_true(array.length != 0, "Array cannot be empty.")?;
		// Finds and returns max
		let max: f64 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					max = org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(array[j], max);
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, a: f64, b: f64) -> f64 {
		if Double::isNaN(a) {
			return b;
		}
		if Double::isNaN(b) {
			return a;
		}
		return Math::max(a, b);
	}

	pub fn max(&self, a: f64, b: f64, c: f64) -> f64 {
		return org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(&org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(a, b), c);
	}

	pub fn max(&self, array: f32) /* thrown(java.lang.IllegalArgumentException) */ -> f32 {
		Objects::requireNonNull(array, "array");
		Validate::is_true(array.length != 0, "Array cannot be empty.")?;
		// Finds and returns max
		let max: f32 = array[0];
		 {
			let j: i32 = 1;
			while j < array.length {
				{
					max = org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(array[j], max);
				}
				j += 1;
			 }
		 }
	
		return max;
	}

	pub fn max(&self, a: f32, b: f32) -> f32 {
		if Float::isNaN(a) {
			return b;
		}
		if Float::isNaN(b) {
			return a;
		}
		return Math::max(a, b);
	}

	pub fn max(&self, a: f32, b: f32, c: f32) -> f32 {
		return org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(&org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::max(a, b), c);
	}

	pub fn min(&self, array: f64) /* thrown(java.lang.IllegalArgumentException) */ -> f64 {
		Objects::requireNonNull(array, "array");
		Validate::is_true(array.length != 0, "Array cannot be empty.")?;
		// Finds and returns min
		let min: f64 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					min = org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(array[i], min);
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, a: f64, b: f64) -> f64 {
		if Double::isNaN(a) {
			return b;
		}
		if Double::isNaN(b) {
			return a;
		}
		return Math::min(a, b);
	}

	pub fn min(&self, a: f64, b: f64, c: f64) -> f64 {
		return org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(&org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(a, b), c);
	}

	pub fn min(&self, array: f32) /* thrown(java.lang.IllegalArgumentException) */ -> f32 {
		Objects::requireNonNull(array, "array");
		Validate::is_true(array.length != 0, "Array cannot be empty.")?;
		// Finds and returns min
		let min: f32 = array[0];
		 {
			let i: i32 = 1;
			while i < array.length {
				{
					min = org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(array[i], min);
				}
				i += 1;
			 }
		 }
	
		return min;
	}

	pub fn min(&self, a: f32, b: f32) -> f32 {
		if Float::isNaN(a) {
			return b;
		}
		if Float::isNaN(b) {
			return a;
		}
		return Math::min(a, b);
	}

	pub fn min(&self, a: f32, b: f32, c: f32) -> f32 {
		return org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(&org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils::min(a, b), c);
	}

	pub fn new() -> org::apache::commons::lang3::math::iee_e754r_utils::IEEE754rUtils {
	// empty
	}
}