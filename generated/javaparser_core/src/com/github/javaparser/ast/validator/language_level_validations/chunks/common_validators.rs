use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::body::ClassOrInterfaceDeclaration;
use crate::com::github::javaparser::ast::body::InitializerDeclaration;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::validator::SimpleValidator;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::TreeVisitorValidator;
use crate::com::github::javaparser::ast::validator::Validators;
use crate::com::github::javaparser::metamodel::NodeMetaModel;
use crate::com::github::javaparser::metamodel::PropertyMetaModel;

pub struct CommonValidators;

impl CommonValidators {
	pub fn new() -> com::github::javaparser::ast::validator::language_level_validations::chunks::common_validators::CommonValidators {
		super(SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|!n.isInterface() && n.getExtendedTypes().size() > 1, |(n, reporter)|reporter.report(&n.getExtendedTypes(1), "A class cannot extend more than one other class.")), SimpleValidator<>::new(ClassOrInterfaceDeclaration.class, |n|n.isInterface() && !n.getImplementedTypes().isEmpty(), |(n, reporter)|reporter.report(&n.getImplementedTypes(0), "An interface cannot implement other interfaces.")), SingleNodeTypeValidator<>::new(ClassOrInterfaceDeclaration.class, |(n, reporter)|{
			if n.isInterface() {
				n.getMembers().forEach(|mem|{
					if mem instanceof InitializerDeclaration {
						reporter.report(mem, "An interface cannot have initializers.");
					}
				});
			}
		}), SingleNodeTypeValidator<>::new(AssignExpr.class, |(n, reporter)|{
			// https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-15.26
			let target: Expression = n.getTarget();
			while target instanceof EnclosedExpr {
				target = (target as EnclosedExpr).get_inner();
			}
			if target instanceof NameExpr || target instanceof ArrayAccessExpr || target instanceof FieldAccessExpr {
				return;
			}
			reporter.report(&n.getTarget(), "Illegal left hand side of an assignment.");
		}), TreeVisitorValidator::new(|(node, problem_reporter)|{
			let mm: NodeMetaModel = node.getMetaModel();
			for ppm in mm.get_all_property_meta_models() {
				if ppm.is_non_empty() {
					if ppm.is_node_list() {
						let value: NodeList<?> = ppm.getValue(node) as NodeList<?>;
						if value.is_empty() {
							problem_reporter.report(node, "%s.%s can not be empty.", &mm.get_type_name(), &ppm.get_name());
						}
					}
				// No need to check empty strings, it should be impossible to set them to ""
				}
			}
		}));
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for CommonValidators {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for CommonValidators {}

impl /* Java */ java::util::function::BiConsumer /**/ for CommonValidators {}