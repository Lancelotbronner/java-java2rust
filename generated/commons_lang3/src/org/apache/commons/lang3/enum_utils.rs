use java::util::ArrayList;
use java::util::Arrays;
use java::util::Collections;
use java::util::EnumSet;
use java::util::List;
use java::util::Map;
use java::util::Objects;
use java::util::function::Function;
use java::util::function::ToIntFunction;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct EnumUtils;

impl EnumUtils {
	static CANNOT_STORE_S_S_VALUES_IN_S_BITS: /* Java */ java::lang::String /**/ = "Cannot store %s %s values in %s bits";

	static ENUM_CLASS_MUST_BE_DEFINED: /* Java */ java::lang::String /**/ = "EnumClass must be defined.";

	static NULL_ELEMENTS_NOT_PERMITTED: /* Java */ java::lang::String /**/ = "null elements not permitted";

	static S_DOES_NOT_SEEM_TO_BE_AN_ENUM_TYPE: /* Java */ java::lang::String /**/ = "%s does not seem to be an Enum type";

	fn as_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Class /**/ {
		Objects::requireNonNull(enum_class, self.ENUM_CLASS_MUST_BE_DEFINED);
		Validate::is_true(&enum_class.isEnum(), self.S_DOES_NOT_SEEM_TO_BE_AN_ENUM_TYPE, enum_class)?;
		return enum_class;
	}

	fn check_bit_vectorable<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Class /**/ {
		/* final */ let constants: Vec<E> = org::apache::commons::lang3::enum_utils::EnumUtils::as_enum(enum_class)?.getEnumConstants();
		Validate::is_true(constants.length <= Long::SIZE, self.CANNOT_STORE_S_S_VALUES_IN_S_BITS, &Integer::valueOf(constants.length), &enum_class.getSimpleName(), &Integer::valueOf(Long::SIZE))?;
		return enum_class;
	}

	pub fn generate_bit_vector<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, values: &E) -> i64 {
		Validate::no_null_elements(values);
		return org::apache::commons::lang3::enum_utils::EnumUtils::generate_bit_vector(enum_class, &Arrays::asList(values));
	}

	pub fn generate_bit_vector<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, values: &/* Java */ java::lang::Iterable /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i64 {
		org::apache::commons::lang3::enum_utils::EnumUtils::check_bit_vectorable(enum_class)?;
		Objects::requireNonNull(values, "values");
		let total: i64 = 0;
		for /* final */ constant in values {
			Objects::requireNonNull(constant, self.NULL_ELEMENTS_NOT_PERMITTED);
			total |= 1 << constant.ordinal();
		}
		return total;
	}

	pub fn generate_bit_vectors<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, values: &E) /* thrown(java.lang.IllegalArgumentException) */ -> &[i64] {
		org::apache::commons::lang3::enum_utils::EnumUtils::as_enum(enum_class)?;
		Validate::no_null_elements(values);
		/* final */ let condensed: EnumSet<E> = EnumSet::noneOf(enum_class);
		Collections::addAll(condensed, values);
		/* final */ let result: [i64; (enum_class.getEnumConstants().length - 1) / Long::SIZE + 1] = [0; (enum_class.getEnumConstants().length - 1) / Long::SIZE + 1];
		for /* final */ value in condensed {
			result[value.ordinal() / Long::SIZE] |= 1 << value.ordinal() % Long::SIZE;
		}
		ArrayUtils::reverse(result);
		return result;
	}

	pub fn generate_bit_vectors<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, values: &/* Java */ java::lang::Iterable /**/) /* thrown(java.lang.IllegalArgumentException) */ -> &[i64] {
		org::apache::commons::lang3::enum_utils::EnumUtils::as_enum(enum_class)?;
		Objects::requireNonNull(values, "values");
		/* final */ let condensed: EnumSet<E> = EnumSet::noneOf(enum_class);
		values.forEach(|constant|condensed.add(&Objects::requireNonNull(constant, self.NULL_ELEMENTS_NOT_PERMITTED)));
		/* final */ let result: [i64; (enum_class.getEnumConstants().length - 1) / Long::SIZE + 1] = [0; (enum_class.getEnumConstants().length - 1) / Long::SIZE + 1];
		for /* final */ value in condensed {
			result[value.ordinal() / Long::SIZE] |= 1 << value.ordinal() % Long::SIZE;
		}
		ArrayUtils.reverse(result);
		return result;
	}

	pub fn get_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/) -> E {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum(enum_class, enum_name, null);
	}

	pub fn get_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/, default_enum: &E) -> E {
		if enum_class == null || enum_name == null {
			return default_enum;
		}
		let r0 = 'try0: {
			return Enum::valueOf(enum_class, enum_name);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalArgumentException) => {
				return default_enum;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_enum_ignore_case<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/) -> E {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum_ignore_case(enum_class, enum_name, null);
	}

	pub fn get_enum_ignore_case<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/, default_enum: &E) -> E {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_first_enum_ignore_case(enum_class, enum_name, Enum::name, default_enum);
	}

	pub fn get_enum_list<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		return ArrayList<>::new(&Arrays::asList(&enum_class.getEnumConstants()));
	}

	pub fn get_enum_map<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::Map /**/ {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum_map(enum_class, E::name);
	}

	pub fn get_enum_map<E: /* Java */ java::lang::Enum /**/, K>(&self, enum_class: &/* Java */ java::lang::Class /**/, key_function: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::Map /**/ {
		return org::apache::commons::lang3::enum_utils::EnumUtils::stream(enum_class).collect(&Collectors::toMap(keyFunction::apply, &Function::identity()));
	}

	pub fn get_enum_system_property<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, prop_name: &/* Java */ java::lang::String /**/, default_enum: &E) -> E {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum(enum_class, &SystemProperties::get_property(prop_name), default_enum);
	}

	pub fn get_first_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, value: i32, to_int_function: &/* Java */ java::util::function::ToIntFunction /**/, default_enum: &E) -> E {
		if !org::apache::commons::lang3::enum_utils::EnumUtils::is_enum(enum_class) {
			return default_enum;
		}
		return org::apache::commons::lang3::enum_utils::EnumUtils::stream(enum_class).filter(|e|value == to_int_function.applyAsInt(e)).findFirst().orElse(default_enum);
	}

	pub fn get_first_enum_ignore_case<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/, string_function: &/* Java */ java::util::function::Function /**/, default_enum: &E) -> E {
		if enum_name == null {
			return default_enum;
		}
		return org::apache::commons::lang3::enum_utils::EnumUtils::stream(enum_class).filter(|e|enum_name.equalsIgnoreCase(&string_function.apply(e))).findFirst().orElse(default_enum);
	}

	fn is_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/) -> bool {
		return enum_class != null && enum_class.isEnum();
	}

	pub fn is_valid_enum<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum(enum_class, enum_name) != null;
	}

	pub fn is_valid_enum_ignore_case<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, enum_name: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::enum_utils::EnumUtils::get_enum_ignore_case(enum_class, enum_name) != null;
	}

	pub fn process_bit_vector<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, value: i64) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::EnumSet /**/ {
		return org::apache::commons::lang3::enum_utils::EnumUtils::process_bit_vectors(&org::apache::commons::lang3::enum_utils::EnumUtils::check_bit_vectorable(enum_class)?, value);
	}

	pub fn process_bit_vectors<E: /* Java */ java::lang::Enum /**/>(&self, enum_class: &/* Java */ java::lang::Class /**/, values: i64) -> /* Java */ java::util::EnumSet /**/ {
		/* final */ let results: EnumSet<E> = EnumSet::noneOf(&org::apache::commons::lang3::enum_utils::EnumUtils::as_enum(enum_class)?);
		/* final */ let lvalues: Vec<i64> = ArrayUtils::clone(&Objects::requireNonNull(values, "values"));
		ArrayUtils::reverse(lvalues);
		org::apache::commons::lang3::enum_utils::EnumUtils::stream(enum_class).forEach(|constant|{
			/* final */ let block: i32 = constant.ordinal() / Long::SIZE;
			if block < lvalues.length && (lvalues[block] & 1 << constant.ordinal() % Long::SIZE) != 0 {
				results.add(constant);
			}
		});
		return results;
	}

	pub fn stream<T>(&self, clazz: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return  if clazz != null { Streams::of(&clazz.getEnumConstants()) } else { Stream::empty() };
	}

	pub fn new() -> org::apache::commons::lang3::enum_utils::EnumUtils {
	// empty
	}
}