use java::util::Map;
use java::util::Objects;
use crate::org::apache::commons::lang3::SystemProperties;

pub struct StrLookup<V>;

impl<V> StrLookup {
	static NONE_LOOKUP: org::apache::commons::lang3::text::str_lookup::StrLookup = MapStrLookup<>::new(null);

	static SYSTEM_PROPERTIES_LOOKUP: org::apache::commons::lang3::text::str_lookup::StrLookup = SystemPropertiesStrLookup::new();

	pub fn map_lookup<V>(&self, map: &/* Java */ java::util::Map /**/) -> org::apache::commons::lang3::text::str_lookup::StrLookup {
		return MapStrLookup<>::new(map);
	}

	pub fn none_lookup(&self) -> org::apache::commons::lang3::text::str_lookup::StrLookup {
		return self.NONE_LOOKUP;
	}

	pub fn system_properties_lookup(&self) -> org::apache::commons::lang3::text::str_lookup::StrLookup {
		return self.SYSTEM_PROPERTIES_LOOKUP;
	}

	fn new() -> org::apache::commons::lang3::text::str_lookup::StrLookup {
	}

	pub fn lookup(&self, key: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ ;
}

struct MapStrLookup<V> {
	map: /* Java */ java::util::Map /**/,
}

impl<V> MapStrLookup {
	fn new(map: &/* Java */ java::util::Map /**/) -> org::apache::commons::lang3::text::str_lookup::MapStrLookup {
		self.map = map;
	}

	pub fn lookup(&self, key: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if self.map == null {
			return null;
		}
		return Objects::toString(&self.map.get(key), null);
	}
}

struct SystemPropertiesStrLookup;

impl SystemPropertiesStrLookup {
	pub fn lookup(&self, key: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return SystemProperties::get_property(key);
	}
}