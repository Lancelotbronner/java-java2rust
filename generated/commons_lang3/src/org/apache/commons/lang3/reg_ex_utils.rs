use java::util::Objects;
use java::util::regex::Matcher;
use java::util::regex::Pattern;

pub struct RegExUtils;

impl RegExUtils {
	static VERSION_SPLIT_PATTERN: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("\\.");

	pub fn dot_all(&self, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::regex::Pattern /**/ {
		return Pattern::compile(regex, Pattern::DOTALL);
	}

	pub fn dot_all_matcher(&self, regex: &/* Java */ java::lang::String /**/, text: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::util::regex::Matcher /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::dot_all(regex).matcher(text);
	}

	pub fn dot_all_matcher(&self, regex: &/* Java */ java::lang::String /**/, text: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::regex::Matcher /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::dot_all(regex).matcher(text);
	}

	pub fn remove_all(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::util::regex::Pattern /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_all(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::util::regex::Pattern /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_all(text as CharSequence, regex, StringUtils::EMPTY);
	}

	pub fn remove_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_all(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_first(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::util::regex::Pattern /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_first(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::util::regex::Pattern /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_first(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_first(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_pattern(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_pattern(text, regex, StringUtils::EMPTY);
	}

	pub fn remove_pattern(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_pattern(text as CharSequence, regex, StringUtils::EMPTY);
	}

	pub fn replace_all(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::util::regex::Pattern /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if ObjectUtils::any_null(text, regex, replacement) {
			return org::apache::commons::lang3::reg_ex_utils::RegExUtils::to_string_or_null(text);
		}
		return regex.matcher(text).replaceAll(replacement);
	}

	pub fn replace_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::util::regex::Pattern /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_all(text as CharSequence, regex, replacement);
	}

	pub fn replace_all(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if ObjectUtils::any_null(text, regex, replacement) {
			return text;
		}
		return text.replaceAll(regex, replacement);
	}

	pub fn replace_first(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::util::regex::Pattern /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if text == null || regex == null || replacement == null {
			return org::apache::commons::lang3::reg_ex_utils::RegExUtils::to_string_or_null(text);
		}
		return regex.matcher(text).replaceFirst(replacement);
	}

	pub fn replace_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::util::regex::Pattern /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_first(text as CharSequence, regex, replacement);
	}

	pub fn replace_first(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if text == null || regex == null || replacement == null {
			return text;
		}
		return text.replaceFirst(regex, replacement);
	}

	pub fn replace_pattern(&self, text: &/* Java */ java::lang::CharSequence /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if ObjectUtils::any_null(text, regex, replacement) {
			return org::apache::commons::lang3::reg_ex_utils::RegExUtils::to_string_or_null(text);
		}
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::dot_all_matcher(regex, text).replaceAll(replacement);
	}

	pub fn replace_pattern(&self, text: &/* Java */ java::lang::String /**/, regex: &/* Java */ java::lang::String /**/, replacement: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::reg_ex_utils::RegExUtils::replace_pattern(text as CharSequence, regex, replacement);
	}

	fn to_string_or_null(&self, text: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(text, null);
	}

	pub fn new() -> org::apache::commons::lang3::reg_ex_utils::RegExUtils {
	// empty
	}
}