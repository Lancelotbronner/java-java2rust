use java::io::IOException;
use java::io::Writer;

pub struct CodePointTranslator;

impl CodePointTranslator {
	pub fn new() -> org::apache::commons::lang3::text::translate::code_point_translator::CodePointTranslator {
	// empty
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		/* final */ let code_point: i32 = Character::codePointAt(input, index);
		/* final */ let consumed: bool = self.translate(code_point, out);
		return  if consumed { 1 } else { 0 };
	}

	pub fn translate(&self, code_point: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> bool ;
}