pub struct DoubleRange;

impl DoubleRange {
	static serialVersionUID: i64 = 1;

	pub fn of(&self, from_inclusive: f64, to_inclusive: f64) -> org::apache::commons::lang3::double_range::DoubleRange {
		return org::apache::commons::lang3::double_range::DoubleRange::of(&Double::valueOf(from_inclusive), &Double::valueOf(to_inclusive));
	}

	pub fn of(&self, from_inclusive: &/* Java */ java::lang::Double /**/, to_inclusive: &/* Java */ java::lang::Double /**/) -> org::apache::commons::lang3::double_range::DoubleRange {
		return DoubleRange::new(from_inclusive, to_inclusive);
	}

	fn new(number1: &/* Java */ java::lang::Double /**/, number2: &/* Java */ java::lang::Double /**/) -> org::apache::commons::lang3::double_range::DoubleRange {
		super(number1, number2, null);
	}

	pub fn fit(&self, element: f64) -> f64 {
		return super.fit(element).doubleValue();
	}
}

impl /* Java */ java::io::Serializable /**/ for DoubleRange {}