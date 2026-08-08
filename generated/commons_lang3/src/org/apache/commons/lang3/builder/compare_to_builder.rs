use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::Collection;
use java::util::Comparator;
use java::util::Objects;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ObjectUtils;

pub struct CompareToBuilder {
	comparison: i32,
}

impl CompareToBuilder {
	fn reflection_append(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, clazz: &/* Java */ java::lang::Class /**/, builder: &org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder, use_transients: bool, exclude_fields: &&[/* Java */ java::lang::String /**/]) /* thrown(java.lang.IllegalArgumentException) */ {
		/* final */ let fields: Vec<Field> = clazz.getDeclaredFields();
		AccessibleObject::setAccessible(fields, true);
		 {
			let i: i32 = 0;
			while i < fields.length && builder.comparison == 0 {
				{
					/* final */ let field: Field = fields[i];
					if !ArrayUtils::contains(exclude_fields, &field.getName()) && !field.getName().contains("$") && (use_transients || !Modifier::isTransient(&field.getModifiers())) && !Modifier::isStatic(&field.getModifiers()) {
						// IllegalAccessException can't happen. Would get a Security exception instead.
						// Throw a runtime exception in case the impossible happens.
						builder.append(&Reflection::get_unchecked(field, lhs)?, &Reflection::get_unchecked(field, rhs)?);
					}
				}
				i += 1;
			 }
		 }
	
	}

	pub fn reflection_compare(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.ClassCastException) */ -> i32 {
		return org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_compare(lhs, rhs, false, null)?;
	}

	pub fn reflection_compare(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, compare_transients: bool) /* thrown(java.lang.ClassCastException) */ -> i32 {
		return org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_compare(lhs, rhs, compare_transients, null)?;
	}

	pub fn reflection_compare(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, compare_transients: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/, exclude_fields: &/* Java */ java::lang::String /**/) /* thrown(java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> i32 {
		if lhs == rhs {
			return 0;
		}
		Objects::requireNonNull(lhs, "lhs");
		Objects::requireNonNull(rhs, "rhs");
		let lhs_clazz: Class<?> = lhs.getClass();
		if !lhs_clazz.isInstance(rhs) {
			return Err(ClassCastException::new());
		}
		/* final */ let compare_to_builder: CompareToBuilder = CompareToBuilder::new();
		org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_append(lhs, rhs, lhs_clazz, compare_to_builder, compare_transients, exclude_fields)?;
		while lhs_clazz.getSuperclass() != null && lhs_clazz != reflect_up_to_class {
			lhs_clazz = lhs_clazz.getSuperclass();
			org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_append(lhs, rhs, lhs_clazz, compare_to_builder, compare_transients, exclude_fields)?;
		}
		return compare_to_builder.to_comparison();
	}

	pub fn reflection_compare(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::util::Collection /**/) -> i32 {
		return org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_compare(lhs, rhs, &ReflectionToStringBuilder::to_no_null_string_array(exclude_fields));
	}

	pub fn reflection_compare(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, exclude_fields: &/* Java */ java::lang::String /**/) /* thrown(java.lang.ClassCastException | java.lang.IllegalArgumentException) */ -> i32 {
		return org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder::reflection_compare(lhs, rhs, false, null, exclude_fields)?;
	}

	pub fn new() -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		self.comparison = 0;
	}

	pub fn append(&mut self, lhs: bool, rhs: bool) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs {
			self.comparison = 1;
		} else {
			self.comparison = -1;
		}
		return self;
	}

	pub fn append(&mut self, lhs: &&[bool], rhs: &&[bool]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i8, rhs: i8) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Byte::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[i8], rhs: &&[i8]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: u16, rhs: u16) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Character::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[u16], rhs: &&[u16]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: f64, rhs: f64) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Double::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[f64], rhs: &&[f64]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: f32, rhs: f32) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Float::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[f32], rhs: &&[f32]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i32, rhs: i32) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Integer::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[i32], rhs: &&[i32]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i64, rhs: i64) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Long::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[i64], rhs: &&[i64]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		return self.append(lhs, rhs, null);
	}

	pub fn append(&mut self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, comparator: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if ObjectUtils::is_array(lhs) {
			// factor out array case in order to keep method small enough to be inlined
			self.append_array(lhs, rhs, comparator);
		} else // the simple case, not an array, just test the element
		if comparator == null {
			/* final */ let comparable: Comparable<Object> = lhs as Comparable<Object>;
			self.comparison = comparable.compareTo(rhs);
		} else {
			/* final */ let comparator2: Comparator<Object> = comparator as Comparator<Object>;
			self.comparison = comparator2.compare(lhs, rhs);
		}
		return self;
	}

	pub fn append(&self, lhs: &&[/* Java */ java::lang::Object /**/], rhs: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		return self.append(lhs, rhs, null);
	}

	pub fn append(&mut self, lhs: &&[/* Java */ java::lang::Object /**/], rhs: &&[/* Java */ java::lang::Object /**/], comparator: &/* Java */ java::util::Comparator /**/) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i], comparator);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	pub fn append(&mut self, lhs: i16, rhs: i16) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = Short::compare(lhs, rhs);
		return self;
	}

	pub fn append(&mut self, lhs: &&[i16], rhs: &&[i16]) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		if lhs == rhs {
			return self;
		}
		if lhs == null {
			self.comparison = -1;
			return self;
		}
		if rhs == null {
			self.comparison = 1;
			return self;
		}
		if lhs.length != rhs.length {
			self.comparison =  if lhs.length < rhs.length { -1 } else { 1 };
			return self;
		}
		 {
			let i: i32 = 0;
			while i < lhs.length && self.comparison == 0 {
				{
					self.append(lhs[i], rhs[i]);
				}
				i += 1;
			 }
		 }
	
		return self;
	}

	fn append_array(&self, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/, comparator: &/* Java */ java::util::Comparator /**/) {
		// throws a ClassCastException if rhs is not the correct array type
		if lhs instanceof Vec<i64> {
			self.append(lhs as Vec<i64>, rhs as Vec<i64>);
		} else if lhs instanceof Vec<i32> {
			.append(lhs as Vec<i32>, rhs as Vec<i32>);
		} else if lhs instanceof Vec<i16> {
			.append(lhs as Vec<i16>, rhs as Vec<i16>);
		} else if lhs instanceof Vec<char> {
			.append(lhs as Vec<char>, rhs as Vec<char>);
		} else if lhs instanceof Vec<i8> {
			.append(lhs as Vec<i8>, rhs as Vec<i8>);
		} else if lhs instanceof Vec<f64> {
			self.append(lhs as Vec<f64>, rhs as Vec<f64>);
		} else if lhs instanceof Vec<f32> {
			self.append(lhs as Vec<f32>, rhs as Vec<f32>);
		} else if lhs instanceof Vec<bool> {
			self.append(lhs as Vec<bool>, rhs as Vec<bool>);
		} else {
			// not an array of primitives
			// throws a ClassCastException if rhs is not an array
			self.append(lhs as Vec<Object>, rhs as Vec<Object>, comparator);
		}
	}

	pub fn append_super(&mut self, super_compare_to: i32) -> org::apache::commons::lang3::builder::compare_to_builder::CompareToBuilder {
		if self.comparison != 0 {
			return self;
		}
		self.comparison = super_compare_to;
		return self;
	}

	pub fn build(&self) -> /* Java */ java::lang::Integer /**/ {
		return Integer::valueOf(&self.to_comparison());
	}

	pub fn to_comparison(&self) -> i32 {
		return self.comparison;
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for CompareToBuilder {}