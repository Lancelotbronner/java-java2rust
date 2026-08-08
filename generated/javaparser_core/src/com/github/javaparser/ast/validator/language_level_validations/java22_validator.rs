use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::stmt::CatchClause;
use crate::com::github::javaparser::ast::stmt::ForStmt;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::SingleNodeTypeValidator;
use crate::com::github::javaparser::ast::validator::Validator;
use crate::com::github::javaparser::resolution::Navigator;

pub struct Java22Validator {
	unnamed_var_only_where_allowed_by_jep456: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(SimpleName.class, |(name, reporter)|{
	if !name.getIdentifier().equals("_") {
		return;
	}
	if .reportNoParent(name, reporter) {
		return;
	}
	let parent_node: Node = name.getParentNode().get();
	if parent_node instanceof VariableDeclarator || parent_node instanceof TypePatternExpr {
		return;
	}
	if parent_node instanceof Parameter {
		let parameter: Parameter = parent_node as Parameter;
		if .reportNoParent(parameter, reporter) {
			return;
		}
		let grand_parent: Node = parameter.get_parent_node().get();
		if grand_parent instanceof CatchClause || grand_parent instanceof LambdaExpr {
			return;
		}
	}
	let r0 = 'try0: {
		let enclosing_for: ForStmt = Navigator.demandParentNode(name, |ancestor|ancestor instanceof ForStmt) as ForStmt;
		if enclosing_for.get_compare().isPresent() && enclosing_for.get_compare().get().containsWithinRange(name) {
			// In a for compare, so now check that it's the LHS of an assignment
			let enclosing_assign: AssignExpr = Navigator.demandParentNode(name, |ancestor|ancestor instanceof AssignExpr) as AssignExpr;
			if enclosing_assign.get_target().containsWithinRange(name) {
				return;
			}
		}
		break 'try0 Ok(());
	};
	match r0 {
		Err(e @ IllegalStateException) => {
		// Didn't find a ForStmt ancestor, so the "_" identifier should not be allowed here.
		},
		Err(e) => Err(e)?,
		Ok => (),
	}
	reporter.report(name, "Unnamed variables only supported in cases described by JEP456");
}),
	match_all_pattern_not_top_level: com::github::javaparser::ast::validator::validator::Validator = SingleNodeTypeValidator<>::new(MatchAllPatternExpr.class, |(pattern_expr, reporter)|{
	if !pattern_expr.getParentNode().isPresent() || !(pattern_expr.getParentNode().get() instanceof PatternExpr) {
		reporter.report(pattern_expr, "MatchAllPatternExpr cannot be used as a top-level pattern");
	}
}),
}

impl Java22Validator {
	fn report_no_parent(&self, node: &com::github::javaparser::ast::node::Node, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) -> bool {
		if node.get_parent_node().isPresent() {
			return false;
		}
		let class_name: String = node.getClass().getCanonicalName();
		reporter.report(node, "Node of type " + class_name + " must have an AST parent");
		return true;
	}

	pub fn new() /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::validator::language_level_validations::java22_validator::Java22Validator {
		super();
		self.remove()?;
		self.add(self.unnamed_var_only_where_allowed_by_jep456);
		self.add(self.match_all_pattern_not_top_level);
	}
}

impl com::github::javaparser::ast::validator::validator::Validator for Java22Validator {}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for Java22Validator {}

impl /* Java */ java::util::function::BiConsumer /**/ for Java22Validator {}