use java::util::Calendar;
use java::util::Date;
use java::util::Locale;
use java::util::TimeZone;

pub struct DateFormatUtils;

impl DateFormatUtils {
	static UTC_TIME_ZONE: /* Java */ java::util::TimeZone /**/ = FastTimeZone::get_gmt_time_zone();

	pub static ISO_8601_EXTENDED_DATETIME_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("yyyy-MM-dd'T'HH:mm:ss");

	pub static ISO_DATETIME_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = ISO_8601_EXTENDED_DATETIME_FORMAT;

	pub static ISO_8601_EXTENDED_DATETIME_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("yyyy-MM-dd'T'HH:mm:ssZZ");

	pub static ISO_DATETIME_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = ISO_8601_EXTENDED_DATETIME_TIME_ZONE_FORMAT;

	pub static ISO_8601_EXTENDED_DATE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("yyyy-MM-dd");

	pub static ISO_DATE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = ISO_8601_EXTENDED_DATE_FORMAT;

	pub static ISO_DATE_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("yyyy-MM-ddZZ");

	pub static ISO_TIME_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("'T'HH:mm:ss");

	pub static ISO_TIME_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("'T'HH:mm:ssZZ");

	pub static ISO_8601_EXTENDED_TIME_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("HH:mm:ss");

	pub static ISO_TIME_NO_T_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = ISO_8601_EXTENDED_TIME_FORMAT;

	pub static ISO_8601_EXTENDED_TIME_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("HH:mm:ssZZ");

	pub static ISO_TIME_NO_T_TIME_ZONE_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = ISO_8601_EXTENDED_TIME_TIME_ZONE_FORMAT;

	pub static SMTP_DATETIME_FORMAT: org::apache::commons::lang3::time::fast_date_format::FastDateFormat = FastDateFormat::get_instance("EEE, dd MMM yyyy HH:mm:ss Z", Locale::US);

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(calendar, pattern, &org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::get_time_zone(calendar), null);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(calendar, pattern, &org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::get_time_zone(calendar), locale);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(calendar, pattern, time_zone, null);
	}

	pub fn format(&self, calendar: &/* Java */ java::util::Calendar /**/, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let df: FastDateFormat = FastDateFormat::get_instance(pattern, time_zone, locale);
		return df.format(calendar);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(date, pattern, null, null);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(date, pattern, null, locale);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(date, pattern, time_zone, null);
	}

	pub fn format(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let df: FastDateFormat = FastDateFormat::get_instance(pattern, time_zone, locale);
		return df.format(date);
	}

	pub fn format(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, null, null);
	}

	pub fn format(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, null, locale);
	}

	pub fn format(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, time_zone, null);
	}

	pub fn format(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, time_zone, locale);
	}

	pub fn formatutc(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(date, pattern, self.UTC_TIME_ZONE, null);
	}

	pub fn formatutc(&self, date: &/* Java */ java::util::Date /**/, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(date, pattern, self.UTC_TIME_ZONE, locale);
	}

	pub fn formatutc(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, self.UTC_TIME_ZONE, null);
	}

	pub fn formatutc(&self, millis: i64, pattern: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::time::date_format_utils::DateFormatUtils::format(Date::new(millis), pattern, self.UTC_TIME_ZONE, locale);
	}

	fn get_time_zone(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::util::TimeZone /**/ {
		return  if calendar == null { null } else { calendar.getTimeZone() };
	}

	pub fn new() -> org::apache::commons::lang3::time::date_format_utils::DateFormatUtils {
	// empty
	}
}