use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::AssertStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct AssertStmt {
	check: com::github::javaparser::ast::expr::expression::Expression,
	message: com::github::javaparser::ast::expr::expression::Expression,
}

impl AssertStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		this(null, BooleanLiteralExpr::new(), null);
	}

	pub fn new(check: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		this(null, check, null);
	}

	pub fn new(check: &com::github::javaparser::ast::expr::expression::Expression, message: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		this(null, check, message);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, check: &com::github::javaparser::ast::expr::expression::Expression, message: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		super(token_range);
		self.set_check(check);
		self.set_message(message);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_check(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.check;
	}

	pub fn get_message(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.message);
	}

	pub fn set_check(&mut self, check: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(check)?;
		if check == self.check {
			return self;
		}
		self.notify_property_change(ObservableProperty::CHECK, self.check, check);
		if self.check != null {
			self.check.set_parent_node(null);
		}
	
		self.check = check;
		self.set_as_parent_node_of(check);
		return self;
	}

	pub fn set_message(&mut self, message: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		if message == self.message {
			return self;
		}
		self.notify_property_change(ObservableProperty::MESSAGE, self.message, message);
		if self.message != null {
			self.message.set_parent_node(null);
		}
	
		self.message = message;
		self.set_as_parent_node_of(message);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.message != null {
			if node == self.message {
				self.remove_message();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_message(&self) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		return self.set_message(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		return self.accept(CloneVisitor::new(), null) as AssertStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::assert_stmt_meta_model::AssertStmtMetaModel {
		return JavaParserMetaModel::assertStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.check {
			self.set_check(replacement_node as Expression)?;
			return true;
		}
		if self.message != null {
			if node == self.message {
				self.set_message(replacement_node as Expression);
				return true;
			}
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_assert_stmt(&self) -> bool {
		return true;
	}

	pub fn as_assert_stmt(&self) -> com::github::javaparser::ast::stmt::assert_stmt::AssertStmt {
		return self;
	}

	pub fn if_assert_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_assert_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for AssertStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for AssertStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for AssertStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for AssertStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for AssertStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for AssertStmt {}