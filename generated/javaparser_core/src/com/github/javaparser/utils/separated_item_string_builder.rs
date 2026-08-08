pub struct SeparatedItemStringBuilder {
	separator: /* Java */ java::lang::String /**/,
	postfix: /* Java */ java::lang::String /**/,
	has_items: bool = false,
	builder: /* Java */ java::lang::StringBuilder /**/,
}

impl SeparatedItemStringBuilder {
	pub fn new(prefix: &/* Java */ java::lang::String /**/, separator: &/* Java */ java::lang::String /**/, postfix: &/* Java */ java::lang::String /**/) -> com::github::javaparser::utils::separated_item_string_builder::SeparatedItemStringBuilder {
		self.builder = StringBuilder::new(prefix);
		self.separator = separator;
		self.postfix = postfix;
	}

	pub fn append(&mut self, format: &/* Java */ java::lang::CharSequence /**/, args: &/* Java */ java::lang::Object /**/) -> com::github::javaparser::utils::separated_item_string_builder::SeparatedItemStringBuilder {
		if self.has_items {
			self.builder.append(self.separator);
		}
		self.builder.append(&String::format(&format.toString(), args));
		self.has_items = true;
		return self;
	}

	pub fn has_items(&self) -> bool {
		return self.has_items;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		// This order of toStringing avoids debuggers from making a mess.
		return self.builder.toString() + self.postfix;
	}
}