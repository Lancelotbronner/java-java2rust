use java::text::SimpleDateFormat;
use java::util::ArrayList;
use java::util::Calendar;
use java::util::Date;
use java::util::GregorianCalendar;
use java::util::Objects;
use java::util::TimeZone;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Strings;
use crate::org::apache::commons::lang3::Validate;

pub struct DurationFormatUtils;

impl DurationFormatUtils {
	static MINUTES_PER_HOUR: i32 = 60;

	static SECONDS_PER_MINUTES: i32 = 60;

	static HOURS_PER_DAY: i32 = 24;

	pub static ISO_EXTENDED_FORMAT_PATTERN: /* Java */ java::lang::String /**/ = "'P'yyyy'Y'M'M'd'DT'H'H'm'M's.SSS'S'";

	static y: /* Java */ java::lang::String /**/ = "y";

	static M: /* Java */ java::lang::String /**/ = "M";

	static d: /* Java */ java::lang::String /**/ = "d";

	static H: /* Java */ java::lang::String /**/ = "H";

	static m: /* Java */ java::lang::String /**/ = "m";

	static s: /* Java */ java::lang::String /**/ = "s";

	static S: /* Java */ java::lang::String /**/ = "S";

	fn format(&self, tokens: &&[org::apache::commons::lang3::time::duration_format_utils::Token], years: i64, months: i64, days: i64, hours: i64, minutes: i64, seconds: i64, milliseconds: i64, pad_with_zeros: bool) -> /* Java */ java::lang::String /**/ {
		/* final */ let buffer: StringBuilder = StringBuilder::new();
		let last_output_seconds: bool = false;
		let last_output_zero: bool = false;
		let optional_start: i32 = -1;
		let first_optional_non_literal: bool = false;
		let optional_index: i32 = -1;
		let in_optional: bool = false;
		for /* final */ token in tokens {
			/* final */ let value: Object = token.get_value();
			/* final */ let is_literal: bool = value instanceof StringBuilder;
			/* final */ let count: i32 = token.get_count();
			if optional_index != token.optionalIndex {
				optional_index = token.optionalIndex;
				if optional_index > -1 {
					//entering new optional block
					optional_start = buffer.length();
					last_output_zero = false;
					in_optional = true;
					first_optional_non_literal = false;
				} else {
					//leaving optional block
					in_optional = false;
				}
			}
			if is_literal {
				if !in_optional || !last_output_zero {
					buffer.append(&value.toString());
				}
			} else if value.equals(self.y) {
				last_output_seconds = false;
				last_output_zero = years == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(years, pad_with_zeros, count));
				}
			} else if value.equals(self.M) {
				last_output_seconds = false;
				last_output_zero = months == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(months, pad_with_zeros, count));
				}
			} else if value.equals(self.d) {
				last_output_seconds = false;
				last_output_zero = days == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(days, pad_with_zeros, count));
				}
			} else if value.equals(self.H) {
				last_output_seconds = false;
				last_output_zero = hours == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(hours, pad_with_zeros, count));
				}
			} else if value.equals(self.m) {
				last_output_seconds = false;
				last_output_zero = minutes == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(minutes, pad_with_zeros, count));
				}
			} else if value.equals(self.s) {
				last_output_seconds = true;
				last_output_zero = seconds == 0;
				if !in_optional || !last_output_zero {
					buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(seconds, pad_with_zeros, count));
				}
			} else if value.equals(self.S) {
				last_output_zero = milliseconds == 0;
				if !in_optional || !last_output_zero {
					if last_output_seconds {
						// ensure at least 3 digits are displayed even if padding is not selected
						/* final */ let width: i32 =  if pad_with_zeros { Math::max(3, count) } else { 3 };
						buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(milliseconds, true, width));
					} else {
						buffer.append(&org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::padded_value(milliseconds, pad_with_zeros, count));
					}
				}
				last_output_seconds = false;
			}
			//as soon as we hit first nonliteral in optional, check for literal prefix
			if in_optional && !is_literal && !first_optional_non_literal {
				first_optional_non_literal = true;
				if last_output_zero {
					buffer.delete(optional_start, &buffer.length());
				}
			}
		}
		return buffer.toString();
	}

	pub fn format_duration(&self, duration_millis: i64, format: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_duration(duration_millis, format, true);
	}

	pub fn format_duration(&self, duration_millis: i64, format: &/* Java */ java::lang::String /**/, pad_with_zeros: bool) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		Validate::inclusive_between(0, Long::MAX_VALUE, duration_millis, "durationMillis must not be negative")?;
		/* final */ let tokens: Vec<Token> = org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::lexx(format)?;
		let days: i64 = 0;
		let hours: i64 = 0;
		let minutes: i64 = 0;
		let seconds: i64 = 0;
		let milliseconds: i64 = duration_millis;
		if Token::contains_token_with_value(tokens, self.d) {
			days = milliseconds / DateUtils::MILLIS_PER_DAY;
			milliseconds -= days * DateUtils::MILLIS_PER_DAY;
		}
		if Token::contains_token_with_value(tokens, self.H) {
			hours = milliseconds / DateUtils::MILLIS_PER_HOUR;
			milliseconds -= hours * DateUtils::MILLIS_PER_HOUR;
		}
		if Token::contains_token_with_value(tokens, self.m) {
			minutes = milliseconds / DateUtils::MILLIS_PER_MINUTE;
			milliseconds -= minutes * DateUtils::MILLIS_PER_MINUTE;
		}
		if Token::contains_token_with_value(tokens, self.s) {
			seconds = milliseconds / DateUtils::MILLIS_PER_SECOND;
			milliseconds -= seconds * DateUtils::MILLIS_PER_SECOND;
		}
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format(tokens, 0, 0, days, hours, minutes, seconds, milliseconds, pad_with_zeros);
	}

	pub fn format_durationhms(&self, duration_millis: i64) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_duration(duration_millis, "HH:mm:ss.SSS");
	}

	pub fn format_durationiso(&self, duration_millis: i64) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_duration(duration_millis, self.ISO_EXTENDED_FORMAT_PATTERN, false)?;
	}

	pub fn format_duration_words(&self, duration_millis: i64, suppress_leading_zero_elements: bool, suppress_trailing_zero_elements: bool) -> /* Java */ java::lang::String /**/ {
		// This method is generally replaceable by the format method, but
		// there are a series of tweaks and special cases that require
		// trickery to replicate.
		let duration: String = org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_duration(duration_millis, "d' days 'H' hours 'm' minutes 's' seconds'");
		if suppress_leading_zero_elements {
			// this is a temporary marker on the front. Like ^ in regexp.
			duration = " " + duration;
			/* final */ let text: String = duration;
			let tmp: String = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text, " 0 days", StringUtils::EMPTY);
			if tmp.length() != duration.length() {
				duration = tmp;
				/* final */ let text1: String = duration;
				tmp = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text1, " 0 hours", StringUtils::EMPTY);
				if tmp.length() != duration.length() {
					duration = tmp;
					/* final */ let text2: String = duration;
					tmp = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text2, " 0 minutes", StringUtils::EMPTY);
					duration = tmp;
				}
			}
			if !duration.isEmpty() {
				// strip the space off again
				duration = duration.substring(1);
			}
		}
		if suppress_trailing_zero_elements {
			/* final */ let text: String = duration;
			let tmp: String = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text, " 0 seconds", StringUtils::EMPTY);
			if tmp.length() != duration.length() {
				duration = tmp;
				/* final */ let text1: String = duration;
				tmp = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text1, " 0 minutes", StringUtils::EMPTY);
				if tmp.length() != duration.length() {
					duration = tmp;
					/* final */ let text2: String = duration;
					tmp = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text2, " 0 hours", StringUtils::EMPTY);
					if tmp.length() != duration.length() {
						/* final */ let text3: String = tmp;
						duration = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text3, " 0 days", StringUtils::EMPTY);
					}
				}
			}
		}
		// handle plurals
		duration = " " + duration;
		/* final */ let text: String = duration;
		duration = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text, " 1 seconds", " 1 second");
		/* final */ let text1: String = duration;
		duration = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text1, " 1 minutes", " 1 minute");
		/* final */ let text2: String = duration;
		duration = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text2, " 1 hours", " 1 hour");
		/* final */ let text3: String = duration;
		duration = Strings::org::apache::commons::lang3::strings::Strings::CS.replace_once(text3, " 1 days", " 1 day");
		return duration.trim();
	}

	pub fn format_period(&self, start_millis: i64, end_millis: i64, format: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_period(start_millis, end_millis, format, true, &TimeZone::getDefault());
	}

	pub fn format_period(&self, start_millis: i64, end_millis: i64, format: &/* Java */ java::lang::String /**/, pad_with_zeros: bool, timezone: &/* Java */ java::util::TimeZone /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		Validate::is_true(start_millis <= end_millis, "startMillis must not be greater than endMillis")?;
		// Used to optimize for differences under 28 days and
		// called formatDuration(millis, format); however this did not work
		// over leap years.
		// TODO: Compare performance to see if anything was lost by
		// losing this optimization.
		/* final */ let tokens: Vec<Token> = org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::lexx(format)?;
		// time zones get funky around 0, so normalizing everything to GMT
		// stops the hours being off
		/* final */ let start: Calendar = Calendar::getInstance(timezone);
		start.setTime(Date::new(start_millis));
		/* final */ let end: Calendar = Calendar::getInstance(timezone);
		end.setTime(Date::new(end_millis));
		// initial estimates
		let milliseconds: i64 = end.get(Calendar::MILLISECOND) - start.get(Calendar::MILLISECOND);
		let seconds: i32 = end.get(Calendar::SECOND) - start.get(Calendar::SECOND);
		let minutes: i32 = end.get(Calendar::MINUTE) - start.get(Calendar::MINUTE);
		let hours: i32 = end.get(Calendar::HOUR_OF_DAY) - start.get(Calendar::HOUR_OF_DAY);
		let days: i32 = end.get(Calendar::DAY_OF_MONTH) - start.get(Calendar::DAY_OF_MONTH);
		let months: i32 = end.get(Calendar::MONTH) - start.get(Calendar::MONTH);
		let years: i32 = end.get(Calendar::YEAR) - start.get(Calendar::YEAR);
		// each initial estimate is adjusted in case it is under 0
		while milliseconds < 0 {
			milliseconds += DateUtils::MILLIS_PER_SECOND;
			seconds -= 1;
		}
		while seconds < 0 {
			seconds += self.SECONDS_PER_MINUTES;
			minutes -= 1;
		}
		while minutes < 0 {
			minutes += self.MINUTES_PER_HOUR;
			hours -= 1;
		}
		while hours < 0 {
			hours += self.HOURS_PER_DAY;
			days -= 1;
		}
		if Token::contains_token_with_value(tokens, self.M) {
			while days < 0 {
				days += start.getActualMaximum(Calendar::DAY_OF_MONTH);
				months -= 1;
				start.add(Calendar::MONTH, 1);
			}
			while months < 0 {
				months += 12;
				years -= 1;
			}
			if !Token::contains_token_with_value(tokens, self.y) && years != 0 {
				while years != 0 {
					months += 12 * years;
					years = 0;
				}
			}
		} else {
			if !Token::contains_token_with_value(tokens, self.y) {
				let target: i32 = end.get(Calendar::YEAR);
				if months < 0 {
					// target is end-year -1
					target -= 1;
				}
				while start.get(Calendar::YEAR) != target {
					days += start.getActualMaximum(Calendar::DAY_OF_YEAR) - start.get(Calendar::DAY_OF_YEAR);
					// Not sure I grok why this is needed, but the brutal tests show it is
					if start instanceof GregorianCalendar && start.get(Calendar::MONTH) == Calendar::FEBRUARY && start.get(Calendar::DAY_OF_MONTH) == 29 {
						days += 1;
					}
					start.add(Calendar::YEAR, 1);
					days += start.get(Calendar::DAY_OF_YEAR);
				}
				years = 0;
			}
			while start.get(Calendar::MONTH) != end.get(Calendar::MONTH) {
				days += start.getActualMaximum(Calendar::DAY_OF_MONTH);
				start.add(Calendar::MONTH, 1);
			}
			months = 0;
			while days < 0 {
				days += start.getActualMaximum(Calendar::DAY_OF_MONTH);
				months -= 1;
				start.add(Calendar::MONTH, 1);
			}
		}
		if !Token::contains_token_with_value(tokens, self.d) {
			hours += self.HOURS_PER_DAY * days;
			days = 0;
		}
		if !Token::contains_token_with_value(tokens, self.H) {
			minutes += self.MINUTES_PER_HOUR * hours;
			hours = 0;
		}
		if !Token::contains_token_with_value(tokens, self.m) {
			seconds += self.SECONDS_PER_MINUTES * minutes;
			minutes = 0;
		}
		if !Token::contains_token_with_value(tokens, self.s) {
			milliseconds += DateUtils::MILLIS_PER_SECOND * seconds;
			seconds = 0;
		}
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format(tokens, years, months, days, hours, minutes, seconds, milliseconds, pad_with_zeros);
	}

	pub fn format_periodiso(&self, start_millis: i64, end_millis: i64) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils::format_period(start_millis, end_millis, self.ISO_EXTENDED_FORMAT_PATTERN, false, &TimeZone::getDefault())?;
	}

	fn lexx(&self, format: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> &[org::apache::commons::lang3::time::duration_format_utils::Token] {
		/* final */ let list: ArrayList<Token> = ArrayList<>::new(&format.length());
		let in_literal: bool = false;
		// Although the buffer is stored in a Token, the Tokens are only
		// used internally, so cannot be accessed by other threads
		let buffer: StringBuilder = null;
		let previous: Token = null;
		let in_optional: bool = false;
		let optional_index: i32 = -1;
		 {
			let i: i32 = 0;
			while i < format.length() {
				{
					/* final */ let ch: char = format.charAt(i);
					if in_literal && ch != '\'' {
						// buffer can't be null if inLiteral is true
						buffer.append(ch);
						continue;
					}
					let value: String = null;
					match ch {
						// TODO: Need to handle escaping of '
						'[' =>  {
							if in_optional {
								return Err(IllegalArgumentException::new("Nested optional block at index: " + i));
							}
							optional_index += 1;
							in_optional = true;
							break;
						}
						']' =>  {
							if !in_optional {
								return Err(IllegalArgumentException::new("Attempting to close unopened optional block at index: " + i));
							}
							in_optional = false;
							break;
						}
						'\'' =>  {
							if in_literal {
								buffer = null;
								in_literal = false;
							} else {
								buffer = StringBuilder::new();
								list.add(Token::new(buffer, in_optional, optional_index));
								in_literal = true;
							}
							break;
						}
						'y' =>  {
							value = self.y;
							break;
						}
						'M' =>  {
							value = self.M;
							break;
						}
						'd' =>  {
							value = self.d;
							break;
						}
						'H' =>  {
							value = self.H;
							break;
						}
						'm' =>  {
							value = self.m;
							break;
						}
						's' =>  {
							value = self.s;
							break;
						}
						'S' =>  {
							value = self.S;
							break;
						}
						_ =>  {
							if buffer == null {
								buffer = StringBuilder::new();
								list.add(Token::new(buffer, in_optional, optional_index));
							}
							buffer.append(ch);
						}
					}
					if value != null {
						if previous != null && previous.get_value().equals(value) {
							previous.increment();
						} else {
							/* final */ let token: Token = Token::new(value, in_optional, optional_index);
							list.add(token);
							previous = token;
						}
						buffer = null;
					}
				}
				i += 1;
			 }
		 }
	
		if in_literal {
			// i.e. we have not found the end of the literal
			return Err(IllegalArgumentException::new("Unmatched quote in format: " + format));
		}
		if in_optional {
			// i.e. we have not found the end of the literal
			return Err(IllegalArgumentException::new("Unmatched optional in format: " + format));
		}
		return list.toArray(Token::EMPTY_ARRAY);
	}

	fn padded_value(&self, value: i64, pad_with_zeros: bool, count: i32) -> /* Java */ java::lang::String /**/ {
		/* final */ let long_string: String = Long::toString(value);
		return  if pad_with_zeros { StringUtils::left_pad(long_string, count, '0') } else { long_string };
	}

	pub fn new() -> org::apache::commons::lang3::time::duration_format_utils::DurationFormatUtils {
	// empty
	}
}

struct Token {
	value: /* Java */ java::lang::CharSequence /**/,
	count: i32,
	optional_index: i32 = -1,
}

impl Token {
	static EMPTY_ARRAY: &[org::apache::commons::lang3::time::duration_format_utils::Token] = ;

	fn contains_token_with_value(&self, tokens: &&[org::apache::commons::lang3::time::duration_format_utils::Token], value: &/* Java */ java::lang::Object /**/) -> bool {
		return Stream::of(tokens).anyMatch(|token|token.get_value() == value);
	}

	fn new(value: &/* Java */ java::lang::CharSequence /**/, optional: bool, optional_index: i32) -> org::apache::commons::lang3::time::duration_format_utils::Token {
		self.value = Objects::requireNonNull(value, "value");
		self.count = 1;
		if optional {
			self.optionalIndex = optional_index;
		}
	}

	pub fn equals(&self, obj2: &/* Java */ java::lang::Object /**/) -> bool {
		if obj2 instanceof Token {
			/* final */ let tok2: Token = obj2 as Token;
			if self.value.getClass() != tok2.value.getClass() {
				return false;
			}
			if self.count != tok2.count {
				return false;
			}
			if self.value instanceof StringBuilder {
				return self.value.toString().equals(&tok2.value.toString());
			}
			if self.value instanceof Number {
				return self.value.equals(tok2.value);
			}
			return self.value == tok2.value;
		}
		return false;
	}

	fn get_count(&self) -> i32 {
		return self.count;
	}

	fn get_value(&self) -> /* Java */ java::lang::Object /**/ {
		return self.value;
	}

	pub fn hash_code(&self) -> i32 {
		return self.value.hashCode();
	}

	fn increment(&self) {
		self.count += 1;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return StringUtils::repeat(&self.value.toString(), self.count);
	}
}