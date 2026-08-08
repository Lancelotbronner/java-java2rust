use java::util::concurrent::atomic::AtomicInteger;

pub struct MutableShort {
	value: i16,
}

impl MutableShort {
	static serialVersionUID: i64 = -2135791679;

	pub fn new() -> org::apache::commons::lang3::mutable::mutable_short::MutableShort {
	}

	pub fn new(value: &/* Java */ java::lang::Number /**/) -> org::apache::commons::lang3::mutable::mutable_short::MutableShort {
		self.value = value.shortValue();
	}

	pub fn new(value: i16) -> org::apache::commons::lang3::mutable::mutable_short::MutableShort {
		self.value = value;
	}

	pub fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::mutable::mutable_short::MutableShort {
		self.value = Short::parseShort(value);
	}

	pub fn add(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value += operand.shortValue();
	}

	pub fn add(&mut self, operand: i16) {
		self.value += operand;
	}

	pub fn add_and_get(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i16 {
		self.value += operand.shortValue();
		return self.value;
	}

	pub fn add_and_get(&mut self, operand: i16) -> i16 {
		self.value += operand;
		return self.value;
	}

	pub fn compare_to(&self, other: &org::apache::commons::lang3::mutable::mutable_short::MutableShort) -> i32 {
		return Short::compare(self.value, other.value);
	}

	pub fn decrement(&self) {
		self.value -= 1;
	}

	pub fn decrement_and_get(&self) -> i16 {
		self.value -= 1;
		return self.value;
	}

	pub fn double_value(&self) -> f64 {
		return self.value;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj instanceof MutableShort {
			return self.value == (obj as MutableShort).short_value();
		}
		return false;
	}

	pub fn float_value(&self) -> f32 {
		return self.value;
	}

	pub fn get_and_add(&mut self, operand: &/* Java */ java::lang::Number /**/) -> i16 {
		/* final */ let last: i16 = self.value;
		self.value += operand.shortValue();
		return last;
	}

	pub fn get_and_add(&mut self, operand: i16) -> i16 {
		/* final */ let last: i16 = self.value;
		self.value += operand;
		return last;
	}

	pub fn get_and_decrement(&self) -> i16 {
		/* final */ let last: i16 = self.value;
		self.value -= 1;
		return last;
	}

	pub fn get_and_increment(&self) -> i16 {
		/* final */ let last: i16 = self.value;
		self.value += 1;
		return last;
	}

	pub fn get_value(&self) -> /* Java */ java::lang::Short /**/ {
		return Short::valueOf(self.value);
	}

	pub fn hash_code(&self) -> i32 {
		return self.value;
	}

	pub fn increment(&self) {
		self.value += 1;
	}

	pub fn increment_and_get(&self) -> i16 {
		self.value += 1;
		return self.value;
	}

	pub fn int_value(&self) -> i32 {
		return self.value;
	}

	pub fn long_value(&self) -> i64 {
		return self.value;
	}

	pub fn set_value(&mut self, value: &/* Java */ java::lang::Number /**/) {
		self.value = value.shortValue();
	}

	pub fn set_value(&mut self, value: i16) {
		self.value = value;
	}

	pub fn short_value(&self) -> i16 {
		return self.value;
	}

	pub fn subtract(&mut self, operand: &/* Java */ java::lang::Number /**/) {
		self.value -= operand.shortValue();
	}

	pub fn subtract(&mut self, operand: i16) {
		self.value -= operand;
	}

	pub fn to_short(&self) -> /* Java */ java::lang::Short /**/ {
		return Short::valueOf(&self.short_value());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::valueOf(self.value);
	}
}

impl /* Java */ java::lang::Comparable /**/ for MutableShort {}

impl org::apache::commons::lang3::mutable::mutable::Mutable for MutableShort {}

impl /* Java */ java::util::function::Supplier /**/ for MutableShort {}

impl /* Java */ java::io::Serializable /**/ for MutableShort {}