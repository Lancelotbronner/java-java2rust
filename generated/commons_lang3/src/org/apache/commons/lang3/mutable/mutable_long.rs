use java::util::concurrent::atomic::AtomicLong;

pub struct MutableLong {
	value: i64,
}

impl MutableLong {
	static serialVersionUID: i64 = 62986528375;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_long::MutableLong {
	}

	pub fn new(value: i64) -> org::apache::commons::lang3::mutable::mutable_long::MutableLong {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_long::MutableLong {
		self.value = value.longValue();
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_long::MutableLong {
		self.value = Long::parseLong(value);
	}

	pub fn add(&mut self, operand: i64) {
		self.value += operand;
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.longValue();
	}

	pub fn add_and_get(&mut self, operand: i64) -> i64 {
		self.value += operand;
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i64 {
		self.value += operand.longValue();
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_long::MutableLong) -> i32 {
		return Long::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> i64 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj instanceof MutableLong {
			return self.value == (obj as MutableLong).long_value();
		}
		return false;
	}

	pub fn float_value(&self) -> f32 {
		return self.value;
	}

	pub fn get_and_add(&mut self, operand: i64) -> i64 {
		/* final */ let last: i64 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i64 {
		/* final */ let last: i64 = self.value;
		self.value += operand.longValue();
		return last;
	}

	pub fn get_and_decrement(&self) -> i64 {
		/* final */ let last: i64 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> i64 {
		/* final */ let last: i64 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Long /**/ {
		return Long::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return (self.value ^ self.value /* unsigned */ >> 32) as i32;
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> i64 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value as i32;
	}

	pub fn long_value(&self) -> i64 {
		return self.value;
	}

	pub fn set_value(&mut self, value: i64) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.longValue();
	}

	pub fn subtract(&mut self, operand: i64) {
		self.value -= operand;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.longValue();
	}

	pub fn to_long(&self) -> /* Java */ java::lang::Long /**/ {
		return Long::valueOf(&self.long_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableLong {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableLong {}

impl /* Java */ java::util::function::Supplier /**/ for MutableLong {}

impl /* Java */ java::io::Serializable /**/ for MutableLong {}