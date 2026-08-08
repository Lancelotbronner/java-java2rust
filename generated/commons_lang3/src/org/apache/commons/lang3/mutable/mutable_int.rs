use java::util::concurrent::atomic::AtomicInteger;

pub struct MutableInt {
	value: i32,
}

impl MutableInt {
	static serialVersionUID: i64 = 512176391864;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_int::MutableInt {
	}

	pub fn new(value: i32) -> org::apache::commons::lang3::mutable::mutable_int::MutableInt {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_int::MutableInt {
		self.value = value.intValue();
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_int::MutableInt {
		self.value = Integer::parseInt(value);
	}

	pub fn add(&mut self, operand: i32) {
		self.value += operand;
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.intValue();
	}

	pub fn add_and_get(&mut self, operand: i32) -> i32 {
		self.value += operand;
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i32 {
		self.value += operand.intValue();
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_int::MutableInt) -> i32 {
		return Integer::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> i32 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj instanceof MutableInt {
			return self.value == (obj as MutableInt).int_value();
		}
		return false;
	}

	pub fn float_value(&self) -> f32 {
		return self.value;
	}

	pub fn get_and_add(&mut self, operand: i32) -> i32 {
		/* final */ let last: i32 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i32 {
		/* final */ let last: i32 = self.value;
		self.value += operand.intValue();
		return last;
	}

	pub fn get_and_decrement(&self) -> i32 {
		/* final */ let last: i32 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> i32 {
		/* final */ let last: i32 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Integer /**/ {
		return Integer::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return self.value;
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> i32 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value;
	}

	pub fn long_value(&self) -> i64 {
		return self.value;
	}

	pub fn set_value(&mut self, value: i32) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.intValue();
	}

	pub fn subtract(&mut self, operand: i32) {
		self.value -= operand;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.intValue();
	}

	pub fn to_integer(&self) -> /* Java */ java::lang::Integer /**/ {
		return Integer::valueOf(&self.int_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableInt {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableInt {}

impl /* Java */ java::util::function::Supplier /**/ for MutableInt {}

impl /* Java */ java::io::Serializable /**/ for MutableInt {}