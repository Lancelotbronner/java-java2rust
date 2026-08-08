use java::util::Objects;

pub struct CharUtils;

impl CharUtils {
	static CHAR_STRING_ARRAY: &[/* Java */ java::lang::String /**/] = ArrayUtils::set_all(: [Option<String>; 128] = [None; 128], |i|String::valueOf(i as char));

	static HEX_DIGITS: &[u16] = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', ]
	;

	pub static LF: u16 = '\n';

	pub static CR: u16 = '\r';

	pub static NUL: u16 = '\0';

	pub fn compare(&self, x: u16, y: u16) -> i32 {
		return x - y;
	}

	pub fn is_ascii(&self, ch: u16) -> bool {
		return ch < 128;
	}

	pub fn is_ascii_alpha(&self, ch: u16) -> bool {
		return org::apache::commons::lang3::char_utils::CharUtils::is_ascii_alpha_upper(ch) || org::apache::commons::lang3::char_utils::CharUtils::is_ascii_alpha_lower(ch);
	}

	pub fn is_ascii_alpha_lower(&self, ch: u16) -> bool {
		return ch >= 'a' && ch <= 'z';
	}

	pub fn is_ascii_alphanumeric(&self, ch: u16) -> bool {
		return org::apache::commons::lang3::char_utils::CharUtils::is_ascii_alpha(ch) || org::apache::commons::lang3::char_utils::CharUtils::is_ascii_numeric(ch);
	}

	pub fn is_ascii_alpha_upper(&self, ch: u16) -> bool {
		return ch >= 'A' && ch <= 'Z';
	}

	pub fn is_ascii_control(&self, ch: u16) -> bool {
		return ch < 32 || ch == 127;
	}

	pub fn is_ascii_numeric(&self, ch: u16) -> bool {
		return ch >= '0' && ch <= '9';
	}

	pub fn is_ascii_printable(&self, ch: u16) -> bool {
		return ch >= 32 && ch < 127;
	}

	pub fn is_hex(&self, ch: u16) -> bool {
		return org::apache::commons::lang3::char_utils::CharUtils::is_ascii_numeric(ch) || ch >= 'a' && ch <= 'f' || ch >= 'A' && ch <= 'F';
	}

	pub fn is_octal(&self, ch: u16) -> bool {
		return ch >= '0' && ch <= '7';
	}

	pub fn to_char(&self, ch: &/* Java */ java::lang::Character /**/) -> u16 {
		return Objects::requireNonNull(ch, "ch").charValue();
	}

	pub fn to_char(&self, ch: &/* Java */ java::lang::Character /**/, default_value: u16) -> u16 {
		return  if ch != null { ch.charValue() } else { default_value };
	}

	pub fn to_char(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> u16 {
		Validate::not_empty(str, "The String must not be empty")?;
		return str.charAt(0);
	}

	pub fn to_char(&self, str: &/* Java */ java::lang::String /**/, default_value: u16) -> u16 {
		return  if StringUtils::is_empty(str) { default_value } else { str.charAt(0) };
	}

	pub fn to_character_object(&self, c: u16) -> /* Java */ java::lang::Character /**/ {
		return Character::valueOf(c);
	}

	pub fn to_character_object(&self, str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Character /**/ {
		return  if StringUtils::is_empty(str) { null } else { Character::valueOf(&str.charAt(0)) };
	}

	pub fn to_int_value(&self, ch: u16) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if !org::apache::commons::lang3::char_utils::CharUtils::is_ascii_numeric(ch) {
			return Err(IllegalArgumentException::new("The character " + ch + " is not in the range '0' - '9'"));
		}
		return ch - 48;
	}

	pub fn to_int_value(&self, ch: u16, default_value: i32) -> i32 {
		return  if org::apache::commons::lang3::char_utils::CharUtils::is_ascii_numeric(ch) { ch - 48 } else { default_value };
	}

	pub fn to_int_value(&self, ch: &/* Java */ java::lang::Character /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		return org::apache::commons::lang3::char_utils::CharUtils::to_int_value(&org::apache::commons::lang3::char_utils::CharUtils::to_char(ch))?;
	}

	pub fn to_int_value(&self, ch: &/* Java */ java::lang::Character /**/, default_value: i32) -> i32 {
		return  if ch != null { org::apache::commons::lang3::char_utils::CharUtils::to_int_value(&ch.charValue(), default_value) } else { default_value };
	}

	pub fn to_string(&self, ch: u16) -> /* Java */ java::lang::String /**/ {
		if ch < self.CHAR_STRING_ARRAY.length {
			return self.CHAR_STRING_ARRAY[ch];
		}
		return String::valueOf(ch);
	}

	pub fn to_string(&self, ch: &/* Java */ java::lang::Character /**/) -> /* Java */ java::lang::String /**/ {
		return  if ch != null { org::apache::commons::lang3::char_utils::CharUtils::to_string(&ch.charValue()) } else { null };
	}

	pub fn unicode_escaped(&self, ch: u16) -> /* Java */ java::lang::String /**/ {
		return "\\u" + self.HEX_DIGITS[ch /* signed */ >> 12 & 15] + self.HEX_DIGITS[ch /* signed */ >> 8 & 15] + self.HEX_DIGITS[ch /* signed */ >> 4 & 15] + self.HEX_DIGITS[ch & 15];
	}

	pub fn unicode_escaped(&self, ch: &/* Java */ java::lang::Character /**/) -> /* Java */ java::lang::String /**/ {
		return  if ch != null { org::apache::commons::lang3::char_utils::CharUtils::unicode_escaped(&ch.charValue()) } else { null };
	}

	pub fn new() -> org::apache::commons::lang3::char_utils::CharUtils {
	// empty
	}
}