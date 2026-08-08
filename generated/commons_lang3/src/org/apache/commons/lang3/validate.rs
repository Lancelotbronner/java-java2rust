use java::util::Collection;
use java::util::Map;
use java::util::Objects;
use java::util::concurrent::atomic::AtomicInteger;
use java::util::function::Supplier;
use java::util::regex::Pattern;

pub struct Validate;

impl Validate {
	static DEFAULT_NOT_NAN_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated value is not a number";

	static DEFAULT_FINITE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The value is invalid: %f";

	static DEFAULT_EXCLUSIVE_BETWEEN_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The value %s is not in the specified exclusive range of %s to %s";

	static DEFAULT_INCLUSIVE_BETWEEN_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The value %s is not in the specified inclusive range of %s to %s";

	static DEFAULT_MATCHES_PATTERN_EX: /* Java */ java::lang::String /**/ = "The string %s does not match the pattern %s";

	static DEFAULT_IS_NULL_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated object is null";

	static DEFAULT_IS_TRUE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated expression is false";

	static DEFAULT_NO_NULL_ELEMENTS_ARRAY_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated array contains null element at index: %d";

	static DEFAULT_NO_NULL_ELEMENTS_COLLECTION_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated collection contains null element at index: %d";

	static DEFAULT_NOT_BLANK_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated character sequence is blank";

	static DEFAULT_NOT_EMPTY_ARRAY_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated array is empty";

	static DEFAULT_NOT_EMPTY_CHAR_SEQUENCE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated character sequence is empty";

	static DEFAULT_NOT_EMPTY_COLLECTION_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated collection is empty";

	static DEFAULT_NOT_EMPTY_MAP_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated map is empty";

	static DEFAULT_VALID_INDEX_ARRAY_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated array index is invalid: %d";

	static DEFAULT_VALID_INDEX_CHAR_SEQUENCE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated character sequence index is invalid: %d";

	static DEFAULT_VALID_INDEX_COLLECTION_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated collection index is invalid: %d";

	static DEFAULT_VALID_STATE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "The validated state is false";

	static DEFAULT_IS_ASSIGNABLE_EX_MESSAGE: /* Java */ java::lang::String /**/ = "Cannot assign a %s to a %s";

	static DEFAULT_IS_INSTANCE_OF_EX_MESSAGE: /* Java */ java::lang::String /**/ = "Expected type: %s, actual: %s";

	pub fn exclusive_between(&self, start: f64, end: f64, value: f64) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value <= start || value >= end {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_EXCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn exclusive_between(&self, start: f64, end: f64, value: f64, message: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value <= start || value >= end {
			return Err(IllegalArgumentException::new(message));
		}
	}

	pub fn exclusive_between(&self, start: i64, end: i64, value: i64) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value <= start || value >= end {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_EXCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn exclusive_between(&self, start: i64, end: i64, value: i64, message: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value <= start || value >= end {
			return Err(IllegalArgumentException::new(message));
		}
	}

	pub fn exclusive_between<T>(&self, start: &T, end: &T, value: &/* Java */ java::lang::Comparable /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value.compareTo(start) <= 0 || value.compareTo(end) >= 0 {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_EXCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn exclusive_between<T>(&self, start: &T, end: &T, value: &/* Java */ java::lang::Comparable /**/, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value.compareTo(start) <= 0 || value.compareTo(end) >= 0 {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn finite(&self, value: f64) /* thrown(java.lang.IllegalArgumentException) */ {
		org::apache::commons::lang3::validate::Validate::finite(value, self.DEFAULT_FINITE_EX_MESSAGE, value)?;
	}

	pub fn finite(&self, value: f64, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if Double::isNaN(value) || Double::isInfinite(value) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	fn get_message(&self, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return  if ArrayUtils::is_empty(values) { message } else { String::format(message, values) };
	}

	pub fn inclusive_between(&self, start: f64, end: f64, value: f64) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value < start || value > end {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_INCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn inclusive_between(&self, start: f64, end: f64, value: f64, message: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value < start || value > end {
			return Err(IllegalArgumentException::new(message));
		}
	}

	pub fn inclusive_between(&self, start: i64, end: i64, value: i64) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value < start || value > end {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_INCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn inclusive_between(&self, start: i64, end: i64, value: i64, message: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value < start || value > end {
			return Err(IllegalArgumentException::new(message));
		}
	}

	pub fn inclusive_between<T>(&self, start: &T, end: &T, value: &/* Java */ java::lang::Comparable /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value.compareTo(start) < 0 || value.compareTo(end) > 0 {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_INCLUSIVE_BETWEEN_EX_MESSAGE, value, start, end)));
		}
	}

	pub fn inclusive_between<T>(&self, start: &T, end: &T, value: &/* Java */ java::lang::Comparable /**/, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning value
		if value.compareTo(start) < 0 || value.compareTo(end) > 0 {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn is_assignable_from(&self, super_type: &/* Java */ java::lang::Class /**/, type: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning type
		if type == null || super_type == null || !super_type.isAssignableFrom(type) {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_IS_ASSIGNABLE_EX_MESSAGE, &ClassUtils::get_name(type, "null type"), &ClassUtils::get_name(super_type, "null type"))));
		}
	}

	pub fn is_assignable_from(&self, super_type: &/* Java */ java::lang::Class /**/, type: &/* Java */ java::lang::Class /**/, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning type
		if !super_type.isAssignableFrom(type) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn is_instance_of(&self, type: &/* Java */ java::lang::Class /**/, obj: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning obj
		if !type.isInstance(obj) {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_IS_INSTANCE_OF_EX_MESSAGE, &type.getName(), &ClassUtils::get_name(obj, "null"))));
		}
	}

	pub fn is_instance_of(&self, type: &/* Java */ java::lang::Class /**/, obj: &/* Java */ java::lang::Object /**/, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning obj
		if !type.isInstance(obj) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn is_true(&self, expression: bool) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(self.DEFAULT_IS_TRUE_EX_MESSAGE));
		}
	}

	pub fn is_true(&self, expression: bool, message: &/* Java */ java::lang::String /**/, value: f64) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(&String::format(message, &Double::valueOf(value))));
		}
	}

	pub fn is_true(&self, expression: bool, message: &/* Java */ java::lang::String /**/, value: i64) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(&String::format(message, &Long::valueOf(value))));
		}
	}

	pub fn is_true(&self, expression: bool, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn is_true(&self, expression: bool, message_supplier: &/* Java */ java::util::function::Supplier /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if !expression {
			return Err(IllegalArgumentException::new(&message_supplier.get()));
		}
	}

	pub fn matches_pattern(&self, input: &/* Java */ java::lang::CharSequence /**/, pattern: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning input
		if !Pattern::matches(pattern, input) {
			return Err(IllegalArgumentException::new(&String::format(self.DEFAULT_MATCHES_PATTERN_EX, input, pattern)));
		}
	}

	pub fn matches_pattern(&self, input: &/* Java */ java::lang::CharSequence /**/, pattern: &/* Java */ java::lang::String /**/, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		// TODO when breaking BC, consider returning input
		if !Pattern::matches(pattern, input) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn no_null_elements<T: /* Java */ java::lang::Iterable /**/>(&self, iterable: &T) -> T {
		return org::apache::commons::lang3::validate::Validate::no_null_elements(iterable, self.DEFAULT_NO_NULL_ELEMENTS_COLLECTION_EX_MESSAGE);
	}

	pub fn no_null_elements<T: /* Java */ java::lang::Iterable /**/>(&self, iterable: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) -> T {
		Objects::requireNonNull(iterable, "iterable");
		/* final */ let ai: AtomicInteger = AtomicInteger::new();
		iterable.forEach(|e|{
			if e == null {
				return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, &ArrayUtils::add_all(values, &ai.getAndIncrement())?)));
			}
		});
		return iterable;
	}

	pub fn no_null_elements<T>(&self, array: &&[T]) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		return org::apache::commons::lang3::validate::Validate::no_null_elements(array, self.DEFAULT_NO_NULL_ELEMENTS_ARRAY_EX_MESSAGE)?;
	}

	pub fn no_null_elements<T>(&self, array: &&[T], message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		Objects::requireNonNull(array, "array");
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if array[i] == null {
						/* final */ let values2: Vec<Object> = ArrayUtils::add(values, &Integer::valueOf(i))?;
						return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values2)));
					}
				}
				i += 1;
			 }
		 }
	
		return array;
	}

	pub fn not_blank<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::not_blank(chars, self.DEFAULT_NOT_BLANK_EX_MESSAGE)?;
	}

	pub fn not_blank<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		Objects::requireNonNull(chars, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
		if StringUtils::is_blank(chars) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return chars;
	}

	pub fn not_empty<T: /* Java */ java::util::Collection /**/>(&self, collection: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::not_empty(collection, self.DEFAULT_NOT_EMPTY_COLLECTION_EX_MESSAGE)?;
	}

	pub fn not_empty<T: /* Java */ java::util::Map /**/>(&self, map: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::not_empty(map, self.DEFAULT_NOT_EMPTY_MAP_EX_MESSAGE)?;
	}

	pub fn not_empty<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::not_empty(chars, self.DEFAULT_NOT_EMPTY_CHAR_SEQUENCE_EX_MESSAGE)?;
	}

	pub fn not_empty<T: /* Java */ java::util::Collection /**/>(&self, collection: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		Objects::requireNonNull(collection, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
		if collection.isEmpty() {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return collection;
	}

	pub fn not_empty<T: /* Java */ java::util::Map /**/>(&self, map: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		Objects::requireNonNull(map, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
		if map.isEmpty() {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return map;
	}

	pub fn not_empty<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		Objects::requireNonNull(chars, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
		if chars.length() == 0 {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return chars;
	}

	pub fn not_empty<T>(&self, array: &&[T]) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		return org::apache::commons::lang3::validate::Validate::not_empty(array, self.DEFAULT_NOT_EMPTY_ARRAY_EX_MESSAGE)?;
	}

	pub fn not_empty<T>(&self, array: &&[T], message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> &[T] {
		Objects::requireNonNull(array, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
		if array.length == 0 {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return array;
	}

	pub fn not_nan(&self, value: f64) /* thrown(java.lang.IllegalArgumentException) */ {
		org::apache::commons::lang3::validate::Validate::not_nan(value, self.DEFAULT_NOT_NAN_EX_MESSAGE)?;
	}

	pub fn not_nan(&self, value: f64, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if Double::isNaN(value) {
			return Err(IllegalArgumentException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn not_null<T>(&self, object: &T) -> T {
		return org::apache::commons::lang3::validate::Validate::not_null(object, self.DEFAULT_IS_NULL_EX_MESSAGE);
	}

	pub fn not_null<T>(&self, object: &T, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) -> T {
		return Objects::requireNonNull(object, &org::apache::commons::lang3::validate::Validate::to_supplier(message, values));
	}

	fn to_supplier(&self, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) -> /* Java */ java::util::function::Supplier /**/ {
		return |()|org::apache::commons::lang3::validate::Validate::get_message(message, values);
	}

	pub fn valid_index<T: /* Java */ java::util::Collection /**/>(&self, collection: &T, index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::valid_index(collection, index, self.DEFAULT_VALID_INDEX_COLLECTION_EX_MESSAGE, &Integer::valueOf(index))?;
	}

	pub fn valid_index<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T, index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> T {
		return org::apache::commons::lang3::validate::Validate::valid_index(chars, index, self.DEFAULT_VALID_INDEX_CHAR_SEQUENCE_EX_MESSAGE, &Integer::valueOf(index))?;
	}

	pub fn valid_index<T: /* Java */ java::util::Collection /**/>(&self, collection: &T, index: i32, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IndexOutOfBoundsException) */ -> T {
		Objects::requireNonNull(collection, "collection");
		if index < 0 || index >= collection.size() {
			return Err(IndexOutOfBoundsException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return collection;
	}

	pub fn valid_index<T: /* Java */ java::lang::CharSequence /**/>(&self, chars: &T, index: i32, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IndexOutOfBoundsException) */ -> T {
		Objects::requireNonNull(chars, "chars");
		if index < 0 || index >= chars.length() {
			return Err(IndexOutOfBoundsException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return chars;
	}

	pub fn valid_index<T>(&self, array: &&[T], index: i32) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		return org::apache::commons::lang3::validate::Validate::valid_index(array, index, self.DEFAULT_VALID_INDEX_ARRAY_EX_MESSAGE, &Integer::valueOf(index))?;
	}

	pub fn valid_index<T>(&self, array: &&[T], index: i32, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[T] {
		Objects::requireNonNull(array, "array");
		if index < 0 || index >= array.length {
			return Err(IndexOutOfBoundsException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
		return array;
	}

	pub fn valid_state(&self, expression: bool) /* thrown(java.lang.IllegalStateException) */ {
		if !expression {
			return Err(IllegalStateException::new(self.DEFAULT_VALID_STATE_EX_MESSAGE));
		}
	}

	pub fn valid_state(&self, expression: bool, message: &/* Java */ java::lang::String /**/, values: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalStateException) */ {
		if !expression {
			return Err(IllegalStateException::new(&org::apache::commons::lang3::validate::Validate::get_message(message, values)));
		}
	}

	pub fn new() -> org::apache::commons::lang3::validate::Validate {
	}
}