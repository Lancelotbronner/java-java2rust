use java::lang::reflect::Array;
use java::lang::reflect::Field;
use java::lang::reflect::Method;
use java::lang::reflect::Type;
use java::security::SecureRandom;
use java::util::Arrays;
use java::util::BitSet;
use java::util::Comparator;
use java::util::Date;
use java::util::HashMap;
use java::util::Map;
use java::util::Objects;
use java::util::Random;
use java::util::concurrent::ThreadLocalRandom;
use java::util::function::Function;
use java::util::function::IntFunction;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::builder::EqualsBuilder;
use crate::org::apache::commons::lang3::builder::HashCodeBuilder;
use crate::org::apache::commons::lang3::builder::ToStringBuilder;
use crate::org::apache::commons::lang3::builder::ToStringStyle;
use crate::org::apache::commons::lang3::function::FailableFunction;
use crate::org::apache::commons::lang3::mutable::MutableInt;
use crate::org::apache::commons::lang3::stream::IntStreams;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct ArrayUtils;

impl ArrayUtils {
	pub static EMPTY_BOOLEAN_ARRAY: &[bool] = ;

	pub static EMPTY_BOOLEAN_OBJECT_ARRAY: &[/* Java */ java::lang::Boolean /**/] = ;

	pub static EMPTY_BYTE_ARRAY: &[i8] = ;

	pub static EMPTY_BYTE_OBJECT_ARRAY: &[/* Java */ java::lang::Byte /**/] = ;

	pub static EMPTY_CHAR_ARRAY: &[u16] = ;

	pub static EMPTY_CHARACTER_OBJECT_ARRAY: &[/* Java */ java::lang::Character /**/] = ;

	pub static EMPTY_CLASS_ARRAY: &[/* Java */ java::lang::Class /**/] = ;

	pub static EMPTY_DOUBLE_ARRAY: &[f64] = ;

	pub static EMPTY_DOUBLE_OBJECT_ARRAY: &[/* Java */ java::lang::Double /**/] = ;

	pub static EMPTY_FIELD_ARRAY: &[/* Java */ java::lang::reflect::Field /**/] = ;

	pub static EMPTY_FLOAT_ARRAY: &[f32] = ;

	pub static EMPTY_FLOAT_OBJECT_ARRAY: &[/* Java */ java::lang::Float /**/] = ;

	pub static EMPTY_INT_ARRAY: &[i32] = ;

	pub static EMPTY_INTEGER_OBJECT_ARRAY: &[/* Java */ java::lang::Integer /**/] = ;

	pub static EMPTY_LONG_ARRAY: &[i64] = ;

	pub static EMPTY_LONG_OBJECT_ARRAY: &[/* Java */ java::lang::Long /**/] = ;

	pub static EMPTY_METHOD_ARRAY: &[/* Java */ java::lang::reflect::Method /**/] = ;

	pub static EMPTY_OBJECT_ARRAY: &[/* Java */ java::lang::Object /**/] = ;

	pub static EMPTY_SHORT_ARRAY: &[i16] = ;

	pub static EMPTY_SHORT_OBJECT_ARRAY: &[/* Java */ java::lang::Short /**/] = ;

	pub static EMPTY_STRING_ARRAY: &[/* Java */ java::lang::String /**/] = ;

	pub static EMPTY_THROWABLE_ARRAY: &[/* Java */ java::lang::Throwable /**/] = ;

	pub static EMPTY_TYPE_ARRAY: &[/* Java */ java::lang::reflect::Type /**/] = ;

	pub static INDEX_NOT_FOUND: i32 = -1;

	pub static SOFT_MAX_ARRAY_LENGTH: i32 = Integer::MAX_VALUE - 8;

	pub fn add(&self, array: &&[bool], element: bool) -> &[bool] {
		/* final */ let new_array: Vec<bool> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Boolean::TYPE) as Vec<bool>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[bool], index: i32, element: bool) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Boolean::valueOf(element), Boolean::TYPE)? as Vec<bool>;
	}

	pub fn add(&self, array: &&[i8], element: i8) -> &[i8] {
		/* final */ let new_array: Vec<i8> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Byte::TYPE) as Vec<i8>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[i8], index: i32, element: i8) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i8] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Byte::valueOf(element), Byte::TYPE)? as Vec<i8>;
	}

	pub fn add(&self, array: &&[u16], element: u16) -> &[u16] {
		/* final */ let new_array: Vec<char> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Character::TYPE) as Vec<char>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[u16], index: i32, element: u16) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[u16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Character::valueOf(element), Character::TYPE)? as Vec<char>;
	}

	pub fn add(&self, array: &&[f64], element: f64) -> &[f64] {
		/* final */ let new_array: Vec<f64> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Double::TYPE) as Vec<f64>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[f64], index: i32, element: f64) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Double::valueOf(element), Double::TYPE)? as Vec<f64>;
	}

	pub fn add(&self, array: &&[f32], element: f32) -> &[f32] {
		/* final */ let new_array: Vec<f32> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Float::TYPE) as Vec<f32>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[f32], index: i32, element: f32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Float::valueOf(element), Float::TYPE)? as Vec<f32>;
	}

	pub fn add(&self, array: &&[i32], element: i32) -> &[i32] {
		/* final */ let new_array: Vec<i32> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Integer::TYPE) as Vec<i32>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add(&self, array: &&[i32], index: i32, element: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Integer::valueOf(element), Integer::TYPE)? as Vec<i32>;
	}

	pub fn add(&self, array: &&[i64], index: i32, element: i64) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Long::valueOf(element), Long::TYPE)? as Vec<i64>;
	}

	pub fn add(&self, array: &&[i64], element: i64) -> &[i64] {
		/* final */ let new_array: Vec<i64> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Long::TYPE) as Vec<i64>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	fn add(&self, array: &/* Java */ java::lang::Object /**/, index: i32, element: &/* Java */ java::lang::Object /**/, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IndexOutOfBoundsException) */ -> /* Java */ java::lang::Object /**/ {
		if array == null {
			if index != 0 {
				return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: 0"));
			}
			/* final */ let joined_array: Object = Array::newInstance(clazz, 1);
			Array::set(joined_array, 0, element);
			return joined_array;
		}
		/* final */ let length: i32 = Array::getLength(array);
		if index > length || index < 0 {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + length));
		}
		/* final */ let result: Object = org::apache::commons::lang3::array_utils::ArrayUtils::arraycopy(array, 0, 0, index, |()|Array::newInstance(clazz, length + 1));
		Array::set(result, index, element);
		if index < length {
			System::arraycopy(array, index, result, index + 1, length - index);
		}
		return result;
	}

	pub fn add(&self, array: &&[i16], index: i32, element: i16) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, &Short::valueOf(element), Short::TYPE)? as Vec<i16>;
	}

	pub fn add(&self, array: &&[i16], element: i16) -> &[i16] {
		/* final */ let new_array: Vec<i16> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, Short::TYPE) as Vec<i16>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add<T>(&self, array: &&[T], index: i32, element: &T) /* thrown(java.lang.IndexOutOfBoundsException | java.lang.IllegalArgumentException) */ -> &[T] {
		/* final */ let clazz: Class<T>;
		if array != null {
			clazz = org::apache::commons::lang3::array_utils::ArrayUtils::get_component_type(array);
		} else if element != null {
			clazz = ObjectUtils::get_class(element);
		} else {
			return Err(IllegalArgumentException::new("Array and element cannot both be null"));
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::add(array, index, element, clazz)? as Vec<T>;
	}

	pub fn add<T>(&self, array: &&[T], element: &T) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		/* final */ let type: Class<?>;
		if array != null {
			type = array.getClass().getComponentType();
		} else if element != null {
			type = element.getClass();
		} else {
			return Err(IllegalArgumentException::new("Arguments cannot both be null"));
		}
		/* final */ let new_array: Vec<T> = org::apache::commons::lang3::array_utils::ArrayUtils::copy_array_grow1(array, type) as Vec<T>;
		new_array[new_array.length - 1] = element;
		return new_array;
	}

	pub fn add_all(&self, array1: &&[bool], array2: bool) -> &[bool] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [bool; array1.length + array2.length] = [false; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[i8], array2: i8) -> &[i8] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [i8; array1.length + array2.length] = [0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[u16], array2: u16) -> &[u16] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return .clone(array1);
		}
		/* final */ let joined_array: [Option<char>; array1.length + array2.length] = [None; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[f64], array2: f64) -> &[f64] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [f64; array1.length + array2.length] = [0.0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[f32], array2: f32) -> &[f32] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [f32; array1.length + array2.length] = [0.0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[i32], array2: i32) -> &[i32] {
		if array1 == null {
			return .clone(array2);
		}
		if array2 == null {
			return .clone(array1);
		}
		/* final */ let joined_array: [i32; array1.length + array2.length] = [0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[i64], array2: i64) -> &[i64] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [i64; array1.length + array2.length] = [0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all(&self, array1: &&[i16], array2: i16) -> &[i16] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let joined_array: [i16; array1.length + array2.length] = [0; array1.length + array2.length];
		System::arraycopy(array1, 0, joined_array, 0, array1.length);
		System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
		return joined_array;
	}

	pub fn add_all<T>(&self, array1: &&[T], array2: &T) /* thrown(java.lang.ArrayStoreException | java.lang.IllegalArgumentException) */ -> &[T] {
		if array1 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array2);
		}
		if array2 == null {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array1);
		}
		/* final */ let type1: Class<T> = org::apache::commons::lang3::array_utils::ArrayUtils::get_component_type(array1);
		/* final */ let joined_array: Vec<T> = org::apache::commons::lang3::array_utils::ArrayUtils::arraycopy(array1, 0, 0, array1.length, |()|org::apache::commons::lang3::array_utils::ArrayUtils::new_instance(type1, array1.length + array2.length));
		let r0 = 'try0: {
			System::arraycopy(array2, 0, joined_array, array1.length, array2.length);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ArrayStoreException) => {
				// Check if problem was due to incompatible types
				/* 
	             * We do this here, rather than before the copy because: - it would be a wasted check most of the time - safer, in case check turns out to be too
	             * strict
	             */ 
				/* final */ let type2: Class<?> = array2.getClass().getComponentType();
				if !type1.isAssignableFrom(type2) {
					break 'try0 Err(IllegalArgumentException::new("Cannot store " + type2.getName() + " in an array of " + type1.getName(), ase));
				}
				// No, so rethrow original
				return Err(ase);
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return joined_array;
	}

	pub fn add_first(&self, array: &&[bool], element: bool) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { org::apache::commons::lang3::array_utils::ArrayUtils::insert(0, array, element)? };
	}

	pub fn add_first(&self, array: &&[i8], element: i8) -> &[i8] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first(&self, array: &&[u16], element: u16) -> &[u16] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first(&self, array: &&[f64], element: f64) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { org::apache::commons::lang3::array_utils::ArrayUtils::insert(0, array, element)? };
	}

	pub fn add_first(&self, array: &&[f32], element: f32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first(&self, array: &&[i32], element: i32) -> &[i32] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first(&self, array: &&[i64], element: i64) -> &[i64] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first(&self, array: &&[i16], element: i16) -> &[i16] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element) } else { .insert(0, array, element) };
	}

	pub fn add_first<T>(&self, array: &&[T], element: &T) /* thrown(java.lang.IndexOutOfBoundsException | java.lang.IllegalArgumentException) */ -> &[T] {
		return  if array == null { org::apache::commons::lang3::array_utils::ArrayUtils::add(array, element)? } else { org::apache::commons::lang3::array_utils::ArrayUtils::insert(0, array, element)? };
	}

	pub fn arraycopy<T>(&self, source: &T, source_pos: i32, dest_pos: i32, length: i32, allocator: &/* Java */ java::util::function::Function /**/) -> T {
		return org::apache::commons::lang3::array_utils::ArrayUtils::arraycopy(source, source_pos, &allocator.apply(length), dest_pos, length);
	}

	pub fn arraycopy<T>(&self, source: &T, source_pos: i32, dest_pos: i32, length: i32, allocator: &/* Java */ java::util::function::Supplier /**/) -> T {
		return org::apache::commons::lang3::array_utils::ArrayUtils::arraycopy(source, source_pos, &allocator.get(), dest_pos, length);
	}

	pub fn arraycopy<T>(&self, source: &T, source_pos: i32, dest: &T, dest_pos: i32, length: i32) -> T {
		System::arraycopy(source, source_pos, dest, dest_pos, length);
		return dest;
	}

	pub fn clone(&self, array: &&[bool]) -> &[bool] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[i8]) -> &[i8] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[u16]) -> &[u16] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[f64]) -> &[f64] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[f32]) -> &[f32] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[i32]) -> &[i32] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[i64]) -> &[i64] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone(&self, array: &&[i16]) -> &[i16] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn clone<T>(&self, array: &&[T]) -> &[T] {
		return  if array != null { array.clone() } else { null };
	}

	pub fn contains(&self, array: &&[bool], value_to_find: bool) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[i8], value_to_find: i8) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[u16], value_to_find: u16) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[f64], value_to_find: f64) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[f64], value_to_find: f64, tolerance: f64) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0, tolerance) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[f32], value_to_find: f32) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[i32], value_to_find: i32) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[i64], value_to_find: i64) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, object_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains(&self, array: &&[i16], value_to_find: i16) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find) != self.INDEX_NOT_FOUND;
	}

	pub fn contains_any(&self, array: &&[i32], objects_to_find: i32) -> bool {
		return IntStreams::of(objects_to_find).anyMatch(|e|org::apache::commons::lang3::array_utils::ArrayUtils::contains(array, e));
	}

	pub fn contains_any(&self, array: &&[/* Java */ java::lang::Object /**/], objects_to_find: &/* Java */ java::lang::Object /**/) -> bool {
		return Streams::of(objects_to_find).anyMatch(|e|org::apache::commons::lang3::array_utils::ArrayUtils::contains(array, e));
	}

	fn copy_array_grow1(&self, array: &/* Java */ java::lang::Object /**/, new_array_component_type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::Object /**/ {
		if array != null {
			/* final */ let array_length: i32 = Array::getLength(array);
			/* final */ let new_array: Object = Array::newInstance(&array.getClass().getComponentType(), array_length + 1);
			System::arraycopy(array, 0, new_array, 0, array_length);
			return new_array;
		}
		return Array::newInstance(new_array_component_type, 1);
	}

	pub fn get<T>(&self, array: &&[T], index: i32) -> T {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get(array, index, null);
	}

	pub fn get<T>(&self, array: &&[T], index: i32, default_value: &T) -> T {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_array_index_valid(array, index) { array[index] } else { default_value };
	}

	pub fn get_component_type<T>(&self, array: &&[T]) -> /* Java */ java::lang::Class /**/ {
		return ClassUtils::get_component_type(&ObjectUtils::get_class(array));
	}

	pub fn get_length(&self, array: &/* Java */ java::lang::Object /**/) -> i32 {
		return  if array != null { Array::getLength(array) } else { 0 };
	}

	pub fn hash_code(&self, array: &/* Java */ java::lang::Object /**/) -> i32 {
		return HashCodeBuilder::new().append(array).to_hash_code();
	}

	fn increment<K>(&self, occurrences: &/* Java */ java::util::Map /**/, boxed: &K) {
		occurrences.computeIfAbsent(boxed, |k|MutableInt::new()).increment();
	}

	pub fn indexes_of(&self, array: &&[bool], value_to_find: bool) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[bool], value_to_find: bool, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[i8], value_to_find: i8) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[i8], value_to_find: i8, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[u16], value_to_find: u16) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[u16], value_to_find: u16, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[f64], value_to_find: f64) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[f64], value_to_find: f64, tolerance: f64) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0, tolerance);
	}

	pub fn indexes_of(&self, array: &&[f64], value_to_find: f64, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[f64], value_to_find: f64, mut start_index: i32, tolerance: f64) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index, tolerance);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[f32], value_to_find: f32) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[f32], value_to_find: f32, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[i32], value_to_find: i32) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[i32], value_to_find: i32, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[i64], value_to_find: i64) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[i64], value_to_find: i64, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, object_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, object_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn indexes_of(&self, array: &&[i16], value_to_find: i16) -> /* Java */ java::util::BitSet /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, value_to_find, 0);
	}

	pub fn indexes_of(&self, array: &&[i16], value_to_find: i16, mut start_index: i32) -> /* Java */ java::util::BitSet /**/ {
		/* final */ let bit_set: BitSet = BitSet::new();
		if array == null {
			return bit_set;
		}
		while start_index < array.length {
			start_index = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, start_index);
			if start_index == self.INDEX_NOT_FOUND {
				break;
			}
			bit_set.set(start_index);
			start_index += 1;
		}
		return bit_set;
	}

	pub fn index_of(&self, array: &&[bool], value_to_find: bool) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[bool], value_to_find: bool, start_index: i32) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[i8], value_to_find: i8) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[i8], value_to_find: i8, start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[u16], value_to_find: u16) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[u16], value_to_find: u16, start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[f64], value_to_find: f64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[f64], value_to_find: f64, tolerance: f64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0, tolerance);
	}

	pub fn index_of(&self, array: &&[f64], value_to_find: f64, start_index: i32) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) {
			return self.INDEX_NOT_FOUND;
		}
		/* final */ let search_na_n: bool = Double::isNaN(value_to_find);
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					/* final */ let element: f64 = array[i];
					if value_to_find == element || search_na_n && Double::isNaN(element) {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[f64], value_to_find: f64, start_index: i32, tolerance: f64) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) {
			return self.INDEX_NOT_FOUND;
		}
		/* final */ let min: f64 = value_to_find - tolerance;
		/* final */ let max: f64 = value_to_find + tolerance;
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if array[i] >= min && array[i] <= max {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[f32], value_to_find: f32) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[f32], value_to_find: f32, start_index: i32) -> i32 {
		if .isEmpty(array) {
			return self.INDEX_NOT_FOUND;
		}
		/* final */ let search_na_n: bool = Float::isNaN(value_to_find);
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					/* final */ let element: f32 = array[i];
					if value_to_find == element || search_na_n && Float::isNaN(element) {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[i32], value_to_find: i32) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[i32], value_to_find: i32, start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[i64], value_to_find: i64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[i64], value_to_find: i64, start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, object_to_find, 0);
	}

	pub fn index_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/, mut start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		start_index = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
		if object_to_find == null {
			 {
				let i: i32 = start_index;
				while i < array.length {
					{
						if array[i] == null {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
		} else {
			 {
				let i: i32 = start_index;
				while i < array.length {
					{
						if object_to_find.equals(array[i]) {
							return i;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return self.INDEX_NOT_FOUND;
	}

	pub fn index_of(&self, array: &&[i16], value_to_find: i16) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, value_to_find, 0);
	}

	pub fn index_of(&self, array: &&[i16], value_to_find: i16, start_index: i32) -> i32 {
		if array == null {
			return self.INDEX_NOT_FOUND;
		}
		 {
			let i: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index);
			while i < array.length {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i += 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn insert(&self, index: i32, array: &&[bool], values: bool) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		if array == null {
			return null;
		}
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [bool; array.length + values.length] = [false; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[i8], values: i8) -> &[i8] {
		if array == null {
			return null;
		}
		if .isEmpty(values) {
			return .clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [i8; array.length + values.length] = [0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[u16], values: u16) -> &[u16] {
		if array == null {
			return null;
		}
		if .isEmpty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [Option<char>; array.length + values.length] = [None; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[f64], values: f64) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		if array == null {
			return null;
		}
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [f64; array.length + values.length] = [0.0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[f32], values: f32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		if array == null {
			return null;
		}
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [f32; array.length + values.length] = [0.0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[i32], values: i32) -> &[i32] {
		if array == null {
			return null;
		}
		if .isEmpty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [i32; array.length + values.length] = [0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[i64], values: i64) -> &[i64] {
		if array == null {
			return null;
		}
		if .isEmpty(values) {
			return .clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [i64; array.length + values.length] = [0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert(&self, index: i32, array: &&[i16], values: i16) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i16] {
		if array == null {
			return null;
		}
		if .isEmpty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let result: [i16; array.length + values.length] = [0; array.length + values.length];
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	pub fn insert<T>(&self, index: i32, array: &&[T], values: &T) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		/* 
	         * Note on use of @SafeVarargs:
	         *
	         * By returning null when 'array' is null, we avoid returning the vararg
	         * array to the caller. We also avoid relying on the type of the vararg
	         * array, by inspecting the component type of 'array'.
	         */ 
		if array == null {
			return null;
		}
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		if index < 0 || index > array.length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + array.length));
		}
		/* final */ let type: Class<T> = org::apache::commons::lang3::array_utils::ArrayUtils::get_component_type(array);
		/* final */ let length: i32 = array.length + values.length;
		/* final */ let result: Vec<T> = org::apache::commons::lang3::array_utils::ArrayUtils::new_instance(type, length);
		System::arraycopy(values, 0, result, index, values.length);
		if index > 0 {
			System::arraycopy(array, 0, result, 0, index);
		}
		if index < array.length {
			System::arraycopy(array, index, result, index + values.length, array.length - index);
		}
		return result;
	}

	fn is_array_empty(&self, array: &/* Java */ java::lang::Object /**/) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) == 0;
	}

	pub fn is_array_index_valid<T>(&self, array: &&[T], index: i32) -> bool {
		return index >= 0 && org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) > index;
	}

	pub fn is_empty(&self, array: &&[bool]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[i8]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[u16]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[f64]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[f32]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[i32]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[i64]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[/* Java */ java::lang::Object /**/]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_empty(&self, array: &&[i16]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_array_empty(array);
	}

	pub fn is_equals(&self, array1: &/* Java */ java::lang::Object /**/, array2: &/* Java */ java::lang::Object /**/) -> bool {
		return EqualsBuilder::new().append(array1, array2).is_equals();
	}

	pub fn is_not_empty(&self, array: &&[bool]) -> bool {
		return !org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array);
	}

	pub fn is_not_empty(&self, array: &&[i8]) -> bool {
		return !.isEmpty(array);
	}

	pub fn is_not_empty(&self, array: &&[u16]) -> bool {
		return !.isEmpty(array);
	}

	pub fn is_not_empty(&self, array: &&[f64]) -> bool {
		return !org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array);
	}

	pub fn is_not_empty(&self, array: &&[f32]) -> bool {
		return !org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array);
	}

	pub fn is_not_empty(&self, array: &&[i32]) -> bool {
		return !.isEmpty(array);
	}

	pub fn is_not_empty(&self, array: &&[i64]) -> bool {
		return !org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array);
	}

	pub fn is_not_empty(&self, array: &&[i16]) -> bool {
		return !.isEmpty(array);
	}

	pub fn is_not_empty<T>(&self, array: &&[T]) -> bool {
		return !org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array);
	}

	pub fn is_same_length(&self, array1: &&[bool], array2: &&[bool]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[i8], array2: &&[i8]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[u16], array2: &&[u16]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[f64], array2: &&[f64]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[f32], array2: &&[f32]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[i32], array2: &&[i32]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[i64], array2: &&[i64]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &/* Java */ java::lang::Object /**/, array2: &/* Java */ java::lang::Object /**/) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[/* Java */ java::lang::Object /**/], array2: &&[/* Java */ java::lang::Object /**/]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_length(&self, array1: &&[i16], array2: &&[i16]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array1) == org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array2);
	}

	pub fn is_same_type(&self, array1: &/* Java */ java::lang::Object /**/, array2: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if array1 == null || array2 == null {
			return Err(IllegalArgumentException::new("The Array must not be null"));
		}
		return array1.getClass().getName().equals(&array2.getClass().getName());
	}

	pub fn is_sorted(&self, array: &&[bool]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: bool = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: bool = array[i];
					if BooleanUtils::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[i8]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: i8 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: i8 = array[i];
					if Byte::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[u16]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: char = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: char = array[i];
					if CharUtils::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[f64]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: f64 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: f64 = array[i];
					if Double::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[f32]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: f32 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: f32 = array[i];
					if Float::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[i32]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: i32 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: i32 = array[i];
					if Integer::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[i64]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: i64 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: i64 = array[i];
					if Long::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted(&self, array: &&[i16]) -> bool {
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: i16 = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: i16 = array[i];
					if Short::compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_sorted<T: /* Java */ java::lang::Comparable /**/>(&self, array: &&[T]) -> bool {
		return org::apache::commons::lang3::array_utils::ArrayUtils::is_sorted(array, Comparable::compareTo);
	}

	pub fn is_sorted<T>(&self, array: &&[T], comparator: &/* Java */ java::util::Comparator /**/) -> bool {
		Objects::requireNonNull(comparator, "comparator");
		if org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array) < 2 {
			return true;
		}
		let previous: T = array[0];
		/* final */ let n: i32 = array.length;
		 {
			let i: i32 = 1;
			while i < n {
				{
					/* final */ let current: T = array[i];
					if comparator.compare(previous, current) > 0 {
						return false;
					}
					previous = current;
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn last_index_of(&self, array: &&[bool], value_to_find: bool) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[bool], value_to_find: bool, mut start_index: i32) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[i8], value_to_find: i8) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[i8], value_to_find: i8, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[u16], value_to_find: u16) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[u16], value_to_find: u16, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[f64], value_to_find: f64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[f64], value_to_find: f64, tolerance: f64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE, tolerance);
	}

	pub fn last_index_of(&self, array: &&[f64], value_to_find: f64, mut start_index: i32) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[f64], value_to_find: f64, mut start_index: i32, tolerance: f64) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		/* final */ let min: f64 = value_to_find - tolerance;
		/* final */ let max: f64 = value_to_find + tolerance;
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if array[i] >= min && array[i] <= max {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[f32], value_to_find: f32) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[f32], value_to_find: f32, mut start_index: i32) -> i32 {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[i32], value_to_find: i32) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[i32], value_to_find: i32, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[i64], value_to_find: i64) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[i64], value_to_find: i64, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, object_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[/* Java */ java::lang::Object /**/], object_to_find: &/* Java */ java::lang::Object /**/, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		if object_to_find == null {
			 {
				let i: i32 = start_index;
				while i >= 0 {
					{
						if array[i] == null {
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
		} else if array.getClass().getComponentType().isInstance(object_to_find) {
			 {
				let i: i32 = start_index;
				while i >= 0 {
					{
						if object_to_find.equals(array[i]) {
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
		}
		return self.INDEX_NOT_FOUND;
	}

	pub fn last_index_of(&self, array: &&[i16], value_to_find: i16) -> i32 {
		return org::apache::commons::lang3::array_utils::ArrayUtils::last_index_of(array, value_to_find, Integer::MAX_VALUE);
	}

	pub fn last_index_of(&self, array: &&[i16], value_to_find: i16, mut start_index: i32) -> i32 {
		if array == null || start_index < 0 {
			return self.INDEX_NOT_FOUND;
		}
		if start_index >= array.length {
			start_index = array.length - 1;
		}
		 {
			let i: i32 = start_index;
			while i >= 0 {
				{
					if value_to_find == array[i] {
						return i;
					}
				}
				i -= 1;
			 }
		 }
	
		return self.INDEX_NOT_FOUND;
	}

	fn map<T, R, E: /* Java */ java::lang::Throwable /**/>(&self, array: &&[T], component_type: &/* Java */ java::lang::Class /**/, mapper: &org::apache::commons::lang3::function::failable_function::FailableFunction) /* thrown(E) */ -> &[R] {
		return ArrayFill::fill(&org::apache::commons::lang3::array_utils::ArrayUtils::new_instance(component_type, array.length), |i|mapper.apply(array[i]))?;
	}

	fn max0(&self, other: i32) -> i32 {
		return Math::max(0, other);
	}

	pub fn new_instance<T>(&self, component_type: &/* Java */ java::lang::Class /**/, length: i32) -> &[T] {
		return Array::newInstance(component_type, length) as Vec<T>;
	}

	pub fn null_to<T>(&self, array: &&[T], default_array: &&[T]) -> &[T] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { default_array } else { array };
	}

	pub fn null_to_empty(&self, array: &&[bool]) -> &[bool] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { self.EMPTY_BOOLEAN_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Boolean /**/]) -> &[/* Java */ java::lang::Boolean /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_BOOLEAN_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[i8]) -> &[i8] {
		return  if .isEmpty(array) { self.EMPTY_BYTE_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Byte /**/]) -> &[/* Java */ java::lang::Byte /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_BYTE_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[u16]) -> &[u16] {
		return  if .isEmpty(array) { self.EMPTY_CHAR_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Character /**/]) -> &[/* Java */ java::lang::Character /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_CHARACTER_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Class /**/]) -> &[/* Java */ java::lang::Class /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_CLASS_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[f64]) -> &[f64] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { self.EMPTY_DOUBLE_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Double /**/]) -> &[/* Java */ java::lang::Double /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_DOUBLE_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[f32]) -> &[f32] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { self.EMPTY_FLOAT_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Float /**/]) -> &[/* Java */ java::lang::Float /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_FLOAT_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[i32]) -> &[i32] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { self.EMPTY_INT_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Integer /**/]) -> &[/* Java */ java::lang::Integer /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_INTEGER_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[i64]) -> &[i64] {
		return  if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) { self.EMPTY_LONG_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Long /**/]) -> &[/* Java */ java::lang::Long /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_LONG_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Object /**/]) -> &[/* Java */ java::lang::Object /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[i16]) -> &[i16] {
		return  if .isEmpty(array) { self.EMPTY_SHORT_ARRAY } else { array };
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::Short /**/]) -> &[/* Java */ java::lang::Short /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_SHORT_OBJECT_ARRAY);
	}

	pub fn null_to_empty(&self, array: &&[/* Java */ java::lang::String /**/]) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::null_to(array, self.EMPTY_STRING_ARRAY);
	}

	pub fn null_to_empty<T>(&self, array: &&[T], type: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		if type == null {
			return Err(IllegalArgumentException::new("The type must not be null"));
		}
		if array == null {
			return type.cast(&Array::newInstance(&type.getComponentType(), 0));
		}
		return array;
	}

	fn random(&self) -> /* Java */ java::util::concurrent::ThreadLocalRandom /**/ {
		return ThreadLocalRandom::current();
	}

	pub fn remove(&self, array: &&[bool], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<bool>;
	}

	pub fn remove(&self, array: &&[i8], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i8] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<i8>;
	}

	pub fn remove(&self, array: &&[u16], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[u16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<char>;
	}

	pub fn remove(&self, array: &&[f64], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<f64>;
	}

	pub fn remove(&self, array: &&[f32], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<f32>;
	}

	pub fn remove(&self, array: &&[i32], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<i32>;
	}

	pub fn remove(&self, array: &&[i64], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<i64>;
	}

	fn remove(&self, array: &/* Java */ java::lang::Object /**/, index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let length: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array);
		if index < 0 || index >= length {
			return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + length));
		}
		/* final */ let result: Object = Array::newInstance(&array.getClass().getComponentType(), length - 1);
		System::arraycopy(array, 0, result, 0, index);
		if index < length - 1 {
			System::arraycopy(array, index + 1, result, index, length - index - 1);
		}
		return result;
	}

	pub fn remove(&self, array: &&[i16], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<i16>;
	}

	pub fn remove<T>(&self, array: &&[T], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove(array as Object, index)? as Vec<T>;
	}

	pub fn remove_all(&self, array: &&[bool], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<bool>;
	}

	pub fn remove_all(&self, array: &&[i8], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i8] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<i8>;
	}

	pub fn remove_all(&self, array: &&[u16], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[u16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<char>;
	}

	pub fn remove_all(&self, array: &&[f64], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<f64>;
	}

	pub fn remove_all(&self, array: &&[f32], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<f32>;
	}

	pub fn remove_all(&self, array: &&[i32], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<i32>;
	}

	pub fn remove_all(&self, array: &&[i64], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<i64>;
	}

	fn remove_all(&self, array: &/* Java */ java::lang::Object /**/, indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> /* Java */ java::lang::Object /**/ {
		if array == null {
			return null;
		}
		/* final */ let length: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array);
		// number of distinct indexes, i.e. number of entries that will be removed
		let diff: i32 = 0;
		/* final */ let cloned_indices: Vec<i32> = ArraySorter::sort(&.clone(indices));
		// identify length of result array
		if .isNotEmpty(cloned_indices) {
			let i: i32 = cloned_indices.length;
			let prev_index: i32 = length;
			while i -= 1 >= 0 {
				/* final */ let index: i32 = cloned_indices[i];
				if index < 0 || index >= length {
					return Err(IndexOutOfBoundsException::new("Index: " + index + ", Length: " + length));
				}
				if index >= prev_index {
					continue;
				}
				diff += 1;
				prev_index = index;
			}
		}
		// create result array
		/* final */ let result: Object = Array::newInstance(&array.getClass().getComponentType(), length - diff);
		if diff < length && cloned_indices != null {
			// index just after last copy
			let end: i32 = length;
			// number of entries so far not copied
			let dest: i32 = length - diff;
			 {
				let i: i32 = cloned_indices.length - 1;
				while i >= 0 {
					{
						/* final */ let index: i32 = cloned_indices[i];
						if end - index > 1 {
							// same as (cp > 0)
							/* final */ let cp: i32 = end - index - 1;
							dest -= cp;
							System::arraycopy(array, index + 1, result, dest, cp);
						// After this copy, we still have room for dest items.
						}
						end = index;
					}
					i -= 1;
				 }
			 }
	
			if end > 0 {
				System::arraycopy(array, 0, result, 0, end);
			}
		}
		return result;
	}

	pub fn remove_all(&self, array: &&[i16], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<i16>;
	}

	pub fn remove_all<T>(&self, array: &&[T], indices: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_all(array as Object, indices)? as Vec<T>;
	}

	pub fn remove_all_occurences(&self, array: &&[bool], element: bool) -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<bool>;
	}

	pub fn remove_all_occurences(&self, array: &&[i8], element: i8) -> &[i8] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i8>;
	}

	pub fn remove_all_occurences(&self, array: &&[u16], element: u16) -> &[u16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<char>;
	}

	pub fn remove_all_occurences(&self, array: &&[f64], element: f64) -> &[f64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<f64>;
	}

	pub fn remove_all_occurences(&self, array: &&[f32], element: f32) -> &[f32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<f32>;
	}

	pub fn remove_all_occurences(&self, array: &&[i32], element: i32) -> &[i32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i32>;
	}

	pub fn remove_all_occurences(&self, array: &&[i64], element: i64) -> &[i64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i64>;
	}

	pub fn remove_all_occurences(&self, array: &&[i16], element: i16) -> &[i16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i16>;
	}

	pub fn remove_all_occurences<T>(&self, array: &&[T], element: &T) -> &[T] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<T>;
	}

	pub fn remove_all_occurrences(&self, array: &&[bool], element: bool) -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<bool>;
	}

	pub fn remove_all_occurrences(&self, array: &&[i8], element: i8) -> &[i8] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i8>;
	}

	pub fn remove_all_occurrences(&self, array: &&[u16], element: u16) -> &[u16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<char>;
	}

	pub fn remove_all_occurrences(&self, array: &&[f64], element: f64) -> &[f64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<f64>;
	}

	pub fn remove_all_occurrences(&self, array: &&[f32], element: f32) -> &[f32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<f32>;
	}

	pub fn remove_all_occurrences(&self, array: &&[i32], element: i32) -> &[i32] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i32>;
	}

	pub fn remove_all_occurrences(&self, array: &&[i64], element: i64) -> &[i64] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i64>;
	}

	pub fn remove_all_occurrences(&self, array: &&[i16], element: i16) -> &[i16] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<i16>;
	}

	pub fn remove_all_occurrences<T>(&self, array: &&[T], element: &T) -> &[T] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, &org::apache::commons::lang3::array_utils::ArrayUtils::indexes_of(array, element)) as Vec<T>;
	}

	fn remove_at(&self, array: &/* Java */ java::lang::Object /**/, indices: &/* Java */ java::util::BitSet /**/) -> /* Java */ java::lang::Object /**/ {
		if array == null {
			return null;
		}
		/* final */ let src_length: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::get_length(array);
		// No need to check maxIndex here, because method only currently called from removeElements()
		// which guarantee to generate only valid bit entries.
		//        final int maxIndex = indices.length();
		//        if (maxIndex > srcLength) {
		//            throw new IndexOutOfBoundsException("Index: " + (maxIndex-1) + ", Length: " + srcLength);
		//        }
		// true bits are items to remove
		/* final */ let removals: i32 = indices.cardinality();
		/* final */ let result: Object = Array::newInstance(&array.getClass().getComponentType(), src_length - removals);
		let src_index: i32 = 0;
		let dest_index: i32 = 0;
		let count: i32;
		let set: i32;
		while (set = indices.nextSetBit(src_index)) != -1 {
			count = set - src_index;
			if count > 0 {
				System::arraycopy(array, src_index, result, dest_index, count);
				dest_index += count;
			}
			src_index = indices.nextClearBit(set);
		}
		count = src_length - src_index;
		if count > 0 {
			System::arraycopy(array, src_index, result, dest_index, count);
		}
		return result;
	}

	pub fn remove_element(&self, array: &&[bool], element: bool) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[bool] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { org::apache::commons::lang3::array_utils::ArrayUtils::remove(array, index)? };
	}

	pub fn remove_element(&self, array: &&[i8], element: i8) -> &[i8] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { .clone(array) } else { .remove(array, index) };
	}

	pub fn remove_element(&self, array: &&[u16], element: u16) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[u16] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { .remove(array, index) };
	}

	pub fn remove_element(&self, array: &&[f64], element: f64) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f64] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { org::apache::commons::lang3::array_utils::ArrayUtils::remove(array, index)? };
	}

	pub fn remove_element(&self, array: &&[f32], element: f32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[f32] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { org::apache::commons::lang3::array_utils::ArrayUtils::remove(array, index)? };
	}

	pub fn remove_element(&self, array: &&[i32], element: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[i32] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { .clone(array) } else { .remove(array, index) };
	}

	pub fn remove_element(&self, array: &&[i64], element: i64) -> &[i64] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { org::apache::commons::lang3::array_utils::ArrayUtils::remove(array, index)? };
	}

	pub fn remove_element(&self, array: &&[i16], element: i16) -> &[i16] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { .remove(array, index) };
	}

	pub fn remove_element<T>(&self, array: &&[T], element: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		/* final */ let index: i32 = org::apache::commons::lang3::array_utils::ArrayUtils::index_of(array, element);
		return  if index == self.INDEX_NOT_FOUND { org::apache::commons::lang3::array_utils::ArrayUtils::clone(array) } else { org::apache::commons::lang3::array_utils::ArrayUtils::remove(array, index)? };
	}

	pub fn remove_elements(&self, array: &&[bool], values: bool) -> &[bool] {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		// only two possible values here
		/* final */ let occurrences: HashMap<Boolean, MutableInt> = HashMap<>::new(2);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Boolean::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: bool = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<bool>;
	}

	pub fn remove_elements(&self, array: &&[i8], values: i8) -> &[i8] {
		if .isEmpty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return .clone(array);
		}
		/* final */ let occurrences: HashMap<Byte, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Byte::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: i8 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<i8>;
	}

	pub fn remove_elements(&self, array: &&[u16], values: u16) -> &[u16] {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || .isEmpty(values) {
			return .clone(array);
		}
		/* final */ let occurrences: HashMap<Character, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Character::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: char = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<char>;
	}

	pub fn remove_elements(&self, array: &&[f64], values: f64) -> &[f64] {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<Double, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Double::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: f64 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<f64>;
	}

	pub fn remove_elements(&self, array: &&[f32], values: f32) -> &[f32] {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<Float, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Float::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: f32 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<f32>;
	}

	pub fn remove_elements(&self, array: &&[i32], values: i32) -> &[i32] {
		if .isEmpty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<Integer, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Integer::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: i32 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<i32>;
	}

	pub fn remove_elements(&self, array: &&[i64], values: i64) -> &[i64] {
		if .isEmpty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<Long, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Long::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: i64 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<i64>;
	}

	pub fn remove_elements(&self, array: &&[i16], values: i16) -> &[i16] {
		if .isEmpty(array) || .isEmpty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<Short, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, &Short::valueOf(v));
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: i16 = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		return org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<i16>;
	}

	pub fn remove_elements<T>(&self, array: &&[T], values: &T) -> &[T] {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(values) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::clone(array);
		}
		/* final */ let occurrences: HashMap<T, MutableInt> = HashMap<>::new(values.length);
		for /* final */ v in values {
			org::apache::commons::lang3::array_utils::ArrayUtils::increment(occurrences, v);
		}
		/* final */ let to_remove: BitSet = BitSet::new();
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let key: T = array[i];
					/* final */ let count: MutableInt = occurrences.get(key);
					if count != null {
						if count.decrement_and_get() == 0 {
							occurrences.remove(key);
						}
						to_remove.set(i);
					}
				}
				i += 1;
			 }
		 }
	
		/* final */ let result: Vec<T> = org::apache::commons::lang3::array_utils::ArrayUtils::remove_at(array, to_remove) as Vec<T>;
		return result;
	}

	pub fn reverse(&self, array: &&[bool]) {
		if array == null {
			return;
		}
		org::apache::commons::lang3::array_utils::ArrayUtils::reverse(array, 0, array.length);
	}

	pub fn reverse(&self, mut array: &&[bool], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: bool;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[i8]) {
		if array != null {
			.reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[i8], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: i8;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[u16]) {
		if array != null {
			.reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[u16], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: char;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[f64]) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[f64], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: f64;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[f32]) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[f32], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: f32;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[i32]) {
		if array != null {
			.reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[i32], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: i32;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[i64]) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[i64], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: i64;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[/* Java */ java::lang::Object /**/]) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[/* Java */ java::lang::Object /**/], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: Object;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn reverse(&self, array: &&[i16]) {
		if array != null {
			.reverse(array, 0, array.length);
		}
	}

	pub fn reverse(&self, mut array: &&[i16], start_index_inclusive: i32, end_index_exclusive: i32) {
		if array == null {
			return;
		}
		let i: i32 = Math::max(start_index_inclusive, 0);
		let j: i32 = Math::min(array.length, end_index_exclusive) - 1;
		let tmp: i16;
		while j > i {
			tmp = array[j];
			array[j] = array[i];
			array[i] = tmp;
			j -= 1;
			i += 1;
		}
	}

	pub fn set_all<T>(&self, array: &&[T], generator: &/* Java */ java::util::function::IntFunction /**/) -> &[T] {
		if array != null && generator != null {
			Arrays::setAll(array, generator);
		}
		return array;
	}

	pub fn set_all<T>(&self, mut array: &&[T], generator: &/* Java */ java::util::function::Supplier /**/) -> &[T] {
		if array != null && generator != null {
			 {
				let i: i32 = 0;
				while i < array.length {
					{
						array[i] = generator.get();
					}
					i += 1;
				 }
			 }
	
		}
		return array;
	}

	pub fn shift(&self, array: &&[bool], offset: i32) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[bool], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[i8], offset: i32) {
		if array != null {
			.shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[i8], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[u16], offset: i32) {
		if array != null {
			.shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[u16], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[f64], offset: i32) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[f64], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[f32], offset: i32) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[f32], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[i32], offset: i32) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[i32], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[i64], offset: i32) {
		if array != null {
			.shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[i64], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[/* Java */ java::lang::Object /**/], offset: i32) {
		if array != null {
			org::apache::commons::lang3::array_utils::ArrayUtils::shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[/* Java */ java::lang::Object /**/], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shift(&self, array: &&[i16], offset: i32) {
		if array != null {
			.shift(array, 0, array.length, offset);
		}
	}

	pub fn shift(&self, array: &&[i16], mut start_index_inclusive: i32, mut end_index_exclusive: i32, mut offset: i32) {
		if array == null || start_index_inclusive >= array.length - 1 || end_index_exclusive <= 0 {
			return;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		let n: i32 = end_index_exclusive - start_index_inclusive;
		if n <= 1 {
			return;
		}
		offset %= n;
		if offset < 0 {
			offset += n;
		}
		// see https://beradrian.wordpress.com/2015/04/07/shift-an-array-in-on-in-place/
		while n > 1 && offset > 0 {
			/* final */ let n_offset: i32 = n - offset;
			if offset > n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n - n_offset, n_offset);
				n = offset;
				offset -= n_offset;
			} else if offset < n_offset {
				.swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				start_index_inclusive += offset;
				n = n_offset;
			} else {
				org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, start_index_inclusive, start_index_inclusive + n_offset, offset);
				break;
			}
		}
	}

	pub fn shuffle(&self, array: &&[bool]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[bool], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[i8]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[i8], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[u16]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[u16], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					.swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[f64]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[f64], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[f32]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[f32], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					.swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[i32]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[i32], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					.swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[i64]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[i64], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[/* Java */ java::lang::Object /**/]) {
		org::apache::commons::lang3::array_utils::ArrayUtils::shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[/* Java */ java::lang::Object /**/], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn shuffle(&self, array: &&[i16]) {
		.shuffle(array, &org::apache::commons::lang3::array_utils::ArrayUtils::random());
	}

	pub fn shuffle(&self, array: &&[i16], random: &/* Java */ java::util::Random /**/) {
		 {
			let i: i32 = array.length;
			while i > 1 {
				{
					.swap(array, i - 1, &random.nextInt(i), 1);
				}
				i -= 1;
			 }
		 }
	
	}

	pub fn starts_with(&self, data: &&[i8], expected: &&[i8]) -> bool {
		if data == expected {
			return true;
		}
		if data == null || expected == null {
			return false;
		}
		/* final */ let data_len: i32 = data.length;
		if expected.length > data_len {
			return false;
		}
		if expected.length == data_len {
			// delegate to Arrays.equals() which has optimizations on Java > 8
			return Arrays::equals(data, expected);
		}
		// Once we are on Java 9+ we can delegate to Arrays here as well (or not).
		 {
			let i: i32 = 0;
			while i < expected.length {
				{
					if data[i] != expected[i] {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn subarray(&self, array: &&[bool], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[bool] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_BOOLEAN_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<bool>::new);
	}

	pub fn subarray(&self, array: &&[i8], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[i8] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_BYTE_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<i8>::new);
	}

	pub fn subarray(&self, array: &&[u16], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[u16] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_CHAR_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<char>::new);
	}

	pub fn subarray(&self, array: &&[f64], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[f64] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_DOUBLE_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<f64>::new);
	}

	pub fn subarray(&self, array: &&[f32], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[f32] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_FLOAT_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<f32>::new);
	}

	pub fn subarray(&self, array: &&[i32], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[i32] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_INT_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<i32>::new);
	}

	pub fn subarray(&self, array: &&[i64], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[i64] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_LONG_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<i64>::new);
	}

	pub fn subarray(&self, array: &&[i16], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[i16] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		if new_size <= 0 {
			return self.EMPTY_SHORT_ARRAY;
		}
		return .arraycopy(array, start_index_inclusive, 0, new_size, Vec<i16>::new);
	}

	pub fn subarray<T>(&self, array: &&[T], mut start_index_inclusive: i32, mut end_index_exclusive: i32) -> &[T] {
		if array == null {
			return null;
		}
		start_index_inclusive = org::apache::commons::lang3::array_utils::ArrayUtils::max0(start_index_inclusive);
		end_index_exclusive = Math::min(end_index_exclusive, array.length);
		/* final */ let new_size: i32 = end_index_exclusive - start_index_inclusive;
		/* final */ let type: Class<T> = org::apache::commons::lang3::array_utils::ArrayUtils::get_component_type(array);
		if new_size <= 0 {
			return org::apache::commons::lang3::array_utils::ArrayUtils::new_instance(type, 0);
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::arraycopy(array, start_index_inclusive, 0, new_size, |()|org::apache::commons::lang3::array_utils::ArrayUtils::new_instance(type, new_size));
	}

	pub fn swap(&self, array: &&[bool], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[bool], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: bool = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[i8], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, array: &&[i8], offset1: i32, offset2: i32, len: i32) {
		if .isEmpty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: i8 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[u16], offset1: i32, offset2: i32) {
		.swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, array: &&[u16], offset1: i32, offset2: i32, len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: char = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[f64], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[f64], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: f64 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[f32], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[f32], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if .isEmpty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: f32 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[i32], offset1: i32, offset2: i32) {
		.swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[i32], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: i32 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[i64], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[i64], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: i64 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[/* Java */ java::lang::Object /**/], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, mut array: &&[/* Java */ java::lang::Object /**/], mut offset1: i32, mut offset2: i32, mut len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: Object = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn swap(&self, array: &&[i16], offset1: i32, offset2: i32) {
		org::apache::commons::lang3::array_utils::ArrayUtils::swap(array, offset1, offset2, 1);
	}

	pub fn swap(&self, array: &&[i16], offset1: i32, offset2: i32, len: i32) {
		if org::apache::commons::lang3::array_utils::ArrayUtils::is_empty(array) || offset1 >= array.length || offset2 >= array.length {
			return;
		}
		offset1 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset1);
		offset2 = org::apache::commons::lang3::array_utils::ArrayUtils::max0(offset2);
		if offset1 == offset2 {
			return;
		}
		len = Math::min(&Math::min(len, array.length - offset1), array.length - offset2);
		 {
			let i: i32 = 0;
			while i < len {
				{
					/* final */ let aux: i16 = array[offset1];
					array[offset1] = array[offset2];
					array[offset2] = aux;
				}
				i += 1;
				offset1 += 1;
				offset2 += 1;
			 }
		 }
	
	}

	pub fn to_array<T>(&self, items: &T) -> &[T] {
		return items;
	}

	pub fn to_map(&self, array: &&[/* Java */ java::lang::Object /**/]) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Map /**/ {
		if array == null {
			return null;
		}
		/* final */ let map: Map<Object, Object> = HashMap<>::new((array.length * 1.5) as i32);
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let object: Object = array[i];
					if object instanceof Map.Entry<?, ?> {
						/* final */ let entry: Map.Entry<?, ?> = object as Map.Entry<?, ?>;
						map.put(&entry.getKey(), &entry.getValue());
					} else if object instanceof Vec<Object> {
						/* final */ let entry: Vec<Object> = object as Vec<Object>;
						if entry.length < 2 {
							return Err(IllegalArgumentException::new("Array element " + i + ", '" + object + "', has a length less than 2"));
						}
						map.put(entry[0], entry[1]);
					} else {
						return Err(IllegalArgumentException::new("Array element " + i + ", '" + object + "', is neither of type Map.Entry nor an Array"));
					}
				}
				i += 1;
			 }
		 }
	
		return map;
	}

	pub fn to_object(&self, array: &&[bool]) -> &[/* Java */ java::lang::Boolean /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_BOOLEAN_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Boolean>; array.length] = [None; array.length], |i| if array[i] { Boolean::TRUE } else { Boolean::FALSE });
	}

	pub fn to_object(&self, array: &&[i8]) -> &[/* Java */ java::lang::Byte /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_BYTE_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Byte>; array.length] = [None; array.length], |i|Byte::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[u16]) -> &[/* Java */ java::lang::Character /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_CHARACTER_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Character>; array.length] = [None; array.length], |i|Character::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[f64]) -> &[/* Java */ java::lang::Double /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_DOUBLE_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Double>; array.length] = [None; array.length], |i|Double::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[f32]) -> &[/* Java */ java::lang::Float /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_FLOAT_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Float>; array.length] = [None; array.length], |i|Float::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[i32]) -> &[/* Java */ java::lang::Integer /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_INTEGER_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Integer>; array.length] = [None; array.length], |i|Integer::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[i64]) -> &[/* Java */ java::lang::Long /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_LONG_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Long>; array.length] = [None; array.length], |i|Long::valueOf(array[i]));
	}

	pub fn to_object(&self, array: &&[i16]) -> &[/* Java */ java::lang::Short /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_SHORT_OBJECT_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::set_all(: [Option<Short>; array.length] = [None; array.length], |i|Short::valueOf(array[i]));
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Boolean /**/]) -> &[bool] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array, false);
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Boolean /**/], value_for_null: bool) -> &[bool] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_BOOLEAN_ARRAY;
		}
		/* final */ let result: [bool; array.length] = [false; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Boolean = array[i];
					result[i] =  if b == null { value_for_null } else { b.booleanValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Byte /**/]) -> &[i8] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_BYTE_ARRAY;
		}
		/* final */ let result: [i8; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].byteValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Byte /**/], value_for_null: i8) -> &[i8] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_BYTE_ARRAY;
		}
		/* final */ let result: [i8; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Byte = array[i];
					result[i] =  if b == null { value_for_null } else { b.byteValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Character /**/]) -> &[u16] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_CHAR_ARRAY;
		}
		/* final */ let result: [Option<char>; array.length] = [None; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].charValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Character /**/], value_for_null: u16) -> &[u16] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_CHAR_ARRAY;
		}
		/* final */ let result: [Option<char>; array.length] = [None; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Character = array[i];
					result[i] =  if b == null { value_for_null } else { b.charValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Double /**/]) -> &[f64] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_DOUBLE_ARRAY;
		}
		/* final */ let result: [f64; array.length] = [0.0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].doubleValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Double /**/], value_for_null: f64) -> &[f64] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_DOUBLE_ARRAY;
		}
		/* final */ let result: [f64; array.length] = [0.0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Double = array[i];
					result[i] =  if b == null { value_for_null } else { b.doubleValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Float /**/]) -> &[f32] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_FLOAT_ARRAY;
		}
		/* final */ let result: [f32; array.length] = [0.0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].floatValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Float /**/], value_for_null: f32) -> &[f32] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_FLOAT_ARRAY;
		}
		/* final */ let result: [f32; array.length] = [0.0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Float = array[i];
					result[i] =  if b == null { value_for_null } else { b.floatValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Integer /**/]) -> &[i32] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_INT_ARRAY;
		}
		/* final */ let result: [i32; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].intValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Integer /**/], value_for_null: i32) -> &[i32] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_INT_ARRAY;
		}
		/* final */ let result: [i32; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Integer = array[i];
					result[i] =  if b == null { value_for_null } else { b.intValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Long /**/]) -> &[i64] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_LONG_ARRAY;
		}
		/* final */ let result: [i64; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].longValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Long /**/], value_for_null: i64) -> &[i64] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_LONG_ARRAY;
		}
		/* final */ let result: [i64; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Long = array[i];
					result[i] =  if b == null { value_for_null } else { b.longValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::Object /**/ {
		if array == null {
			return null;
		}
		/* final */ let ct: Class<?> = array.getClass().getComponentType();
		/* final */ let pt: Class<?> = ClassUtils::wrapper_to_primitive(ct);
		if Boolean::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Boolean>);
		}
		if Character::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Character>);
		}
		if Byte::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Byte>);
		}
		if Integer::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Integer>);
		}
		if Long::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Long>);
		}
		if Short::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Short>);
		}
		if Double::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Double>);
		}
		if Float::TYPE.equals(pt) {
			return org::apache::commons::lang3::array_utils::ArrayUtils::to_primitive(array as Vec<Float>);
		}
		return array;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Short /**/]) -> &[i16] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_SHORT_ARRAY;
		}
		/* final */ let result: [i16; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					result[i] = array[i].shortValue();
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_primitive(&self, array: &&[/* Java */ java::lang::Short /**/], value_for_null: i16) -> &[i16] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_SHORT_ARRAY;
		}
		/* final */ let result: [i16; array.length] = [0; array.length];
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					/* final */ let b: Short = array[i];
					result[i] =  if b == null { value_for_null } else { b.shortValue() };
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn to_string(&self, array: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::array_utils::ArrayUtils::to_string(array, "{}");
	}

	pub fn to_string(&self, array: &/* Java */ java::lang::Object /**/, string_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if array == null {
			return string_if_null;
		}
		return ToStringBuilder::new(array, ToStringStyle::org::apache::commons::lang3::builder::to_string_style::ToStringStyle::SIMPLE_STYLE).append(array).to_string();
	}

	pub fn to_string_array(&self, array: &&[/* Java */ java::lang::Object /**/]) -> &[/* Java */ java::lang::String /**/] {
		return org::apache::commons::lang3::array_utils::ArrayUtils::to_string_array(array, "null");
	}

	pub fn to_string_array(&self, array: &&[/* Java */ java::lang::Object /**/], value_for_null_elements: &/* Java */ java::lang::String /**/) /* thrown(E) */ -> &[/* Java */ java::lang::String /**/] {
		if null == array {
			return null;
		}
		if array.length == 0 {
			return self.EMPTY_STRING_ARRAY;
		}
		return org::apache::commons::lang3::array_utils::ArrayUtils::map(array, String.class, |e|Objects::toString(e, value_for_null_elements))?;
	}

	pub fn new() -> org::apache::commons::lang3::array_utils::ArrayUtils {
	// empty
	}
}