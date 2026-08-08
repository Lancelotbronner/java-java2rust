use java::io::IOException;
use java::io::ObjectInputStream;
use java::io::Serializable;
use java::text::DateFormat;
use java::text::DateFormatSymbols;
use java::text::FieldPosition;
use java::text::SimpleDateFormat;
use java::util::ArrayList;
use java::util::Calendar;
use java::util::Date;
use java::util::List;
use java::util::Locale;
use java::util::TimeZone;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ConcurrentMap;
use crate::org::apache::commons::lang3::CharUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::LocaleUtils;
use crate::org::apache::commons::lang3::exception::ExceptionUtils;

pub struct FastDatePrinter {
	pattern: /* Java */ java::lang::String /**/,
	time_zone: /* Java */ java::util::TimeZone /**/,
	locale: /* Java */ java::util::Locale /**/,
	rules: &[org::apache::commons::lang3::time::fast_date_printer::Rule],
	max_length_estimate: i32,
}

impl FastDatePrinter {
	static EMPTY_RULE_ARRAY: &[org::apache::commons::lang3::time::fast_date_printer::Rule] = ;

	static serialVersionUID: i64 = 1;

	pub static FULL: i32 = DateFormat::FULL;

	pub static LONG: i32 = DateFormat::LONG;

	pub static MEDIUM: i32 = DateFormat::MEDIUM;

	pub static SHORT: i32 = DateFormat::SHORT;

	static MAX_DIGITS: i32 = 10;

	static timeZoneDisplayCache: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new(7);

	fn append_digits(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		buffer.append((value / 10 + '0') as char);
		buffer.append((value % 10 + '0') as char);
	}

	fn append_full_digits(&self, buffer: &/* Java */ java::lang::Appendable /**/, mut value: i32, min_field_width: i32) /* thrown(java.io.IOException) */ {
		// see LANG-1248
		if value < 10000 {
			// less memory allocation path works for four digits or less
			let n_digits: i32 = 4;
			if value < 1000 {
				n_digits -= 1;
				if value < 100 {
					n_digits -= 1;
					if value < 10 {
						n_digits -= 1;
					}
				}
			}
			// left zero pad
			 {
				let i: i32 = min_field_width - n_digits;
				while i > 0 {
					{
						buffer.append('0');
					}
					i -= 1;
				 }
			 }
	
			match n_digits {
				4 =>  {
					buffer.append((value / 1000 + '0') as char);
					value %= 1000;
				}
				// falls-through
				3 =>  {
					if value >= 100 {
						buffer.append((value / 100 + '0') as char);
						value %= 100;
					} else {
						buffer.append('0');
					}
				}
				// falls-through
				2 =>  {
					if value >= 10 {
						buffer.append((value / 10 + '0') as char);
						value %= 10;
					} else {
						buffer.append('0');
					}
				}
				// falls-through
				1 => buffer.append((value + '0') as char),
			}
		} else {
			// more memory allocation path works for any digits
			// build up decimal representation in reverse
			/* final */ let work: [Option<char>; self.MAX_DIGITS] = [None; self.MAX_DIGITS];
			let digit: i32 = 0;
			while value != 0 {
				work[digit += 1 !!!check!!! post increment] = (value % 10 + '0') as char;
				value /= 10;
			}
			// pad with zeros
			while digit < min_field_width {
				buffer.append('0');
				min_field_width -= 1;
			}
			// reverse
			while digit -= 1 >= 0 {
				buffer.append(work[digit]);
			}
		}
	}

	fn clear(&self) {
		self.time_zone_display_cache.clear();
	}

	fn get_time_zone_display(&self, tz: &/* Java */ java::util::TimeZone /**/, daylight: bool, style: i32, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let key: TimeZoneDisplayKey = TimeZoneDisplayKey::new(tz, daylight, style, locale);
		// This is a very slow call, so cache the results.
		return self.time_zone_display_cache.computeIfAbsent(key, |k|tz.getDisplayName(daylight, style, locale));
	}

	fn new(pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter {
		self.pattern = pattern;
		self.timeZone = time_zone;
		self.locale = LocaleUtils::to_locale(locale);
		self.init();
	}

	fn apply_rules<B: /* Java */ java::lang::Appendable /**/>(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &B) /* thrown(T) */ -> B {
		let r0 = 'try0: {
			for /* final */ rule in self.rules {
				rule.append_to(buf, calendar);
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				match ExceptionUtils::as_runtime_exception(ioe) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return buf;
	}

	fn apply_rules(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &/* Java */ java::lang::StringBuffer /**/) /* thrown(T) */ -> /* Java */ java::lang::StringBuffer /**/ {
		return self.apply_rules(calendar, buf as Appendable)? as StringBuffer;
	}

	fn apply_rules_to_string(&self, c: &/* Java */ java::util::Calendar /**/) /* thrown(T) */ -> /* Java */ java::lang::String /**/ {
		return self.apply_rules(c, StringBuilder::new(self.max_length_estimate))?.toString();
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if !(obj instanceof FastDatePrinter) {
			return false;
		}
		/* final */ let other: FastDatePrinter = obj as FastDatePrinter;
		return self.pattern.equals(other.pattern) && self.time_zone.equals(other.timeZone) && self.locale.equals(other.locale);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::lang::String /**/ {
		return self.format(calendar, StringBuilder::new(self.max_length_estimate)).toString();
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &B) /* thrown(T) */ -> B {
		// Don't edit the given Calendar, clone it only if needed.
		let actual: Calendar = calendar;
		if !calendar.getTimeZone().equals(self.time_zone) {
			actual = calendar.clone() as Calendar;
			actual.setTimeZone(self.time_zone);
		}
		return self.apply_rules(actual, buf)?;
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		// do not pass in calendar directly, this will cause TimeZone of FastDatePrinter to be ignored
		return self.format(&calendar.getTime(), buf);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/) /* thrown(T) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTime(date);
		return self.apply_rules_to_string(c)?;
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, date: &/* Java */ java::util::Date /**/, buf: &B) /* thrown(T) */ -> B {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTime(date);
		return self.apply_rules(c, buf)?;
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, buf: &/* Java */ java::lang::StringBuffer /**/) /* thrown(T) */ -> /* Java */ java::lang::StringBuffer /**/ {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTime(date);
		return self.apply_rules(c, buf as Appendable)? as StringBuffer;
	}

	pub fn format(&self, millis: i64) /* thrown(T) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTimeInMillis(millis);
		return self.apply_rules_to_string(c)?;
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, millis: i64, buf: &B) /* thrown(T) */ -> B {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTimeInMillis(millis);
		return self.apply_rules(c, buf)?;
	}

	pub fn format(&self, millis: i64, buf: &/* Java */ java::lang::StringBuffer /**/) /* thrown(T) */ -> /* Java */ java::lang::StringBuffer /**/ {
		/* final */ let c: Calendar = self.new_calendar();
		c.setTimeInMillis(millis);
		return self.apply_rules(c, buf as Appendable)? as StringBuffer;
	}

	fn format(&self, obj: &/* Java */ java::lang::Object /**/) /* thrown(T | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if obj instanceof Date {
			return self.format(obj as Date)?;
		}
		if obj instanceof Calendar {
			return self.format(obj as Calendar);
		}
		if obj instanceof Long {
			return self.format(&(obj as Long).longValue())?;
		}
		return Err(IllegalArgumentException::new("Unknown class: " + ClassUtils::get_name(obj, "<null>")));
	}

	pub fn format(&self, obj: &/* Java */ java::lang::Object /**/, to_append_to: &/* Java */ java::lang::StringBuffer /**/, pos: &/* Java */ java::text::FieldPosition /**/) /* thrown(T | java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::StringBuffer /**/ {
		if obj instanceof Date {
			return self.format(obj as Date, to_append_to)?;
		}
		if obj instanceof Calendar {
			return self.format(obj as Calendar, to_append_to);
		}
		if obj instanceof Long {
			return self.format(&(obj as Long).longValue(), to_append_to)?;
		}
		return Err(IllegalArgumentException::new("Unknown class: " + ClassUtils::get_name(obj, "<null>")));
	}

	pub fn get_locale(&self) -> /* Java */ java::util::Locale /**/ {
		return self.locale;
	}

	pub fn get_max_length_estimate(&self) -> i32 {
		return self.max_length_estimate;
	}

	pub fn get_pattern(&self) -> /* Java */ java::lang::String /**/ {
		return self.pattern;
	}

	pub fn get_time_zone(&self) -> /* Java */ java::util::TimeZone /**/ {
		return self.time_zone;
	}

	pub fn hash_code(&self) -> i32 {
		return self.pattern.hashCode() + 13 * (self.time_zone.hashCode() + 13 * self.locale.hashCode());
	}

	fn init(&mut self) {
		/* final */ let rules_list: List<Rule> = self.parse_pattern()?;
		self.rules = rules_list.toArray(self.EMPTY_RULE_ARRAY);
		let len: i32 = 0;
		 {
			let i: i32 = self.rules.length;
			while i -= 1 >= 0{
				len += self.rules[i].estimate_length();
			}
		 }
	
		self.max_length_estimate = len;
	}

	fn new_calendar(&self) -> /* Java */ java::util::Calendar /**/ {
		return Calendar::getInstance(self.time_zone, self.locale);
	}

	fn parse_pattern(&self) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::List /**/ {
		/* final */ let symbols: DateFormatSymbols = DateFormatSymbols::new(self.locale);
		/* final */ let rules: List<Rule> = ArrayList<>::new();
		/* final */ const ERAs: Vec<String> = symbols.getEras();
		/* final */ let months: Vec<String> = symbols.getMonths();
		/* final */ let short_months: Vec<String> = symbols.getShortMonths();
		/* final */ let weekdays: Vec<String> = symbols.getWeekdays();
		/* final */ let short_weekdays: Vec<String> = symbols.getShortWeekdays();
		/* final */ const AmPmStrings: Vec<String> = symbols.getAmPmStrings();
		/* final */ let length: i32 = self.pattern.length();
		/* final */ let index_ref: [i32; 1] = [0; 1];
		 {
			let i: i32 = 0;
			while i < length {
				{
					index_ref[0] = i;
					/* final */ let token: String = self.parse_token(self.pattern, index_ref);
					i = index_ref[0];
					/* final */ let token_len: i32 = token.length();
					if token_len == 0 {
						break;
					}
					let rule: Rule;
					/* final */ let c: char = token.charAt(0);
					match c {
						// era designator (text)
						'G' =>  {
							rule = TextField::new(Calendar::ERA, ERAs);
							break;
						}
						// year (number)
						'y' =>  {
						}
						// week year
						'Y' =>  {
							if token_len == 2 {
								rule = TwoDigitYearField::org::apache::commons::lang3::time::fast_date_printer::TwoDigitYearField::INSTANCE;
							} else {
								rule = self.select_number_rule(Calendar::YEAR, &Math::max(token_len, 4));
							}
							if c == 'Y' {
								rule = WeekYear::new(rule as NumberRule);
							}
							break;
						}
						// month in year (text and number)
						'M' =>  {
							if token_len >= 4 {
								rule = TextField::new(Calendar::MONTH, months);
							} else if token_len == 3 {
								rule = TextField::new(Calendar::MONTH, short_months);
							} else if token_len == 2 {
								rule = TwoDigitMonthField::org::apache::commons::lang3::time::fast_date_printer::TwoDigitMonthField::INSTANCE;
							} else {
								rule = UnpaddedMonthField::org::apache::commons::lang3::time::fast_date_printer::UnpaddedMonthField::INSTANCE;
							}
							break;
						}
						// month in year (text and number)
						'L' =>  {
							if token_len >= 4 {
								rule = TextField::new(Calendar::MONTH, &CalendarUtils::get_instance(self.locale).get_standalone_long_month_names());
							} else if token_len == 3 {
								rule = TextField::new(Calendar::MONTH, &CalendarUtils::get_instance(self.locale).get_standalone_short_month_names());
							} else if token_len == 2 {
								rule = TwoDigitMonthField::org::apache::commons::lang3::time::fast_date_printer::TwoDigitMonthField::INSTANCE;
							} else {
								rule = UnpaddedMonthField::org::apache::commons::lang3::time::fast_date_printer::UnpaddedMonthField::INSTANCE;
							}
							break;
						}
						// day in month (number)
						'd' =>  {
							rule = self.select_number_rule(Calendar::DAY_OF_MONTH, token_len);
							break;
						}
						// hour in am/pm (number, 1..12)
						'h' =>  {
							rule = TwelveHourField::new(&self.select_number_rule(Calendar::HOUR, token_len));
							break;
						}
						// hour in day (number, 0..23)
						'H' =>  {
							rule = self.select_number_rule(Calendar::HOUR_OF_DAY, token_len);
							break;
						}
						// minute in hour (number)
						'm' =>  {
							rule = self.select_number_rule(Calendar::MINUTE, token_len);
							break;
						}
						// second in minute (number)
						's' =>  {
							rule = self.select_number_rule(Calendar::SECOND, token_len);
							break;
						}
						// millisecond (number)
						'S' =>  {
							rule = self.select_number_rule(Calendar::MILLISECOND, token_len);
							break;
						}
						// day in week (text)
						'E' =>  {
							rule = TextField::new(Calendar::DAY_OF_WEEK,  if token_len < 4 { short_weekdays } else { weekdays });
							break;
						}
						// day in week (number)
						'u' =>  {
							rule = DayInWeekField::new(&self.select_number_rule(Calendar::DAY_OF_WEEK, token_len));
							break;
						}
						// day in year (number)
						'D' =>  {
							rule = self.select_number_rule(Calendar::DAY_OF_YEAR, token_len);
							break;
						}
						// day of week in month (number)
						'F' =>  {
							rule = self.select_number_rule(Calendar::DAY_OF_WEEK_IN_MONTH, token_len);
							break;
						}
						// week in year (number)
						'w' =>  {
							rule = self.select_number_rule(Calendar::WEEK_OF_YEAR, token_len);
							break;
						}
						// week in month (number)
						'W' =>  {
							rule = self.select_number_rule(Calendar::WEEK_OF_MONTH, token_len);
							break;
						}
						// am/pm marker (text)
						'a' =>  {
							rule = TextField::new(Calendar::AM_PM, AmPmStrings);
							break;
						}
						// hour in day (1..24)
						'k' =>  {
							rule = TwentyFourHourField::new(&self.select_number_rule(Calendar::HOUR_OF_DAY, token_len));
							break;
						}
						// hour in am/pm (0..11)
						'K' =>  {
							rule = self.select_number_rule(Calendar::HOUR, token_len);
							break;
						}
						// ISO 8601
						'X' =>  {
							rule = Iso8601_Rule::get_rule(token_len)?;
							break;
						}
						// time zone (text)
						'z' =>  {
							rule = TimeZoneNameRule::new(self.time_zone, self.locale,  if token_len >= 4 { TimeZone::LONG } else { TimeZone::SHORT });
							break;
						}
						// time zone (value)
						'Z' =>  {
							if token_len == 1 {
								rule = TimeZoneNumberRule::org::apache::commons::lang3::time::fast_date_printer::TimeZoneNumberRule::INSTANCE_NO_COLON;
							} else if token_len == 2 {
								rule = Iso8601_Rule::org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule::ISO8601_HOURS_COLON_MINUTES;
							} else {
								rule = TimeZoneNumberRule::org::apache::commons::lang3::time::fast_date_printer::TimeZoneNumberRule::INSTANCE_COLON;
							}
							break;
						}
						// literal text
						'\'' =>  {
							/* final */ let sub: String = token.substring(1);
							if sub.length() == 1 {
								rule = CharacterLiteral::new(&sub.charAt(0));
							} else {
								rule = StringLiteral::new(sub);
							}
							break;
						}
						_ =>  {
							return Err(IllegalArgumentException::new("Illegal pattern component: " + token));
						}
					}
					rules.add(rule);
				}
				i += 1;
			 }
		 }
	
		return rules;
	}

	fn parse_token(&self, pattern: &/* Java */ java::lang::String /**/, mut index_ref: &&[i32]) -> /* Java */ java::lang::String /**/ {
		/* final */ let buf: StringBuilder = StringBuilder::new();
		let i: i32 = index_ref[0];
		/* final */ let length: i32 = pattern.length();
		let c: char = pattern.charAt(i);
		/* final */ let c1: char = c;
		if CharUtils::is_ascii_alpha(c1) {
			// Scan a run of the same character, which indicates a time
			// pattern.
			buf.append(c);
			while i + 1 < length {
				/* final */ let peek: char = pattern.charAt(i + 1);
				if peek != c {
					break;
				}
				buf.append(c);
				i += 1;
			}
		} else {
			// This will identify token as text.
			buf.append('\'');
			let in_literal: bool = false;
			while i < length {
				{
					c = pattern.charAt(i);
					if c == '\'' {
						if i + 1 < length && pattern.charAt(i + 1) == '\'' {
							// '' is treated as escaped '
							i += 1;
							buf.append(c);
						} else {
							in_literal = !in_literal;
						}
					} else {
						/* final */ let c2: char = c;
						if !in_literal && CharUtils::is_ascii_alpha(c2) {
							i -= 1;
							break;
						}
						buf.append(c);
					}
				}
				i += 1;
			 }
	
		}
		index_ref[0] = i;
		return buf.toString();
	}

	fn read_object(&self, in: &/* Java */ java::io::ObjectInputStream /**/) /* thrown(java.io.IOException | java.lang.ClassNotFoundException) */ {
		in.defaultReadObject();
		self.init();
	}

	fn select_number_rule(&self, field: i32, padding: i32) -> org::apache::commons::lang3::time::fast_date_printer::NumberRule {
		match padding {
			1 =>  {
				return UnpaddedNumberField::new(field);
			}
			2 =>  {
				return TwoDigitNumberField::new(field);
			}
			_ =>  {
				return PaddedNumberField::new(field, padding);
			}
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "FastDatePrinter[" + self.pattern + "," + self.locale + "," + self.time_zone.getID() + "]";
	}
}

impl org::apache::commons::lang3::time::date_printer::DatePrinter for FastDatePrinter {}

impl /* Java */ java::io::Serializable /**/ for FastDatePrinter {}

struct CharacterLiteral {
	value: u16,
}

impl CharacterLiteral {
	fn new(value: u16) -> org::apache::commons::lang3::time::fast_date_printer::CharacterLiteral {
		self.value = value;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		buffer.append(self.value);
	}

	pub fn estimate_length(&self) -> i32 {
		return 1;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for CharacterLiteral {}

struct DayInWeekField {
	rule: org::apache::commons::lang3::time::fast_date_printer::NumberRule,
}

impl DayInWeekField {
	fn new(rule: &org::apache::commons::lang3::time::fast_date_printer::NumberRule) -> org::apache::commons::lang3::time::fast_date_printer::DayInWeekField {
		self.rule = rule;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		/* final */ let value: i32 = calendar.get(Calendar::DAY_OF_WEEK);
		self.rule.append_to(buffer,  if value == Calendar::SUNDAY { 7 } else { value - 1 });
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		self.rule.append_to(buffer, value);
	}

	pub fn estimate_length(&self) -> i32 {
		return self.rule.estimate_length();
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for DayInWeekField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for DayInWeekField {}

struct Iso8601_Rule {
	length: i32,
}

impl Iso8601_Rule {
	static ISO8601_HOURS: org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule = Iso8601_Rule::new(3);

	static ISO8601_HOURS_MINUTES: org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule = Iso8601_Rule::new(5);

	static ISO8601_HOURS_COLON_MINUTES: org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule = Iso8601_Rule::new(6);

	fn get_rule(&self, token_len: i32) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule {
		match token_len {
			1 =>  {
				return self.ISO8601_HOURS;
			}
			2 =>  {
				return self.ISO8601_HOURS_MINUTES;
			}
			3 =>  {
				return self.ISO8601_HOURS_COLON_MINUTES;
			}
			_ =>  {
				return Err(IllegalArgumentException::new("invalid number of X"));
			}
		}
	}

	fn new(length: i32) -> org::apache::commons::lang3::time::fast_date_printer::Iso8601_Rule {
		self.length = length;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		let offset: i32 = calendar.get(Calendar::ZONE_OFFSET) + calendar.get(Calendar::DST_OFFSET);
		if offset == 0 {
			buffer.append("Z");
			return;
		}
		if offset < 0 {
			buffer.append('-');
			offset = -offset;
		} else {
			buffer.append('+');
		}
		/* final */ let hours: i32 = offset / (60 * 60 * 1000);
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, hours)?;
		if self.length < 5 {
			return;
		}
		if self.length == 6 {
			buffer.append(':');
		}
		/* final */ let minutes: i32 = offset / (60 * 1000) - 60 * hours;
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, minutes)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return self.length;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for Iso8601_Rule {}

trait NumberRule;

struct PaddedNumberField {
	field: i32,
	size: i32,
}

impl PaddedNumberField {
	fn new(field: i32, size: i32) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_printer::PaddedNumberField {
		if size < 3 {
			// Should use UnpaddedNumberField or TwoDigitNumberField.
			return Err(IllegalArgumentException::new());
		}
		self.field = field;
		self.size = size;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, &calendar.get(self.field));
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		// Checkstyle complains about redundant qualifier
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_full_digits(buffer, value, self.size)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return self.size;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for PaddedNumberField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for PaddedNumberField {}

trait Rule;

struct StringLiteral {
	value: /* Java */ java::lang::String /**/,
}

impl StringLiteral {
	fn new(value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::time::fast_date_printer::StringLiteral {
		self.value = value;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		buffer.append(self.value);
	}

	pub fn estimate_length(&self) -> i32 {
		return self.value.length();
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for StringLiteral {}

struct TextField {
	field: i32,
	values: &[/* Java */ java::lang::String /**/],
}

impl TextField {
	fn new(field: i32, values: &&[/* Java */ java::lang::String /**/]) -> org::apache::commons::lang3::time::fast_date_printer::TextField {
		self.field = field;
		self.values = values;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		buffer.append(self.values[calendar.get(self.field)]);
	}

	pub fn estimate_length(&self) -> i32 {
		let max: i32 = 0;
		 {
			let i: i32 = self.values.length;
			while i -= 1 >= 0{
				/* final */ let len: i32 = self.values[i].length();
				if len > max {
					max = len;
				}
			}
		 }
	
		return max;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TextField {}

struct TimeZoneDisplayKey {
	time_zone: /* Java */ java::util::TimeZone /**/,
	style: i32,
	locale: /* Java */ java::util::Locale /**/,
}

impl TimeZoneDisplayKey {
	fn new(time_zone: &/* Java */ java::util::TimeZone /**/, daylight: bool, style: i32, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_printer::TimeZoneDisplayKey {
		self.timeZone = time_zone;
		if daylight {
			self.style = style | 0x80000000;
		} else {
			self.style = style;
		}
		self.locale = LocaleUtils::to_locale(locale);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if obj instanceof TimeZoneDisplayKey {
			/* final */ let other: TimeZoneDisplayKey = obj as TimeZoneDisplayKey;
			return self.time_zone.equals(other.timeZone) && self.style == other.style && self.locale.equals(other.locale);
		}
		return false;
	}

	pub fn hash_code(&self) -> i32 {
		return (self.style * 31 + self.locale.hashCode()) * 31 + self.time_zone.hashCode();
	}
}

struct TimeZoneNameRule {
	locale: /* Java */ java::util::Locale /**/,
	style: i32,
	standard: /* Java */ java::lang::String /**/,
	daylight: /* Java */ java::lang::String /**/,
}

impl TimeZoneNameRule {
	fn new(time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/, style: i32) -> org::apache::commons::lang3::time::fast_date_printer::TimeZoneNameRule {
		self.locale = LocaleUtils::to_locale(locale);
		self.style = style;
		self.standard = org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::get_time_zone_display(time_zone, false, style, locale);
		self.daylight = org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::get_time_zone_display(time_zone, true, style, locale);
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		/* final */ let zone: TimeZone = calendar.getTimeZone();
		/* final */ let daylight: bool = calendar.get(Calendar::DST_OFFSET) != 0;
		buffer.append(&org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::get_time_zone_display(zone, daylight, self.style, self.locale));
	}

	pub fn estimate_length(&self) -> i32 {
		// constructor
		return Math::max(&self.standard.length(), &self.daylight.length());
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TimeZoneNameRule {}

struct TimeZoneNumberRule {
	colon: bool,
}

impl TimeZoneNumberRule {
	static INSTANCE_COLON: org::apache::commons::lang3::time::fast_date_printer::TimeZoneNumberRule = TimeZoneNumberRule::new(true);

	static INSTANCE_NO_COLON: org::apache::commons::lang3::time::fast_date_printer::TimeZoneNumberRule = TimeZoneNumberRule::new(false);

	fn new(colon: bool) -> org::apache::commons::lang3::time::fast_date_printer::TimeZoneNumberRule {
		self.colon = colon;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		let offset: i32 = calendar.get(Calendar::ZONE_OFFSET) + calendar.get(Calendar::DST_OFFSET);
		if offset < 0 {
			buffer.append('-');
			offset = -offset;
		} else {
			buffer.append('+');
		}
		/* final */ let hours: i32 = offset / (60 * 60 * 1000);
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, hours)?;
		if self.colon {
			buffer.append(':');
		}
		/* final */ let minutes: i32 = offset / (60 * 1000) - 60 * hours;
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, minutes)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return 5;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TimeZoneNumberRule {}

struct TwelveHourField {
	rule: org::apache::commons::lang3::time::fast_date_printer::NumberRule,
}

impl TwelveHourField {
	fn new(rule: &org::apache::commons::lang3::time::fast_date_printer::NumberRule) -> org::apache::commons::lang3::time::fast_date_printer::TwelveHourField {
		self.rule = rule;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		let value: i32 = calendar.get(Calendar::HOUR);
		if value == 0 {
			value = calendar.getLeastMaximum(Calendar::HOUR) + 1;
		}
		self.rule.append_to(buffer, value)?;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		self.rule.append_to(buffer, value)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return self.rule.estimate_length();
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for TwelveHourField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TwelveHourField {}

struct TwentyFourHourField {
	rule: org::apache::commons::lang3::time::fast_date_printer::NumberRule,
}

impl TwentyFourHourField {
	fn new(rule: &org::apache::commons::lang3::time::fast_date_printer::NumberRule) -> org::apache::commons::lang3::time::fast_date_printer::TwentyFourHourField {
		self.rule = rule;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		let value: i32 = calendar.get(Calendar::HOUR_OF_DAY);
		if value == 0 {
			value = calendar.getMaximum(Calendar::HOUR_OF_DAY) + 1;
		}
		self.rule.append_to(buffer, value)?;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		self.rule.append_to(buffer, value)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return self.rule.estimate_length();
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for TwentyFourHourField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TwentyFourHourField {}

struct TwoDigitMonthField;

impl TwoDigitMonthField {
	static INSTANCE: org::apache::commons::lang3::time::fast_date_printer::TwoDigitMonthField = TwoDigitMonthField::new();

	fn new() -> org::apache::commons::lang3::time::fast_date_printer::TwoDigitMonthField {
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, calendar.get(Calendar::MONTH) + 1);
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, value)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return 2;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for TwoDigitMonthField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TwoDigitMonthField {}

struct TwoDigitNumberField {
	field: i32,
}

impl TwoDigitNumberField {
	fn new(field: i32) -> org::apache::commons::lang3::time::fast_date_printer::TwoDigitNumberField {
		self.field = field;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, &calendar.get(self.field));
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		if value < 100 {
			org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, value)?;
		} else {
			org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_full_digits(buffer, value, 2)?;
		}
	}

	pub fn estimate_length(&self) -> i32 {
		return 2;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for TwoDigitNumberField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TwoDigitNumberField {}

struct TwoDigitYearField;

impl TwoDigitYearField {
	static INSTANCE: org::apache::commons::lang3::time::fast_date_printer::TwoDigitYearField = TwoDigitYearField::new();

	fn new() -> org::apache::commons::lang3::time::fast_date_printer::TwoDigitYearField {
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, calendar.get(Calendar::YEAR) % 100);
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, value % 100)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return 2;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for TwoDigitYearField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for TwoDigitYearField {}

struct UnpaddedMonthField;

impl UnpaddedMonthField {
	static INSTANCE: org::apache::commons::lang3::time::fast_date_printer::UnpaddedMonthField = UnpaddedMonthField::new();

	fn new() -> org::apache::commons::lang3::time::fast_date_printer::UnpaddedMonthField {
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, calendar.get(Calendar::MONTH) + 1);
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		if value < 10 {
			buffer.append((value + '0') as char);
		} else {
			org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, value)?;
		}
	}

	pub fn estimate_length(&self) -> i32 {
		return 2;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for UnpaddedMonthField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for UnpaddedMonthField {}

struct UnpaddedNumberField {
	field: i32,
}

impl UnpaddedNumberField {
	fn new(field: i32) -> org::apache::commons::lang3::time::fast_date_printer::UnpaddedNumberField {
		self.field = field;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.append_to(buffer, &calendar.get(self.field));
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		if value < 10 {
			buffer.append((value + '0') as char);
		} else if value < 100 {
			org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_digits(buffer, value)?;
		} else {
			org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter::append_full_digits(buffer, value, 1)?;
		}
	}

	pub fn estimate_length(&self) -> i32 {
		return 4;
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for UnpaddedNumberField {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for UnpaddedNumberField {}

struct WeekYear {
	rule: org::apache::commons::lang3::time::fast_date_printer::NumberRule,
}

impl WeekYear {
	fn new(rule: &org::apache::commons::lang3::time::fast_date_printer::NumberRule) -> org::apache::commons::lang3::time::fast_date_printer::WeekYear {
		self.rule = rule;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.io.IOException) */ {
		self.rule.append_to(buffer, &calendar.getWeekYear())?;
	}

	pub fn append_to(&self, buffer: &/* Java */ java::lang::Appendable /**/, value: i32) /* thrown(java.io.IOException) */ {
		self.rule.append_to(buffer, value)?;
	}

	pub fn estimate_length(&self) -> i32 {
		return self.rule.estimate_length();
	}
}

impl org::apache::commons::lang3::time::fast_date_printer::NumberRule for WeekYear {}

impl org::apache::commons::lang3::time::fast_date_printer::Rule for WeekYear {}