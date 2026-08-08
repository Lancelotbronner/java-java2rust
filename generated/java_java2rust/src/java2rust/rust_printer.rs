use javaparser_core::com::github::javaparser::ast::DataKey;
use javaparser_core::com::github::javaparser::ast::Node;
use java::util::ArrayList;
use java::util::List;
use java::util::Objects;
use java::util::Optional;

pub struct RustPrinter {
	indentation: /* Java */ java::lang::String /**/,
	buf: /* Java */ java::lang::StringBuilder /**/ = StringBuilder::new(),
	marks: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	level: i32 = 0,
	indented: bool = false,
	is_within_comment: bool = false,
}

impl RustPrinter {
	static PRINTER_KEY: com::github::javaparser::ast::data_key::DataKey = Key::new();

	pub fn new(indentation: &/* Java */ java::lang::String /**/) -> java2rust::rust_printer::RustPrinter {
		self.indentation = indentation;
	}

	pub fn indent(&self) {
		self.level += 1;
	}

	pub fn unindent(&self) {
		self.level -= 1;
	}

	pub fn println(&self, arg: &/* Java */ java::lang::String /**/) {
		self.print(arg);
		self.println();
	}

	pub fn print(&mut self, arg: &/* Java */ java::lang::String /**/) {
		if !self.indented {
			self.make_indent();
			self.indented = true;
		}
		self.buf.append(arg);
	}

	pub fn println(&mut self) {
		self.buf.append(&System::lineSeparator());
		self.indented = false;
	}

	pub fn print(&self, n: &com::github::javaparser::ast::node::Node) {
		let code: Optional<IRustCode> = n.find_data(self.PRINTER_KEY)?;
		if code.isPresent() {
			code.get().print(self);
			return;
		}
		self.comment("Java");
		self.end_comment();
		self.print(&n.to_string());
		self.start_comment();
		self.end_comment();
	}

	pub fn delete_last(&self, count: i32) {
		self.buf.delete(self.buf.length() - count, &self.buf.length());
	}

	fn make_indent(&self) {
		self.buf.repeat(&Objects::requireNonNull(&String::valueOf(self.indentation)), &Math::max(0, self.level));
	}

	pub fn start_comment(&mut self) {
		if self.is_within_comment {
			return;
		}
	
		self.is_within_comment = true;
		self.print("/* ");
	}

	pub fn comment(&self, arg: &/* Java */ java::lang::String /**/) {
		self.start_comment();
		self.print(arg);
	}

	pub fn end_comment(&mut self) {
		if !self.is_within_comment {
			return;
		}
	
		self.is_within_comment = false;
		self.print("*/ ");
	}

	pub fn push(&self) -> i32 {
		self.marks.add(&self.buf.length());
		return self.marks.size();
	}

	pub fn get_mark(&self, mark: i32) -> /* Java */ java::lang::String /**/ {
		return self.buf.substring(&self.marks.get(mark - 1));
	}

	pub fn pop(&self) {
		self.buf.delete(&self.marks.getLast(), &self.buf.length());
		self.marks.removeLast();
	}

	pub fn drop(&self) {
		self.marks.removeLast();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_source();
	}

	pub fn get_source(&self) -> /* Java */ java::lang::String /**/ {
		return self.buf.toString();
	}
}

struct Key;