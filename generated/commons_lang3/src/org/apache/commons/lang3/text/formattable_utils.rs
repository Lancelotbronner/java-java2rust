use java::util::Formattable;
use java::util::FormattableFlags;
use java::util::Formatter;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Validate;

pub struct FormattableUtils;

impl FormattableUtils {
	static SIMPLEST_FORMAT: /* Java */ java::lang::String /**/ = "%s";

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/, formatter: &/* Java */ java::util::Formatter /**/, flags: i32, width: i32, precision: i32) -> /* Java */ java::util::Formatter /**/ {
		return org::apache::commons::lang3::text::formattable_utils::FormattableUtils::append(seq, formatter, flags, width, precision, ' ', null);
	}

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/, formatter: &/* Java */ java::util::Formatter /**/, flags: i32, width: i32, precision: i32, pad_char: u16) -> /* Java */ java::util::Formatter /**/ {
		return org::apache::commons::lang3::text::formattable_utils::FormattableUtils::append(seq, formatter, flags, width, precision, pad_char, null);
	}

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/, formatter: &/* Java */ java::util::Formatter /**/, flags: i32, width: i32, precision: i32, pad_char: u16, ellipsis: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Formatter /**/ {
		Validate::is_true(ellipsis == null || precision < 0 || ellipsis.length() <= precision, "Specified ellipsis '%1$s' exceeds precision of %2$s", ellipsis, &Integer::valueOf(precision))?;
		/* final */ let buf: StringBuilder = StringBuilder::new(seq);
		if precision >= 0 && precision < seq.length() {
			/* final */ let actual_ellipsis: CharSequence = ObjectUtils::get_if_null(ellipsis, StringUtils::EMPTY);
			buf.replace(precision - actual_ellipsis.length(), &seq.length(), &actual_ellipsis.toString());
		}
		/* final */ let left_justify: bool = (flags & FormattableFlags::LEFT_JUSTIFY) == FormattableFlags::LEFT_JUSTIFY;
		 {
			let i: i32 = buf.length();
			while i < width {
				{
					buf.insert( if left_justify { i } else { 0 }, pad_char);
				}
				i += 1;
			 }
		 }
	
		formatter.format(&buf.toString());
		return formatter;
	}

	pub fn append(&self, seq: &/* Java */ java::lang::CharSequence /**/, formatter: &/* Java */ java::util::Formatter /**/, flags: i32, width: i32, precision: i32, ellipsis: &/* Java */ java::lang::CharSequence /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Formatter /**/ {
		return org::apache::commons::lang3::text::formattable_utils::FormattableUtils::append(seq, formatter, flags, width, precision, ' ', ellipsis)?;
	}

	pub fn to_string(&self, formattable: &/* Java */ java::util::Formattable /**/) -> /* Java */ java::lang::String /**/ {
		return String::format(self.SIMPLEST_FORMAT, formattable);
	}

	pub fn new() -> org::apache::commons::lang3::text::formattable_utils::FormattableUtils {
	}
}