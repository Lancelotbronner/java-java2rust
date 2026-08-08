use java::text::ParseException;
use java::text::ParsePosition;
use java::time::LocalDateTime;
use java::time::OffsetDateTime;
use java::time::ZoneId;
use java::time::ZonedDateTime;
use java::util::Calendar;
use java::util::Date;
use java::util::Iterator;
use java::util::Locale;
use java::util::NoSuchElementException;
use java::util::Objects;
use java::util::TimeZone;
use java::util::concurrent::TimeUnit;
use crate::org::apache::commons::lang3::LocaleUtils;

pub struct DateUtils;

impl DateUtils {
	pub static MILLIS_PER_SECOND: i64 = 1_000;

	pub static MILLIS_PER_MINUTE: i64 = 60 * MILLIS_PER_SECOND;

	pub static MILLIS_PER_HOUR: i64 = 60 * MILLIS_PER_MINUTE;

	pub static MILLIS_PER_DAY: i64 = 24 * MILLIS_PER_HOUR;

	pub static SEMI_MONTH: i32 = 1001;

	static fields: &[&[i32]] = vec![vec![Calendar::MILLISECOND, ]
	, vec![Calendar::SECOND, ]
	, vec![Calendar::MINUTE, ]
	, vec![Calendar::HOUR_OF_DAY, Calendar::HOUR, ]
	, vec![Calendar::DATE, Calendar::DAY_OF_MONTH, Calendar::AM_PM, ]
	, vec![Calendar::MONTH, SEMI_MONTH, ]
	, vec![Calendar::YEAR, ]
	, vec![Calendar::ERA, ]
	, ]
	;

	pub static RANGE_WEEK_SUNDAY: i32 = 1;

	pub static RANGE_WEEK_MONDAY: i32 = 2;

	pub static RANGE_WEEK_RELATIVE: i32 = 3;

	pub static RANGE_WEEK_CENTER: i32 = 4;

	pub static RANGE_MONTH_SUNDAY: i32 = 5;

	pub static RANGE_MONTH_MONDAY: i32 = 6;

	fn add(&self, date: &/* Java */ java::util::Date /**/, calendar_field: i32, amount: i32) -> /* Java */ java::util::Date /**/ {
		org::apache::commons::lang3::time::date_utils::DateUtils::validate_date_not_null(date);
		/* final */ let c: Calendar = Calendar::getInstance();
		c.setTime(date);
		c.add(calendar_field, amount);
		return c.getTime();
	}

	pub fn add_days(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::DAY_OF_MONTH, amount);
	}

	pub fn add_hours(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::HOUR_OF_DAY, amount);
	}

	pub fn add_milliseconds(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::MILLISECOND, amount);
	}

	pub fn add_minutes(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::MINUTE, amount);
	}

	pub fn add_months(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::MONTH, amount);
	}

	pub fn add_seconds(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::SECOND, amount);
	}

	pub fn add_weeks(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::WEEK_OF_YEAR, amount);
	}

	pub fn add_years(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::add(date, Calendar::YEAR, amount);
	}

	pub fn ceiling(&self, calendar: &/* Java */ java::util::Calendar /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Calendar /**/ {
		Objects::requireNonNull(calendar, "calendar");
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(calendar.clone() as Calendar, field, ModifyType::CEILING)?;
	}

	pub fn ceiling(&self, date: &/* Java */ java::util::Date /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(&org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(date), field, ModifyType::CEILING)?.getTime();
	}

	pub fn ceiling(&self, date: &/* Java */ java::lang::Object /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		Objects::requireNonNull(date, "date");
		if date instanceof Date {
			return org::apache::commons::lang3::time::date_utils::DateUtils::ceiling(date as Date, field)?;
		}
		if date instanceof Calendar {
			return org::apache::commons::lang3::time::date_utils::DateUtils::ceiling(date as Calendar, field)?.getTime();
		}
		return Err(ClassCastException::new("Could not find ceiling of for type: " + date.getClass()));
	}

	fn get_fragment(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32, unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		Objects::requireNonNull(calendar, "calendar");
		let result: i64 = 0;
		/* final */ let offset: i32 =  if unit == TimeUnit::DAYS { 0 } else { 1 };
		// Fragments bigger than a day require a breakdown to days
		match fragment {
			Calendar::YEAR =>  {
				result += unit.convert(calendar.get(Calendar::DAY_OF_YEAR) - offset, TimeUnit::DAYS);
				break;
			}
			Calendar::MONTH =>  {
				result += unit.convert(calendar.get(Calendar::DAY_OF_MONTH) - offset, TimeUnit::DAYS);
				break;
			}
			_ =>  {
				break;
			}
		}
		match fragment {
			// Number of days already calculated for these cases
			Calendar::YEAR =>  {
			}
			Calendar::MONTH =>  {
			}
			// The rest of the valid cases
			Calendar::DAY_OF_YEAR =>  {
			}
			Calendar::DATE => result += unit.convert(&calendar.get(Calendar::HOUR_OF_DAY), TimeUnit::HOURS),
			// falls-through
			Calendar::HOUR_OF_DAY => result += unit.convert(&calendar.get(Calendar::MINUTE), TimeUnit::MINUTES),
			// falls-through
			Calendar::MINUTE => result += unit.convert(&calendar.get(Calendar::SECOND), TimeUnit::SECONDS),
			// falls-through
			Calendar::SECOND =>  {
				result += unit.convert(&calendar.get(Calendar::MILLISECOND), TimeUnit::MILLISECONDS);
				break;
			}
			//never useful
			Calendar::MILLISECOND =>  {
				break;
			}
			_ =>  {
				return Err(IllegalArgumentException::new("The fragment " + fragment + " is not supported"));
			}
		}
		return result;
	}

	fn get_fragment(&self, date: &/* Java */ java::util::Date /**/, fragment: i32, unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		org::apache::commons::lang3::time::date_utils::DateUtils::validate_date_not_null(date);
		/* final */ let calendar: Calendar = Calendar::getInstance();
		calendar.setTime(date);
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, unit)?;
	}

	pub fn get_fragment_in_days(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, TimeUnit::DAYS)?;
	}

	pub fn get_fragment_in_days(&self, date: &/* Java */ java::util::Date /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(date, fragment, TimeUnit::DAYS)?;
	}

	pub fn get_fragment_in_hours(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, TimeUnit::HOURS)?;
	}

	pub fn get_fragment_in_hours(&self, date: &/* Java */ java::util::Date /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(date, fragment, TimeUnit::HOURS)?;
	}

	pub fn get_fragment_in_milliseconds(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, TimeUnit::MILLISECONDS)?;
	}

	pub fn get_fragment_in_milliseconds(&self, date: &/* Java */ java::util::Date /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(date, fragment, TimeUnit::MILLISECONDS)?;
	}

	pub fn get_fragment_in_minutes(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, TimeUnit::MINUTES)?;
	}

	pub fn get_fragment_in_minutes(&self, date: &/* Java */ java::util::Date /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(date, fragment, TimeUnit::MINUTES)?;
	}

	pub fn get_fragment_in_seconds(&self, calendar: &/* Java */ java::util::Calendar /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(calendar, fragment, TimeUnit::SECONDS)?;
	}

	pub fn get_fragment_in_seconds(&self, date: &/* Java */ java::util::Date /**/, fragment: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		return org::apache::commons::lang3::time::date_utils::DateUtils::get_fragment(date, fragment, TimeUnit::SECONDS)?;
	}

	pub fn is_same_day(&self, cal1: &/* Java */ java::util::Calendar /**/, cal2: &/* Java */ java::util::Calendar /**/) -> bool {
		Objects::requireNonNull(cal1, "cal1");
		Objects::requireNonNull(cal2, "cal2");
		return cal1.get(Calendar::ERA) == cal2.get(Calendar::ERA) && cal1.get(Calendar::YEAR) == cal2.get(Calendar::YEAR) && cal1.get(Calendar::DAY_OF_YEAR) == cal2.get(Calendar::DAY_OF_YEAR);
	}

	pub fn is_same_day(&self, date1: &/* Java */ java::util::Date /**/, date2: &/* Java */ java::util::Date /**/) -> bool {
		return org::apache::commons::lang3::time::date_utils::DateUtils::is_same_day(&org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(date1), &org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(date2));
	}

	pub fn is_same_instant(&self, cal1: &/* Java */ java::util::Calendar /**/, cal2: &/* Java */ java::util::Calendar /**/) -> bool {
		Objects::requireNonNull(cal1, "cal1");
		Objects::requireNonNull(cal2, "cal2");
		return cal1.getTime().getTime() == cal2.getTime().getTime();
	}

	pub fn is_same_instant(&self, date1: &/* Java */ java::util::Date /**/, date2: &/* Java */ java::util::Date /**/) -> bool {
		Objects::requireNonNull(date1, "date1");
		Objects::requireNonNull(date2, "date2");
		return date1.getTime() == date2.getTime();
	}

	pub fn is_same_local_time(&self, cal1: &/* Java */ java::util::Calendar /**/, cal2: &/* Java */ java::util::Calendar /**/) -> bool {
		Objects::requireNonNull(cal1, "cal1");
		Objects::requireNonNull(cal2, "cal2");
		return cal1.get(Calendar::MILLISECOND) == cal2.get(Calendar::MILLISECOND) && cal1.get(Calendar::SECOND) == cal2.get(Calendar::SECOND) && cal1.get(Calendar::MINUTE) == cal2.get(Calendar::MINUTE) && cal1.get(Calendar::HOUR_OF_DAY) == cal2.get(Calendar::HOUR_OF_DAY) && cal1.get(Calendar::DAY_OF_YEAR) == cal2.get(Calendar::DAY_OF_YEAR) && cal1.get(Calendar::YEAR) == cal2.get(Calendar::YEAR) && cal1.get(Calendar::ERA) == cal2.get(Calendar::ERA) && cal1.getClass() == cal2.getClass();
	}

	pub fn iterator(&self, calendar: &/* Java */ java::util::Calendar /**/, range_style: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		Objects::requireNonNull(calendar, "calendar");
		/* final */ let start: Calendar;
		/* final */ let end: Calendar;
		let start_cutoff: i32 = Calendar::SUNDAY;
		let end_cutoff: i32 = Calendar::SATURDAY;
		match range_style {
			self.RANGE_MONTH_SUNDAY =>  {
			}
			self.RANGE_MONTH_MONDAY =>  {
				//Set start to the first of the month
				start = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(calendar, Calendar::MONTH);
				//Set end to the last of the month
				end = start.clone() as Calendar;
				end.add(Calendar::MONTH, 1);
				end.add(Calendar::DATE, -1);
				//Loop start back to the previous sunday or monday
				if range_style == self.RANGE_MONTH_MONDAY {
					start_cutoff = Calendar::MONDAY;
					end_cutoff = Calendar::SUNDAY;
				}
				break;
			}
			self.RANGE_WEEK_SUNDAY =>  {
			}
			self.RANGE_WEEK_MONDAY =>  {
			}
			self.RANGE_WEEK_RELATIVE =>  {
			}
			self.RANGE_WEEK_CENTER =>  {
				//Set start and end to the current date
				start = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(calendar, Calendar::DATE);
				end = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(calendar, Calendar::DATE);
				match range_style {
					self.RANGE_WEEK_SUNDAY =>  {
						//already set by default
						break;
					}
					self.RANGE_WEEK_MONDAY =>  {
						start_cutoff = Calendar::MONDAY;
						end_cutoff = Calendar::SUNDAY;
						break;
					}
					self.RANGE_WEEK_RELATIVE =>  {
						start_cutoff = calendar.get(Calendar::DAY_OF_WEEK);
						end_cutoff = start_cutoff - 1;
						break;
					}
					self.RANGE_WEEK_CENTER =>  {
						start_cutoff = calendar.get(Calendar::DAY_OF_WEEK) - 3;
						end_cutoff = calendar.get(Calendar::DAY_OF_WEEK) + 3;
						break;
					}
					_ =>  {
						break;
					}
				}
				break;
			}
			_ =>  {
				return Err(IllegalArgumentException::new("The range style " + range_style + " is not valid."));
			}
		}
		if start_cutoff < Calendar::SUNDAY {
			start_cutoff += 7;
		}
		if start_cutoff > Calendar::SATURDAY {
			start_cutoff -= 7;
		}
		if end_cutoff < Calendar::SUNDAY {
			end_cutoff += 7;
		}
		if end_cutoff > Calendar::SATURDAY {
			end_cutoff -= 7;
		}
		while start.get(Calendar::DAY_OF_WEEK) != start_cutoff {
			start.add(Calendar::DATE, -1);
		}
		while end.get(Calendar::DAY_OF_WEEK) != end_cutoff {
			end.add(Calendar::DATE, 1);
		}
		return DateIterator::new(start, end);
	}

	pub fn iterator(&self, focus: &/* Java */ java::util::Date /**/, range_style: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::iterator(&org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(focus), range_style)?;
	}

	pub fn iterator(&self, calendar: &/* Java */ java::lang::Object /**/, range_style: i32) /* thrown(java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		Objects::requireNonNull(calendar, "calendar");
		if calendar instanceof Date {
			return org::apache::commons::lang3::time::date_utils::DateUtils::iterator(calendar as Date, range_style)?;
		}
		if calendar instanceof Calendar {
			return org::apache::commons::lang3::time::date_utils::DateUtils::iterator(calendar as Calendar, range_style)?;
		}
		return Err(ClassCastException::new("Could not iterate based on " + calendar));
	}

	fn modify(&self, val: &/* Java */ java::util::Calendar /**/, field: i32, mod_type: &org::apache::commons::lang3::time::date_utils::ModifyType) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Calendar /**/ {
		if val.get(Calendar::YEAR) > 280000000 {
			return Err(ArithmeticException::new("Calendar value too large for accurate calculations"));
		}
		if field == Calendar::MILLISECOND {
			return val;
		}
		// Fix for LANG-59 START
		// see https://issues.apache.org/jira/browse/LANG-59
		//
		// Manually truncate milliseconds, seconds and minutes, rather than using
		// Calendar methods.
		/* final */ let date: Date = val.getTime();
		let time: i64 = date.getTime();
		let done: bool = false;
		// truncate milliseconds
		/* final */ let millisecs: i32 = val.get(Calendar::MILLISECOND);
		if ModifyType::TRUNCATE == mod_type || millisecs < 500 {
			time -= millisecs;
		}
		if field == Calendar::SECOND {
			done = true;
		}
		// truncate seconds
		/* final */ let seconds: i32 = val.get(Calendar::SECOND);
		if !done && (ModifyType::TRUNCATE == mod_type || seconds < 30) {
			time = time - seconds * 1000;
		}
		if field == Calendar::MINUTE {
			done = true;
		}
		// truncate minutes
		/* final */ let minutes: i32 = val.get(Calendar::MINUTE);
		if !done && (ModifyType::TRUNCATE == mod_type || minutes < 30) {
			time = time - minutes * 60000;
		}
		// reset time
		if date.getTime() != time {
			date.setTime(time);
			val.setTime(date);
		}
		// Fix for LANG-59 END
		let round_up: bool = false;
		for /* final */ a_field in self.fields {
			for /* final */ element in a_field {
				if element == field {
					//This is our field... we stop looping
					if mod_type == ModifyType::CEILING || mod_type == ModifyType::ROUND && round_up {
						if field == self.SEMI_MONTH {
							//  we subtract 15 days and add 1 month
							if val.get(Calendar::DATE) == 1 {
								val.add(Calendar::DATE, 15);
							} else {
								val.add(Calendar::DATE, -15);
								val.add(Calendar::MONTH, 1);
							}
						// Fix for LANG-440 START
						} else if field == Calendar::AM_PM {
							//  we subtract 12 hours and add 1 day
							if val.get(Calendar::HOUR_OF_DAY) == 0 {
								val.add(Calendar::HOUR_OF_DAY, 12);
							} else {
								val.add(Calendar::HOUR_OF_DAY, -12);
								val.add(Calendar::DATE, 1);
							}
						// Fix for LANG-440 END
						} else {
							//We need at add one to this field since the
							//  last number causes us to round up
							val.add(a_field[0], 1);
						}
					}
					return val;
				}
			}
			//We have various fields that are not easy roundings
			let offset: i32 = 0;
			let offset_set: bool = false;
			//These are special types of fields that require different rounding rules
			match field {
				self.SEMI_MONTH =>  {
					if a_field[0] == Calendar::DATE {
						//If we're going to drop the DATE field's value,
						//  we want to do this our own way.
						//We need to subtract 1 since the date has a minimum of 1
						offset = val.get(Calendar::DATE) - 1;
						//  bottom half of the month and should stay accordingly.
						if offset >= 15 {
							offset -= 15;
						}
						//Record whether we're in the top or bottom half of that range
						round_up = offset > 7;
						offset_set = true;
					}
					break;
				}
				Calendar::AM_PM =>  {
					if a_field[0] == Calendar::HOUR_OF_DAY {
						//If we're going to drop the HOUR field's value,
						//  we want to do this our own way.
						offset = val.get(Calendar::HOUR_OF_DAY);
						if offset >= 12 {
							offset -= 12;
						}
						round_up = offset >= 6;
						offset_set = true;
					}
					break;
				}
				_ =>  {
					break;
				}
			}
			if !offset_set {
				/* final */ let min: i32 = val.getActualMinimum(a_field[0]);
				/* final */ let max: i32 = val.getActualMaximum(a_field[0]);
				//Calculate the offset from the minimum allowed value
				offset = val.get(a_field[0]) - min;
				//Set roundUp if this is more than halfway between the minimum and maximum
				round_up = offset > (max - min) / 2;
			}
			//We need to remove this field
			if offset != 0 {
				val.set(a_field[0], val.get(a_field[0]) - offset);
			}
		}
		return Err(IllegalArgumentException::new("The field " + field + " is not supported"));
	}

	pub fn parse_date(&self, str: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/, parse_patterns: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::parse_date_with_leniency(str, locale, parse_patterns, true)?;
	}

	pub fn parse_date(&self, str: &/* Java */ java::lang::String /**/, parse_patterns: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::parse_date(str, null, parse_patterns)?;
	}

	pub fn parse_date_strictly(&self, str: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/, parse_patterns: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::parse_date_with_leniency(str, locale, parse_patterns, false)?;
	}

	pub fn parse_date_strictly(&self, str: &/* Java */ java::lang::String /**/, parse_patterns: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::parse_date_strictly(str, null, parse_patterns)?;
	}

	fn parse_date_with_leniency(&self, date_str: &/* Java */ java::lang::String /**/, locale: &/* Java */ java::util::Locale /**/, parse_patterns: &&[/* Java */ java::lang::String /**/], lenient: bool) /* thrown(java.text.ParseException) */ -> /* Java */ java::util::Date /**/ {
		Objects::requireNonNull(date_str, "str");
		Objects::requireNonNull(parse_patterns, "parsePatterns");
		/* final */ let tz: TimeZone = TimeZone::getDefault();
		/* final */ let lcl: Locale = LocaleUtils::to_locale(locale);
		/* final */ let pos: ParsePosition = ParsePosition::new(0);
		/* final */ let calendar: Calendar = Calendar::getInstance(tz, lcl);
		calendar.setLenient(lenient);
		for /* final */ parse_pattern in parse_patterns {
			/* final */ let fdp: FastDateParser = FastDateParser::new(parse_pattern, tz, lcl);
			calendar.clear();
			let r0 = 'try0: {
				if fdp.parse(date_str, pos, calendar) && pos.getIndex() == date_str.length() {
					return calendar.getTime();
				}
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ IllegalArgumentException) => {
				// leniency is preventing calendar from being set
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			pos.setIndex(0);
		}
		break 'try0 Err(ParseException::new("Unable to parse the date: " + date_str, -1));
	}

	pub fn round(&self, calendar: &/* Java */ java::util::Calendar /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Calendar /**/ {
		Objects::requireNonNull(calendar, "calendar");
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(calendar.clone() as Calendar, field, ModifyType::ROUND)?;
	}

	pub fn round(&self, date: &/* Java */ java::util::Date /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(&org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(date), field, ModifyType::ROUND)?.getTime();
	}

	pub fn round(&self, date: &/* Java */ java::lang::Object /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		Objects::requireNonNull(date, "date");
		if date instanceof Date {
			return org::apache::commons::lang3::time::date_utils::DateUtils::round(date as Date, field)?;
		}
		if date instanceof Calendar {
			return org::apache::commons::lang3::time::date_utils::DateUtils::round(date as Calendar, field)?.getTime();
		}
		return Err(ClassCastException::new("Could not round " + date));
	}

	fn set(&self, date: &/* Java */ java::util::Date /**/, calendar_field: i32, amount: i32) -> /* Java */ java::util::Date /**/ {
		org::apache::commons::lang3::time::date_utils::DateUtils::validate_date_not_null(date);
		// getInstance() returns a new object, so this method is thread safe.
		/* final */ let c: Calendar = Calendar::getInstance();
		c.setLenient(false);
		c.setTime(date);
		c.set(calendar_field, amount);
		return c.getTime();
	}

	pub fn set_days(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::DAY_OF_MONTH, amount);
	}

	pub fn set_hours(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::HOUR_OF_DAY, amount);
	}

	pub fn set_milliseconds(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::MILLISECOND, amount);
	}

	pub fn set_minutes(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::MINUTE, amount);
	}

	pub fn set_months(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::MONTH, amount);
	}

	pub fn set_seconds(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::SECOND, amount);
	}

	pub fn set_years(&self, date: &/* Java */ java::util::Date /**/, amount: i32) -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::set(date, Calendar::YEAR, amount);
	}

	pub fn to_calendar(&self, date: &/* Java */ java::util::Date /**/) -> /* Java */ java::util::Calendar /**/ {
		/* final */ let c: Calendar = Calendar::getInstance();
		c.setTime(&Objects::requireNonNull(date, "date"));
		return c;
	}

	pub fn to_calendar(&self, date: &/* Java */ java::util::Date /**/, tz: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::util::Calendar /**/ {
		/* final */ let c: Calendar = Calendar::getInstance(tz);
		c.setTime(&Objects::requireNonNull(date, "date"));
		return c;
	}

	pub fn to_local_date_time(&self, date: &/* Java */ java::util::Date /**/) -> /* Java */ java::time::LocalDateTime /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::to_local_date_time(date, &TimeZone::getDefault());
	}

	pub fn to_local_date_time(&self, date: &/* Java */ java::util::Date /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::time::LocalDateTime /**/ {
		return LocalDateTime::ofInstant(&date.toInstant(), &org::apache::commons::lang3::time::date_utils::DateUtils::to_zone_id(time_zone));
	}

	pub fn to_offset_date_time(&self, date: &/* Java */ java::util::Date /**/) -> /* Java */ java::time::OffsetDateTime /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::to_offset_date_time(date, &TimeZone::getDefault());
	}

	pub fn to_offset_date_time(&self, date: &/* Java */ java::util::Date /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::time::OffsetDateTime /**/ {
		return OffsetDateTime::ofInstant(&date.toInstant(), &org::apache::commons::lang3::time::date_utils::DateUtils::to_zone_id(time_zone));
	}

	pub fn to_zoned_date_time(&self, date: &/* Java */ java::util::Date /**/) -> /* Java */ java::time::ZonedDateTime /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::to_zoned_date_time(date, &TimeZone::getDefault());
	}

	pub fn to_zoned_date_time(&self, date: &/* Java */ java::util::Date /**/, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::time::ZonedDateTime /**/ {
		return ZonedDateTime::ofInstant(&date.toInstant(), &org::apache::commons::lang3::time::date_utils::DateUtils::to_zone_id(time_zone));
	}

	fn to_zone_id(&self, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::time::ZoneId /**/ {
		return TimeZones::to_time_zone(time_zone).toZoneId();
	}

	pub fn truncate(&self, date: &/* Java */ java::util::Calendar /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Calendar /**/ {
		Objects::requireNonNull(date, "date");
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(date.clone() as Calendar, field, ModifyType::TRUNCATE)?;
	}

	pub fn truncate(&self, date: &/* Java */ java::util::Date /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		return org::apache::commons::lang3::time::date_utils::DateUtils::modify(&org::apache::commons::lang3::time::date_utils::DateUtils::to_calendar(date), field, ModifyType::TRUNCATE)?.getTime();
	}

	pub fn truncate(&self, date: &/* Java */ java::lang::Object /**/, field: i32) /* thrown(java.lang.ArithmeticException | java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Date /**/ {
		Objects::requireNonNull(date, "date");
		if date instanceof Date {
			return org::apache::commons::lang3::time::date_utils::DateUtils::truncate(date as Date, field)?;
		}
		if date instanceof Calendar {
			return org::apache::commons::lang3::time::date_utils::DateUtils::truncate(date as Calendar, field)?.getTime();
		}
		return Err(ClassCastException::new("Could not truncate " + date));
	}

	pub fn truncated_compare_to(&self, cal1: &/* Java */ java::util::Calendar /**/, cal2: &/* Java */ java::util::Calendar /**/, field: i32) -> i32 {
		/* final */ let truncated_cal1: Calendar = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(cal1, field)?;
		/* final */ let truncated_cal2: Calendar = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(cal2, field)?;
		return truncated_cal1.compareTo(truncated_cal2);
	}

	pub fn truncated_compare_to(&self, date1: &/* Java */ java::util::Date /**/, date2: &/* Java */ java::util::Date /**/, field: i32) -> i32 {
		/* final */ let truncated_date1: Date = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(date1, field)?;
		/* final */ let truncated_date2: Date = org::apache::commons::lang3::time::date_utils::DateUtils::truncate(date2, field)?;
		return truncated_date1.compareTo(truncated_date2);
	}

	pub fn truncated_equals(&self, cal1: &/* Java */ java::util::Calendar /**/, cal2: &/* Java */ java::util::Calendar /**/, field: i32) -> bool {
		return org::apache::commons::lang3::time::date_utils::DateUtils::truncated_compare_to(cal1, cal2, field) == 0;
	}

	pub fn truncated_equals(&self, date1: &/* Java */ java::util::Date /**/, date2: &/* Java */ java::util::Date /**/, field: i32) -> bool {
		return org::apache::commons::lang3::time::date_utils::DateUtils::truncated_compare_to(date1, date2, field) == 0;
	}

	fn validate_date_not_null(&self, date: &/* Java */ java::util::Date /**/) {
		Objects::requireNonNull(date, "date");
	}

	pub fn new() -> org::apache::commons::lang3::time::date_utils::DateUtils {
	// empty
	}
}

struct DateIterator {
	end_final: /* Java */ java::util::Calendar /**/,
	spot: /* Java */ java::util::Calendar /**/,
}

impl DateIterator {
	fn new(start_final: &/* Java */ java::util::Calendar /**/, end_final: &/* Java */ java::util::Calendar /**/) -> org::apache::commons::lang3::time::date_utils::DateIterator {
		self.endFinal = end_final;
		self.spot = start_final;
		self.spot.add(Calendar::DATE, -1);
	}

	pub fn has_next(&self) -> bool {
		return self.spot.before(self.end_final);
	}

	pub fn next(&self) /* thrown(java.util.NoSuchElementException) */ -> /* Java */ java::util::Calendar /**/ {
		if self.spot.equals(self.end_final) {
			return Err(NoSuchElementException::new());
		}
		self.spot.add(Calendar::DATE, 1);
		return self.spot.clone() as Calendar;
	}

	pub fn remove(&self) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}
}

impl /* Java */ java::util::Iterator /**/ for DateIterator {}

enum ModifyType;