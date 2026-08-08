use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::ReturnStmtMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ReturnStmt {
	expression: com::github::javaparser::ast::expr::expression::Expression,
}

impl ReturnStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		this(null, null);
	}

	pub fn new(expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		this(null, expression);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		super(token_range);
		self.set_expression(expression);
		self.custom_initialization();
	}

	pub fn new(expression: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		this(null, NameExpr::new(expression));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_expression(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.expression);
	}

	pub fn set_expression(&mut self, expression: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		if expression == self.expression {
			return self;
		}
		self.notify_property_change(ObservableProperty::EXPRESSION, self.expression, expression);
		if self.expression != null {
			self.expression.set_parent_node(null);
		}
	
		self.expression = expression;
		self.set_as_parent_node_of(expression);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.expression != null {
			if node == self.expression {
				self.remove_expression();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_expression(&self) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return self.set_expression(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return self.accept(CloneVisitor::new(), null) as ReturnStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::return_stmt_meta_model::ReturnStmtMetaModel {
		return JavaParserMetaModel::returnStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.expression != null {
			if node == self.expression {
				self.set_expression(replacement_node as Expression);
				return true;
			}
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_return_stmt(&self) -> bool {
		return true;
	}

	pub fn as_return_stmt(&self) -> com::github::javaparser::ast::stmt::return_stmt::ReturnStmt {
		return self;
	}

	pub fn if_return_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_return_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ReturnStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ReturnStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ReturnStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ReturnStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ReturnStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ReturnStmt {}