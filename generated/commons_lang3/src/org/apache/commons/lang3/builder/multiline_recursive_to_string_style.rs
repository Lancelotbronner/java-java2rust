use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::StringUtils;

pub struct MultilineRecursiveToStringStyle {
	spaces: i32 = 2,
}

impl MultilineRecursiveToStringStyle {
	static serialVersionUID: i64 = 1;

	static INDENT: i32 = 2;

	pub fn new() -> org::apache::commons::lang3::builder::multiline_recursive_to_string_style::MultilineRecursiveToStringStyle {
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[bool]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.append_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i8]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.appendDetail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[u16]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.appendDetail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f64]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.append_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f32]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.appendDetail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i32]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.append_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i64]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.appendDetail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	pub fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		if !ClassUtils::is_primitive_wrapper(&value.getClass()) && !String.class.equals(&value.getClass()) && self.accept(&value.getClass()) {
			self.spaces += self.INDENT;
			self.reset_indent();
			buffer.append(&ReflectionToStringBuilder::to_string(value, self));
			self.spaces -= self.INDENT;
			self.reset_indent();
		} else {
			super.append_detail(buffer, field_name, value);
		}
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.append_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn append_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i16]) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.append_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn reflection_append_array_detail(&mut self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &/* Java */ java::lang::Object /**/) {
		self.spaces += self.INDENT;
		self.reset_indent();
		super.reflection_append_array_detail(buffer, field_name, array);
		self.spaces -= self.INDENT;
		self.reset_indent();
	}

	fn reset_indent(&self) {
		self.set_array_start("{" + System::lineSeparator() + self.spacer(self.spaces));
		self.set_array_separator("," + System::lineSeparator() + self.spacer(self.spaces));
		self.set_array_end(System::lineSeparator() + self.spacer(self.spaces - self.INDENT) + "}");
		self.set_content_start("[" + System::lineSeparator() + self.spacer(self.spaces));
		self.set_field_separator("," + System::lineSeparator() + self.spacer(self.spaces));
		self.set_content_end(System::lineSeparator() + self.spacer(self.spaces - self.INDENT) + "]");
	}

	fn spacer(&self, spaces: i32) -> /* Java */ java::lang::String /**/ {
		return StringUtils::repeat(' ', spaces);
	}
}

impl /* Java */ java::io::Serializable /**/ for MultilineRecursiveToStringStyle {}