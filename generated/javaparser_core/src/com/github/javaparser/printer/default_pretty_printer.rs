use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::printer::configuration::DefaultPrinterConfiguration;
use crate::com::github::javaparser::printer::configuration::PrinterConfiguration;
use java::util::function::Function;

pub struct DefaultPrettyPrinter {
	configuration: com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration,
	visitor_factory: /* Java */ java::util::function::Function /**/,
}

impl DefaultPrettyPrinter {
	fn create_default_visitor(&self) -> /* Java */ java::util::function::Function /**/ {
		return |(config)|DefaultPrettyPrinterVisitor::new(config, SourcePrinter::new(config));
	}

	fn create_default_configuration(&self) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return DefaultPrinterConfiguration::new();
	}

	pub fn new() -> com::github::javaparser::printer::default_pretty_printer::DefaultPrettyPrinter {
		this(&com::github::javaparser::printer::default_pretty_printer::DefaultPrettyPrinter::create_default_configuration());
	}

	pub fn new(configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::default_pretty_printer::DefaultPrettyPrinter {
		this(&com::github::javaparser::printer::default_pretty_printer::DefaultPrettyPrinter::create_default_visitor(), configuration);
	}

	pub fn new(visitor_factory: &/* Java */ java::util::function::Function /**/, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::default_pretty_printer::DefaultPrettyPrinter {
		self.configuration = configuration;
		self.visitorFactory = visitor_factory;
	}

	pub fn get_configuration(&self) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return self.configuration;
	}

	pub fn set_configuration(&mut self, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) -> com::github::javaparser::printer::printer::Printer {
		self.configuration = configuration;
		return self;
	}

	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		// lazy initialization of visitor which can have a state (like a buffer)
		let visitor: VoidVisitor<Void> = self.visitor_factory.apply(self.configuration);
		node.accept(visitor, null);
		return visitor.toString();
	}
}

impl com::github::javaparser::printer::configurable_printer::ConfigurablePrinter for DefaultPrettyPrinter {}

impl com::github::javaparser::printer::printer::Printer for DefaultPrettyPrinter {}