use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::Parameter;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::expr;
use crate::com::github::javaparser::ast::stmt::ExpressionStmt;
use crate::com::github::javaparser::ast::stmt::ForEachStmt;
use crate::com::github::javaparser::ast::stmt::ForStmt;
use crate::com::github::javaparser::ast::stmt::TryStmt;
use crate::com::github::javaparser::ast::type::VarType;
use crate::com::github::javaparser::ast::validator::ProblemReporter;
use crate::com::github::javaparser::ast::validator::TypedValidator;
use java::util::Optional;

pub struct VarValidator {
	var_allowed_in_lambda_parameters: bool,
}

impl VarValidator {
	pub fn new(var_allowed_in_lambda_parameters: bool) -> com::github::javaparser::ast::validator::language_level_validations::chunks::var_validator::VarValidator {
		self.varAllowedInLambdaParameters = var_allowed_in_lambda_parameters;
	}

	pub fn accept(&self, node: &com::github::javaparser::ast::type::var_type::VarType, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		// };
		if node.has_parent_node() && node.get_parent_node().get() instanceof TypePatternExpr {
			return;
		}
		// All allowed locations are within a VariableDeclaration inside a VariableDeclarationExpr inside something
		// else.
		let variable_declarator: Optional<VariableDeclarator> = node.findAncestor(VariableDeclarator.class);
		if !variable_declarator.isPresent() {
			// Java 11's var in lambda's
			if self.var_allowed_in_lambda_parameters {
				let valid: bool = node.findAncestor(Parameter.class).flatMap(Node::getParentNode).map(|(p: &Node)|p instanceof LambdaExpr).orElse(false);
				if valid {
					return;
				}
			}
			self.report_illegal_position(node, reporter);
			return;
		}
		variable_declarator.ifPresent(|vd|{
			if vd.get_type().is_array_type() {
				reporter.report(vd, "\"var\" cannot have extra array brackets.");
			}
			let variable_declaration_expr: Optional<Node> = vd.get_parent_node();
			if !variable_declaration_expr.isPresent() {
				self.report_illegal_position(node, reporter);
				return;
			}
			variable_declaration_expr.ifPresent(|vde_node|{
				if !(vde_node instanceof VariableDeclarationExpr) {
					self.report_illegal_position(node, reporter);
					return;
				}
				let vde: VariableDeclarationExpr = vde_node as VariableDeclarationExpr;
				if vde.get_variables().size() > 1 {
					reporter.report(vde, "\"var\" only takes a single variable.");
				}
				let container: Optional<Node> = vde_node.get_parent_node();
				if !container.isPresent() {
					self.report_illegal_position(node, reporter);
					return;
				}
				container.ifPresent(|c|{
					let position_is_fine: bool = c instanceof ForStmt || c instanceof ForEachStmt || c instanceof ExpressionStmt || c instanceof TryStmt;
					if !position_is_fine {
						self.report_illegal_position(node, reporter);
					}
					// A local variable declaration ends up inside an ExpressionStmt.
					if c instanceof ExpressionStmt {
						if !vd.get_initializer().isPresent() {
							reporter.report(node, "\"var\" needs an initializer.");
						}
						vd.get_initializer().ifPresent(|initializer|{
							if initializer instanceof NullLiteralExpr {
								reporter.report(node, "\"var\" cannot infer type from just null.");
							}
							if initializer instanceof ArrayInitializerExpr {
								reporter.report(node, "\"var\" cannot infer array types.");
							}
						});
					}
				});
			});
		});
	}

	fn report_illegal_position(&self, n: &com::github::javaparser::ast::type::var_type::VarType, reporter: &com::github::javaparser::ast::validator::problem_reporter::ProblemReporter) {
		reporter.report(n, "\"var\" is not allowed here.");
	}
}

impl com::github::javaparser::ast::validator::typed_validator::TypedValidator for VarValidator {}

impl /* Java */ java::util::function::BiConsumer /**/ for VarValidator {}