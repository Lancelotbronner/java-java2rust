use java::util::Comparator;

pub struct NumberRange<N: /* Java */ java::lang::Number /**/>;

impl<N: /* Java */ java::lang::Number /**/> NumberRange {
	static serialVersionUID: i64 = 1;

	pub fn new(number1: &N, number2: &N, comp: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::number_range::NumberRange {
		super(number1, number2, comp);
	}
}

impl<N: /* Java */ java::lang::Number /**/> /* Java */ java::io::Serializable /**/ for NumberRange<N> {}