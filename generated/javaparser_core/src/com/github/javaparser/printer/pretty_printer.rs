use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::printer::configuration::PrettyPrinterConfiguration;
use crate::com::github::javaparser::printer::configuration::PrinterConfiguration;
use java::util::function::Function;

pub struct PrettyPrinter {
	configuration: com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration,
	visitor_factory: /* Java */ java::util::function::Function /**/,
}

impl PrettyPrinter {
	pub fn new() -> com::github::javaparser::printer::pretty_printer::PrettyPrinter {
		this(PrettyPrinterConfiguration::new());
	}

	pub fn new(configuration: &com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration) -> com::github::javaparser::printer::pretty_printer::PrettyPrinter {
		this(configuration, PrettyPrintVisitor::new);
	}

	pub fn new(configuration: &com::github::javaparser::printer::configuration::pretty_printer_configuration::PrettyPrinterConfiguration, visitor_factory: &/* Java */ java::util::function::Function /**/) -> com::github::javaparser::printer::pretty_printer::PrettyPrinter {
		self.configuration = configuration;
		self.visitorFactory = visitor_factory;
	}

	pub fn get_configuration(&self) -> com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration {
		return self.configuration;
	}

	pub fn set_configuration(&mut self, configuration: &com::github::javaparser::printer::configuration::printer_configuration::PrinterConfiguration) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::printer::Printer {
		if !(configuration instanceof PrettyPrinterConfiguration) {
			return Err(IllegalArgumentException::new("PrettyPrinter must be configured with a PrettyPrinterConfiguration class"));
		}
	
		self.configuration = configuration;
		return self;
	}

	pub fn print(&self, node: &com::github::javaparser::ast::node::Node) -> /* Java */ java::lang::String /**/ {
		/* final */ let visitor: VoidVisitor<Void> = self.visitor_factory.apply(self.configuration as PrettyPrinterConfiguration);
		node.accept(visitor, null);
		return visitor.toString();
	}
}

impl com::github::javaparser::printer::configurable_printer::ConfigurablePrinter for PrettyPrinter {}

impl com::github::javaparser::printer::printer::Printer for PrettyPrinter {}