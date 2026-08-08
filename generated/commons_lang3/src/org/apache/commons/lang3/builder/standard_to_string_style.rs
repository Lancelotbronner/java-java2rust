use java::lang::reflect::Array;
use java::util::Collection;
use java::util::Map;

pub struct StandardToStringStyle;

impl StandardToStringStyle {
	static serialVersionUID: i64 = 1;

	pub fn new() -> org::apache::commons::lang3::builder::standard_to_string_style::StandardToStringStyle {
	}

	pub fn get_array_end(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_array_end();
	}

	pub fn get_array_separator(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_array_separator();
	}

	pub fn get_array_start(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_array_start();
	}

	pub fn get_content_end(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_content_end();
	}

	pub fn get_content_start(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_content_start();
	}

	pub fn get_field_name_value_separator(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_field_name_value_separator();
	}

	pub fn get_field_separator(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_field_separator();
	}

	pub fn get_null_text(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_null_text();
	}

	pub fn get_size_end_text(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_size_end_text();
	}

	pub fn get_size_start_text(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_size_start_text();
	}

	pub fn get_summary_object_end_text(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_summary_object_end_text();
	}

	pub fn get_summary_object_start_text(&self) -> /* Java */ java::lang::String /**/ {
		return super.get_summary_object_start_text();
	}

	pub fn is_array_content_detail(&self) -> bool {
		return super.is_array_content_detail();
	}

	pub fn is_default_full_detail(&self) -> bool {
		return super.is_default_full_detail();
	}

	pub fn is_field_separator_at_end(&self) -> bool {
		return super.is_field_separator_at_end();
	}

	pub fn is_field_separator_at_start(&self) -> bool {
		return super.is_field_separator_at_start();
	}

	pub fn is_use_class_name(&self) -> bool {
		return super.is_use_class_name();
	}

	pub fn is_use_field_names(&self) -> bool {
		return super.is_use_field_names();
	}

	pub fn is_use_identity_hash_code(&self) -> bool {
		return super.is_use_identity_hash_code();
	}

	pub fn is_use_short_class_name(&self) -> bool {
		return super.is_use_short_class_name();
	}

	pub fn set_array_content_detail(&self, array_content_detail: bool) {
		super.set_array_content_detail(array_content_detail);
	}

	pub fn set_array_end(&self, array_end: &/* Java */ java::lang::String /**/) {
		super.set_array_end(array_end);
	}

	pub fn set_array_separator(&self, array_separator: &/* Java */ java::lang::String /**/) {
		super.set_array_separator(array_separator);
	}

	pub fn set_array_start(&self, array_start: &/* Java */ java::lang::String /**/) {
		super.set_array_start(array_start);
	}

	pub fn set_content_end(&self, content_end: &/* Java */ java::lang::String /**/) {
		super.set_content_end(content_end);
	}

	pub fn set_content_start(&self, content_start: &/* Java */ java::lang::String /**/) {
		super.set_content_start(content_start);
	}

	pub fn set_default_full_detail(&self, default_full_detail: bool) {
		super.set_default_full_detail(default_full_detail);
	}

	pub fn set_field_name_value_separator(&self, field_name_value_separator: &/* Java */ java::lang::String /**/) {
		super.set_field_name_value_separator(field_name_value_separator);
	}

	pub fn set_field_separator(&self, field_separator: &/* Java */ java::lang::String /**/) {
		super.set_field_separator(field_separator);
	}

	pub fn set_field_separator_at_end(&self, field_separator_at_end: bool) {
		super.set_field_separator_at_end(field_separator_at_end);
	}

	pub fn set_field_separator_at_start(&self, field_separator_at_start: bool) {
		super.set_field_separator_at_start(field_separator_at_start);
	}

	pub fn set_null_text(&self, null_text: &/* Java */ java::lang::String /**/) {
		super.set_null_text(null_text);
	}

	pub fn set_size_end_text(&self, size_end_text: &/* Java */ java::lang::String /**/) {
		super.set_size_end_text(size_end_text);
	}

	pub fn set_size_start_text(&self, size_start_text: &/* Java */ java::lang::String /**/) {
		super.set_size_start_text(size_start_text);
	}

	pub fn set_summary_object_end_text(&self, summary_object_end_text: &/* Java */ java::lang::String /**/) {
		super.set_summary_object_end_text(summary_object_end_text);
	}

	pub fn set_summary_object_start_text(&self, summary_object_start_text: &/* Java */ java::lang::String /**/) {
		super.set_summary_object_start_text(summary_object_start_text);
	}

	pub fn set_use_class_name(&self, use_class_name: bool) {
		super.set_use_class_name(use_class_name);
	}

	pub fn set_use_field_names(&self, use_field_names: bool) {
		super.set_use_field_names(use_field_names);
	}

	pub fn set_use_identity_hash_code(&self, use_identity_hash_code: bool) {
		super.set_use_identity_hash_code(use_identity_hash_code);
	}

	pub fn set_use_short_class_name(&self, use_short_class_name: bool) {
		super.set_use_short_class_name(use_short_class_name);
	}
}

impl /* Java */ java::io::Serializable /**/ for StandardToStringStyle {}