use java::util::concurrent::atomic::DoubleAccumulator;
use java::util::concurrent::atomic::DoubleAdder;

pub struct MutableDouble {
	value: f64,
}

impl MutableDouble {
	static serialVersionUID: i64 = 1587163916;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_double::MutableDouble {
	}

	pub fn new(value: f64) -> org::apache::commons::lang3::mutable::mutable_double::MutableDouble {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_double::MutableDouble {
		self.value = value.doubleValue();
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_double::MutableDouble {
		self.value = Double::parseDouble(value);
	}

	pub fn add(&mut self, operand: f64) {
		self.value += operand;
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.doubleValue();
	}

	pub fn add_and_get(&mut self, operand: f64) -> f64 {
		self.value += operand;
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> f64 {
		self.value += operand.doubleValue();
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_double::MutableDouble) -> i32 {
		return Double::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> f64 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj instanceof MutableDouble && Double::doubleToLongBits((obj as MutableDouble).value) == Double::doubleToLongBits(self.value);
	}

	pub fn float_value(&self) -> f32 {
		return self.value as f32;
	}

	pub fn get_and_add(&mut self, operand: f64) -> f64 {
		/* final */ let last: f64 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> f64 {
		/* final */ let last: f64 = self.value;
		self.value += operand.doubleValue();
		return last;
	}

	pub fn get_and_decrement(&self) -> f64 {
		/* final */ let last: f64 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> f64 {
		/* final */ let last: f64 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Double /**/ {
		return Double::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		/* final */ let bits: i64 = Double::doubleToLongBits(self.value);
		return (bits ^ bits /* unsigned */ >> 32) as i32;
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> f64 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value as i32;
	}

	pub fn is_infinite(&self) -> bool {
		return Double::isInfinite(self.value);
	}

	pub fn is_nan(&self) -> bool {
		return Double::isNaN(self.value);
	}

	pub fn long_value(&self) -> i64 {
		return self.value as i64;
	}

	pub fn set_value(&mut self, value: f64) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.doubleValue();
	}

	pub fn subtract(&mut self, operand: f64) {
		self.value -= operand;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.doubleValue();
	}

	pub fn to_double(&self) -> /* Java */ java::lang::Double /**/ {
		return Double::valueOf(&self.double_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableDouble {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableDouble {}

impl /* Java */ java::util::function::Supplier /**/ for MutableDouble {}

impl /* Java */ java::io::Serializable /**/ for MutableDouble {}