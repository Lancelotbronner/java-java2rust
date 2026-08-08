use java::io::Serializable;
use java::util::Objects;
use java::util::concurrent::atomic::AtomicReference;

pub struct MutableObject<T> {
	value: T,
}

impl<T> MutableObject {
	static serialVersionUID: i64 = 86241875189;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_object::MutableObject {
	}

	pub fn new(value: &T) -> org::apache::commons::lang3::mutable::mutable_object::MutableObject {
		self.value = value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == null {
			return false;
		}
		if self == obj {
			return true;
		}
		if self.getClass() == obj.getClass() {
			/* final */ let that: MutableObject<?> = obj as MutableObject<?>;
			return Objects::equals(self.value, that.value);
		}
		return false;
	}

	pub fn get_value(&self) -> T {
		return self.value;
	}

	pub fn hash_code(&self) -> i32 {
		return Objects::hashCode(self.value);
	}

	pub fn set_value(&mut self, value: &T) {
		self.value = value;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(self.value);
	}
}

impl<T> org::apache::commons::lang3::mutable::mutable::Mutable for MutableObject<T> {}

impl<T> /* Java */ java::util::function::Supplier /**/ for MutableObject<T> {}

impl<T> /* Java */ java::io::Serializable /**/ for MutableObject<T> {}