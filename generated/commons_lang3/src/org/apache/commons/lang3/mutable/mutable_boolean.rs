use java::io::Serializable;
use java::util::concurrent::atomic::AtomicBoolean;
use crate::org::apache::commons::lang3::BooleanUtils;

pub struct MutableBoolean {
	value: bool,
}

impl MutableBoolean {
	static serialVersionUID: i64 = -4830728138360036487;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_boolean::MutableBoolean {
	}

	pub fn new(value: bool) -> org::apache::commons::lang3::mutable::mutable_boolean::MutableBoolean {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Boolean /**/) -> org::apache::commons::lang3::mutable::mutable_boolean::MutableBoolean {
		self.value = value.booleanValue();
	}

	pub fn boolean_value(&self) -> bool {
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_boolean::MutableBoolean) -> i32 {
		return BooleanUtils::compare(self.value, other.value);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj instanceof MutableBoolean {
			return self.value == (obj as MutableBoolean).boolean_value();
		}
		return false;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Boolean /**/ {
		return Boolean::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return  if self.value { Boolean::TRUE.hashCode() } else { Boolean::FALSE.hashCode() };
	}

	pub fn is_false(&self) -> bool {
		return !self.value;
	}

	pub fn is_true(&self) -> bool {
		return self.value;
	}

	pub fn set_false(&mut self) {
		self.value = false;
	}

	pub fn set_true(&mut self) {
		self.value = true;
	}

	pub fn set_value(&mut self, value: bool) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Boolean /**/) {
		self.value = value.booleanValue();
	}

	pub fn to_boolean(&self) -> /* Java */ java::lang::Boolean /**/ {
		return Boolean::valueOf(&self.boolean_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableBoolean {}

impl /* Java */ java::util::function::Supplier /**/ for MutableBoolean {}

impl /* Java */ java::io::Serializable /**/ for MutableBoolean {}

impl /* Java */ java::lang::Comparable /**/ for MutableBoolean {}