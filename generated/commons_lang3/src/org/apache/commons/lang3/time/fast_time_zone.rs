use java::util::TimeZone;
use java::util::regex::Matcher;
use java::util::regex::Pattern;

pub struct FastTimeZone;

impl FastTimeZone {
	static GMT_PATTERN: /* Java */ java::util::regex::Pattern /**/ = Pattern::compile("^(?:(?i)GMT)?([+-])?(\\d\\d?)?(:?(\\d\\d?))?$");

	static GREENWICH: /* Java */ java::util::TimeZone /**/ = GmtTimeZone::new(false, 0, 0);

	pub fn get_gmt_time_zone(&self) -> /* Java */ java::util::TimeZone /**/ {
		return self.GREENWICH;
	}

	pub fn get_gmt_time_zone(&self, pattern: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::TimeZone /**/ {
		if "Z".equals(pattern) || "UTC".equals(pattern) {
			return self.GREENWICH;
		}
		/* final */ let m: Matcher = self.GMT_PATTERN.matcher(pattern);
		if m.matches() {
			/* final */ let hours: i32 = org::apache::commons::lang3::time::fast_time_zone::FastTimeZone::parse_int(&m.group(2));
			/* final */ let minutes: i32 = org::apache::commons::lang3::time::fast_time_zone::FastTimeZone::parse_int(&m.group(4));
			if hours == 0 && minutes == 0 {
				return self.GREENWICH;
			}
			return GmtTimeZone::new(&org::apache::commons::lang3::time::fast_time_zone::FastTimeZone::parse_sign(&m.group(1)), hours, minutes);
		}
		return null;
	}

	pub fn get_time_zone(&self, id: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::TimeZone /**/ {
		/* final */ let tz: TimeZone = org::apache::commons::lang3::time::fast_time_zone::FastTimeZone::get_gmt_time_zone(id);
		if tz != null {
			return tz;
		}
		return TimeZones::get_time_zone(id);
	}

	fn parse_int(&self, group: &/* Java */ java::lang::String /**/) -> i32 {
		return  if group != null { Integer::parseInt(group) } else { 0 };
	}

	fn parse_sign(&self, group: &/* Java */ java::lang::String /**/) -> bool {
		return group != null && group.charAt(0) == '-';
	}

	fn new() -> org::apache::commons::lang3::time::fast_time_zone::FastTimeZone {
	}
}