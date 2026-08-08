use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithIdentifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::SimpleNameMetaModel;

pub struct SimpleName {
	identifier: /* Java */ java::lang::String /**/,
}

impl SimpleName {
	pub fn new() -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		this(null, "empty");
	}

	pub fn new(identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		this(null, identifier);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		super(token_range);
		self.set_identifier(identifier);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_identifier(&self) -> /* Java */ java::lang::String /**/ {
		return self.identifier;
	}

	pub fn set_identifier(&mut self, identifier: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		com::github::javaparser::utils::utils::Utils::assert_non_empty(identifier)?;
		if identifier.equals(self.identifier) {
			return self;
		}
		self.notify_property_change(ObservableProperty::IDENTIFIER, self.identifier, identifier);
		self.identifier = identifier;
		return self;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.identifier;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.accept(CloneVisitor::new(), null) as SimpleName;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::simple_name_meta_model::SimpleNameMetaModel {
		return JavaParserMetaModel::simpleNameMetaModel;
	}
}

impl com::github::javaparser::ast::node_types::node_with_identifier::NodeWithIdentifier for SimpleName {}

impl /* Java */ java::lang::Cloneable /**/ for SimpleName {}

impl com::github::javaparser::has_parent_node::HasParentNode for SimpleName {}

impl com::github::javaparser::ast::observer::observable::Observable for SimpleName {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for SimpleName {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for SimpleName {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for SimpleName {}