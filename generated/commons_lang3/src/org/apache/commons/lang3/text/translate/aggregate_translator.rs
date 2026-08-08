use java::io::IOException;
use java::io::Writer;
use crate::org::apache::commons::lang3::ArrayUtils;

pub struct AggregateTranslator {
	translators: &[org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator],
}

impl AggregateTranslator {
	pub fn new(translators: &org::apache::commons::lang3::text::translate::char_sequence_translator::CharSequenceTranslator) -> org::apache::commons::lang3::text::translate::aggregate_translator::AggregateTranslator {
		self.translators = ArrayUtils::clone(translators);
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		for /* final */ translator in self.translators {
			/* final */ let consumed: i32 = translator.translate(input, index, out);
			if consumed != 0 {
				return consumed;
			}
		}
		return 0;
	}
}