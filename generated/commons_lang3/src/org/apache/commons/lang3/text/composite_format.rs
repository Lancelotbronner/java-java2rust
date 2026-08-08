use java::text::FieldPosition;
use java::text::Format;
use java::text::ParseException;
use java::text::ParsePosition;

pub struct CompositeFormat {
	parser: /* Java */ java::text::Format /**/,
	formatter: /* Java */ java::text::Format /**/,
}

impl CompositeFormat {
	static serialVersionUID: i64 = -4329119827877627683;

	pub fn new(parser: &/* Java */ java::text::Format /**/, formatter: &/* Java */ java::text::Format /**/) -> org::apache::commons::lang3::text::composite_format::CompositeFormat {
		self.parser = parser;
		self.formatter = formatter;
	}

	pub fn format(&self, obj: &/* Java */ java::lang::Object /**/, to_append_to: &/* Java */ java::lang::StringBuffer /**/, pos: &/* Java */ java::text::FieldPosition /**/) -> /* Java */ java::lang::StringBuffer /**/ {
		return self.formatter.format(obj, to_append_to, pos);
	}

	pub fn get_formatter(&self) -> /* Java */ java::text::Format /**/ {
		return self.formatter;
	}

	pub fn get_parser(&self) -> /* Java */ java::text::Format /**/ {
		return self.parser;
	}

	pub fn parse_object(&self, source: &/* Java */ java::lang::String /**/, pos: &/* Java */ java::text::ParsePosition /**/) -> /* Java */ java::lang::Object /**/ {
		return self.parser.parseObject(source, pos);
	}

	pub fn reformat(&self, input: &/* Java */ java::lang::String /**/) /* thrown(java.text.ParseException) */ -> /* Java */ java::lang::String /**/ {
		return self.format(&self.parseObject(input));
	}
}

impl /* Java */ java::io::Serializable /**/ for CompositeFormat {}

impl /* Java */ java::lang::Cloneable /**/ for CompositeFormat {}