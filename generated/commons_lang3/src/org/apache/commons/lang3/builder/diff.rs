use java::lang::reflect::Type;
use java::util::Objects;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::reflect::TypeUtils;
use crate::org::apache::commons::lang3::tuple::Pair;

pub struct Diff<T> {
	type: /* Java */ java::lang::reflect::Type /**/,
	field_name: /* Java */ java::lang::String /**/,
}

impl<T> Diff {
	static serialVersionUID: i64 = 1;

	fn new(field_name: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::diff::Diff {
		self.fieldName = Objects::requireNonNull(field_name);
		self.type = ObjectUtils::get_if_null(&TypeUtils::get_type_arguments(&self.getClass(), Diff.class).get(Diff.class.getTypeParameters()[0]), Object.class);
	}

	fn new(field_name: &/* Java */ java::lang::String /**/, type: &/* Java */ java::lang::reflect::Type /**/) -> org::apache::commons::lang3::builder::diff::Diff {
		self.fieldName = Objects::requireNonNull(field_name);
		self.type = Objects::requireNonNull(type);
	}

	pub fn get_field_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.field_name;
	}

	pub fn get_type(&self) -> /* Java */ java::lang::reflect::Type /**/ {
		return self.type;
	}

	pub fn set_value(&self, value: &T) /* thrown(java.lang.UnsupportedOperationException) */ -> T {
		return Err(UnsupportedOperationException::new("Cannot alter Diff object."));
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("[%s: %s, %s]", self.field_name, &self.get_left(), &self.get_right());
	}
}

impl<T> /* Java */ java::util::Map::Entry /**/ for Diff<T> {}

impl<T> /* Java */ java::lang::Comparable /**/ for Diff<T> {}

impl<T> /* Java */ java::io::Serializable /**/ for Diff<T> {}