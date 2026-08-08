use java::lang::reflect::Field;
use java::lang::reflect::Modifier;
use java::util::Arrays;
use crate::org::apache::commons::lang3::ArraySorter;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::reflect::FieldUtils;

pub struct ReflectionDiffBuilder<T> {
	diff_builder: org::apache::commons::lang3::builder::diff_builder::DiffBuilder,
	exclude_field_names: &[/* Java */ java::lang::String /**/],
}

impl<T> ReflectionDiffBuilder {
	pub fn builder<T>(&self) -> org::apache::commons::lang3::builder::reflection_diff_builder::Builder {
		return Builder<>::new();
	}

	fn to_exclude_field_names(&self, exclude_field_names: &&[/* Java */ java::lang::String /**/]) -> &[/* Java */ java::lang::String /**/] {
		if exclude_field_names == null {
			return ArrayUtils::EMPTY_STRING_ARRAY;
		}
		// clone and remove nulls
		return ArraySorter::sort(&ReflectionToStringBuilder::to_no_null_string_array(exclude_field_names));
	}

	fn new(diff_builder: &org::apache::commons::lang3::builder::diff_builder::DiffBuilder, exclude_field_names: &&[/* Java */ java::lang::String /**/]) -> org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder {
		self.diffBuilder = diff_builder;
		self.excludeFieldNames = exclude_field_names;
	}

	pub fn new(left: &T, right: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder {
		this(&DiffBuilder<T>::builder().set_left(left).set_right(right).set_style(style).build(), null);
	}

	fn accept(&self, field: &/* Java */ java::lang::reflect::Field /**/) -> bool {
		if field.getName().indexOf(ClassUtils::INNER_CLASS_SEPARATOR_CHAR) != -1 {
			return false;
		}
		if Modifier::isTransient(&field.getModifiers()) {
			return false;
		}
		if Modifier::isStatic(&field.getModifiers()) {
			return false;
		}
		if self.excludeFieldNames != null && Arrays::binarySearch(self.excludeFieldNames, &field.getName()) >= 0 {
			// Reject fields from the getExcludeFieldNames list.
			return false;
		}
		return !field.isAnnotationPresent(DiffExclude.class);
	}

	fn append_fields(&self, clazz: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ {
		for /* final */ field in FieldUtils::get_all_fields(clazz) {
			if self.accept(field) {
				let r0 = 'try0: {
					self.diff_builder.append(&field.getName(), &self.read_field(field, &self.get_left()), &self.read_field(field, &self.get_right()));
					break 'try0 Ok(());
				};
				match r0 {
					Err(e @ IllegalAccessException) => {
						// throw a runtime exception in case the impossible happens.
						break 'try0 Err(IllegalArgumentException::new("Unexpected IllegalAccessException: " + e.getMessage(), e));
					},
					Err(e) => Err(e)?,
					Ok => (),
				}
			}
		}
	}

	pub fn build(&self) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::builder::diff_result::DiffResult {
		if self.get_left().equals(&self.get_right()) {
			return self.diff_builder.build();
		}
		self.append_fields(&self.get_left().getClass())?;
		return self.diff_builder.build();
	}

	pub fn get_exclude_field_names(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.excludeFieldNames.clone();
	}

	fn get_left(&self) -> T {
		return self.diff_builder.get_left();
	}

	fn get_right(&self) -> T {
		return self.diff_builder.get_right();
	}

	fn read_field(&self, field: &/* Java */ java::lang::reflect::Field /**/, target: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::Object /**/ {
		return FieldUtils::read_field(field, target, true);
	}

	pub fn set_exclude_field_names(&mut self, exclude_field_names: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder {
		self.excludeFieldNames = org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder::to_exclude_field_names(exclude_field_names);
		return self;
	}
}

impl<T> org::apache::commons::lang3::builder::builder::Builder for ReflectionDiffBuilder<T> {}

pub struct Builder<T> {
	exclude_field_names: &[/* Java */ java::lang::String /**/] = ArrayUtils::EMPTY_STRING_ARRAY,
	diff_builder: org::apache::commons::lang3::builder::diff_builder::DiffBuilder,
}

impl<T> Builder {
	pub fn new() -> org::apache::commons::lang3::builder::reflection_diff_builder::Builder {
	// empty
	}

	pub fn build(&self) -> org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder {
		return ReflectionDiffBuilder<>::new(self.diff_builder, self.exclude_field_names);
	}

	pub fn set_diff_builder(&mut self, diff_builder: &org::apache::commons::lang3::builder::diff_builder::DiffBuilder) -> org::apache::commons::lang3::builder::reflection_diff_builder::Builder {
		self.diffBuilder = diff_builder;
		return self;
	}

	pub fn set_exclude_field_names(&mut self, exclude_field_names: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::reflection_diff_builder::Builder {
		self.excludeFieldNames = org::apache::commons::lang3::builder::reflection_diff_builder::ReflectionDiffBuilder::to_exclude_field_names(exclude_field_names);
		return self;
	}
}