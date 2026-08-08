use crate::com::github::javaparser::utils::Utils;
use crate::com::github::javaparser::printer::PrettyPrinter;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration::ConfigOption;
use crate::com::github::javaparser::printer::configuration::Indentation::IndentType;
use java::util::Optional;
use java::util::Set;

pub struct PrettyPrinterConfiguration {
	wrapped_configuration: com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration,
}

impl PrettyPrinterConfiguration {
	pub fn new() -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrappedConfiguration = DefaultPrinterConfiguration::new();
	}

	pub fn get_indentation(&self) -> com::github::javaparser::printer::configuration::indentation::Indentation {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::INDENTATION)).get().as_value();
	}

	pub fn set_indentation(&self, indentation: &com::github::javaparser::printer::configuration::indentation::Indentation) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration.add_option(DefaultConfigurationOption::new(ConfigOption::INDENTATION, indentation));
		return self;
	}

	pub fn get_indent(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_indentation().get_indent();
	}

	pub fn get_indent_size(&self) -> i32 {
		return self.get_indentation().get_size();
	}

	pub fn set_indent_size(&self, indent_size: i32) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		let indentation: Indentation = self.get_indentation().set_size(&com::github::javaparser::utils::utils::Utils::assert_non_negative(indent_size)?);
		self.set_indentation(indentation);
		return self;
	}

	pub fn get_indent_type(&self) -> com::github::javaparser::printer::configuration::indentation::IndentType {
		return self.get_indentation().get_type();
	}

	pub fn set_indent_type(&self, indent_type: &com::github::javaparser::printer::configuration::indentation::IndentType) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		let indentation: Indentation = self.get_indentation().set_type(&com::github::javaparser::utils::utils::Utils::assert_not_null(indent_type)?);
		self.set_indentation(indentation);
		return self;
	}

	pub fn get_tab_width(&self) -> i32 {
		return self.get_indentation().get_size();
	}

	pub fn set_tab_width(&self, tab_width: i32) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.set_indent_size(&com::github::javaparser::utils::utils::Utils::assert_positive(tab_width)?);
		return self;
	}

	pub fn is_order_imports(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::ORDER_IMPORTS)).isPresent();
	}

	pub fn is_print_comments(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS)).isPresent();
	}

	pub fn is_ignore_comments(&self) -> bool {
		return !self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS)).isPresent();
	}

	pub fn is_space_around_operators(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::SPACE_AROUND_OPERATORS)).isPresent();
	}

	pub fn is_print_javadoc(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::PRINT_JAVADOC)).isPresent();
	}

	pub fn is_column_align_parameters(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_PARAMETERS)).isPresent();
	}

	pub fn is_column_align_first_method_chain(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_FIRST_METHOD_CHAIN)).isPresent();
	}

	pub fn is_indent_case_in_switch(&self) -> bool {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::INDENT_CASE_IN_SWITCH)).isPresent();
	}

	pub fn set_print_comments(&mut self, print_comments: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if print_comments { self.add_option(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS)) };
		return self;
	}

	pub fn set_print_javadoc(&mut self, print_javadoc: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if print_javadoc { self.add_option(DefaultConfigurationOption::new(ConfigOption::PRINT_JAVADOC)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::PRINT_JAVADOC)) };
		return self;
	}

	pub fn set_space_around_operators(&mut self, space_around_operators: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if space_around_operators { self.add_option(DefaultConfigurationOption::new(ConfigOption::SPACE_AROUND_OPERATORS)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::SPACE_AROUND_OPERATORS)) };
		return self;
	}

	pub fn set_column_align_parameters(&mut self, column_align_parameters: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if column_align_parameters { self.add_option(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_PARAMETERS)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_PARAMETERS)) };
		return self;
	}

	pub fn set_column_align_first_method_chain(&mut self, column_align_first_method_chain: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if column_align_first_method_chain { self.add_option(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_FIRST_METHOD_CHAIN)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::COLUMN_ALIGN_FIRST_METHOD_CHAIN)) };
		return self;
	}

	pub fn set_indent_case_in_switch(&mut self, indent_in_switch: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if indent_in_switch { self.add_option(DefaultConfigurationOption::new(ConfigOption::INDENT_CASE_IN_SWITCH)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::INDENT_CASE_IN_SWITCH)) };
		return self;
	}

	pub fn get_end_of_line_character(&self) -> /* Java */ java::lang::String /**/ {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::END_OF_LINE_CHARACTER)).get().as_value();
	}

	pub fn set_end_of_line_character(&self, end_of_line_character: &/* Java */ java::lang::String /**/) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.add_option(DefaultConfigurationOption::new(ConfigOption::END_OF_LINE_CHARACTER, end_of_line_character));
		return self;
	}

	pub fn set_order_imports(&mut self, order_imports: bool) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.wrapped_configuration =  if order_imports { self.add_option(DefaultConfigurationOption::new(ConfigOption::ORDER_IMPORTS)) } else { self.remove_option(DefaultConfigurationOption::new(ConfigOption::ORDER_IMPORTS)) };
		return self;
	}

	pub fn get_max_enum_constants_to_align_horizontally(&self) -> i32 {
		return self.wrapped_configuration.get(DefaultConfigurationOption::new(ConfigOption::MAX_ENUM_CONSTANTS_TO_ALIGN_HORIZONTALLY)).get().as_integer();
	}

	pub fn set_max_enum_constants_to_align_horizontally(&self, max_enum_constants_to_align_horizontally: i32) -> com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration {
		self.add_option(DefaultConfigurationOption::new(ConfigOption::MAX_ENUM_CONSTANTS_TO_ALIGN_HORIZONTALLY, max_enum_constants_to_align_horizontally));
		return self;
	}

	pub fn add_option(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return self.wrapped_configuration.add_option(option);
	}

	pub fn is_activated(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> bool {
		return self.wrapped_configuration.is_activated(option);
	}

	pub fn get(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> /* Java */ java::util::Optional /**/ {
		return self.wrapped_configuration.get(option);
	}

	pub fn get(&self) -> /* Java */ java::util::Set /**/ {
		return self.wrapped_configuration.get();
	}

	pub fn remove_option(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return self.wrapped_configuration.remove_option(option);
	}
}

impl com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration for PrettyPrinterConfiguration {}