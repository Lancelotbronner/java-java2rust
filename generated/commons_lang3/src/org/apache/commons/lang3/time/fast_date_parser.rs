use java::io::IOException;
use java::io::ObjectInputStream;
use java::io::Serializable;
use java::text::DateFormatSymbols;
use java::text::ParseException;
use java::text::ParsePosition;
use java::text::SimpleDateFormat;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::Calendar;
use java::util::Comparator;
use java::util::Date;
use java::util::HashMap;
use java::util::List;
use java::util::ListIterator;
use java::util::Locale;
use java::util::Map;
use java::util::Objects;
use java::util::Set;
use java::util::TimeZone;
use java::util::TreeMap;
use java::util::TreeSet;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ConcurrentMap;
use java::util::regex::Matcher;
use java::util::regex::Pattern;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::ArraySorter;
use crate::org::apache::commons::lang3::CharUtils;
use crate::org::apache::commons::lang3::LocaleUtils;

pub struct FastDateParser {
	pattern: /* Java */ java::lang::String /**/,
	time_zone: /* Java */ java::util::TimeZone /**/,
	locale: /* Java */ java::util::Locale /**/,
	century: i32,
	start_year: i32,
	patterns: /* Java */ java::util::List /**/,
}

impl FastDateParser {
	static serialVersionUID: i64 = 3;

	static JAPANESE_IMPERIAL: /* Java */ java::util::Locale /**/ = Locale::new("ja", "JP", "JP");

	static LONGER_FIRST_LOWERCASE: /* Java */ java::util::Comparator /**/ = Comparator::reverseOrder();

	static CACHES: &[/* Java */ java::util::concurrent::ConcurrentMap /**/] = : [Option<ConcurrentMap>; Calendar::FIELD_COUNT] = [None; Calendar::FIELD_COUNT];

	static ABBREVIATED_YEAR_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::YEAR) {
		/* TraditionalJavadocComment/**
	 * {@inheritDoc}
	 */
	
		*/ fn modify(&self, /* final */ parser: &FastDateParser, /* final */ i_value: i32) -> i32 {
			return  if i_value < 100 { parser.adjust_year(i_value) } else { i_value };
		}
	
	};

	static NUMBER_MONTH_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::MONTH) {
		fn modify(&self, /* final */ parser: &FastDateParser, /* final */ i_value: i32) -> i32 {
			return i_value - 1;
		}
	
	};

	static LITERAL_YEAR_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::YEAR);

	static WEEK_OF_YEAR_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::WEEK_OF_YEAR);

	static WEEK_OF_MONTH_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::WEEK_OF_MONTH);

	static DAY_OF_YEAR_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::DAY_OF_YEAR);

	static DAY_OF_MONTH_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::DAY_OF_MONTH);

	static DAY_OF_WEEK_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::DAY_OF_WEEK) {
		fn modify(&self, /* final */ parser: &FastDateParser, /* final */ i_value: i32) -> i32 {
			return  if i_value == 7 { Calendar::SUNDAY } else { i_value + 1 };
		}
	
	};

	static DAY_OF_WEEK_IN_MONTH_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::DAY_OF_WEEK_IN_MONTH);

	static HOUR_OF_DAY_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::HOUR_OF_DAY);

	static HOUR24_OF_DAY_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::HOUR_OF_DAY) {
		fn modify(&self, /* final */ parser: &FastDateParser, /* final */ i_value: i32) -> i32 {
			return  if i_value == 24 { 0 } else { i_value };
		}
	
	};

	static HOUR12_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::HOUR) {
		fn modify(&self, /* final */ parser: &FastDateParser, /* final */ i_value: i32) -> i32 {
			return  if i_value == 12 { 0 } else { i_value };
		}
	
	};

	static HOUR_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::HOUR);

	static MINUTE_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::MINUTE);

	static SECOND_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::SECOND);

	static MILLISECOND_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = NumberStrategy::new(Calendar::MILLISECOND);

	fn append_display_names(&self, calendar: &/* Java */ java::util::Calendar /**/, locale: &/* Java */ java::util::Locale /**/, field: i32, regex: &/* Java */ java::lang::StringBuilder /**/) -> /* Java */ java::util::Map /**/ {
		Objects::requireNonNull(calendar, "calendar");
		/* final */ let values: Map<String, Integer> = HashMap<>::new();
		/* final */ let actual_locale: Locale = LocaleUtils::to_locale(locale);
		/* final */ let display_names: Map<String, Integer> = calendar.getDisplayNames(field, Calendar::ALL_STYLES, actual_locale);
		/* final */ let sorted: TreeSet<String> = TreeSet<>::new(self.LONGER_FIRST_LOWERCASE);
		display_names.forEach(|(k, v)|{
			/* final */ let key_lc: String = k.toLowerCase(actual_locale);
			if sorted.add(key_lc) {
				values.put(key_lc, v);
			}
		});
		sorted.forEach(|symbol|.simpleQuote(regex, symbol).append('|'));
		return values;
	}

	fn clear(&self) {
		Stream::of(self.CACHES).filter(Objects::nonNull).forEach(ConcurrentMap::clear);
	}

	fn get_cache(&mut self, mut field: i32) -> /* Java */ java::util::concurrent::ConcurrentMap /**/ {
		synchronized (self.CACHES) {
			if self.CACHES[field] == null {
				self.CACHES[field] = ConcurrentHashMap<>::new(3);
			}
			return self.CACHES[field];
		}
	}

	fn simple_quote(&self, sb: &/* Java */ java::lang::StringBuilder /**/, value: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::StringBuilder /**/ {
		 {
			let i: i32 = 0;
			while i < value.length() {
				{
					/* final */ let c: char = value.charAt(i);
					match c {
						'\\' =>  {
						}
						'^' =>  {
						}
						'$' =>  {
						}
						'.' =>  {
						}
						'|' =>  {
						}
						'?' =>  {
						}
						'*' =>  {
						}
						'+' =>  {
						}
						'(' =>  {
						}
						')' =>  {
						}
						'[' =>  {
						}
						'{' => sb.append('\\'),
						// falls-through
						_ => sb.append(c),
					}
				}
				i += 1;
			 }
		 }
	
		if sb.charAt(sb.length() - 1) == '.' {
			// trailing '.' is optional
			sb.append('?');
		}
		return sb;
	}

	fn new(pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_parser::FastDateParser {
		this(pattern, time_zone, locale, null);
	}

	fn new(pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/, century_start: &/* Java */ java::util::Date /**/) -> org::apache::commons::lang3::time::fast_date_parser::FastDateParser {
		self.pattern = Objects::requireNonNull(pattern, "pattern");
		self.timeZone = Objects::requireNonNull(time_zone, "timeZone");
		self.locale = LocaleUtils::to_locale(locale);
		/* final */ let defining_calendar: Calendar = Calendar::getInstance(time_zone, self.locale);
		/* final */ let century_start_year: i32;
		if century_start != null {
			defining_calendar.setTime(century_start);
			century_start_year = defining_calendar.get(Calendar::YEAR);
		} else if self.locale.equals(self.JAPANESE_IMPERIAL) {
			century_start_year = 0;
		} else {
			// from 80 years ago to 20 years from now
			defining_calendar.setTime(Date::new());
			century_start_year = defining_calendar.get(Calendar::YEAR) - 80;
		}
		self.century = century_start_year / 100 * 100;
		self.start_year = century_start_year - self.century;
		self.init(defining_calendar);
	}

	fn adjust_year(&self, two_digit_year: i32) -> i32 {
		/* final */ let trial: i32 = self.century + two_digit_year;
		return  if two_digit_year >= self.start_year { trial } else { trial + 100 };
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if !(obj instanceof FastDateParser) {
			return false;
		}
		/* final */ let other: FastDateParser = obj as FastDateParser;
		return self.pattern.equals(other.pattern) && self.time_zone.equals(other.timeZone) && self.locale.equals(other.locale);
	}

	pub fn get_locale(&self) -> /* Java */ java::util::Locale /**/ {
		return self.locale;
	}

	fn get_locale_specific_strategy(&self, field: i32, defining_calendar: &/* Java */ java::util::Calendar /**/) -> org::apache::commons::lang3::time::fast_date_parser::Strategy {
		/* final */ let cache: ConcurrentMap<Locale, Strategy> = org::apache::commons::lang3::time::fast_date_parser::FastDateParser::get_cache(field);
		return cache.computeIfAbsent(self.locale, |k| if field == Calendar::ZONE_OFFSET { TimeZoneStrategy::new(self.locale) } else { CaseInsensitiveTextStrategy::new(field, defining_calendar, self.locale) });
	}

	pub fn get_pattern(&self) -> /* Java */ java::lang::String /**/ {
		return self.pattern;
	}

	fn get_strategy(&self, f: u16, width: i32, defining_calendar: &/* Java */ java::util::Calendar /**/) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_parser::Strategy {
		match f {
			'D' =>  {
				return self.DAY_OF_YEAR_STRATEGY;
			}
			'E' =>  {
				return self.get_locale_specific_strategy(Calendar::DAY_OF_WEEK, defining_calendar);
			}
			'F' =>  {
				return self.DAY_OF_WEEK_IN_MONTH_STRATEGY;
			}
			'G' =>  {
				return self.get_locale_specific_strategy(Calendar::ERA, defining_calendar);
			}
			// Hour in day (0-23)
			'H' =>  {
				return self.HOUR_OF_DAY_STRATEGY;
			}
			// Hour in am/pm (0-11)
			'K' =>  {
				return self.HOUR_STRATEGY;
			}
			'M' =>  {
			}
			'L' =>  {
				return  if width >= 3 { self.get_locale_specific_strategy(Calendar::MONTH, defining_calendar) } else { self.NUMBER_MONTH_STRATEGY };
			}
			'S' =>  {
				return self.MILLISECOND_STRATEGY;
			}
			'W' =>  {
				return self.WEEK_OF_MONTH_STRATEGY;
			}
			'a' =>  {
				return self.get_locale_specific_strategy(Calendar::AM_PM, defining_calendar);
			}
			'd' =>  {
				return self.DAY_OF_MONTH_STRATEGY;
			}
			// Hour in am/pm (1-12), i.e. midday/midnight is 12, not 0
			'h' =>  {
				return self.HOUR12_STRATEGY;
			}
			// Hour in day (1-24), i.e. midnight is 24, not 0
			'k' =>  {
				return self.HOUR24_OF_DAY_STRATEGY;
			}
			'm' =>  {
				return self.MINUTE_STRATEGY;
			}
			's' =>  {
				return self.SECOND_STRATEGY;
			}
			'u' =>  {
				return self.DAY_OF_WEEK_STRATEGY;
			}
			'w' =>  {
				return self.WEEK_OF_YEAR_STRATEGY;
			}
			'y' =>  {
			}
			'Y' =>  {
				return  if width > 2 { self.LITERAL_YEAR_STRATEGY } else { self.ABBREVIATED_YEAR_STRATEGY };
			}
			'X' =>  {
				return ISO8601TimeZoneStrategy::get_strategy(width)?;
			}
			'Z' =>  {
				if width == 2 {
					return ISO8601TimeZoneStrategy::ISO_8601_3_STRATEGY;
				}
			}
			// falls-through
			'z' =>  {
				return self.get_locale_specific_strategy(Calendar::ZONE_OFFSET, defining_calendar);
			}
			_ =>  {
				return Err(IllegalArgumentException::new("Format '" + f + "' not supported"));
			}
		}
	}

	pub fn get_time_zone(&self) -> /* Java */ java::util::TimeZone /**/ {
		return self.time_zone;
	}

	pub fn hash_code(&self) -> i32 {
		return self.pattern.hashCode() + 13 * (self.time_zone.hashCode() + 13 * self.locale.hashCode());
	}

	fn init(&mut self, defining_calendar: &/* Java */ java::util::Calendar /**/) {
		self.patterns = ArrayList<>::new();
		/* final */ let strategy_parser: StrategyParser = StrategyParser::new(defining_calendar);
		loop {
			/* final */ let field: StrategyAndWidth = strategy_parser.get_next_strategy();
			if field == null {
				break;
			}
			self.patterns.add(field);
		}
	
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		/* final */ let pp: ParsePosition = ParsePosition::new(0);
		/* final */ let date: Date = self.parse(source, pp);
		if date == null {
			// Add a note regarding supported date range
			if self.locale.equals(self.JAPANESE_IMPERIAL) {
				return Err(ParseException::new("(The " + self.locale + " locale does not support dates before 1868 AD)\nUnparseable date: \"" + source, &pp.getErrorIndex()));
			}
			return Err(ParseException::new("Unparseable date: " + source, &pp.getErrorIndex()));
		}
		return date;
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::util::Date /**/ {
		// timing tests indicate getting new instance is 19% faster than cloning
		/* final */ let cal: Calendar = Calendar::getInstance(self.time_zone, self.locale);
		cal.clear();
		return  if self.parse(source, pos, cal) { cal.getTime() } else { null };
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, calendar: &/* Java */ java::util::Calendar /**/) -> bool {
		/* final */ let lt: ListIterator<StrategyAndWidth> = self.patterns.listIterator();
		while lt.hasNext() {
			/* final */ let strategy_and_width: StrategyAndWidth = lt.next();
			/* final */ let max_width: i32 = strategy_and_width.get_max_width(lt);
			if !strategy_and_width.strategy.parse(self, calendar, source, pos, max_width) {
				return false;
			}
		}
		return true;
	}

	pub fn parse_object(&self, source: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::lang::Object /**/ {
		return self.parse(source)?;
	}

	pub fn parse_object(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::lang::Object /**/ {
		return self.parse(source, pos);
	}

	fn read_object(&self, in: &/* Java */ java::io::ObjectInputStream /**/) /* thrown(java.lang.ClassNotFoundException | java.io.IOException) */ {
		in.defaultReadObject();
		/* final */ let defining_calendar: Calendar = Calendar::getInstance(self.time_zone, self.locale);
		self.init(defining_calendar);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "FastDateParser[" + self.pattern + ", " + self.locale + ", " + self.time_zone.getID() + "]";
	}

	pub fn to_string_all(&self) -> /* Java */ java::lang::String /**/ {
		return "FastDateParser [pattern=" + self.pattern + ", timeZone=" + self.time_zone + ", locale=" + self.locale + ", century=" + self.century + ", startYear=" + self.start_year + ", patterns=" + self.patterns + "]";
	}
}

impl org::apache::commons::lang3::time::date_parser::DateParser for FastDateParser {}

impl /* Java */ java::io::Serializable /**/ for FastDateParser {}

struct CaseInsensitiveTextStrategy {
	field: i32,
	locale: /* Java */ java::util::Locale /**/,
	l_key_values: /* Java */ java::util::Map /**/,
}

impl CaseInsensitiveTextStrategy {
	fn new(field: i32, defining_calendar: &/* Java */ java::util::Calendar /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_parser::CaseInsensitiveTextStrategy {
		self.field = field;
		self.locale = LocaleUtils::to_locale(locale);
		/* final */ let regex: StringBuilder = StringBuilder::new();
		regex.append("((?iu)");
		self.l_key_values = org::apache::commons::lang3::time::fast_date_parser::FastDateParser::append_display_names(defining_calendar, locale, field, regex);
		regex.setLength(regex.length() - 1);
		regex.append(")");
		self.create_pattern(regex);
	}

	fn set_calendar(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, value: &/* Java */ java::lang::String /**/) {
		/* final */ let lower_case: String = value.toLowerCase(self.locale);
		let i_val: Integer = self.l_key_values.get(lower_case);
		if i_val == null {
			// match missing the optional trailing period
			i_val = self.l_key_values.get(lower_case + '.');
		}
		// LANG-1669: Mimic fix done in OpenJDK 17 to resolve issue with parsing newly supported day periods added in OpenJDK 16
		if Calendar::AM_PM != self.field || i_val <= 1 {
			calendar.set(self.field, &i_val.intValue());
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "CaseInsensitiveTextStrategy [field=" + self.field + ", locale=" + self.locale + ", lKeyValues=" + self.l_key_values + ", pattern=" +  + "]";
	}
}

struct CopyQuotedStrategy {
	format_field: /* Java */ java::lang::String /**/,
}

impl CopyQuotedStrategy {
	fn new(format_field: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::time::fast_date_parser::CopyQuotedStrategy {
		self.formatField = format_field;
	}

	fn is_number(&self) -> bool {
		return false;
	}

	fn parse(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, max_width: i32) -> bool {
		 {
			let idx: i32 = 0;
			while idx < self.format_field.length() {
				{
					/* final */ let s_idx: i32 = idx + pos.getIndex();
					if s_idx == source.length() {
						pos.setErrorIndex(s_idx);
						return false;
					}
					if self.format_field.charAt(idx) != source.charAt(s_idx) {
						pos.setErrorIndex(s_idx);
						return false;
					}
				}
				idx += 1;
			 }
		 }
	
		pos.setIndex(self.format_field.length() + pos.getIndex());
		return true;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "CopyQuotedStrategy [formatField=" + self.format_field + "]";
	}
}

struct ISO8601TimeZoneStrategy;

impl ISO8601TimeZoneStrategy {
	static ISO_8601_1_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = ISO8601TimeZoneStrategy::new("(Z|(?:[+-]\\d{2}))");

	static ISO_8601_2_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = ISO8601TimeZoneStrategy::new("(Z|(?:[+-]\\d{2}\\d{2}))");

	static ISO_8601_3_STRATEGY: org::apache::commons::lang3::time::fast_date_parser::Strategy = ISO8601TimeZoneStrategy::new("(Z|(?:[+-]\\d{2}(?::)\\d{2}))");

	fn get_strategy(&self, token_len: i32) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_parser::Strategy {
		match token_len {
			1 =>  {
				return self.ISO_8601_1_STRATEGY;
			}
			2 =>  {
				return self.ISO_8601_2_STRATEGY;
			}
			3 =>  {
				return self.ISO_8601_3_STRATEGY;
			}
			_ =>  {
				return Err(IllegalArgumentException::new("invalid number of X"));
			}
		}
	}

	fn new(pattern: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::time::fast_date_parser::ISO8601TimeZoneStrategy {
		self.create_pattern(pattern);
	}

	fn set_calendar(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, value: &/* Java */ java::lang::String /**/) {
		calendar.setTimeZone(&FastTimeZone::get_gmt_time_zone(value));
	}
}

struct NumberStrategy {
	field: i32,
}

impl NumberStrategy {
	fn new(field: i32) -> org::apache::commons::lang3::time::fast_date_parser::NumberStrategy {
		self.field = field;
	}

	fn is_number(&self) -> bool {
		return true;
	}

	fn modify(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, i_value: i32) -> i32 {
		return i_value;
	}

	fn parse(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, max_width: i32) -> bool {
		let idx: i32 = pos.getIndex();
		let last: i32 = source.length();
		if max_width == 0 {
			// if no maxWidth, strip leading white space
			while idx < last {
				{
					/* final */ let c: char = source.charAt(idx);
					if !Character::isWhitespace(c) {
						break;
					}
				}
				idx += 1;
			 }
	
			pos.setIndex(idx);
		} else {
			/* final */ let end: i32 = idx + max_width;
			if last > end {
				last = end;
			}
		}
		while idx < last {
			{
				/* final */ let c: char = source.charAt(idx);
				if !Character::isDigit(c) {
					break;
				}
			}
			idx += 1;
		 }
	
		if pos.getIndex() == idx {
			pos.setErrorIndex(idx);
			return false;
		}
		/* final */ let value: i32 = Integer::parseInt(&source.substring(&pos.getIndex(), idx));
		pos.setIndex(idx);
		calendar.set(self.field, &self.modify(parser, value));
		return true;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "NumberStrategy [field=" + self.field + "]";
	}
}

struct PatternStrategy {
	pattern: /* Java */ java::util::regex::Pattern /**/,
}

impl PatternStrategy {
	fn create_pattern(&mut self, regex: &/* Java */ java::lang::String /**/) {
		self.pattern = Pattern::compile(regex);
	}

	fn create_pattern(&self, regex: &/* Java */ java::lang::StringBuilder /**/) {
		self.create_pattern(&regex.toString());
	}

	fn is_number(&self) -> bool {
		return false;
	}

	fn parse(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, max_width: i32) -> bool {
		/* final */ let matcher: Matcher = self.pattern.matcher(&source.substring(&pos.getIndex()));
		if !matcher.lookingAt() {
			pos.setErrorIndex(&pos.getIndex());
			return false;
		}
		pos.setIndex(pos.getIndex() + matcher.end(1));
		self.set_calendar(parser, calendar, &matcher.group(1));
		return true;
	}

	fn set_calendar(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, value: &/* Java */ java::lang::String /**/) ;

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.getClass().getSimpleName() + " [pattern=" + self.pattern + "]";
	}
}

struct Strategy;

impl Strategy {
	fn is_number(&self) -> bool {
		return false;
	}

	fn parse(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, max_width: i32) -> bool ;
}

struct StrategyAndWidth {
	strategy: org::apache::commons::lang3::time::fast_date_parser::Strategy,
	width: i32,
}

impl StrategyAndWidth {
	fn new(strategy: &org::apache::commons::lang3::time::fast_date_parser::Strategy, width: i32) -> org::apache::commons::lang3::time::fast_date_parser::StrategyAndWidth {
		self.strategy = Objects::requireNonNull(strategy, "strategy");
		self.width = width;
	}

	fn get_max_width(&self, lt: &/* Java */ java::util::ListIterator /**/) -> i32 {
		if !self.strategy.is_number() || !lt.hasNext() {
			return 0;
		}
		/* final */ let next_strategy: Strategy = lt.next().strategy;
		lt.previous();
		return  if next_strategy.is_number() { self.width } else { 0 };
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "StrategyAndWidth [strategy=" + self.strategy + ", width=" + self.width + "]";
	}
}

struct StrategyParser {
	defining_calendar: /* Java */ java::util::Calendar /**/,
	current_idx: i32,
}

impl StrategyParser {
	fn new(defining_calendar: &/* Java */ java::util::Calendar /**/) -> org::apache::commons::lang3::time::fast_date_parser::StrategyParser {
		self.definingCalendar = Objects::requireNonNull(defining_calendar, "definingCalendar");
	}

	fn get_next_strategy(&self) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_parser::StrategyAndWidth {
		if self.current_idx >= .length() {
			return null;
		}
		/* final */ let c: char = .charAt(self.current_idx);
		if CharUtils::is_ascii_alpha(c) {
			return self.letter_pattern(c);
		}
		return self.literal()?;
	}

	fn letter_pattern(&self, c: u16) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_parser::StrategyAndWidth {
		/* final */ let begin: i32 = self.current_idx;
		while self.current_idx += 1 < .length() {
			if .charAt(self.current_idx) != c {
				break;
			}
		}
		/* final */ let width: i32 = self.current_idx - begin;
		return StrategyAndWidth::new(&self.get_strategy(c, width, self.defining_calendar)?, width);
	}

	fn literal(&self) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::fast_date_parser::StrategyAndWidth {
		let active_quote: bool = false;
		/* final */ let sb: StringBuilder = StringBuilder::new();
		while self.current_idx < .length() {
			/* final */ let c: char = .charAt(self.current_idx);
			if !active_quote && CharUtils::is_ascii_alpha(c) {
				break;
			}
			if c == '\'' && (self.current_idx += 1 == .length() || .charAt(self.current_idx) != '\'') {
				active_quote = !active_quote;
				continue;
			}
			self.current_idx += 1;
			sb.append(c);
		}
		if active_quote {
			return Err(IllegalArgumentException::new("Unterminated quote"));
		}
		/* final */ let format_field: String = sb.toString();
		return StrategyAndWidth::new(CopyQuotedStrategy::new(format_field), &format_field.length());
	}
}

struct TimeZoneStrategy {
	locale: /* Java */ java::util::Locale /**/,
	tz_names: /* Java */ java::util::Map /**/ = TreeMap<>::new(String::CASE_INSENSITIVE_ORDER),
}

impl TimeZoneStrategy {
	static RFC_822_TIME_ZONE: /* Java */ java::lang::String /**/ = "[+-]\\d{4}";

	static GMT_OPTION: /* Java */ java::lang::String /**/ = TimeZones::GMT_ID + "[+-]\\d{1,2}:\\d{2}";

	static ID: i32 = 0;

	fn skip_time_zone(&self, tz_id: &/* Java */ java::lang::String /**/) -> bool {
		return tz_id.equalsIgnoreCase(TimeZones::GMT_ID);
	}

	fn new(locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_parser::TimeZoneStrategy {
		self.locale = LocaleUtils::to_locale(locale);
		/* final */ let sb: StringBuilder = StringBuilder::new();
		sb.append("((?iu)" + self.RFC_822_TIME_ZONE + "|" + self.GMT_OPTION);
		/* final */ let sorted: Set<String> = TreeSet<>::new();
		// Order is undefined.
		// TODO Use of getZoneStrings() is discouraged per its Javadoc.
		/* final */ let zones: Vec<Vec<String>> = DateFormatSymbols::getInstance(locale).getZoneStrings();
		for /* final */ zone_names in zones {
			// offset 0 is the time zone ID and is not localized
			/* final */ let tz_id: String = zone_names[self.ID];
			if org::apache::commons::lang3::time::fast_date_parser::TimeZoneStrategy::skip_time_zone(tz_id) {
				continue;
			}
			/* final */ let tz: TimeZone = TimeZones::get_time_zone(tz_id);
			// offset 1 is long standard name
			// offset 2 is short standard name
			/* final */ let standard: TzInfo = TzInfo::new(tz, false);
			let tz_info: TzInfo = standard;
			 {
				let i: i32 = 1;
				while i < zone_names.length {
					{
						match i {
							// offset 3 is long daylight savings (or summertime) name
							3 =>  {
								// offset 4 is the short summertime name
								tz_info = TzInfo::new(tz, true);
								break;
							}
							// offset 5 starts additional names, probably standard time
							5 =>  {
								tz_info = standard;
								break;
							}
							_ =>  {
								break;
							}
						}
						/* final */ let zone_name: String = zone_names[i];
						// ignore the data associated with duplicates supplied in the additional names
						if zone_name != null && sorted.add(zone_name) {
							self.tz_names.put(zone_name, tz_info);
						}
					}
					i += 1;
				 }
			 }
	
		}
		// Order is undefined.
		for /* final */ tz_id in ArraySorter::sort(&TimeZone::getAvailableIDs()) {
			if org::apache::commons::lang3::time::fast_date_parser::TimeZoneStrategy::skip_time_zone(tz_id) {
				continue;
			}
			/* final */ let tz: TimeZone = TimeZones::get_time_zone(tz_id);
			/* final */ let zone_name: String = tz.getDisplayName(locale);
			if sorted.add(zone_name) {
				self.tz_names.put(zone_name, TzInfo::new(tz, &tz.observesDaylightTime()));
			}
		}
		// order the regex alternatives with longer strings first, greedy
		// match will ensure the longest string will be consumed
		sorted.forEach(|zone_name|org::apache::commons::lang3::time::fast_date_parser::FastDateParser::simple_quote(&sb.append('|'), zone_name));
		sb.append(")");
		self.create_pattern(sb);
	}

	fn set_calendar(&self, parser: &org::apache::commons::lang3::time::fast_date_parser::FastDateParser, calendar: &/* Java */ java::util::Calendar /**/, time_zone: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException) */ {
		/* final */ let tz: TimeZone = FastTimeZone::get_gmt_time_zone(time_zone);
		if tz != null {
			calendar.setTimeZone(tz);
		} else {
			let tz_info: TzInfo = self.tz_names.get(time_zone);
			if tz_info == null {
				// match missing the optional trailing period
				tz_info = self.tz_names.get(time_zone + '.');
				if tz_info == null {
					// show chars in case this is multiple byte character issue
					/* final */ let char_array: Vec<char> = time_zone.toCharArray();
					return Err(IllegalStateException::new(&String::format("Can't find time zone '%s' (%d %s) in %s", time_zone, char_array.length, &Arrays::toString(char_array), TreeSet<>::new(&self.tz_names.keySet()))));
				}
			}
			calendar.set(Calendar::DST_OFFSET, tz_info.dstOffset);
			calendar.set(Calendar::ZONE_OFFSET, &tz_info.zone.getRawOffset());
		}
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "TimeZoneStrategy [locale=" + self.locale + ", tzNames=" + self.tz_names + ", pattern=" +  + "]";
	}
}

struct TzInfo {
	zone: /* Java */ java::util::TimeZone /**/,
	dst_offset: i32,
}

impl TzInfo {
	fn new(tz: &/* Java */ java::util::TimeZone /**/, use_dst: bool) -> org::apache::commons::lang3::time::fast_date_parser::TzInfo {
		self.zone = tz;
		self.dst_offset =  if use_dst { tz.getDSTSavings() } else { 0 };
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "TzInfo [zone=" + self.zone + ", dstOffset=" + self.dst_offset + "]";
	}
}