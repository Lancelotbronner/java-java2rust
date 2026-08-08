use java::io::IOException;
use java::io::Writer;

pub struct UnicodeUnpairedSurrogateRemover;

impl UnicodeUnpairedSurrogateRemover {
	pub fn new() -> org::apache::commons::lang3::text::translate::unicode_unpaired_surrogate_remover::UnicodeUnpairedSurrogateRemover {
	// empty
	}

	pub fn translate(&self, code_point: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> bool {
		// true: It's a surrogate. Write nothing and say we've translated.
		return code_point >= Character::MIN_SURROGATE && code_point <= Character::MAX_SURROGATE;
	// It's not a surrogate. Don't translate it.
	}
}