use java::time::LocalDate;
use java::time::LocalDateTime;
use java::time::OffsetDateTime;
use java::time::ZoneId;
use java::time::ZonedDateTime;
use java::util::Calendar;
use java::util::Locale;
use java::util::Locale::Category;
use java::util::Map;
use java::util::Objects;

pub struct CalendarUtils {
	calendar: /* Java */ java::util::Calendar /**/,
	locale: /* Java */ java::util::Locale /**/,
}

impl CalendarUtils {
	pub static INSTANCE: org::apache::commons::lang3::time::calendar_utils::CalendarUtils = org::apache::commons::lang3::time::calendar_utils::CalendarUtils::get_instance();

	pub fn get_instance(&self) -> org::apache::commons::lang3::time::calendar_utils::CalendarUtils {
		return CalendarUtils::new(&Calendar::getInstance());
	}

	fn get_instance(&self, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::calendar_utils::CalendarUtils {
		return CalendarUtils::new(&Calendar::getInstance(locale), locale);
	}

	pub fn to_local_date_time(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::time::LocalDateTime /**/ {
		return LocalDateTime::ofInstant(&calendar.toInstant(), &org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_zone_id(calendar));
	}

	pub fn to_offset_date_time(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::time::OffsetDateTime /**/ {
		return OffsetDateTime::ofInstant(&calendar.toInstant(), &org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_zone_id(calendar));
	}

	pub fn to_zoned_date_time(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::time::ZonedDateTime /**/ {
		return ZonedDateTime::ofInstant(&calendar.toInstant(), &org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_zone_id(calendar));
	}

	fn to_zone_id(&self, calendar: &/* Java */ java::util::Calendar /**/) -> /* Java */ java::time::ZoneId /**/ {
		return calendar.getTimeZone().toZoneId();
	}

	pub fn new(calendar: &/* Java */ java::util::Calendar /**/) -> org::apache::commons::lang3::time::calendar_utils::CalendarUtils {
		this(calendar, &Locale::getDefault());
	}

	fn new(calendar: &/* Java */ java::util::Calendar /**/, locale: &/* Java */ java::util::Locale /**/) -> org::apache::commons::lang3::time::calendar_utils::CalendarUtils {
		self.calendar = Objects::requireNonNull(calendar, "calendar");
		self.locale = Objects::requireNonNull(locale, "locale");
	}

	pub fn get_day_of_month(&self) -> i32 {
		return self.calendar.get(Calendar::DAY_OF_MONTH);
	}

	pub fn get_day_of_year(&self) -> i32 {
		return self.calendar.get(Calendar::DAY_OF_YEAR);
	}

	pub fn get_month(&self) -> i32 {
		return self.calendar.get(Calendar::MONTH);
	}

	fn get_month_display_names(&self, style: i32) -> &[/* Java */ java::lang::String /**/] {
		// Unfortunately standalone month names are not available in DateFormatSymbols,
		// so we have to extract them.
		/* final */ let display_names: Map<String, Integer> = self.calendar.getDisplayNames(Calendar::MONTH, style, self.locale);
		if display_names == null {
			return null;
		}
		/* final */ let month_names: [Option<String>; display_names.size()] = [None; display_names.size()];
		display_names.forEach(|(k, v)|month_names[v] = k);
		return month_names;
	}

	fn get_standalone_long_month_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.get_month_display_names(Calendar::LONG_STANDALONE);
	}

	fn get_standalone_short_month_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.get_month_display_names(Calendar::SHORT_STANDALONE);
	}

	pub fn get_year(&mut self) -> i32 {
		return self.calendar.get(Calendar::YEAR);
	}

	pub fn to_local_date(&self) -> /* Java */ java::time::LocalDate /**/ {
		return self.to_local_date_time().toLocalDate();
	}

	pub fn to_local_date_time(&mut self) -> /* Java */ java::time::LocalDateTime /**/ {
		return org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_local_date_time(self.calendar);
	}

	pub fn to_offset_date_time(&mut self) -> /* Java */ java::time::OffsetDateTime /**/ {
		return org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_offset_date_time(self.calendar);
	}

	pub fn to_zoned_date_time(&mut self) -> /* Java */ java::time::ZonedDateTime /**/ {
		return org::apache::commons::lang3::time::calendar_utils::CalendarUtils::to_zoned_date_time(self.calendar);
	}
}