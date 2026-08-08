use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::body::RecordDeclaration;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::TypedValidator;

pub struct RecordDeclarationValidator;

impl RecordDeclarationValidator {
	pub fn accept(&self, node: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		self.forbid_abstract_modifier(node, reporter);
		self.forbid_non_static_fields_in_records(node, reporter);
		self.validate_record_component_accessor_methods(node, reporter);
	}

	fn forbid_abstract_modifier(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		if n.get_modifiers().contains(&Modifier::abstract_modifier()) {
			reporter.report(n, "Record Declarations must not be declared as abstract.");
		}
	}

	fn forbid_non_static_fields_in_records(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		let non_static_field_count: i64 = n.get_fields().stream().filter(|field_declaration|!field_declaration.is_static()).count();
		if non_static_field_count > 0 {
			reporter.report(n, "Record Declarations must have zero non-static fields.");
		}
	}

	fn validate_record_component_accessor_methods(&self, n: &com::github::javaparser::ast::body::record_declaration::RecordDeclaration, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		n.get_parameters().forEach(|parameter|{
			n.get_methods_by_name(&parameter.get_name_as_string()).stream().filter(|method_declaration|method_declaration.get_parameters().is_empty()).forEach(|method_declaration|{
				if !method_declaration.get_type().equals(&parameter.get_type()) {
					reporter.report(n, &String::format("Incorrect component accessor return type. Expected: '%s', found: '%s'.", &parameter.get_type_as_string(), &method_declaration.get_type_as_string()));
				}
			});
		});
	}
}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for RecordDeclarationValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for RecordDeclarationValidator {}