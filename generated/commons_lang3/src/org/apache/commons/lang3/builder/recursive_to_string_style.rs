use java::util::Collection;
use crate::org::apache::commons::lang3::ClassUtils;

pub struct RecursiveToStringStyle;

impl RecursiveToStringStyle {
	static serialVersionUID: i64 = 1;

	pub fn new() -> org::apache::commons::lang3::builder::recursive_to_string_style::RecursiveToStringStyle {
	}

	fn accept(&self, clazz: &/* Java */ java::lang::Class /**/) -> bool {
		return true;
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, coll: &/* Java */ java::util::Collection /**/) {
		self.append_class_name(buffer, coll);
		self.append_identity_hash_code(buffer, coll);
		self.append_detail(buffer, field_name, &coll.toArray());
	}

	pub fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		if !ClassUtils::is_primitive_wrapper(&value.getClass()) && !String.class.equals(&value.getClass()) && self.accept(&value.getClass()) {
			buffer.append(&ReflectionToStringBuilder::to_string(value, self));
		} else {
			super.append_detail(buffer, field_name, value);
		}
	}
}

impl /* Java */ java::io::Serializable /**/ for RecursiveToStringStyle {}