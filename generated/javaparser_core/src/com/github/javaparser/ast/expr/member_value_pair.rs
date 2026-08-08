use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MemberValuePairMetaModel;

pub struct MemberValuePair {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	value: com::github::javaparser::ast::expr::expression::Expression,
}

impl MemberValuePair {
	pub fn new() -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		this(null, SimpleName::new(), StringLiteralExpr::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		this(null, SimpleName::new(name), value);
	}

	pub fn new(name: &com::github::javaparser::ast::expr::simple_name::SimpleName, value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		this(null, name, value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		super(token_range);
		self.set_name(name);
		self.set_value(value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn get_value(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.value;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self;
	}

	pub fn set_value(&mut self, value: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		com::github::javaparser::utils::utils::Utils::assert_not_null(value)?;
		if value == self.value {
			return self;
		}
		self.notify_property_change(ObservableProperty::VALUE, self.value, value);
		if self.value != null {
			self.value.set_parent_node(null);
		}
	
		self.value = value;
		self.set_as_parent_node_of(value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::member_value_pair::MemberValuePair {
		return self.accept(CloneVisitor::new(), null) as MemberValuePair;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::member_value_pair_meta_model::MemberValuePairMetaModel {
		return JavaParserMetaModel::memberValuePairMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if node == self.value {
			self.set_value(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for MemberValuePair {}

impl /* Java */ java::lang::Cloneable /**/ for MemberValuePair {}

impl com::github::javaparser::has_parent_node::HasParentNode for MemberValuePair {}

impl com::github::javaparser::ast::observer::observable::Observable for MemberValuePair {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MemberValuePair {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MemberValuePair {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MemberValuePair {}