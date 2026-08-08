use java::time::ZoneId;
use java::util::TimeZone;
use crate::org::apache::commons::lang3::JavaVersion;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::SystemProperties;
use crate::org::apache::commons::lang3::SystemUtils;

pub struct TimeZones;

impl TimeZones {
	pub static GMT_ID: /* Java */ java::lang::String /**/ = "GMT";

	pub static GMT: /* Java */ java::util::TimeZone /**/ = TimeZones::get_time_zone(GMT_ID);

	static JAVA_25: bool = SystemUtils::is_java_version_at_least(JavaVersion::JAVA_25);

	pub fn get_time_zone(&self, id: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::TimeZone /**/ {
		return TimeZone::getTimeZone( if self.JAVA_25 && org::apache::commons::lang3::time::time_zones::TimeZones::map_shorti_ds() { ZoneId::SHORT_IDS.getOrDefault(id, id) } else { id });
	}

	fn map_shorti_ds(&self) -> bool {
		return SystemProperties::get_boolean(TimeZones.class, "mapShortIDs", |()|true);
	}

	pub fn to_time_zone(&self, time_zone: &/* Java */ java::util::TimeZone /**/) -> /* Java */ java::util::TimeZone /**/ {
		return ObjectUtils::get_if_null(time_zone, TimeZone::getDefault);
	}

	fn new() -> org::apache::commons::lang3::time::time_zones::TimeZones {
	}
}