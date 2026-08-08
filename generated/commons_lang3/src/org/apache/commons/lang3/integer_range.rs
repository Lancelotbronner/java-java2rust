use java::util::stream::IntStream;

pub struct IntegerRange;

impl IntegerRange {
	static serialVersionUID: i64 = 1;

	pub fn of(&self, from_inclusive: i32, to_inclusive: i32) -> org::apache::commons::lang3::integer_range::IntegerRange {
		return org::apache::commons::lang3::integer_range::IntegerRange::of(&Integer::valueOf(from_inclusive), &Integer::valueOf(to_inclusive));
	}

	pub fn of(&self, from_inclusive: &/* Java */ java::lang::Integer /**/, to_inclusive: &/* Java */ java::lang::Integer /**/) -> org::apache::commons::lang3::integer_range::IntegerRange {
		return IntegerRange::new(from_inclusive, to_inclusive);
	}

	fn new(number1: &/* Java */ java::lang::Integer /**/, number2: &/* Java */ java::lang::Integer /**/) -> org::apache::commons::lang3::integer_range::IntegerRange {
		super(number1, number2, null);
	}

	pub fn fit(&self, element: i32) -> i32 {
		return super.fit(element).intValue();
	}

	pub fn to_int_stream(&self) -> /* Java */ java::util::stream::IntStream /**/ {
		return IntStream::rangeClosed(&self.get_minimum(), &self.get_maximum());
	}
}

impl /* Java */ java::io::Serializable /**/ for IntegerRange {}