use java::text::DateFormat;
use java::text::Format;
use java::text::SimpleDateFormat;
use java::util::Arrays;
use java::util::Locale;
use java::util::Objects;
use java::util::TimeZone;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ConcurrentMap;
use crate::org::apache::commons::lang3::LocaleUtils;

struct AbstractFormatCache<F: /* Java */ java::text::Format /**/> {
	instance_cache: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new(7),
}

impl<F: /* Java */ java::text::Format /**/> AbstractFormatCache {
	static NONE: i32 = -1;

	static dateTimeInstanceCache: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new(7);

	fn clear(&self) {
		self.date_time_instance_cache.clear();
	}

	fn get_pattern_for_style(&self, date_style: &/* Java */ java::lang::Integer /**/, time_style: &/* Java */ java::lang::Integer /**/, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::lang::String /**/ {
		/* final */ let safe_locale: Locale = LocaleUtils::to_locale(locale);
		/* final */ let key: ArrayKey = ArrayKey::new(date_style, time_style, safe_locale);
		return self.date_time_instance_cache.computeIfAbsent(key, |k|{
			let r0 = 'try0: {
				/* final */ let formatter: DateFormat;
				if date_style == null {
					formatter = DateFormat::getTimeInstance(&time_style.intValue(), safe_locale);
				} else if time_style == null {
					formatter = DateFormat::getDateInstance(&date_style.intValue(), safe_locale);
				} else {
					formatter = DateFormat::getDateTimeInstance(&date_style.intValue(), &time_style.intValue(), safe_locale);
				}
				return (formatter as SimpleDateFormat).toPattern();
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ ClassCastException) => {
					break 'try0 Err(IllegalArgumentException::new("No date time pattern for locale: " + safe_locale));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		});
	}

	fn clear_instance(&self) {
		self.instance_cache.clear();
	}

	fn create_instance(&self, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> F ;

	fn get_date_instance(&self, date_style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> F {
		return self.get_date_time_instance(&Integer::valueOf(date_style), null, time_zone, locale);
	}

	fn get_date_time_instance(&self, date_style: i32, time_style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> F {
		return self.get_date_time_instance(&Integer::valueOf(date_style), &Integer::valueOf(time_style), time_zone, locale);
	}

	fn get_date_time_instance(&self, date_style: &/* Java */ java::lang::Integer /**/, time_style: &/* Java */ java::lang::Integer /**/, time_zone: &/* Java */ java::util::TimeZone /**/, mut locale: &/* Java */ java::util::Locale /**/) -> F {
		locale = LocaleUtils::to_locale(locale);
		return self.get_instance(&org::apache::commons::lang3::time::abstract_format_cache::AbstractFormatCache::get_pattern_for_style(date_style, time_style, locale), time_zone, locale);
	}

	pub fn get_instance(&self) -> F {
		return self.get_date_time_instance(DateFormat::SHORT, DateFormat::SHORT, &TimeZone::getDefault(), &Locale::getDefault());
	}

	pub fn get_instance(&self, pattern: &/* Java */ java::lang::String /**/, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> F {
		Objects::requireNonNull(pattern, "pattern");
		/* final */ let actual_time_zone: TimeZone = TimeZones::to_time_zone(time_zone);
		/* final */ let actual_locale: Locale = LocaleUtils::to_locale(locale);
		/* final */ let key: ArrayKey = ArrayKey::new(pattern, actual_time_zone, actual_locale);
		return self.instance_cache.computeIfAbsent(key, |k|self.create_instance(pattern, actual_time_zone, actual_locale));
	}

	fn get_time_instance(&self, time_style: i32, time_zone: &/* Java */ java::util::TimeZone /**/, locale: &/* Java */ java::util::Locale /**/) -> F {
		return self.get_date_time_instance(null, &Integer::valueOf(time_style), time_zone, locale);
	}
}

struct ArrayKey {
	keys: &[/* Java */ java::lang::Object /**/],
	hash_code: i32,
}

impl ArrayKey {
	fn new(keys: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::time::abstract_format_cache::ArrayKey {
		self.keys = keys;
		self.hashCode = Objects::hash(keys);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if obj == null {
			return false;
		}
		if self.getClass() != obj.getClass() {
			return false;
		}
		/* final */ let other: ArrayKey = obj as ArrayKey;
		return Arrays::deepEquals(self.keys, other.keys);
	}

	pub fn hash_code(&self) -> i32 {
		return self.hash_code;
	}
}