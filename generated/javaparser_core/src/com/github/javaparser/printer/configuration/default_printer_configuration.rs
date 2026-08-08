use crate::com::github::javaparser::printer::Printer;
use crate::com::github::javaparser::printer::configuration::Indentation::IndentType;
use crate::com::github::javaparser::utils::LineSeparator;
use java::util::Arrays;
use java::util::HashSet;
use java::util::Optional;
use java::util::Set;

pub struct DefaultPrinterConfiguration {
	default_options: /* Java */ java::util::Set /**/ = HashSet<>::new(&Arrays::asList(DefaultConfigurationOption::new(ConfigOption::PRINT_COMMENTS, ConfigOption::PRINT_COMMENTS.defaultValue), DefaultConfigurationOption::new(ConfigOption::PRINT_JAVADOC, ConfigOption::PRINT_JAVADOC.defaultValue), DefaultConfigurationOption::new(ConfigOption::SPACE_AROUND_OPERATORS, ConfigOption::SPACE_AROUND_OPERATORS.defaultValue), DefaultConfigurationOption::new(ConfigOption::INDENT_CASE_IN_SWITCH, ConfigOption::INDENT_CASE_IN_SWITCH.defaultValue), DefaultConfigurationOption::new(ConfigOption::MAX_ENUM_CONSTANTS_TO_ALIGN_HORIZONTALLY, ConfigOption::MAX_ENUM_CONSTANTS_TO_ALIGN_HORIZONTALLY.defaultValue), DefaultConfigurationOption::new(ConfigOption::END_OF_LINE_CHARACTER, ConfigOption::END_OF_LINE_CHARACTER.defaultValue), DefaultConfigurationOption::new(ConfigOption::INDENTATION, ConfigOption::INDENTATION.defaultValue))),
}

impl DefaultPrinterConfiguration {
	pub fn new() -> com::github::javaparser::printer::configuration::default_printer_configuration::DefaultPrinterConfiguration {
	}

	pub fn add_option(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		self.remove_option(option);
		self.default_options.add(option);
		return self;
	}

	pub fn remove_option(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		self.default_options.remove(option);
		return self;
	}

	pub fn is_activated(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> bool {
		return self.default_options.contains(option);
	}

	pub fn get(&self, option: &com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption) -> /* Java */ java::util::Optional /**/ {
		return self.default_options.stream().filter(|o|o.equals(option)).findFirst();
	}

	pub fn get(&self) -> /* Java */ java::util::Set /**/ {
		return self.default_options;
	}
}

impl com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration for DefaultPrinterConfiguration {}

pub enum ConfigOption {
	default_value: /* Java */ java::lang::Object /**/,
	type: /* Java */ java::lang::Class /**/,
}