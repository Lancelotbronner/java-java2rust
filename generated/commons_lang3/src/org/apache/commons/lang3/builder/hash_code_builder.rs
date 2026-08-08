use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::Collection;
use java::util::Comparator;
use java::util::HashSet;
use java::util::Objects;
use java::util::Set;
use crate::org::apache::commons::lang3::ArraySorter;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::Validate;

pub struct HashCodeBuilder {
	i_constant: i32,
	i_total: i32,
}

impl HashCodeBuilder {
	static DEFAULT_INITIAL_VALUE: i32 = 17;

	static DEFAULT_MULTIPLIER_VALUE: i32 = 37;

	static REGISTRY: /* Java */ java::lang::ThreadLocal /**/ = ThreadLocal::withInitial(HashSet::new);

	fn get_registry(&self) -> /* Java */ java::util::Set /**/ {
		return self.REGISTRY.get();
	}

	fn is_registered(&self, value: &/* Java */ java::lang::Object /**/) -> bool {
		/* final */ let registry: Set<IDKey> = org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::get_registry();
		return registry != null && registry.contains(IDKey::new(value));
	}

	fn reflection_append(&self, object: &/* Java */ java::lang::Object /**/, clazz: &/* Java */ java::lang::Class /**/, builder: &org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder, use_transients: bool, exclude_fields: &&[/* Java */ java::lang::String /**/]) /* thrown(java.lang.IllegalArgumentException) */ {
		if org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::is_registered(object) {
			return;
		}
		let r0 = 'try0: {
			org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::register(object);
			// The elements in the returned array are not sorted and are not in any particular order.
			/* final */ let fields: Vec<Field> = ArraySorter::sort(&clazz.getDeclaredFields(), &Comparator::comparing(Field::getName));
			AccessibleObject::setAccessible(fields, true);
			for /* final */ field in fields {
				if !ArrayUtils::contains(exclude_fields, &field.getName()) && !field.getName().contains("$") && (use_transients || !Modifier::isTransient(&field.getModifiers())) && !Modifier::isStatic(&field.getModifiers()) && !field.isAnnotationPresent(HashCodeExclude.class) {
					builder.append(&match Reflection::get_unchecked(field, object) {
						Err(e) => break 'try0 Err(e),
						Ok(s) => s,
					});
				}
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::unregister(object);
	
	}

	pub fn reflection_hash_code(&self, initial_non_zero_odd_number: i32, multiplier_non_zero_odd_number: i32, object: &/* Java */ java::lang::Object /**/) -> i32 {
		return org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_hash_code(initial_non_zero_odd_number, multiplier_non_zero_odd_number, object, false, null);
	}

	pub fn reflection_hash_code(&self, initial_non_zero_odd_number: i32, multiplier_non_zero_odd_number: i32, object: &/* Java */ java::lang::Object /**/, test_transients: bool) -> i32 {
		return org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_hash_code(initial_non_zero_odd_number, multiplier_non_zero_odd_number, object, test_transients, null);
	}

	pub fn reflection_hash_code<T>(&self, initial_non_zero_odd_number: i32, multiplier_non_zero_odd_number: i32, object: &T, test_transients: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/, exclude_fields: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		Objects::requireNonNull(object, "object");
		/* final */ let builder: HashCodeBuilder = HashCodeBuilder::new(initial_non_zero_odd_number, multiplier_non_zero_odd_number);
		let clazz: Class<?> = object.getClass();
		org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_append(object, clazz, builder, test_transients, exclude_fields)?;
		while clazz.getSuperclass() != null && clazz != reflect_up_to_class {
			clazz = clazz.getSuperclass();
			org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_append(object, clazz, builder, test_transients, exclude_fields)?;
		}
		return builder.to_hash_code();
	}

	pub fn reflection_hash_code(&self, object: &/* Java */ java::lang::Object /**/, test_transients: bool) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		return org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_hash_code(self.DEFAULT_INITIAL_VALUE, self.DEFAULT_MULTIPLIER_VALUE, object, test_transients, null)?;
	}

	pub fn reflection_hash_code(&self, object: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::util::Collection /**/) -> i32 {
		return org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_hash_code(object, &ReflectionToStringBuilder::to_no_null_string_array(exclude_fields));
	}

	pub fn reflection_hash_code(&self, object: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		return org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::reflection_hash_code(self.DEFAULT_INITIAL_VALUE, self.DEFAULT_MULTIPLIER_VALUE, object, false, null, exclude_fields)?;
	}

	fn register(&self, value: &/* Java */ java::lang::Object /**/) {
		org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::get_registry().add(IDKey::new(value));
	}

	fn unregister(&self, value: &/* Java */ java::lang::Object /**/) {
		/* final */ let registry: Set<IDKey> = org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder::get_registry();
		registry.remove(IDKey::new(value));
		if registry.isEmpty() {
			self.REGISTRY.remove();
		}
	}

	pub fn new() -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_constant = 37;
		self.i_total = 17;
	}

	pub fn new(initial_odd_number: i32, multiplier_odd_number: i32) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		Validate::is_true(initial_odd_number % 2 != 0, "HashCodeBuilder requires an odd initial value")?;
		Validate::is_true(multiplier_odd_number % 2 != 0, "HashCodeBuilder requires an odd multiplier")?;
		self.i_constant = multiplier_odd_number;
		self.i_total = initial_odd_number;
	}

	pub fn append(&mut self, value: bool) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + ( if value { 0 } else { 1 });
		return self;
	}

	pub fn append(&mut self, array: &&[bool]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: i8) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + value;
		return self;
	}

	pub fn append(&mut self, array: &&[i8]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: u16) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + value;
		return self;
	}

	pub fn append(&mut self, array: &&[u16]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&self, value: f64) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		return self.append(&Double::doubleToLongBits(value));
	}

	pub fn append(&mut self, array: &&[f64]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: f32) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + Float::floatToIntBits(value);
		return self;
	}

	pub fn append(&mut self, array: &&[f32]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: i32) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + value;
		return self;
	}

	pub fn append(&mut self, array: &&[i32]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: i64) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + (value ^ value /* signed */ >> 32) as i32;
		return self;
	}

	pub fn append(&mut self, array: &&[i64]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, object: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if object == null {
			self.i_total = self.i_total * self.i_constant;
		} else if ObjectUtils::is_array(object) {
			// factor out array case in order to keep method small enough
			// to be inlined
			self.append_array(object);
		} else {
			self.i_total = self.i_total * self.i_constant + object.hashCode();
		}
		return self;
	}

	pub fn append(&mut self, array: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	pub fn append(&mut self, value: i16) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + value;
		return self;
	}

	pub fn append(&mut self, array: &&[i16]) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		if array == null {
			self.i_total = self.i_total * self.i_constant;
		} else {
			for /* final */ element in array {
				self.append(element);
			}
		}
		return self;
	}

	fn append_array(&self, object: &/* Java */ java::lang::Object /**/) {
		// This handles multidimensional arrays
		if object instanceof Vec<i64> {
			.append(object as Vec<i64>);
		} else if object instanceof Vec<i32> {
			self.append(object as Vec<i32>);
		} else if object instanceof Vec<i16> {
			.append(object as Vec<i16>);
		} else if object instanceof Vec<char> {
			self.append(object as Vec<char>);
		} else if object instanceof Vec<i8> {
			.append(object as Vec<i8>);
		} else if object instanceof Vec<f64> {
			self.append(object as Vec<f64>);
		} else if object instanceof Vec<f32> {
			self.append(object as Vec<f32>);
		} else if object instanceof Vec<bool> {
			self.append(object as Vec<bool>);
		} else {
			// Not an array of primitives
			self.append(object as Vec<Object>);
		}
	}

	pub fn append_super(&mut self, super_hash_code: i32) -> org::apache::commons::lang3::builder::hash_code_builder::HashCodeBuilder {
		self.i_total = self.i_total * self.i_constant + super_hash_code;
		return self;
	}

	pub fn build(&self) -> /* Java */ java::lang::Integer /**/ {
		return Integer::valueOf(&self.to_hash_code());
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if !(obj instanceof HashCodeBuilder) {
			return false;
		}
		/* final */ let other: HashCodeBuilder = obj as HashCodeBuilder;
		return self.i_total == other.iTotal;
	}

	pub fn hash_code(&self) -> i32 {
		return self.to_hash_code();
	}

	pub fn to_hash_code(&self) -> i32 {
		return self.i_total;
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for HashCodeBuilder {}