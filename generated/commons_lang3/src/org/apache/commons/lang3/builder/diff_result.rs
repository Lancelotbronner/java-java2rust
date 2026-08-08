use java::util::Collections;
use java::util::Iterator;
use java::util::List;
use java::util::Objects;
use crate::org::apache::commons::lang3::StringUtils;

pub struct DiffResult<T> {
	diff_list: /* Java */ java::util::List /**/,
	lhs: T,
	rhs: T,
	style: org::apache::commons::lang3::builder::to_string_style::ToStringStyle,
	to_string_format: /* Java */ java::lang::String /**/,
}

impl<T> DiffResult {
	pub static OBJECTS_SAME_STRING: /* Java */ java::lang::String /**/ = StringUtils::EMPTY;

	fn new(lhs: &T, rhs: &T, diff_list: &/* Java */ java::util::List /**/, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, to_string_format: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::diff_result::DiffResult {
		self.diffList = Objects::requireNonNull(diff_list, "diffList");
		self.lhs = Objects::requireNonNull(lhs, "lhs");
		self.rhs = Objects::requireNonNull(rhs, "rhs");
		self.style = Objects::requireNonNull(style, "style");
		self.toStringFormat = Objects::requireNonNull(to_string_format, "toStringFormat");
	}

	pub fn get_diffs(&self) -> /* Java */ java::util::List /**/ {
		return Collections::unmodifiableList(self.diff_list);
	}

	pub fn get_left(&self) -> T {
		return self.lhs;
	}

	pub fn get_number_of_diffs(&self) -> i32 {
		return self.diff_list.size();
	}

	pub fn get_right(&self) -> T {
		return self.rhs;
	}

	pub fn get_to_string_style(&self) -> org::apache::commons::lang3::builder::to_string_style::ToStringStyle {
		return self.style;
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return self.diff_list.iterator();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_string(self.style);
	}

	pub fn to_string(&self, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> /* Java */ java::lang::String /**/ {
		if self.diff_list.isEmpty() {
			return self.OBJECTS_SAME_STRING;
		}
		/* final */ let lhs_builder: ToStringBuilder = ToStringBuilder::new(self.lhs, style);
		/* final */ let rhs_builder: ToStringBuilder = ToStringBuilder::new(self.rhs, style);
		self.diff_list.forEach(|diff|{
			lhs_builder.append(&diff.get_field_name(), &diff.get_left());
			rhs_builder.append(&diff.get_field_name(), &diff.get_right());
		});
		return String::format(self.to_string_format, &lhs_builder.build(), &rhs_builder.build());
	}
}

impl<T> /* Java */ java::lang::Iterable /**/ for DiffResult<T> {}