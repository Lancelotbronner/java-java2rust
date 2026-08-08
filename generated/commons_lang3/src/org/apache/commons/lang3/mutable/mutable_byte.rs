use java::util::concurrent::atomic::AtomicInteger;

pub struct MutableByte {
	value: i8,
}

impl MutableByte {
	static serialVersionUID: i64 = -1585823265;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_byte::MutableByte {
	}

	pub fn new(value: i8) -> org::apache::commons::lang3::mutable::mutable_byte::MutableByte {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_byte::MutableByte {
		self.value = value.byteValue();
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_byte::MutableByte {
		self.value = Byte::parseByte(value);
	}

	pub fn add(&mut self, operand: i8) {
		self.value += operand;
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.byteValue();
	}

	pub fn add_and_get(&mut self, operand: i8) -> i8 {
		self.value += operand;
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i8 {
		self.value += operand.byteValue();
		return self.value;
	}

	pub fn byte_value(&self) -> i8 {
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_byte::MutableByte) -> i32 {
		return Byte::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> i8 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj instanceof MutableByte {
			return self.value == (obj as MutableByte).byte_value();
		}
		return false;
	}

	pub fn float_value(&self) -> f32 {
		return self.value;
	}

	pub fn get_and_add(&mut self, operand: i8) -> i8 {
		/* final */ let last: i8 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i8 {
		/* final */ let last: i8 = self.value;
		self.value += operand.byteValue();
		return last;
	}

	pub fn get_and_decrement(&self) -> i8 {
		/* final */ let last: i8 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> i8 {
		/* final */ let last: i8 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Byte /**/ {
		return Byte::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return self.value;
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> i8 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value;
	}

	pub fn long_value(&self) -> i64 {
		return self.value;
	}

	pub fn set_value(&mut self, value: i8) {
		self.value = value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.byteValue();
	}

	pub fn subtract(&mut self, operand: i8) {
		self.value -= operand;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.byteValue();
	}

	pub fn to_byte(&self) -> /* Java */ java::lang::Byte /**/ {
		return Byte::valueOf(&self.byte_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableByte {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableByte {}

impl /* Java */ java::util::function::Supplier /**/ for MutableByte {}

impl /* Java */ java::io::Serializable /**/ for MutableByte {}