use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration::ConfigOption;
use crate::com::github::javaparser::utils::Utils;

pub struct DefaultConfigurationOption {
	config_option: com::github::javaparser::printer::configuration::default_printer_configuration::ConfigOption,
	current_value: /* Java */ java::lang::Object /**/,
}

impl DefaultConfigurationOption {
	pub fn new(config_option: &com::github::javaparser::printer::configuration::default_printer_configuration::ConfigOption) -> com::github::javaparser::printer::configuration::default_configuration_option::DefaultConfigurationOption {
		this(config_option, null);
	}

	pub fn new(config_option: &com::github::javaparser::printer::configuration::default_printer_configuration::ConfigOption, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::configuration::default_configuration_option::DefaultConfigurationOption {
		self.configOption = config_option;
		if value != null {
			self.value(value)?;
		}
	
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if o == null || !(o instanceof DefaultConfigurationOption) {
			return false;
		}
	
		let other: DefaultConfigurationOption = o as DefaultConfigurationOption;
		return self.config_option.equals(other.configOption);
	}

	pub fn hash_code(&self) -> i32 {
		return self.config_option.hashCode();
	}

	pub fn value(&mut self, value: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption {
		Utils::assert_not_null(value)?;
		self.currentValue = value;
		// verify the currentValue's type
		if !(self.config_option.type.isAssignableFrom(&value.getClass())) {
			return Err(IllegalArgumentException::new(&String::format("%s is not an instance of %s", value, &self.config_option.type.getName())));
		}
		return self;
	}

	pub fn has_value(&self) -> bool {
		return self.currentValue != null;
	}

	pub fn as_integer(&self) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Integer /**/ {
		return self.cast()?;
	}

	pub fn as_string(&self) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return self.cast()?;
	}

	pub fn as_boolean(&self) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Boolean /**/ {
		return self.cast()?;
	}

	pub fn as_value<T: /* Java */ java::lang::Object /**/>(&self) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		return self.cast()?;
	}

	fn cast<T: /* Java */ java::lang::Object /**/>(&self) /* thrown(java.lang.IllegalArgumentException) */ -> T {
		if !self.has_value() {
			return Err(IllegalArgumentException::new(&String::format("The option %s has no currentValue", &self.config_option.name())));
		}
	
		if self.config_option.type.isAssignableFrom(&self.current_value.getClass()) {
			return self.config_option.type.cast(self.current_value) as T;
		}
	
		return Err(IllegalArgumentException::new(&String::format("%s cannot be cast to %s", self.current_value, &self.config_option.type.getName())));
	}
}

impl com::github::javaparser::printer::configuration::configuration_option::ConfigurationOption for DefaultConfigurationOption {}