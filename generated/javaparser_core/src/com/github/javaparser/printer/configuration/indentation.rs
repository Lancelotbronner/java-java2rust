pub struct Indentation {
	type: com::github::javaparser::printer::configuration::indentation::IndentType,
	size: i32,
	formatted_indentation: /* Java */ java::lang::String /**/ = "",
}

impl Indentation {
	static DEFAULT_SIZE: i32 = 4;

	pub fn new(type: &com::github::javaparser::printer::configuration::indentation::IndentType, size: i32) -> com::github::javaparser::printer::configuration::indentation::Indentation {
		self.type = type;
		self.size = size;
		self.format();
	}

	pub fn new(type: &com::github::javaparser::printer::configuration::indentation::IndentType) -> com::github::javaparser::printer::configuration::indentation::Indentation {
		this(type, self.DEFAULT_SIZE);
	}

	pub fn set_size(&mut self, size: i32) -> com::github::javaparser::printer::configuration::indentation::Indentation {
		self.size = size;
		self.format();
		return self;
	}

	pub fn get_size(&self) -> i32 {
		return self.size;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::printer::configuration::indentation::IndentType) -> com::github::javaparser::printer::configuration::indentation::Indentation {
		self.type = type;
		self.format();
		return self;
	}

	pub fn get_type(&self) -> com::github::javaparser::printer::configuration::indentation::IndentType {
		return self.type;
	}

	pub fn get_indent(&self) -> /* Java */ java::lang::String /**/ {
		return self.formatted_indentation;
	}

	fn format(&mut self) {
		let indent_string: StringBuilder = StringBuilder::new();
		let indent_char: char = self.type.car;
		 {
			let i: i32 = 0;
			while i < self.size {
				{
					indent_string.append(indent_char);
				}
				i += 1;
			 }
		 }
	
		self.formatted_indentation = indent_string.toString();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.type.name() + " size=" + self.size;
	}
}

pub enum IndentType {
	car: /* Java */ java::lang::Character /**/,
	width: i32,
}