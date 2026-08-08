use java::io::StringWriter;

pub struct LexicalPreservingVisitor {
	writer: /* Java */ java::io::StringWriter /**/,
}

impl LexicalPreservingVisitor {
	pub fn new() -> com::github::javaparser::printer::lexicalpreservation::lexical_preserving_visitor::LexicalPreservingVisitor {
		this(StringWriter::new());
	}

	pub fn new(writer: &/* Java */ java::io::StringWriter /**/) -> com::github::javaparser::printer::lexicalpreservation::lexical_preserving_visitor::LexicalPreservingVisitor {
		self.writer = writer;
	}

	pub fn visit(&self, child: &com::github::javaparser::printer::lexicalpreservation::child_text_element::ChildTextElement) {
		child.accept(self);
	}

	pub fn visit(&self, token: &com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement) {
		self.writer.append(&token.get_text());
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.writer.toString();
	}
}