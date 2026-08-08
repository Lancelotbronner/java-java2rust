use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::Arrays;
use java::util::Collection;
use java::util::Comparator;
use java::util::Objects;
use crate::org::apache::commons::lang3::ArraySorter;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct ReflectionToStringBuilder {
	append_statics: bool,
	append_transients: bool,
	exclude_null_values: bool,
	exclude_field_names: &[/* Java */ java::lang::String /**/],
	include_field_names: &[/* Java */ java::lang::String /**/],
	up_to_class: /* Java */ java::lang::Class /**/,
}

impl ReflectionToStringBuilder {
	fn to_no_null_string_array(&self, collection: &/* Java */ java::util::Collection /**/) -> &[/* Java */ java::lang::String /**/] {
		if collection == null {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_no_null_string_array(&collection.toArray());
	}

	fn to_no_null_string_array(&self, array: &&[/* Java */ java::lang::Object /**/]) -> &[/* Java */ java::lang::String /**/] {
		return Streams::non_null(array).map(Objects::toString).toArray(Vec<String>::new);
	}

	pub fn to_string(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string(object, null, false, false, null);
	}

	pub fn to_string(&self, object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string(object, style, false, false, null);
	}

	pub fn to_string(&self, object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string(object, style, output_transients, false, null);
	}

	pub fn to_string(&self, object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool, output_statics: bool) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string(object, style, output_transients, output_statics, null);
	}

	pub fn to_string<T>(&self, object: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool, output_statics: bool, exclude_null_values: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::new(object, style, null, reflect_up_to_class, output_transients, output_statics, exclude_null_values).to_string();
	}

	pub fn to_string<T>(&self, object: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool, output_statics: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::new(object, style, null, reflect_up_to_class, output_transients, output_statics).to_string();
	}

	pub fn to_string_exclude(&self, object: &/* Java */ java::lang::Object /**/, exclude_field_names: &/* Java */ java::util::Collection /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string_exclude(object, &org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_no_null_string_array(exclude_field_names));
	}

	pub fn to_string_exclude(&self, object: &/* Java */ java::lang::Object /**/, exclude_field_names: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::new(object).set_exclude_field_names(exclude_field_names).to_string();
	}

	pub fn to_string_include(&self, object: &/* Java */ java::lang::Object /**/, include_field_names: &/* Java */ java::util::Collection /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_string_include(object, &org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_no_null_string_array(include_field_names));
	}

	pub fn to_string_include(&self, object: &/* Java */ java::lang::Object /**/, include_field_names: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::new(object).set_include_field_names(include_field_names).to_string();
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		super(object);
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		super(object, style);
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, buffer: &/* Java */ java::lang::StringBuffer /**/) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		super(object, style, buffer);
	}

	pub fn new<T>(object: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, buffer: &/* Java */ java::lang::StringBuffer /**/, reflect_up_to_class: &/* Java */ java::lang::Class /**/, output_transients: bool, output_statics: bool) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		super(object, style, buffer);
		self.set_up_to_class(reflect_up_to_class)?;
		self.set_append_transients(output_transients);
		self.set_append_statics(output_statics);
	}

	pub fn new<T>(object: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, buffer: &/* Java */ java::lang::StringBuffer /**/, reflect_up_to_class: &/* Java */ java::lang::Class /**/, output_transients: bool, output_statics: bool, exclude_null_values: bool) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		super(object, style, buffer);
		self.set_up_to_class(reflect_up_to_class)?;
		self.set_append_transients(output_transients);
		self.set_append_statics(output_statics);
		self.set_exclude_null_values(exclude_null_values);
	}

	fn accept(&self, field: &/* Java */ java::lang::reflect::Field /**/) -> bool {
		if field.getName().indexOf(ClassUtils::INNER_CLASS_SEPARATOR_CHAR) != -1 {
			// Reject field from inner class.
			return false;
		}
		if Modifier::isTransient(&field.getModifiers()) && !self.is_append_transients() {
			// Reject transient fields.
			return false;
		}
		if Modifier::isStatic(&field.getModifiers()) && !self.is_append_statics() {
			// Reject static fields.
			return false;
		}
		if self.excludeFieldNames != null && Arrays::binarySearch(self.excludeFieldNames, &field.getName()) >= 0 {
			// Reject fields from the getExcludeFieldNames list.
			return false;
		}
		if ArrayUtils::is_not_empty(self.include_field_names) {
			// Accept fields from the getIncludeFieldNames list. {@code null} or empty means all fields are included. All fields are included by default.
			return Arrays::binarySearch(self.includeFieldNames, &field.getName()) >= 0;
		}
		return !field.isAnnotationPresent(ToStringExclude.class);
	}

	fn append_fields_in(&self, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalStateException) */ {
		if clazz.isArray() {
			self.reflection_append_array(&self.get_object());
			return;
		}
		// The elements in the returned array are not sorted and are not in any particular order.
		/* final */ let fields: Vec<Field> = ArraySorter::sort(&clazz.getDeclaredFields(), &Comparator::comparing(Field::getName));
		AccessibleObject::setAccessible(fields, true);
		for /* final */ field in fields {
			/* final */ let field_name: String = field.getName();
			if self.accept(field) {
				let r0 = 'try0: {
					// Warning: Field.get(Object) creates wrappers objects
					// for primitive types.
					/* final */ let field_value: Object = self.get_value(field);
					if !self.exclude_null_values || field_value != null {
						self.append(field_name, field_value, !field.isAnnotationPresent(ToStringSummary.class));
					}
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IllegalAccessException) => {
						// impossible happens.
						break 'try0 Err(IllegalStateException::new(e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
	}

	pub fn get_exclude_field_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.excludeFieldNames.clone();
	}

	pub fn get_include_field_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.includeFieldNames.clone();
	}

	pub fn get_up_to_class(&self) -> /* Java */ java::lang::Class /**/ {
		return self.upToClass;
	}

	fn get_value(&self, field: &/* Java */ java::lang::reflect::Field /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return field.get(&self.get_object());
	}

	pub fn is_append_statics(&self) -> bool {
		return self.appendStatics;
	}

	pub fn is_append_transients(&self) -> bool {
		return self.appendTransients;
	}

	pub fn is_exclude_null_values(&self) -> bool {
		return self.excludeNullValues;
	}

	pub fn reflection_append_array(&self, array: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		self.get_style().reflection_append_array_detail(&self.get_string_buffer(), null, array);
		return self;
	}

	pub fn set_append_statics(&mut self, append_statics: bool) {
		self.appendStatics = append_statics;
	}

	pub fn set_append_transients(&mut self, append_transients: bool) {
		self.appendTransients = append_transients;
	}

	pub fn set_exclude_field_names(&mut self, exclude_field_names_param: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		if exclude_field_names_param == null {
			self.excludeFieldNames = null;
		} else {
			// clone and remove nulls
			self.excludeFieldNames = ArraySorter::sort(&org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_no_null_string_array(exclude_field_names_param));
		}
		return self;
	}

	pub fn set_exclude_null_values(&mut self, exclude_null_values: bool) {
		self.excludeNullValues = exclude_null_values;
	}

	pub fn set_include_field_names(&mut self, include_field_names_param: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder {
		if include_field_names_param == null {
			self.includeFieldNames = null;
		} else {
			// clone and remove nulls
			self.includeFieldNames = ArraySorter::sort(&org::apache::commons::lang3::builder::reflection_to_string_builder::ReflectionToStringBuilder::to_no_null_string_array(include_field_names_param));
		}
		return self;
	}

	pub fn set_up_to_class(&mut self, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		if clazz != null {
			/* final */ let object: Object = self.get_object();
			if object != null && !clazz.isInstance(object) {
				return Err(IllegalArgumentException::new("Specified class is not a superclass of the object"));
			}
		}
		self.upToClass = clazz;
	}

	pub fn to_string(&self) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		if self.get_object() == null {
			return self.get_style().get_null_text();
		}
		self.validate()?;
		let clazz: Class<?> = self.get_object().getClass();
		self.append_fields_in(clazz)?;
		while clazz.getSuperclass() != null && clazz != self.get_up_to_class() {
			clazz = clazz.getSuperclass();
			self.append_fields_in(clazz)?;
		}
		return super.to_string();
	}

	fn validate(&self) /* thrown(java.lang.IllegalStateException) */ {
		if ArrayUtils::contains_any(self.excludeFieldNames, self.includeFieldNames as Vec<Object>) {
			ToStringStyle::unregister(&self.get_object());
			return Err(IllegalStateException::new("includeFieldNames and excludeFieldNames must not intersect"));
		}
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for ReflectionToStringBuilder {}