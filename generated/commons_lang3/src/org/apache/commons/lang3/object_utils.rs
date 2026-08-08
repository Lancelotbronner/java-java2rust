use java::io::IOException;
use java::io::Serializable;
use java::lang::reflect::Array;
use java::time::Duration;
use java::util::ArrayList;
use java::util::Collection;
use java::util::Collections;
use java::util::Comparator;
use java::util::HashMap;
use java::util::Hashtable;
use java::util::Map;
use java::util::Objects;
use java::util::Optional;
use java::util::TreeSet;
use java::util::function::Consumer;
use java::util::function::Supplier;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::exception::CloneFailedException;
use crate::org::apache::commons::lang3::function::Consumers;
use crate::org::apache::commons::lang3::function::Suppliers;
use crate::org::apache::commons::lang3::mutable::MutableInt;
use crate::org::apache::commons::lang3::stream::Streams;
use crate::org::apache::commons::lang3::text::StrBuilder;
use crate::org::apache::commons::lang3::time::DurationUtils;

pub struct ObjectUtils;

impl ObjectUtils {
	static AT_SIGN: u16 = '@';

	pub static NULL: org::apache::commons::lang3::object_utils::Null = Null::new();

	pub fn all_not_null(&self, values: &/* Java */ java::lang::Object /**/) -> bool {
		return values != null && Stream::of(values).noneMatch(Objects::isNull);
	}

	pub fn all_null(&self, values: &/* Java */ java::lang::Object /**/) -> bool {
		return !org::apache::commons::lang3::object_utils::ObjectUtils::any_not_null(values);
	}

	pub fn any_not_null(&self, values: &/* Java */ java::lang::Object /**/) -> bool {
		return org::apache::commons::lang3::object_utils::ObjectUtils::first_non_null(values) != null;
	}

	pub fn any_null(&self, values: &/* Java */ java::lang::Object /**/) -> bool {
		return !org::apache::commons::lang3::object_utils::ObjectUtils::all_not_null(values);
	}

	pub fn clone<T>(&self, obj: &T) /* thrown(org.apache.commons.lang3.exception.CloneFailedException) */ -> T {
		if obj instanceof Cloneable {
			/* final */ let result: Object;
			/* final */ let obj_class: Class<? extends Object> = obj.getClass();
			if org::apache::commons::lang3::object_utils::ObjectUtils::is_array(obj) {
				/* final */ let component_type: Class<?> = obj_class.getComponentType();
				if component_type.isPrimitive() {
					let length: i32 = Array::getLength(obj);
					result = Array::newInstance(component_type, length);
					while length -= 1 !!!check!!! post decrement > 0 {
						Array::set(result, length, &Array::get(obj, length));
					}
				} else {
					result = (obj as Vec<Object>).clone();
				}
			} else {
				let r0 = 'try0: {
					result = obj_class.getMethod("clone").invoke(obj);
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ ReflectiveOperationException) => {
						break 'try0 Err(CloneFailedException::new("Exception cloning Cloneable type " + obj_class.getName(), e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
			return result as T;
		}
		return null;
	}

	pub fn clone_if_possible<T>(&self, obj: &T) -> T {
		/* final */ let clone: T = org::apache::commons::lang3::object_utils::ObjectUtils::clone(obj)?;
		return  if clone == null { obj } else { clone };
	}

	pub fn compare<T: /* Java */ java::lang::Comparable /**/>(&self, c1: &T, c2: &T) -> i32 {
		return org::apache::commons::lang3::object_utils::ObjectUtils::compare(c1, c2, false);
	}

	pub fn compare<T: /* Java */ java::lang::Comparable /**/>(&self, c1: &T, c2: &T, null_greater: bool) -> i32 {
		if c1 == c2 {
			return 0;
		}
		if c1 == null {
			return  if null_greater { 1 } else { -1 };
		}
		if c2 == null {
			return  if null_greater { -1 } else { 1 };
		}
		return c1.compareTo(c2);
	}

	pub fn const(&self, v: bool) -> bool {
		return v;
	}

	pub fn const(&self, v: i8) -> i8 {
		return v;
	}

	pub fn const(&self, v: u16) -> u16 {
		return v;
	}

	pub fn const(&self, v: f64) -> f64 {
		return v;
	}

	pub fn const(&self, v: f32) -> f32 {
		return v;
	}

	pub fn const(&self, v: i32) -> i32 {
		return v;
	}

	pub fn const(&self, v: i64) -> i64 {
		return v;
	}

	pub fn const(&self, v: i16) -> i16 {
		return v;
	}

	pub fn const<T>(&self, v: &T) -> T {
		return v;
	}

	pub fn cons_t_byte(&self, v: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i8 {
		if v < Byte::MIN_VALUE || v > Byte::MAX_VALUE {
			return Err(IllegalArgumentException::new("Supplied value must be a valid byte literal between -128 and 127: [" + v + "]"));
		}
		return v as i8;
	}

	pub fn cons_t_short(&self, v: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i16 {
		if v < Short::MIN_VALUE || v > Short::MAX_VALUE {
			return Err(IllegalArgumentException::new("Supplied value must be a valid byte literal between -32768 and 32767: [" + v + "]"));
		}
		return v as i16;
	}

	pub fn default_if_null<T>(&self, object: &T, default_value: &T) -> T {
		return org::apache::commons::lang3::object_utils::ObjectUtils::get_if_null(object, default_value);
	}

	pub fn equals(&self, object1: &/* Java */ java::lang::Object /**/, object2: &/* Java */ java::lang::Object /**/) -> bool {
		return Objects::equals(object1, object2);
	}

	pub fn first_non_null<T>(&self, values: &T) -> T {
		return Streams::of(values).filter(Objects::nonNull).findFirst().orElse(null);
	}

	pub fn get_class<T>(&self, object: &T) -> /* Java */ java::lang::Class /**/ {
		return  if object == null { null } else { object.getClass() as Class<T> };
	}

	pub fn get_first_non_null<T>(&self, suppliers: &/* Java */ java::util::function::Supplier /**/) -> T {
		return Streams::of(suppliers).filter(Objects::nonNull).map(Supplier::get).filter(Objects::nonNull).findFirst().orElse(null);
	}

	pub fn get_if_null<T>(&self, object: &T, default_supplier: &/* Java */ java::util::function::Supplier /**/) -> T {
		return  if object != null { object } else { Suppliers::get(default_supplier) };
	}

	pub fn get_if_null<T>(&self, object: &T, default_value: &T) -> T {
		return  if object != null { object } else { default_value };
	}

	pub fn hash_code(&self, obj: &/* Java */ java::lang::Object /**/) -> i32 {
		// hashCode(Object) for performance vs. hashCodeMulti(Object[]), as hash code is often critical
		return Objects::hashCode(obj);
	}

	pub fn hash_code_hex(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return Integer::toHexString(&Objects::hashCode(object));
	}

	pub fn hash_code_multi(&self, objects: &/* Java */ java::lang::Object /**/) -> i32 {
		let hash: i32 = 1;
		if objects != null {
			for /* final */ object in objects {
				/* final */ let tmp_hash: i32 = Objects::hashCode(object);
				hash = hash * 31 + tmp_hash;
			}
		}
		return hash;
	}

	pub fn identity_hash_code_hex(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return Integer::toHexString(&System::identityHashCode(object));
	}

	pub fn identity_to_string(&self, appendable: &/* Java */ java::lang::Appendable /**/, object: &/* Java */ java::lang::Object /**/) /* thrown(java.io.IOException) */ {
		Objects::requireNonNull(object, "object");
		appendable.append(&object.getClass().getName()).append(self.AT_SIGN).append(&org::apache::commons::lang3::object_utils::ObjectUtils::identity_hash_code_hex(object));
	}

	pub fn identity_to_string(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		if object == null {
			return null;
		}
		/* final */ let name: String = object.getClass().getName();
		/* final */ let hex_string: String = org::apache::commons::lang3::object_utils::ObjectUtils::identity_hash_code_hex(object);
		/* final */ let builder: StringBuilder = StringBuilder::new(name.length() + 1 + hex_string.length());
		// @formatter:off
		builder.append(name).append(self.AT_SIGN).append(hex_string);
		// @formatter:on
		return builder.toString();
	}

	pub fn identity_to_string(&self, builder: &org::apache::commons::lang3::text::str_builder::StrBuilder, object: &/* Java */ java::lang::Object /**/) {
		Objects::requireNonNull(object, "object");
		/* final */ let name: String = object.getClass().getName();
		/* final */ let hex_string: String = org::apache::commons::lang3::object_utils::ObjectUtils::identity_hash_code_hex(object);
		builder.ensure_capacity(builder.length() + name.length() + 1 + hex_string.length());
		builder.append(name).append(self.AT_SIGN).append(hex_string);
	}

	pub fn identity_to_string(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, object: &/* Java */ java::lang::Object /**/) {
		Objects::requireNonNull(object, "object");
		/* final */ let name: String = object.getClass().getName();
		/* final */ let hex_string: String = org::apache::commons::lang3::object_utils::ObjectUtils::identity_hash_code_hex(object);
		buffer.ensureCapacity(buffer.length() + name.length() + 1 + hex_string.length());
		buffer.append(name).append(self.AT_SIGN).append(hex_string);
	}

	pub fn identity_to_string(&self, builder: &/* Java */ java::lang::StringBuilder /**/, object: &/* Java */ java::lang::Object /**/) {
		Objects::requireNonNull(object, "object");
		/* final */ let name: String = object.getClass().getName();
		/* final */ let hex_string: String = org::apache::commons::lang3::object_utils::ObjectUtils::identity_hash_code_hex(object);
		builder.ensureCapacity(builder.length() + name.length() + 1 + hex_string.length());
		builder.append(name).append(self.AT_SIGN).append(hex_string);
	}

	pub fn is_array(&self, object: &/* Java */ java::lang::Object /**/) -> bool {
		return object != null && object.getClass().isArray();
	}

	pub fn is_empty(&self, object: &/* Java */ java::lang::Object /**/) -> bool {
		if object == null {
			return true;
		}
		if object instanceof CharSequence {
			return (object as CharSequence).length() == 0;
		}
		if org::apache::commons::lang3::object_utils::ObjectUtils::is_array(object) {
			return Array::getLength(object) == 0;
		}
		if object instanceof Collection<?> {
			return (object as Collection<?>).isEmpty();
		}
		if object instanceof Map<?, ?> {
			return (object as Map<?, ?>).isEmpty();
		}
		if object instanceof Optional<?> {
			// TODO Java 11 Use Optional#isEmpty()
			return !(object as Optional<?>).isPresent();
		}
		return false;
	}

	pub fn is_not_empty(&self, object: &/* Java */ java::lang::Object /**/) -> bool {
		return !org::apache::commons::lang3::object_utils::ObjectUtils::is_empty(object);
	}

	pub fn max<T: /* Java */ java::lang::Comparable /**/>(&self, values: &T) -> T {
		let result: T = null;
		if values != null {
			for /* final */ value in values {
				if org::apache::commons::lang3::object_utils::ObjectUtils::compare(value, result, false) > 0 {
					result = value;
				}
			}
		}
		return result;
	}

	pub fn median<T>(&self, comparator: &/* Java */ java::util::Comparator /**/, items: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		Validate::not_empty(items, "null/empty items")?;
		Validate::no_null_elements(items);
		Objects::requireNonNull(comparator, "comparator");
		/* final */ let tree_set: TreeSet<T> = TreeSet<>::new(comparator);
		Collections::addAll(tree_set, items);
		return tree_set.toArray()[(tree_set.size() - 1) / 2] as T;
	}

	pub fn median<T: /* Java */ java::lang::Comparable /**/>(&self, items: &T) -> T {
		Validate::not_empty(items);
		Validate::no_null_elements(items);
		/* final */ let sort: TreeSet<T> = TreeSet<>::new();
		Collections::addAll(sort, items);
		return sort.toArray()[(sort.size() - 1) / 2] as T;
	}

	pub fn min<T: /* Java */ java::lang::Comparable /**/>(&self, values: &T) -> T {
		let result: T = null;
		if values != null {
			for /* final */ value in values {
				if org::apache::commons::lang3::object_utils::ObjectUtils::compare(value, result, true) < 0 {
					result = value;
				}
			}
		}
		return result;
	}

	pub fn mode<T>(&self, items: &T) -> T {
		if ArrayUtils::is_not_empty(items) {
			/* final */ let occurrences: HashMap<T, MutableInt> = HashMap<>::new(items.length);
			for /* final */ t in items {
				ArrayUtils::increment(occurrences, t);
			}
			let result: T = null;
			let max: i32 = 0;
			for /* final */ e in occurrences.entrySet() {
				/* final */ let cmp: i32 = e.getValue().int_value();
				if cmp == max {
					result = null;
				} else if cmp > max {
					max = cmp;
					result = e.getKey();
				}
			}
			return result;
		}
		return null;
	}

	pub fn not_equal(&self, object1: &/* Java */ java::lang::Object /**/, object2: &/* Java */ java::lang::Object /**/) -> bool {
		return !Objects::equals(object1, object2);
	}

	pub fn require_non_empty<T>(&self, obj: &T) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return org::apache::commons::lang3::object_utils::ObjectUtils::require_non_empty(obj, "object")?;
	}

	pub fn require_non_empty<T>(&self, obj: &T, message: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		// check for null first to give the most precise exception.
		Objects::requireNonNull(obj, message);
		if org::apache::commons::lang3::object_utils::ObjectUtils::is_empty(obj) {
			return Err(IllegalArgumentException::new(message));
		}
		return obj;
	}

	pub fn to_string(&self, obj: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(obj, StringUtils::EMPTY);
	}

	pub fn to_string(&self, obj: &/* Java */ java::lang::Object /**/, null_str: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(obj, null_str);
	}

	pub fn to_string(&self, obj: &/* Java */ java::util::function::Supplier /**/, supplier: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::lang::String /**/ {
		return  if obj == null { Suppliers::get(supplier) } else { org::apache::commons::lang3::object_utils::ObjectUtils::to_string(&obj.get(), supplier) };
	}

	pub fn to_string<T>(&self, obj: &T, supplier: &/* Java */ java::util::function::Supplier /**/) -> /* Java */ java::lang::String /**/ {
		return  if obj == null { Suppliers::get(supplier) } else { obj.toString() };
	}

	pub fn wait(&self, obj: &/* Java */ java::lang::Object /**/, duration: &/* Java */ java::time::Duration /**/) /* thrown(java.lang.InterruptedException) */ {
		DurationUtils::accept(obj::wait, &DurationUtils::zero_if_null(duration));
	}

	pub fn new() -> org::apache::commons::lang3::object_utils::ObjectUtils {
	// empty
	}
}

pub struct Null;

impl Null {
	static serialVersionUID: i64 = 7092611880189329093;

	fn new() -> org::apache::commons::lang3::object_utils::Null {
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for Null {}