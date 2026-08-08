use java::io::Serializable;
use java::lang::reflect::Array;
use java::util::Collection;
use java::util::Map;
use java::util::Map::Entry;
use java::util::Objects;
use java::util::WeakHashMap;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::StringEscapeUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::Strings;

pub struct ToStringStyle {
	use_field_names: bool = true,
	use_class_name: bool = true,
	use_short_class_name: bool,
	use_identity_hash_code: bool = true,
	content_start: /* Java */ java::lang::String /**/ = "[",
	content_end: /* Java */ java::lang::String /**/ = "]",
	field_name_value_separator: /* Java */ java::lang::String /**/ = "=",
	field_separator_at_start: bool,
	field_separator_at_end: bool,
	field_separator: /* Java */ java::lang::String /**/ = ",",
	array_start: /* Java */ java::lang::String /**/ = "{",
	array_separator: /* Java */ java::lang::String /**/ = ",",
	array_content_detail: bool = true,
	array_end: /* Java */ java::lang::String /**/ = "}",
	default_full_detail: bool = true,
	null_text: /* Java */ java::lang::String /**/ = "<null>",
	size_start_text: /* Java */ java::lang::String /**/ = "<size=",
	size_end_text: /* Java */ java::lang::String /**/ = ">",
	summary_object_start_text: /* Java */ java::lang::String /**/ = "<",
	summary_object_end_text: /* Java */ java::lang::String /**/ = ">",
}

impl ToStringStyle {
	static serialVersionUID: i64 = -2587890625525655916;

	pub static DEFAULT_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = DefaultToStringStyle::new();

	pub static MULTI_LINE_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = MultiLineToStringStyle::new();

	pub static NO_FIELD_NAMES_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = NoFieldNameToStringStyle::new();

	pub static SHORT_PREFIX_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = ShortPrefixToStringStyle::new();

	pub static SIMPLE_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = SimpleToStringStyle::new();

	pub static NO_CLASS_NAME_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = NoClassNameToStringStyle::new();

	pub static JSON_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = JsonToStringStyle::new();

	static REGISTRY: /* Java */ java::lang::ThreadLocal /**/ = ThreadLocal::withInitial(WeakHashMap::new);

	pub fn get_registry(&self) -> /* Java */ java::util::Map /**/ {
		return self.REGISTRY.get();
	}

	fn is_registered(&self, value: &/* Java */ java::lang::Object /**/) -> bool {
		return org::apache::commons::lang3::builder::to_string_style::ToStringStyle::get_registry().containsKey(value);
	}

	fn register(&self, value: &/* Java */ java::lang::Object /**/) {
		if value != null {
			org::apache::commons::lang3::builder::to_string_style::ToStringStyle::get_registry().put(value, null);
		}
	}

	fn unregister(&self, value: &/* Java */ java::lang::Object /**/) {
		if value != null {
			/* final */ let m: Map<Object, Object> = org::apache::commons::lang3::builder::to_string_style::ToStringStyle::get_registry();
			m.remove(value);
			if m.isEmpty() {
				self.REGISTRY.remove();
			}
		}
	}

	fn new() -> org::apache::commons::lang3::builder::to_string_style::ToStringStyle {
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: bool) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[bool], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i8) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i8], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: u16) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[u16], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			.appendSummary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: f64) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f64], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: f32) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f32], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i32) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i32], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			.appendSummary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i64) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i64], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			.appendSummary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if value == null {
			self.append_null_text(buffer, field_name);
		} else {
			self.append_internal(buffer, field_name, value, &self.is_full_detail(full_detail));
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			self.append_detail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i16) {
		self.append_field_start(buffer, field_name);
		self.append_detail(buffer, field_name, value);
		self.append_field_end(buffer, field_name);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i16], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.append_field_start(buffer, field_name);
		if array == null {
			self.append_null_text(buffer, field_name);
		} else if self.is_full_detail(full_detail) {
			.appendDetail(buffer, field_name, array);
		} else {
			self.append_summary(buffer, field_name, array);
		}
		self.append_field_end(buffer, field_name);
	}

	fn append_class_name(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, object: &/* Java */ java::lang::Object /**/) {
		if self.is_use_class_name() && object != null {
			org::apache::commons::lang3::builder::to_string_style::ToStringStyle::register(object);
			if self.is_use_short_class_name() {
				buffer.append(&self.get_short_class_name(&object.getClass()));
			} else {
				buffer.append(&object.getClass().getName());
			}
		}
	}

	fn append_content_end(&self, buffer: &/* Java */ java::lang::StringBuffer /**/) {
		buffer.append(&self.get_content_end());
	}

	fn append_content_start(&self, buffer: &/* Java */ java::lang::StringBuffer /**/) {
		buffer.append(&self.get_content_start());
	}

	fn append_cyclic_object(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		ObjectUtils::identity_to_string(buffer, value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: bool) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[bool]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i8) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i8]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: u16) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[u16]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, coll: &/* Java */ java::util::Collection /**/) {
		buffer.append(coll);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: f64) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f64]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: f32) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f32]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i32) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, i: i32, item: &/* Java */ java::lang::Object /**/) {
		if i > 0 {
			buffer.append(&self.get_array_separator());
		}
		if item == null {
			self.append_null_text(buffer, field_name);
		} else {
			self.append_internal(buffer, field_name, item, &self.is_array_content_detail());
		}
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i32]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i64) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i64]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, map: &/* Java */ java::util::Map /**/) {
		buffer.append(map);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					self.append_detail(buffer, field_name, i, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: i16) {
		buffer.append(value);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i16]) {
		buffer.append(&self.get_array_start());
		 {
			let i: i32 = 0;
			while i < array.length {
				{
					if i > 0 {
						buffer.append(&self.get_array_separator());
					}
					self.append_detail(buffer, field_name, array[i]);
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	pub fn append_end(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, object: &/* Java */ java::lang::Object /**/) {
		if !self.is_field_separator_at_end() {
			self.remove_last_field_separator(buffer);
		}
		self.append_content_end(buffer);
		org::apache::commons::lang3::builder::to_string_style::ToStringStyle::unregister(object);
	}

	fn append_field_end(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/) {
		self.append_field_separator(buffer);
	}

	fn append_field_separator(&self, buffer: &/* Java */ java::lang::StringBuffer /**/) {
		buffer.append(&self.get_field_separator());
	}

	fn append_field_start(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/) {
		if self.is_use_field_names() && field_name != null {
			buffer.append(field_name);
			buffer.append(&self.get_field_name_value_separator());
		}
	}

	fn append_identity_hash_code(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, object: &/* Java */ java::lang::Object /**/) {
		if self.is_use_identity_hash_code() && object != null {
			org::apache::commons::lang3::builder::to_string_style::ToStringStyle::register(object);
			buffer.append('@');
			buffer.append(&ObjectUtils::identity_hash_code_hex(object));
		}
	}

	fn append_internal(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, detail: bool) {
		if org::apache::commons::lang3::builder::to_string_style::ToStringStyle::is_registered(value) && !(value instanceof Number || value instanceof Boolean || value instanceof Character) {
			self.append_cyclic_object(buffer, field_name, value);
			return;
		}
		org::apache::commons::lang3::builder::to_string_style::ToStringStyle::register(value);
		let r0 = 'try0: {
			if value instanceof Collection<?> {
				if detail {
					self.append_detail(buffer, field_name, value as Collection<?>);
				} else {
					self.append_summary_size(buffer, field_name, &(value as Collection<?>).size());
				}
			} else if value instanceof Map<?, ?> {
				if detail {
					self.append_detail(buffer, field_name, value as Map<?, ?>);
				} else {
					self.append_summary_size(buffer, field_name, &(value as Map<?, ?>).size());
				}
			} else if value instanceof Vec<i64> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<i64>);
				} else {
					self.append_summary(buffer, field_name, value as Vec<i64>);
				}
			} else if value instanceof Vec<i32> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<i32>);
				} else {
					.appendSummary(buffer, field_name, value as Vec<i32>);
				}
			} else if value instanceof Vec<i16> {
				if detail {
					.appendDetail(buffer, field_name, value as Vec<i16>);
				} else {
					.appendSummary(buffer, field_name, value as Vec<i16>);
				}
			} else if value instanceof Vec<i8> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<i8>);
				} else {
					.appendSummary(buffer, field_name, value as Vec<i8>);
				}
			} else if value instanceof Vec<char> {
				if detail {
					.appendDetail(buffer, field_name, value as Vec<char>);
				} else {
					.appendSummary(buffer, field_name, value as Vec<char>);
				}
			} else if value instanceof Vec<f64> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<f64>);
				} else {
					self.append_summary(buffer, field_name, value as Vec<f64>);
				}
			} else if value instanceof Vec<f32> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<f32>);
				} else {
					self.append_summary(buffer, field_name, value as Vec<f32>);
				}
			} else if value instanceof Vec<bool> {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<bool>);
				} else {
					self.append_summary(buffer, field_name, value as Vec<bool>);
				}
			} else if ObjectUtils::is_array(value) {
				if detail {
					self.append_detail(buffer, field_name, value as Vec<Object>);
				} else {
					self.append_summary(buffer, field_name, value as Vec<Object>);
				}
			} else if detail {
				self.append_detail(buffer, field_name, value);
			} else {
				self.append_summary(buffer, field_name, value);
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		org::apache::commons::lang3::builder::to_string_style::ToStringStyle::unregister(value);
	
	}

	fn append_null_text(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/) {
		buffer.append(&self.get_null_text());
	}

	pub fn append_start(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, object: &/* Java */ java::lang::Object /**/) {
		if object != null {
			self.append_class_name(buffer, object);
			self.append_identity_hash_code(buffer, object);
			self.append_content_start(buffer);
			if self.is_field_separator_at_start() {
				self.append_field_separator(buffer);
			}
		}
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[bool]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i8]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[u16]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f64]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f32]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i32]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i64]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		buffer.append(&self.get_summary_object_start_text());
		buffer.append(&self.get_short_class_name(&value.getClass()));
		buffer.append(&self.get_summary_object_end_text());
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i16]) {
		self.append_summary_size(buffer, field_name, array.length);
	}

	fn append_summary_size(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, size: i32) {
		buffer.append(&self.get_size_start_text());
		buffer.append(size);
		buffer.append(&self.get_size_end_text());
	}

	pub fn append_super(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, super_to_string: &/* Java */ java::lang::String /**/) {
		self.append_to_string(buffer, super_to_string);
	}

	pub fn append_to_string(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, to_string: &/* Java */ java::lang::String /**/) {
		if to_string != null {
			/* final */ let pos1: i32 = to_string.indexOf(&self.get_content_start()) + self.get_content_start().length();
			/* final */ let pos2: i32 = to_string.lastIndexOf(&self.get_content_end());
			if pos1 != pos2 && pos1 >= 0 && pos2 >= 0 {
				if self.is_field_separator_at_start() {
					self.remove_last_field_separator(buffer);
				}
				buffer.append(to_string, pos1, pos2);
				self.append_field_separator(buffer);
			}
		}
	}

	fn get_array_end(&self) -> /* Java */ java::lang::String /**/ {
		return self.array_end;
	}

	fn get_array_separator(&self) -> /* Java */ java::lang::String /**/ {
		return self.array_separator;
	}

	fn get_array_start(&self) -> /* Java */ java::lang::String /**/ {
		return self.array_start;
	}

	fn get_content_end(&self) -> /* Java */ java::lang::String /**/ {
		return self.content_end;
	}

	fn get_content_start(&self) -> /* Java */ java::lang::String /**/ {
		return self.content_start;
	}

	fn get_field_name_value_separator(&self) -> /* Java */ java::lang::String /**/ {
		return self.field_name_value_separator;
	}

	fn get_field_separator(&self) -> /* Java */ java::lang::String /**/ {
		return self.field_separator;
	}

	fn get_null_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.null_text;
	}

	fn get_short_class_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return ClassUtils::get_short_class_name(cls);
	}

	fn get_size_end_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.size_end_text;
	}

	fn get_size_start_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.size_start_text;
	}

	fn get_summary_object_end_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.summary_object_end_text;
	}

	fn get_summary_object_start_text(&self) -> /* Java */ java::lang::String /**/ {
		return self.summary_object_start_text;
	}

	fn is_array_content_detail(&self) -> bool {
		return self.array_content_detail;
	}

	fn is_default_full_detail(&self) -> bool {
		return self.default_full_detail;
	}

	fn is_field_separator_at_end(&self) -> bool {
		return self.field_separator_at_end;
	}

	fn is_field_separator_at_start(&self) -> bool {
		return self.field_separator_at_start;
	}

	fn is_full_detail(&self, full_detail_request: &/* Java */ java::lang::Boolean /**/) -> bool {
		if full_detail_request == null {
			return self.is_default_full_detail();
		}
		return full_detail_request.booleanValue();
	}

	fn is_use_class_name(&self) -> bool {
		return self.use_class_name;
	}

	fn is_use_field_names(&self) -> bool {
		return self.use_field_names;
	}

	fn is_use_identity_hash_code(&self) -> bool {
		return self.use_identity_hash_code;
	}

	fn is_use_short_class_name(&self) -> bool {
		return self.use_short_class_name;
	}

	fn reflection_append_array_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &/* Java */ java::lang::Object /**/) {
		buffer.append(&self.get_array_start());
		/* final */ let length: i32 = Array::getLength(array);
		 {
			let i: i32 = 0;
			while i < length {
				{
					self.append_detail(buffer, field_name, i, &Array::get(array, i));
				}
				i += 1;
			 }
		 }
	
		buffer.append(&self.get_array_end());
	}

	fn remove_last_field_separator(&self, buffer: &/* Java */ java::lang::StringBuffer /**/) {
		if Strings::org::apache::commons::lang3::strings::Strings::CS.ends_with(buffer, &self.get_field_separator()) {
			buffer.setLength(buffer.length() - self.get_field_separator().length());
		}
	}

	fn set_array_content_detail(&mut self, array_content_detail: bool) {
		self.arrayContentDetail = array_content_detail;
	}

	fn set_array_end(&mut self, array_end: &/* Java */ java::lang::String /**/) {
		self.arrayEnd = ObjectUtils::to_string(array_end);
	}

	fn set_array_separator(&mut self, array_separator: &/* Java */ java::lang::String /**/) {
		self.arraySeparator = ObjectUtils::to_string(array_separator);
	}

	fn set_array_start(&mut self, array_start: &/* Java */ java::lang::String /**/) {
		self.arrayStart = ObjectUtils::to_string(array_start);
	}

	fn set_content_end(&mut self, content_end: &/* Java */ java::lang::String /**/) {
		self.contentEnd = ObjectUtils::to_string(content_end);
	}

	fn set_content_start(&mut self, content_start: &/* Java */ java::lang::String /**/) {
		self.contentStart = ObjectUtils::to_string(content_start);
	}

	fn set_default_full_detail(&mut self, default_full_detail: bool) {
		self.defaultFullDetail = default_full_detail;
	}

	fn set_field_name_value_separator(&mut self, field_name_value_separator: &/* Java */ java::lang::String /**/) {
		self.fieldNameValueSeparator = ObjectUtils::to_string(field_name_value_separator);
	}

	fn set_field_separator(&mut self, field_separator: &/* Java */ java::lang::String /**/) {
		self.fieldSeparator = ObjectUtils::to_string(field_separator);
	}

	fn set_field_separator_at_end(&mut self, field_separator_at_end: bool) {
		self.fieldSeparatorAtEnd = field_separator_at_end;
	}

	fn set_field_separator_at_start(&mut self, field_separator_at_start: bool) {
		self.fieldSeparatorAtStart = field_separator_at_start;
	}

	fn set_null_text(&mut self, null_text: &/* Java */ java::lang::String /**/) {
		self.nullText = ObjectUtils::to_string(null_text);
	}

	fn set_size_end_text(&mut self, size_end_text: &/* Java */ java::lang::String /**/) {
		self.sizeEndText = ObjectUtils::to_string(size_end_text);
	}

	fn set_size_start_text(&mut self, size_start_text: &/* Java */ java::lang::String /**/) {
		self.sizeStartText = ObjectUtils::to_string(size_start_text);
	}

	fn set_summary_object_end_text(&mut self, summary_object_end_text: &/* Java */ java::lang::String /**/) {
		self.summaryObjectEndText = ObjectUtils::to_string(summary_object_end_text);
	}

	fn set_summary_object_start_text(&mut self, summary_object_start_text: &/* Java */ java::lang::String /**/) {
		self.summaryObjectStartText = ObjectUtils::to_string(summary_object_start_text);
	}

	fn set_use_class_name(&mut self, use_class_name: bool) {
		self.useClassName = use_class_name;
	}

	fn set_use_field_names(&mut self, use_field_names: bool) {
		self.useFieldNames = use_field_names;
	}

	fn set_use_identity_hash_code(&mut self, use_identity_hash_code: bool) {
		self.useIdentityHashCode = use_identity_hash_code;
	}

	fn set_use_short_class_name(&mut self, use_short_class_name: bool) {
		self.useShortClassName = use_short_class_name;
	}
}

impl /* Java */ java::io::Serializable /**/ for ToStringStyle {}

struct DefaultToStringStyle;

impl DefaultToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::DefaultToStringStyle {
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for DefaultToStringStyle {}

struct JsonToStringStyle;

impl JsonToStringStyle {
	static serialVersionUID: i64 = 1;

	static FIELD_NAME_QUOTE: /* Java */ java::lang::String /**/ = "\"";

	fn new() -> org::apache::commons::lang3::builder::to_string_style::JsonToStringStyle {
		self.set_use_class_name(false);
		self.set_use_identity_hash_code(false);
		self.set_content_start("{");
		self.set_content_end("}");
		self.set_array_start("[");
		self.set_array_end("]");
		self.set_field_separator(",");
		self.set_field_name_value_separator(":");
		self.set_null_text("null");
		self.set_summary_object_start_text("\"<");
		self.set_summary_object_end_text(">\"");
		self.set_size_start_text("\"<size=");
		self.set_size_end_text(">\"");
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[bool], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i8], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[u16], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f64], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[f32], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i32], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i64], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/, full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, value, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[/* Java */ java::lang::Object /**/], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	pub fn append(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, array: &&[i16], full_detail: &/* Java */ java::lang::Boolean /**/) {
		self.check_append_input(field_name, full_detail);
		super.append(buffer, field_name, array, full_detail);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: u16) {
		self.append_value_as_string(buffer, &String::valueOf(value));
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, coll: &/* Java */ java::util::Collection /**/) {
		if coll != null && !coll.isEmpty() {
			buffer.append(&self.get_array_start());
			let i: i32 = 0;
			for /* final */ item in coll {
				self.append_detail(buffer, field_name, i += 1 !!!check!!! post increment, item);
			}
			buffer.append(&self.get_array_end());
			return;
		}
		buffer.append(coll);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, map: &/* Java */ java::util::Map /**/) {
		if map != null && !map.isEmpty() {
			buffer.append(&self.get_content_start());
			let first_item: bool = true;
			for /* final */ entry in map.entrySet() {
				/* final */ let key_str: String = Objects::toString(&entry.getKey(), null);
				if key_str != null {
					if first_item {
						first_item = false;
					} else {
						self.append_field_end(buffer, key_str);
					}
					self.append_field_start(buffer, key_str);
					/* final */ let value: Object = entry.getValue();
					if value == null {
						self.append_null_text(buffer, key_str);
					} else {
						self.append_internal(buffer, key_str, value, true);
					}
				}
			}
			buffer.append(&self.get_content_end());
			return;
		}
		buffer.append(map);
	}

	fn append_detail(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) {
		if value == null {
			self.append_null_text(buffer, field_name);
			return;
		}
		if value instanceof String || value instanceof Character {
			self.append_value_as_string(buffer, &value.toString());
			return;
		}
		if value instanceof Number || value instanceof Boolean {
			buffer.append(value);
			return;
		}
		/* final */ let value_as_string: String = value.toString();
		if self.is_json_object(value_as_string) || self.is_json_array(value_as_string) {
			buffer.append(value);
			return;
		}
		self.append_detail(buffer, field_name, value_as_string);
	}

	fn append_field_start(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.UnsupportedOperationException | java.io.UncheckedIOException) */ {
		self.check_field_name(field_name)?;
		super.append_field_start(buffer, self.FIELD_NAME_QUOTE + StringEscapeUtils::escape_json(field_name)? + self.FIELD_NAME_QUOTE);
	}

	fn append_value_as_string(&self, buffer: &/* Java */ java::lang::StringBuffer /**/, value: &/* Java */ java::lang::String /**/) /* thrown(java.io.UncheckedIOException) */ {
		buffer.append('"').append(&StringEscapeUtils::escape_json(value)?).append('"');
	}

	fn check_append_input(&self, field_name: &/* Java */ java::lang::String /**/, full_detail: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		self.check_field_name(field_name)?;
		self.check_is_full_detail(full_detail)?;
	}

	fn check_field_name(&self, field_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		if field_name == null {
			return Err(UnsupportedOperationException::new("Field names are mandatory when using JsonToStringStyle"));
		}
	}

	fn check_is_full_detail(&self, full_detail: &/* Java */ java::lang::Boolean /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		if !self.is_full_detail(full_detail) {
			return Err(UnsupportedOperationException::new("FullDetail must be true when using JsonToStringStyle"));
		}
	}

	fn is_json_array(&self, value_as_string: &/* Java */ java::lang::String /**/) -> bool {
		return value_as_string.startsWith(&self.get_array_start()) && value_as_string.endsWith(&self.get_array_end());
	}

	fn is_json_object(&self, value_as_string: &/* Java */ java::lang::String /**/) -> bool {
		return value_as_string.startsWith(&self.get_content_start()) && value_as_string.endsWith(&self.get_content_end());
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for JsonToStringStyle {}

struct MultiLineToStringStyle;

impl MultiLineToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::MultiLineToStringStyle {
		self.set_content_start("[");
		self.set_field_separator(System::lineSeparator() + "  ");
		self.set_field_separator_at_start(true);
		self.set_content_end(System::lineSeparator() + "]");
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for MultiLineToStringStyle {}

struct NoClassNameToStringStyle;

impl NoClassNameToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::NoClassNameToStringStyle {
		self.set_use_class_name(false);
		self.set_use_identity_hash_code(false);
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for NoClassNameToStringStyle {}

struct NoFieldNameToStringStyle;

impl NoFieldNameToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::NoFieldNameToStringStyle {
		self.set_use_field_names(false);
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for NoFieldNameToStringStyle {}

struct ShortPrefixToStringStyle;

impl ShortPrefixToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::ShortPrefixToStringStyle {
		self.set_use_short_class_name(true);
		self.set_use_identity_hash_code(false);
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for ShortPrefixToStringStyle {}

struct SimpleToStringStyle;

impl SimpleToStringStyle {
	static serialVersionUID: i64 = 1;

	fn new() -> org::apache::commons::lang3::builder::to_string_style::SimpleToStringStyle {
		self.set_use_class_name(false);
		self.set_use_identity_hash_code(false);
		self.set_use_field_names(false);
		self.set_content_start(StringUtils::EMPTY);
		self.set_content_end(StringUtils::EMPTY);
	}

	fn read_resolve(&self) -> /* Java */ java::lang::Object /**/ {
		return ;
	}
}

impl /* Java */ java::io::Serializable /**/ for SimpleToStringStyle {}