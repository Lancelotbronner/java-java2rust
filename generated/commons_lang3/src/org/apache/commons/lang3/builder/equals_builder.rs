use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::ArrayList;
use java::util::Collection;
use java::util::HashSet;
use java::util::List;
use java::util::Set;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::tuple::Pair;

pub struct EqualsBuilder {
	is_equals: bool = true,
	test_transients: bool,
	test_recursive: bool,
	bypass_reflection_classes: /* Java */ java::util::List /**/,
	reflect_up_to_class: /* Java */ java::lang::Class /**/,
	exclude_fields: &[/* Java */ java::lang::String /**/],
}

impl EqualsBuilder {
	static REGISTRY: /* Java */ java::lang::ThreadLocal /**/ = ThreadLocal::withInitial(HashSet::new);

	fn get_register_pair(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::tuple::pair::Pair {
		return Pair::of(IDKey::new(lhs), IDKey::new(rhs));
	}

	fn get_registry(&self) -> /* Java */ java::util::Set /**/ {
		return self.REGISTRY.get();
	}

	fn is_registered(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> bool {
		/* final */ let registry: Set<Pair<IDKey, IDKey>> = org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_registry();
		/* final */ let pair: Pair<IDKey, IDKey> = org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_register_pair(lhs, rhs);
		/* final */ let swapped_pair: Pair<IDKey, IDKey> = Pair::of(&pair.get_right(), &pair.get_left());
		return registry != null && (registry.contains(pair) || registry.contains(swapped_pair));
	}

	pub fn reflection_equals(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, test_transients: bool) -> bool {
		return org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::reflection_equals(lhs, rhs, test_transients, null);
	}

	pub fn reflection_equals(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, test_transients: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/, test_recursive: bool, exclude_fields: &/* Java */ java::lang::String /**/) -> bool {
		if lhs == rhs {
			return true;
		}
		if lhs == null || rhs == null {
			return false;
		}
		// @formatter:off
		return EqualsBuilder::new().set_exclude_fields(exclude_fields).set_reflect_up_to_class(reflect_up_to_class).set_test_transients(test_transients).set_test_recursive(test_recursive).reflection_append(lhs, rhs).is_equals();
	// @formatter:on
	}

	pub fn reflection_equals(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, test_transients: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/, exclude_fields: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::reflection_equals(lhs, rhs, test_transients, reflect_up_to_class, false, exclude_fields);
	}

	pub fn reflection_equals(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::util::Collection /**/) -> bool {
		return org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::reflection_equals(lhs, rhs, &ReflectionToStringBuilder::to_no_null_string_array(exclude_fields));
	}

	pub fn reflection_equals(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::lang::String /**/) -> bool {
		return org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::reflection_equals(lhs, rhs, false, null, exclude_fields);
	}

	fn register(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) {
		org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_registry().add(&org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_register_pair(lhs, rhs));
	}

	fn unregister(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) {
		/* final */ let registry: Set<Pair<IDKey, IDKey>> = org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_registry();
		registry.remove(&org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::get_register_pair(lhs, rhs));
		if registry.isEmpty() {
			self.REGISTRY.remove();
		}
	}

	pub fn new() -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		// set up default classes to bypass reflection for
		self.bypass_reflection_classes = ArrayList<>::new(1);
		//hashCode field being lazy but not transient
		self.bypass_reflection_classes.add(String.class);
	}

	pub fn append(&mut self, lhs: bool, rhs: bool) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		self.is_equals = lhs == rhs;
		return self;
	}

	pub fn append(&self, lhs: &&[bool], rhs: &&[bool]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i8, rhs: i8) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			self.is_equals = lhs == rhs;
		}
		return self;
	}

	pub fn append(&self, lhs: &&[i8], rhs: &&[i8]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: u16, rhs: u16) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			self.is_equals = lhs == rhs;
		}
		return self;
	}

	pub fn append(&self, lhs: &&[u16], rhs: &&[u16]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&self, lhs: f64, rhs: f64) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			return self.append(&Double::doubleToLongBits(lhs), &Double::doubleToLongBits(rhs));
		}
		return self;
	}

	pub fn append(&self, lhs: &&[f64], rhs: &&[f64]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&self, lhs: f32, rhs: f32) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			return self.append(&Float::floatToIntBits(lhs), &Float::floatToIntBits(rhs));
		}
		return self;
	}

	pub fn append(&self, lhs: &&[f32], rhs: &&[f32]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i32, rhs: i32) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			self.is_equals = lhs == rhs;
		}
		return self;
	}

	pub fn append(&self, lhs: &&[i32], rhs: &&[i32]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i64, rhs: i64) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			self.is_equals = lhs == rhs;
		}
		return self;
	}

	pub fn append(&self, lhs: &&[i64], rhs: &&[i64]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		/* final */ let lhs_class: Class<?> = lhs.getClass();
		if lhs_class.isArray() {
			// factor out array case in order to keep method small enough
			// to be inlined
			self.append_array(lhs, rhs);
		} else // The simple case, not an array, just test the element
		if self.test_recursive && !ClassUtils::is_primitive_or_wrapper(lhs_class) {
			self.reflection_append(lhs, rhs);
		} else {
			self.is_equals = lhs.equals(rhs);
		}
		return self;
	}

	pub fn append(&self, lhs: &&[/* Java */ java::lang::Object /**/], rhs: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i16, rhs: i16) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if self.is_equals {
			self.is_equals = lhs == rhs;
		}
		return self;
	}

	pub fn append(&self, lhs: &&[i16], rhs: &&[i16]) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.set_equals(false);
			return self;
		}
		if lhs.length != rhs.length {
			self.set_equals(false);
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.is_equals {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	fn append_array(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) {
		// This handles multidimensional arrays of the same depth
		if lhs.getClass() != rhs.getClass() {
			self.set_equals(false);
		} else if lhs instanceof Vec<i64> {
			.append(lhs as Vec<i64>, rhs as Vec<i64>);
		} else if lhs instanceof Vec<i32> {
			.append(lhs as Vec<i32>, rhs as Vec<i32>);
		} else if lhs instanceof Vec<i16> {
			self.append(lhs as Vec<i16>, rhs as Vec<i16>);
		} else if lhs instanceof Vec<char> {
			self.append(lhs as Vec<char>, rhs as Vec<char>);
		} else if lhs instanceof Vec<i8> {
			self.append(lhs as Vec<i8>, rhs as Vec<i8>);
		} else if lhs instanceof Vec<f64> {
			self.append(lhs as Vec<f64>, rhs as Vec<f64>);
		} else if lhs instanceof Vec<f32> {
			self.append(lhs as Vec<f32>, rhs as Vec<f32>);
		} else if lhs instanceof Vec<bool> {
			self.append(lhs as Vec<bool>, rhs as Vec<bool>);
		} else {
			// Not an array of primitives
			self.append(lhs as Vec<Object>, rhs as Vec<Object>);
		}
	}

	pub fn append_super(&mut self, super_equals: bool) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		self.is_equals = super_equals;
		return self;
	}

	pub fn build(&self) -> /* Java */ java::lang::Boolean /**/ {
		return Boolean::valueOf(&self.is_equals());
	}

	pub fn is_equals(&self) -> bool {
		return self.is_equals;
	}

	pub fn reflection_append(&mut self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		if !self.is_equals {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null || rhs == null {
			self.is_equals = false;
			return self;
		}
		// Find the leaf class since there may be transients in the leaf
		// class or in classes between the leaf and root.
		// If we are not testing transients or a subclass has no ivars,
		// then a subclass can test equals to a superclass.
		/* final */ let lhs_class: Class<?> = lhs.getClass();
		/* final */ let rhs_class: Class<?> = rhs.getClass();
		let test_class: Class<?>;
		if lhs_class.isInstance(rhs) {
			test_class = lhs_class;
			if !rhs_class.isInstance(lhs) {
				// rhsClass is a subclass of lhsClass
				test_class = rhs_class;
			}
		} else if rhs_class.isInstance(lhs) {
			test_class = rhs_class;
			if !lhs_class.isInstance(rhs) {
				// lhsClass is a subclass of rhsClass
				test_class = lhs_class;
			}
		} else {
			// The two classes are not related.
			self.is_equals = false;
			return self;
		}
		let r0 = 'try0: {
			if test_class.isArray() {
				self.append(lhs, rhs);
			} else //If either class is being excluded, call normal object equals method on lhsClass.
			if self.bypass_reflection_classes != null && (self.bypass_reflection_classes.contains(lhs_class) || self.bypass_reflection_classes.contains(rhs_class)) {
				self.is_equals = lhs.equals(rhs);
			} else {
				self.reflection_append(lhs, rhs, test_class);
				while test_class.getSuperclass() != null && test_class != self.reflect_up_to_class {
					test_class = test_class.getSuperclass();
					self.reflection_append(lhs, rhs, test_class);
				}
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalArgumentException) => {
				// In this case, we tried to test a subclass vs. a superclass and
				// the subclass has ivars or the ivars are transient and
				// we are testing transients.
				// If a subclass has ivars that we are trying to test them, we get an
				// exception and we know that the objects are not equal.
				self.is_equals = false;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return self;
	}

	fn reflection_append(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::is_registered(lhs, rhs) {
			return;
		}
		let r0 = 'try0: {
			org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::register(lhs, rhs);
			/* final */ let fields: Vec<Field> = clazz.getDeclaredFields();
			AccessibleObject::setAccessible(fields, true);
			 {
				let i: i32 = 0;
				while i < fields.length && self.is_equals {
					{
						/* final */ let field: Field = fields[i];
						if !ArrayUtils::contains(self.exclude_fields, &field.getName()) && !field.getName().contains("$") && (self.test_transients || !Modifier::isTransient(&field.getModifiers())) && !Modifier::isStatic(&field.getModifiers()) && !field.isAnnotationPresent(EqualsExclude.class) {
							self.append(&match Reflection::get_unchecked(field, lhs) {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							}, &match Reflection::get_unchecked(field, rhs) {
								Err(e) => break 'try0 Err(e),
								Ok(s) => s,
							});
						}
					}
					i += 1;
				 }
			 }
	
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		org::apache::commons::lang3::builder::equals_builder::EqualsBuilder::unregister(lhs, rhs);
	
	}

	pub fn reset(&mut self) {
		self.is_equals = true;
	}

	pub fn set_bypass_reflection_classes(&mut self, bypass_reflection_classes: &/* Java */ java::util::List /**/) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		self.bypassReflectionClasses = bypass_reflection_classes;
		return self;
	}

	fn set_equals(&mut self, is_equals: bool) {
		self.isEquals = is_equals;
	}

	pub fn set_exclude_fields(&mut self, exclude_fields: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		self.excludeFields = exclude_fields;
		return self;
	}

	pub fn set_reflect_up_to_class(&mut self, reflect_up_to_class: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		self.reflectUpToClass = reflect_up_to_class;
		return self;
	}

	pub fn set_test_recursive(&mut self, test_recursive: bool) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		self.testRecursive = test_recursive;
		return self;
	}

	pub fn set_test_transients(&mut self, test_transients: bool) -> org::apache::commons::lang3::builder::equals_builder::EqualsBuilder {
		self.testTransients = test_transients;
		return self;
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for EqualsBuilder {}