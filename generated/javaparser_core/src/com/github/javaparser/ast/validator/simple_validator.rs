use crate::com::github::javaparser::ast::Node;
use java::util::function::BiConsumer;
use java::util::function::Predicate;

pub struct SimpleValidator<N: com::github::javaparser::ast::node::Node>;

impl<N: com::github::javaparser::ast::node::Node> SimpleValidator {
	pub fn new(type: &/* Java */ java::lang::Class /**/, condition: &/* Java */ java::util::function::Predicate /**/, problem_supplier: &/* Java */ java::util::function::BiConsumer /**/) -> com::github::javaparser::ast::validator::simple_validator::SimpleValidator {
		super(type, |(node, problem_reporter)|{
			if condition.test(node) {
				problem_supplier.accept(node, problem_reporter);
			}
		});
	}
}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::validator::validator::Validator for SimpleValidator<N> {}

impl<N: com::github::javaparser::ast::node::Node> com::github::javaparser::ast::validator::typed_validator::TypedValidator for SimpleValidator<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::function::BiConsumer /**/ for SimpleValidator<N> {}