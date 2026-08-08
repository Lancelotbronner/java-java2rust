use java::util::Date;
use java::util::Objects;
use java::util::TimeZone;

struct GmtTimeZone {
	offset: i32,
	zone_id: /* Java */ java::lang::String /**/,
}

impl GmtTimeZone {
	static MILLISECONDS_PER_MINUTE: i32 = 60 * 1000;

	static MINUTES_PER_HOUR: i32 = 60;

	static HOURS_PER_DAY: i32 = 24;

	static serialVersionUID: i64 = 1;

	fn two_digits(&self, sb: &/* Java */ java::lang::StringBuilder /**/, n: i32) -> /* Java */ java::lang::StringBuilder /**/ {
		return sb.append(('0' + n / 10) as char).append(('0' + n % 10) as char);
	}

	fn new(negate: bool, hours: i32, minutes: i32) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::time::gmt_time_zone::GmtTimeZone {
		if hours >= self.HOURS_PER_DAY {
			return Err(IllegalArgumentException::new(hours + " hours out of range"));
		}
		if minutes >= self.MINUTES_PER_HOUR {
			return Err(IllegalArgumentException::new(minutes + " minutes out of range"));
		}
		/* final */ let milliseconds: i32 = (minutes + hours * self.MINUTES_PER_HOUR) * self.MILLISECONDS_PER_MINUTE;
		self.offset =  if negate { -milliseconds } else { milliseconds };
		// @formatter:off
		self.zone_id = org::apache::commons::lang3::time::gmt_time_zone::GmtTimeZone::two_digits(&org::apache::commons::lang3::time::gmt_time_zone::GmtTimeZone::two_digits(&StringBuilder::new(9).append(TimeZones::GMT_ID).append( if negate { '-' } else { '+' }), hours).append(':'), minutes).toString();
	// @formatter:on
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if !(obj instanceof GmtTimeZone) {
			return false;
		}
		/* final */ let other: GmtTimeZone = obj as GmtTimeZone;
		return self.offset == other.offset && Objects::equals(self.zone_id, other.zoneId);
	}

	pub fn getid(&self) -> /* Java */ java::lang::String /**/ {
		return self.zone_id;
	}

	pub fn get_offset(&self, era: i32, year: i32, month: i32, day: i32, day_of_week: i32, milliseconds: i32) -> i32 {
		return self.offset;
	}

	pub fn get_raw_offset(&self) -> i32 {
		return self.offset;
	}

	pub fn hash_code(&self) -> i32 {
		return Objects::hash(self.offset, self.zone_id);
	}

	pub fn in_daylight_time(&self, date: &/* Java */ java::util::Date /**/) -> bool {
		return false;
	}

	pub fn set_raw_offset(&self, offset_millis: i32) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "[GmtTimeZone id=\"" + self.zone_id + "\",offset=" + self.offset + ']';
	}

	pub fn use_daylight_time(&self) -> bool {
		return false;
	}
}

impl /* Java */ java::io::Serializable /**/ for GmtTimeZone {}

impl /* Java */ java::lang::Cloneable /**/ for GmtTimeZone {}