use java::util::Objects;
use crate::org::apache::commons::lang3::ObjectUtils;

pub struct ToStringBuilder {
	buffer: /* Java */ java::lang::StringBuffer /**/,
	object: /* Java */ java::lang::Object /**/,
	style: org::apache::commons::lang3::builder::to_string_style::ToStringStyle,
}

impl ToStringBuilder {
	static defaultStyle: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = ToStringStyle::org::apache::commons::lang3::builder::to_string_style::ToStringStyle::DEFAULT_STYLE;

	pub fn get_default_style(&self) -> org::apache::commons::lang3::builder::to_string_style::ToStringStyle {
		return self.default_style;
	}

	pub fn reflection_to_string(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::to_string(object);
	}

	pub fn reflection_to_string(&self, object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::to_string(object, style);
	}

	pub fn reflection_to_string(&self, object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::to_string(object, style, output_transients, false, null);
	}

	pub fn reflection_to_string<T>(&self, object: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, output_transients: bool, reflect_up_to_class: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return ReflectionToStringBuilder::to_string(object, style, output_transients, false, reflect_up_to_class);
	}

	pub fn set_default_style(&mut self, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) {
		self.default_style = Objects::requireNonNull(style, "style");
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		this(object, null, null);
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		this(object, style, null);
	}

	pub fn new(object: &/* Java */ java::lang::Object /**/, mut style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, mut buffer: &/* Java */ java::lang::StringBuffer /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		if style == null {
			style = org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder::get_default_style();
		}
		if buffer == null {
			buffer = StringBuffer::new(512);
		}
		self.buffer = buffer;
		self.style = style;
		self.object = object;
		style.append_start(buffer, object);
	}

	pub fn append(&self, value: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[bool]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: i8) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[i8]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: u16) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[u16]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: f64) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[f64]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: f32) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[f32]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: i32) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[i32]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: i64) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[i64]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, obj: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, obj, null);
		return self;
	}

	pub fn append(&self, array: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, value: i16) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, value);
		return self;
	}

	pub fn append(&self, array: &&[i16]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, null, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[bool]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[bool], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: i8) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i8]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i8], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: u16) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[u16]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[u16], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: f64) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[f64]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[f64], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: f32) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[f32]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[f32], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: i32) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i32]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i32], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: i64) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i64]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i64], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, obj: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, obj, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, obj: &/* Java */ java::lang::Object /**/, full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, obj, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, value: i16) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, value);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i16]) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, null);
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, array: &&[i16], full_detail: bool) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		self.style.append(self.buffer, field_name, array, &Boolean::valueOf(full_detail));
		return self;
	}

	pub fn append_as_object_to_string(&self, src_object: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		ObjectUtils::identity_to_string(&self.get_string_buffer(), src_object);
		return self;
	}

	pub fn append_super(&self, super_to_string: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		if super_to_string != null {
			self.style.append_super(self.buffer, super_to_string);
		}
		return self;
	}

	pub fn append_to_string(&self, to_string: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::to_string_builder::ToStringBuilder {
		if to_string != null {
			self.style.append_to_string(self.buffer, to_string);
		}
		return self;
	}

	pub fn build(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_string();
	}

	pub fn get_object(&self) -> /* Java */ java::lang::Object /**/ {
		return self.object;
	}

	pub fn get_string_buffer(&self) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.buffer;
	}

	pub fn get_style(&self) -> org::apache::commons::lang3::builder::to_string_style::ToStringStyle {
		return self.style;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		if self.get_object() == null {
			self.get_string_buffer().append(&self.get_style().get_null_text());
		} else {
			self.style.append_end(&self.get_string_buffer(), &self.get_object());
		}
		return self.get_string_buffer().toString();
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for ToStringBuilder {}