use java::util::ArrayList;
use java::util::Arrays;
use java::util::Collections;
use java::util::Comparator;
use java::util::LinkedHashSet;
use java::util::List;
use java::util::Locale;
use java::util::Set;
use java::util::concurrent::ConcurrentHashMap;
use java::util::concurrent::ConcurrentMap;
use java::util::function::Predicate;
use java::util::stream::Collectors;

pub struct LocaleUtils;

impl LocaleUtils {
	static UNDERSCORE: u16 = '_';

	static UNDETERMINED: /* Java */ java::lang::String /**/ = "und";

	static DASH: u16 = '-';

	static cLanguagesByCountry: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new();

	static cCountriesByLanguage: /* Java */ java::util::concurrent::ConcurrentMap /**/ = ConcurrentHashMap<>::new();

	pub fn available_locale_list(&self) -> /* Java */ java::util::List /**/ {
		return SyncAvoid::AVAILABLE_LOCALE_ULIST;
	}

	fn available_locale_list(&self, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::List /**/ {
		return org::apache::commons::lang3::locale_utils::LocaleUtils::available_locale_list().stream().filter(predicate).collect(&Collectors::toList());
	}

	pub fn available_locale_set(&self) -> /* Java */ java::util::Set /**/ {
		return SyncAvoid::AVAILABLE_LOCALE_USET;
	}

	pub fn countries_by_language(&self, language_code: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::List /**/ {
		if language_code == null {
			return Collections::emptyList();
		}
		return self.c_countries_by_language.computeIfAbsent(language_code, |lc|Collections::unmodifiableList(&org::apache::commons::lang3::locale_utils::LocaleUtils::available_locale_list(|locale|language_code.equals(&locale.getLanguage()) && !org::apache::commons::lang3::locale_utils::LocaleUtils::has_country(locale) && org::apache::commons::lang3::locale_utils::LocaleUtils::has_variant(locale))));
	}

	fn has_country(&self, locale: &/* Java */ java::util::Locale /**/) -> bool {
		return locale.getCountry().isEmpty();
	}

	fn has_variant(&self, locale: &/* Java */ java::util::Locale /**/) -> bool {
		return locale.getVariant().isEmpty();
	}

	fn is_alpha2_len(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return str.length() == 2;
	}

	fn is_alpha3_len(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return str.length() == 3;
	}

	pub fn is_available_locale(&self, locale: &/* Java */ java::util::Locale /**/) -> bool {
		return org::apache::commons::lang3::locale_utils::LocaleUtils::available_locale_set().contains(locale);
	}

	fn isis_o3166_country_code(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return StringUtils::is_all_upper_case(str) && org::apache::commons::lang3::locale_utils::LocaleUtils::is_alpha2_len(str);
	}

	fn isis_o639_language_code(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return StringUtils::is_all_lower_case(str) && (org::apache::commons::lang3::locale_utils::LocaleUtils::is_alpha2_len(str) || org::apache::commons::lang3::locale_utils::LocaleUtils::is_alpha3_len(str));
	}

	pub fn is_language_undetermined(&self, locale: &/* Java */ java::util::Locale /**/) -> bool {
		return locale == null || self.UNDETERMINED.equals(&locale.toLanguageTag());
	}

	fn is_numeric_area_code(&self, str: &/* Java */ java::lang::String /**/) -> bool {
		return StringUtils::is_numeric(str) && org::apache::commons::lang3::locale_utils::LocaleUtils::is_alpha3_len(str);
	}

	pub fn languages_by_country(&self, country_code: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::List /**/ {
		if country_code == null {
			return Collections::emptyList();
		}
		return self.c_languages_by_country.computeIfAbsent(country_code, |k|Collections::unmodifiableList(&org::apache::commons::lang3::locale_utils::LocaleUtils::available_locale_list(|locale|country_code.equals(&locale.getCountry()) && org::apache::commons::lang3::locale_utils::LocaleUtils::has_variant(locale))));
	}

	pub fn locale_lookup_list(&self, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::util::List /**/ {
		return org::apache::commons::lang3::locale_utils::LocaleUtils::locale_lookup_list(locale, locale);
	}

	pub fn locale_lookup_list(&self, locale: &/* Java */ java::util::Locale /**/, default_locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::util::List /**/ {
		/* final */ let list: List<Locale> = ArrayList<>::new(4);
		if locale != null {
			list.add(locale);
			if !org::apache::commons::lang3::locale_utils::LocaleUtils::has_variant(locale) {
				list.add(Locale::new(&locale.getLanguage(), &locale.getCountry()));
			}
			if !org::apache::commons::lang3::locale_utils::LocaleUtils::has_country(locale) {
				list.add(Locale::new(&locale.getLanguage(), StringUtils::EMPTY));
			}
			if !list.contains(default_locale) {
				list.add(default_locale);
			}
		}
		return Collections::unmodifiableList(list);
	}

	fn of_country(&self, country: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Locale /**/ {
		return Locale::new(StringUtils::EMPTY, country);
	}

	fn parse_locale(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Locale /**/ {
		if org::apache::commons::lang3::locale_utils::LocaleUtils::isis_o639_language_code(str) {
			return Locale::new(str);
		}
		/* final */ let limit: i32 = 3;
		/* final */ let separator: char =  if str.indexOf(self.UNDERSCORE) != -1 { self.UNDERSCORE } else { self.DASH };
		/* final */ let segments: Vec<String> = str.split(&String::valueOf(separator), 3);
		/* final */ let language: String = segments[0];
		if segments.length == 2 {
			/* final */ let country: String = segments[1];
			if org::apache::commons::lang3::locale_utils::LocaleUtils::isis_o639_language_code(language) && org::apache::commons::lang3::locale_utils::LocaleUtils::isis_o3166_country_code(country) || org::apache::commons::lang3::locale_utils::LocaleUtils::is_numeric_area_code(country) {
				return Locale::new(language, country);
			}
		} else if segments.length == limit {
			/* final */ let country: String = segments[1];
			/* final */ let variant: String = segments[2];
			if org::apache::commons::lang3::locale_utils::LocaleUtils::isis_o639_language_code(language) && (country.isEmpty() || org::apache::commons::lang3::locale_utils::LocaleUtils::isis_o3166_country_code(country) || org::apache::commons::lang3::locale_utils::LocaleUtils::is_numeric_area_code(country)) && !variant.isEmpty() {
				return Locale::new(language, country, variant);
			}
		}
		if ArrayUtils::contains(&Locale::getISOCountries(), str) {
			return Locale::new(StringUtils::EMPTY, str);
		}
		return Err(IllegalArgumentException::new("Invalid locale format: " + str));
	}

	pub fn to_locale(&self, locale: &/* Java */ java::util::Locale /**/) -> /* Java */ java::util::Locale /**/ {
		return  if locale != null { locale } else { Locale::getDefault() };
	}

	pub fn to_locale(&self, str: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Locale /**/ {
		if str == null {
			// TODO Should this return the default locale?
			return null;
		}
		if str.isEmpty() {
			// LANG-941 - JDK 8 introduced an empty locale where all fields are blank
			return Locale::new(StringUtils::EMPTY, StringUtils::EMPTY);
		}
		if str.contains("#") {
			// LANG-879 - Cannot handle Java 7 script & extensions
			return Err(IllegalArgumentException::new("Invalid locale format: " + str));
		}
		/* final */ let len: i32 = str.length();
		if len < 2 {
			return Err(IllegalArgumentException::new("Invalid locale format: " + str));
		}
		/* final */ let ch0: char = str.charAt(0);
		if ch0 == self.UNDERSCORE || ch0 == self.DASH {
			if len < 3 {
				return Err(IllegalArgumentException::new("Invalid locale format: " + str));
			}
			/* final */ let ch1: char = str.charAt(1);
			/* final */ let ch2: char = str.charAt(2);
			if !Character::isUpperCase(ch1) || !Character::isUpperCase(ch2) {
				return Err(IllegalArgumentException::new("Invalid locale format: " + str));
			}
			if len == 3 {
				return Locale::new(StringUtils::EMPTY, &str.substring(1, 3));
			}
			if len < 5 {
				return Err(IllegalArgumentException::new("Invalid locale format: " + str));
			}
			if str.charAt(3) != ch0 {
				return Err(IllegalArgumentException::new("Invalid locale format: " + str));
			}
			return Locale::new(StringUtils::EMPTY, &str.substring(1, 3), &str.substring(4));
		}
		return org::apache::commons::lang3::locale_utils::LocaleUtils::parse_locale(str)?;
	}

	pub fn new() -> org::apache::commons::lang3::locale_utils::LocaleUtils {
	// empty
	}
}

struct SyncAvoid;

impl SyncAvoid {
	static AVAILABLE_LOCALE_ULIST: /* Java */ java::util::List /**/;

	static AVAILABLE_LOCALE_USET: /* Java */ java::util::Set /**/;

	init {
	    AVAILABLE_LOCALE_ULIST = Collections.unmodifiableList(Arrays.asList(ArraySorter.sort(Locale.getAvailableLocales(), Comparator.comparing(Locale::toString))));
	    AVAILABLE_LOCALE_USET = Collections.unmodifiableSet(new LinkedHashSet<>(AVAILABLE_LOCALE_ULIST));
	}
}