use java::util::stream::LongStream;

pub struct LongRange;

impl LongRange {
	static serialVersionUID: i64 = 1;

	pub fn of(&self, from_inclusive: i64, to_inclusive: i64) -> org::apache::commons::lang3::long_range::LongRange {
		return org::apache::commons::lang3::long_range::LongRange::of(&Long::valueOf(from_inclusive), &Long::valueOf(to_inclusive));
	}

	pub fn of(&self, from_inclusive: &/* Java */ java::lang::Long /**/, to_inclusive: &/* Java */ java::lang::Long /**/) -> org::apache::commons::lang3::long_range::LongRange {
		return LongRange::new(from_inclusive, to_inclusive);
	}

	fn new(number1: &/* Java */ java::lang::Long /**/, number2: &/* Java */ java::lang::Long /**/) -> org::apache::commons::lang3::long_range::LongRange {
		super(number1, number2, null);
	}

	pub fn fit(&self, element: i64) -> i64 {
		return super.fit(element).longValue();
	}

	pub fn to_long_stream(&self) -> /* Java */ java::util::stream::LongStream /**/ {
		return LongStream::rangeClosed(&self.get_minimum(), &self.get_maximum());
	}
}

impl /* Java */ java::io::Serializable /**/ for LongRange {}