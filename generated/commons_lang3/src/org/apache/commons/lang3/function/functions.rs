use java::util::Objects;
use java::util::Optional;
use java::util::function::Function;

pub struct Functions;

impl Functions {
	pub fn apply<T, R>(&self, function: &/* Java */ java::util::function::Function /**/, object: &T) -> R {
		return  if function != null { function.apply(object) } else { null };
	}

	pub fn apply_non_null<T, R>(&self, value: &T, mapper: &/* Java */ java::util::function::Function /**/) -> R {
		return  if value != null { Objects::requireNonNull(mapper, "mapper").apply(value) } else { null };
	}

	pub fn apply_non_null<T, U, R>(&self, value1: &T, mapper1: &/* Java */ java::util::function::Function /**/, mapper2: &/* Java */ java::util::function::Function /**/) -> R {
		return org::apache::commons::lang3::function::functions::Functions::apply_non_null(&org::apache::commons::lang3::function::functions::Functions::apply_non_null(value1, mapper1), mapper2);
	}

	pub fn apply_non_null<T, U, V, R>(&self, value1: &T, mapper1: &/* Java */ java::util::function::Function /**/, mapper2: &/* Java */ java::util::function::Function /**/, mapper3: &/* Java */ java::util::function::Function /**/) -> R {
		return org::apache::commons::lang3::function::functions::Functions::apply_non_null(&org::apache::commons::lang3::function::functions::Functions::apply_non_null(&org::apache::commons::lang3::function::functions::Functions::apply_non_null(value1, mapper1), mapper2), mapper3);
	}

	pub fn function<T, R>(&self, function: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::function::Function /**/ {
		return function;
	}

	fn new() -> org::apache::commons::lang3::function::functions::Functions {
	// no instances needed.
	}
}