use java::io::IOException;
use java::io::Writer;

pub struct NumericEntityEscaper {
	below: i32,
	above: i32,
	between: bool,
}

impl NumericEntityEscaper {
	pub fn above(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		return org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper::outside_of(0, code_point);
	}

	pub fn below(&self, code_point: i32) -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		return org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper::outside_of(code_point, Integer::MAX_VALUE);
	}

	pub fn between(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		return NumericEntityEscaper::new(code_point_low, code_point_high, true);
	}

	pub fn outside_of(&self, code_point_low: i32, code_point_high: i32) -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		return NumericEntityEscaper::new(code_point_low, code_point_high, false);
	}

	pub fn new() -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		this(0, Integer::MAX_VALUE, true);
	}

	fn new(below: i32, above: i32, between: bool) -> org::apache::commons::lang3::text::translate::numeric_entity_escaper::NumericEntityEscaper {
		self.below = below;
		self.above = above;
		self.between = between;
	}

	pub fn translate(&self, code_point: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> bool {
		if self.between {
			if code_point < self.below || code_point > self.above {
				return false;
			}
		} else if code_point >= self.below && code_point <= self.above {
			return false;
		}
		out.write("&#");
		out.write(&Integer::toString(code_point, 10));
		out.write(';');
		return true;
	}
}