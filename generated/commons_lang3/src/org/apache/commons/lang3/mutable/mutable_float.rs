use java::util::concurrent::atomic::DoubleAccumulator;
use java::util::concurrent::atomic::DoubleAdder;

pub struct MutableFloat {
	value: f32,
}

impl MutableFloat {
	static serialVersionUID: i64 = 5787169186;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_float::MutableFloat {
	}

	pub fn new(value: f32) -> org::apache::commons::lang3::mutable::mutable_float::MutableFloat {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_float::MutableFloat {
		self.value = value.floatValue();
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_float::MutableFloat {
		self.value = Float::parseFloat(value);
	}

	pub fn add(&mut self, operand: f32) {
		self.value += operand;
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.floatValue();
	}

	pub fn add_and_get(&mut self, operand: f32) -> f32 {
		self.value += operand;
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> f32 {
		self.value += operand.floatValue();
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_float::MutableFloat) -> i32 {
		return Float::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> f32 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj instanceof MutableFloat && Float::floatToIntBits((obj as MutableFloat).value) == Float::floatToIntBits(self.value);
	}

	pub fn float_value(&self) -> f32 {
		return self.value;
	}

	pub fn get_and_add(&mut self, operand: f32) -> f32 {
		/* final */ let last: f32 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> f32 {
		/* final */ let last: f32 = self.value;
		self.value += operand.floatValue();
		return last;
	}

	pub fn get_and_decrement(&self) -> f32 {
		/* final */ let last: f32 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> f32 {
		/* final */ let last: f32 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Float /**/ {
		return Float::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return Float::floatToIntBits(self.value);
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> f32 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value as i32;
	}

	pub fn is_infinite(&self) -> bool {
		return Float::isInfinite(self.value);
	}

	pub fn is_nan(&self) -> bool {
		return Float::isNaN(self.value);
	}

	pub fn long_value(&self) -> i64 {
		return self.value as i64;
	}

	pub fn set_value(&mut self, value: f32) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.floatValue();
	}

	pub fn subtract(&mut self, operand: f32) {
		self.value -= operand;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.floatValue();
	}

	pub fn to_float(&self) -> /* Java */ java::lang::Float /**/ {
		return Float::valueOf(&self.float_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableFloat {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableFloat {}

impl /* Java */ java::util::function::Supplier /**/ for MutableFloat {}

impl /* Java */ java::io::Serializable /**/ for MutableFloat {}