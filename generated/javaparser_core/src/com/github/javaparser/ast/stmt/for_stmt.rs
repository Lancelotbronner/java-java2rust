use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::BooleanLiteralExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithBody;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ForStmtMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ForStmt {
	initialization: com::github::javaparser::ast::node_list::NodeList,
	compare: com::github::javaparser::ast::expr::expression::Expression,
	update: com::github::javaparser::ast::node_list::NodeList,
	body: com::github::javaparser::ast::stmt::statement::Statement,
}

impl ForStmt {
	pub fn new() -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		this(null, NodeList<>::new(), BooleanLiteralExpr::new(), NodeList<>::new(), ReturnStmt::new());
	}

	pub fn new(initialization: &com::github::javaparser::ast::node_list::NodeList, compare: &com::github::javaparser::ast::expr::expression::Expression, update: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		this(null, initialization, compare, update, body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, initialization: &com::github::javaparser::ast::node_list::NodeList, compare: &com::github::javaparser::ast::expr::expression::Expression, update: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::statement::Statement) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		super(token_range);
		self.set_initialization(initialization);
		self.set_compare(compare);
		self.set_update(update);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::statement::Statement {
		return self.body;
	}

	pub fn get_compare(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.compare);
	}

	pub fn get_initialization(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.initialization;
	}

	pub fn get_update(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.update;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::statement::Statement) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(body)?;
		if body == self.body {
			return self;
		}
		self.notify_property_change(ObservableProperty::BODY, self.body, body);
		if self.body != null {
			self.body.set_parent_node(null);
		}
	
		self.body = body;
		self.set_as_parent_node_of(body);
		return self;
	}

	pub fn set_compare(&mut self, compare: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		if compare == self.compare {
			return self;
		}
		self.notify_property_change(ObservableProperty::COMPARE, self.compare, compare);
		if self.compare != null {
			self.compare.set_parent_node(null);
		}
	
		self.compare = compare;
		self.set_as_parent_node_of(compare);
		return self;
	}

	pub fn set_initialization(&mut self, initialization: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(initialization)?;
		if initialization == self.initialization {
			return self;
		}
		self.notify_property_change(ObservableProperty::INITIALIZATION, self.initialization, initialization);
		if self.initialization != null {
			self.initialization.set_parent_node(null);
		}
	
		self.initialization = initialization;
		self.set_as_parent_node_of(initialization);
		return self;
	}

	pub fn set_update(&mut self, update: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		com::github::javaparser::utils::utils::Utils::assert_not_null(update)?;
		if update == self.update {
			return self;
		}
		self.notify_property_change(ObservableProperty::UPDATE, self.update, update);
		if self.update != null {
			self.update.set_parent_node(null);
		}
	
		self.update = update;
		self.set_as_parent_node_of(update);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.compare != null {
			if node == self.compare {
				self.remove_compare();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.initialization.size() {
				{
					if self.initialization.get(i) == node {
						self.initialization.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.update.size() {
				{
					if self.update.get(i) == node {
						self.update.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn remove_compare(&self) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		return self.set_compare(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		return self.accept(CloneVisitor::new(), null) as ForStmt;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::for_stmt_meta_model::ForStmtMetaModel {
		return JavaParserMetaModel::forStmtMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as Statement)?;
			return true;
		}
		if self.compare != null {
			if node == self.compare {
				self.set_compare(replacement_node as Expression);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.initialization.size() {
				{
					if self.initialization.get(i) == node {
						self.initialization.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.update.size() {
				{
					if self.update.get(i) == node {
						self.update.set(i, replacement_node as Expression)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_for_stmt(&self) -> bool {
		return true;
	}

	pub fn as_for_stmt(&self) -> com::github::javaparser::ast::stmt::for_stmt::ForStmt {
		return self;
	}

	pub fn if_for_stmt(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_for_stmt(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_body::NodeWithBody for ForStmt {}

impl /* Java */ java::lang::Cloneable /**/ for ForStmt {}

impl com::github::javaparser::has_parent_node::HasParentNode for ForStmt {}

impl com::github::javaparser::ast::observer::observable::Observable for ForStmt {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ForStmt {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ForStmt {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ForStmt {}