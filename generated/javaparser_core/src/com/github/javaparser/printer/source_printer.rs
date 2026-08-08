use crate::com::github::javaparser::Position;
use crate::com::github::javaparser::printer::configuration;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration::ConfigOption;
use crate::com::github::javaparser::printer::configuration::Indentation::IndentType;
use crate::com::github::javaparser::utils::Utils;
use java::util::Deque;
use java::util::LinkedList;

pub struct SourcePrinter {
	end_of_line_character: /* Java */ java::lang::String /**/,
	indentation: com::github::javaparser::printer::configuration::indentation::Indentation,
	indents: /* Java */ java::util::Deque /**/ = LinkedList<>::new(),
	reindented_indents: /* Java */ java::util::Deque /**/ = LinkedList<>::new(),
	last_printed_indent: /* Java */ java::lang::String /**/ = "",
	buf: /* Java */ java::lang::StringBuilder /**/ = StringBuilder::new(),
	cursor: com::github::javaparser::position::Position = Position::new(Position::FIRST_LINE, Position::FIRST_COLUMN - 1),
	indented: bool = false,
}

impl SourcePrinter {
	fn new() -> com::github::javaparser::printer::source_printer::SourcePrinter {
		this(DefaultPrinterConfiguration::new());
	}

	fn new(configuration: &com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		this(&configuration.get_indentation(), &configuration.get_end_of_line_character());
	}

	fn new(configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		this(&configuration.get(DefaultConfigurationOption::new(ConfigOption::INDENTATION)).get().as_value(), &configuration.get(DefaultConfigurationOption::new(ConfigOption::END_OF_LINE_CHARACTER)).get().as_string());
	}

	fn new(indentation: &com::github::javaparser::printer::configuration::indentation::Indentation, eol: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		self.indentation = indentation;
		self.endOfLineCharacter = eol;
		self.indents.push("");
	}

	pub fn indent(&self) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::printer::source_printer::SourcePrinter {
		let current_indent: String = self.indents.peek();
		match self.indentation.get_type() {
			SPACES =>  {
			}
			TABS_WITH_SPACE_ALIGN =>  {
				self.indents.push(current_indent + self.indentation.get_indent());
				break;
			}
			TABS =>  {
				self.indents.push(self.indentation.get_indent() + current_indent);
				break;
			}
			_ =>  {
				return Err(AssertionError::new("Unhandled indent type"));
			}
		}
		return self;
	}

	pub fn indent_with_align_to(&self, column: i32) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::printer::source_printer::SourcePrinter {
		self.indents.push(&self.calculate_indent_with_align_to(column)?);
		return self;
	}

	fn calculate_indent_with_align_to(&self, column: i32) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException) */ -> /* Java */ java::lang::String /**/ {
		if column < self.last_printed_indent.length() {
			return Err(IllegalStateException::new("Attempt to indent less than the previous indent."));
		}
		let new_indent: StringBuilder = StringBuilder::new(self.last_printed_indent);
		match self.indentation.get_type() {
			SPACES =>  {
			}
			TABS_WITH_SPACE_ALIGN =>  {
				while new_indent.length() < column {
					new_indent.append(&IndentType::SPACES.get_car());
				}
				break;
			}
			TABS =>  {
				let current_indent_type: IndentType = self.indentation.get_type();
				let logical_indent_length: i32 = new_indent.length();
				while (logical_indent_length + current_indent_type.get_width()) <= column {
					new_indent.insert(0, &current_indent_type.get_car());
					logical_indent_length += current_indent_type.get_width();
				}
				while logical_indent_length < column {
					new_indent.append(&IndentType::SPACES.get_car());
					logical_indent_length += 1;
				}
				let full_tab: StringBuilder = StringBuilder::new();
				 {
					let i: i32 = 0;
					while i < current_indent_type.get_width() {
						{
							full_tab.append(&IndentType::SPACES.get_car());
						}
						i += 1;
					 }
				 }
	
				let full_tab_string: String = full_tab.toString();
				if (new_indent.length() >= current_indent_type.get_width()) && new_indent.substring(new_indent.length() - current_indent_type.get_width()).equals(full_tab_string) {
					let i: i32 = new_indent.indexOf(full_tab_string);
					new_indent.replace(i, i + current_indent_type.get_width(), &current_indent_type.get_car().toString());
				}
				break;
			}
			_ =>  {
				return Err(AssertionError::new("Unhandled indent type"));
			}
		}
		return new_indent.toString();
	}

	pub fn unindent(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::printer::source_printer::SourcePrinter {
		if self.indents.isEmpty() {
			// the second time we over-unindent.
			return Err(IllegalStateException::new("Indent/unindent calls are not well-balanced."));
		}
		self.indents.pop();
		return self;
	}

	fn append(&mut self, arg: &/* Java */ java::lang::String /**/) {
		self.buf.append(arg);
		self.cursor = self.cursor.with_column(self.cursor.column + arg.length());
	}

	pub fn print(&mut self, arg: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		if !self.indented {
			self.last_printed_indent = self.indents.peek();
			self.append(self.last_printed_indent);
			self.indented = true;
		}
		self.append(arg);
		return self;
	}

	pub fn println(&self, arg: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		self.print(arg);
		self.println();
		return self;
	}

	pub fn println(&mut self) -> com::github::javaparser::printer::source_printer::SourcePrinter {
		self.buf.append(self.end_of_line_character);
		// Start before the first column
		self.cursor = Position::new(self.cursor.line + 1, Position::FIRST_COLUMN - 1);
		self.indented = false;
		return self;
	}

	pub fn get_cursor(&self) -> com::github::javaparser::position::Position {
		return self.cursor;
	}

	pub fn get_source(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_string();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.buf.toString();
	}

	pub fn normalize_eol_in_text_block(&self, content: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return Utils::normalize_eol_in_text_block(content, self.end_of_line_character);
	}

	pub fn reindent_with_align_to_cursor(&self) {
		let new_indent: String = self.calculate_indent_with_align_to(self.cursor.column)?;
		self.reindented_indents.push(&self.indents.pop());
		self.indents.push(new_indent);
	}

	pub fn reindent_to_previous_level(&self) /* thrown(java.lang.IllegalStateException) */ {
		if self.reindented_indents.isEmpty() {
			return Err(IllegalStateException::new("Reindent calls are not well-balanced."));
		}
		self.indents.pop();
		self.indents.push(&self.reindented_indents.pop());
	}

	pub fn duplicate_indent(&self) {
		self.indents.push(&self.indents.peek());
	}
}