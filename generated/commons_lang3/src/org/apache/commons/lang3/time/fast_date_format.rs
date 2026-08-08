use java::text::DateFormat;
use java::text::FieldPosition;
use java::text::Format;
use java::text::ParseException;
use java::text::ParsePosition;
use java::text::SimpleDateFormat;
use java::util::Calendar;
use java::util::Date;
use java::util::GregorianCalendar;
use java::util::Locale;
use java::util::TimeZone;

pub struct FastDateFormat {
	printer: org::apache::commons::lang3::time::fast_date_printer::FastDatePrinter,
	parser: org::apache::commons::lang3::time::fast_date_parser::FastDateParser,
}

impl FastDateFormat {
	static serialVersionUID: i64 = 2;

	pub static FULL: i32 = DateFormat::FULL;

	pub static LONG: i32 = DateFormat::LONG;

	pub static MEDIUM: i32 = DateFormat::MEDIUM;

	pub static SHORT: i32 = DateFormat::SHORT;

	static CACHE: org::apache::commons::lang3::time::abstract_format_cache::AbstractFormatCache = AbstractFormatCache<FastDateFormat>::new() {
		/* protected */ fn create_instance(&self, /* final */ pattern: &String, /* final */ time_zone: &TimeZone, /* final */ locale: &Locale) -> FastDateFormat {
			return FastDateFormat::new(pattern, time_zone, locale);
		}
	
	};

	fn clear(&self) {
		AbstractFormatCache::clear();
		self.CACHE.clear_instance();
	}

	pub fn get_date_instance(&self, style: i32) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_instance(style, null, null);
	}

	pub fn get_date_instance(&self, style: i32, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_instance(style, null, locale);
	}

	pub fn get_date_instance(&self, style: i32, time_zone: &/* Java */ java::util::TimeZone /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_instance(style, time_zone, null);
	}

	pub fn get_date_instance(&self, style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_instance(style, time_zone, locale);
	}

	pub fn get_date_time_instance(&self, date_style: i32, time_style: i32) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_time_instance(date_style, time_style, null, null);
	}

	pub fn get_date_time_instance(&self, date_style: i32, time_style: i32, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_time_instance(date_style, time_style, null, locale);
	}

	pub fn get_date_time_instance(&self, date_style: i32, time_style: i32, time_zone: &/* Java */ java::util::TimeZone /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return org::apache::commons::lang3::time::fast_date_format::FastDateFormat::get_date_time_instance(date_style, time_style, time_zone, null);
	}

	pub fn get_date_time_instance(&self, date_style: i32, time_style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_date_time_instance(date_style, time_style, time_zone, locale);
	}

	pub fn get_instance(&self) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_instance();
	}

	pub fn get_instance(&self, pattern: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_instance(pattern, null, null);
	}

	pub fn get_instance(&self, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_instance(pattern, null, locale);
	}

	pub fn get_instance(&self, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_instance(pattern, time_zone, null);
	}

	pub fn get_instance(&self, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_instance(pattern, time_zone, locale);
	}

	pub fn get_time_instance(&self, style: i32) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_time_instance(style, null, null);
	}

	pub fn get_time_instance(&self, style: i32, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_time_instance(style, null, locale);
	}

	pub fn get_time_instance(&self, style: i32, time_zone: &/* Java */ java::util::TimeZone /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_time_instance(style, time_zone, null);
	}

	pub fn get_time_instance(&self, style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		return self.CACHE.get_time_instance(style, time_zone, locale);
	}

	fn new(pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		this(pattern, time_zone, locale, null);
	}

	fn new(pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/, century_start: &/* Java */ java::util::Date /**/) -> org::apache::commons::lang3::time::fast_date_format::FastDateFormat {
		self.printer = FastDatePrinter::new(pattern, time_zone, locale);
		self.parser = FastDateParser::new(pattern, time_zone, locale, century_start);
	}

	fn apply_rules(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.printer.format(calendar, buf);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if !(obj instanceof FastDateFormat) {
			return false;
		}
		/* final */ let other: FastDateFormat = obj as FastDateFormat;
		// no need to check parser, as it has same invariants as printer
		return self.printer.equals(other.printer);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::lang::String /**/ {
		return self.printer.format(calendar);
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &B) -> B {
		return self.printer.format(calendar, buf);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, buf: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.printer.format(calendar, buf);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/) -> /* Java */ java::lang::String /**/ {
		return self.printer.format(date);
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, date: &/* Java */ java::util::Date /**/, buf: &B) -> B {
		return self.printer.format(date, buf);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, buf: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.printer.format(date, buf);
	}

	pub fn format(&self, millis: i64) -> /* Java */ java::lang::String /**/ {
		return self.printer.format(millis);
	}

	pub fn format<B: /* Java */ java::lang::Appendable /**/>(&self, millis: i64, buf: &B) -> B {
		return self.printer.format(millis, buf);
	}

	pub fn format(&self, millis: i64, buf: &/* Java */ java::lang::StringBuffer /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.printer.format(millis, buf);
	}

	pub fn format(&self, obj: &/* Java */ java::lang::Object /**/, to_append_to: &/* Java */ java::lang::StringBuffer /**/, pos: &/* Java */ java::text::FieldPosition /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::StringBuffer /**/ {
		return to_append_to.append(&self.printer.format(obj)?);
	}

	pub fn get_locale(&self) -> /* Java */ java::util::Locale /**/ {
		return self.printer.get_locale();
	}

	pub fn get_max_length_estimate(&self) -> i32 {
		return self.printer.get_max_length_estimate();
	}

	pub fn get_pattern(&self) -> /* Java */ java::lang::String /**/ {
		return self.printer.get_pattern();
	}

	pub fn get_time_zone(&self) -> /* Java */ java::util::TimeZone /**/ {
		return self.printer.get_time_zone();
	}

	pub fn hash_code(&self) -> i32 {
		return self.printer.hash_code();
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		return self.parser.parse(source)?;
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::util::Date /**/ {
		return self.parser.parse(source, pos);
	}

	pub fn parse(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/, calendar: &/* Java */ java::util::Calendar /**/) -> bool {
		return self.parser.parse(source, pos, calendar);
	}

	pub fn parse_object(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::lang::Object /**/ {
		return self.parser.parse_object(source, pos);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "FastDateFormat[" + self.printer.get_pattern() + "," + self.printer.get_locale() + "," + self.printer.get_time_zone().getID() + "]";
	}
}

impl org::apache::commons::lang3::time::date_parser::DateParser for FastDateFormat {}

impl org::apache::commons::lang3::time::date_printer::DatePrinter for FastDateFormat {}

impl /* Java */ java::io::Serializable /**/ for FastDateFormat {}

impl /* Java */ java::lang::Cloneable /**/ for FastDateFormat {}