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
use crate::com::github::javaparser::metamodel::NameMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;

pub struct Name {
	identifier: /* Java */ java::lang::String /**/,
	qualifier: com::github::javaparser::ast::expr::name::Name,
}

impl Name {
	pub fn new() -> com::github::javaparser::ast::expr::name::Name {
		this(null, null, "empty");
	}

	pub fn new(identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::name::Name {
		this(null, null, identifier);
	}

	pub fn new(qualifier: &com::github::javaparser::ast::expr::name::Name, identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::name::Name {
		this(null, qualifier, identifier);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, qualifier: &com::github::javaparser::ast::expr::name::Name, identifier: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::name::Name {
		super(token_range);
		self.set_qualifier(qualifier);
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

	pub fn set_identifier(&mut self, identifier: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::name::Name {
		com::github::javaparser::utils::utils::Utils::assert_non_empty(identifier)?;
		if identifier.equals(self.identifier) {
			return self;
		}
		self.notify_property_change(ObservableProperty::IDENTIFIER, self.identifier, identifier);
		self.identifier = identifier;
		return self;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		if self.qualifier != null {
			return self.qualifier.as_string() + "." + self.identifier;
		}
		return self.identifier;
	}

	pub fn has_qualifier(&self) -> bool {
		return self.qualifier != null;
	}

	pub fn get_qualifier(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.qualifier);
	}

	pub fn set_qualifier(&mut self, qualifier: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::name::Name {
		if qualifier == self.qualifier {
			return self;
		}
		self.notify_property_change(ObservableProperty::QUALIFIER, self.qualifier, qualifier);
		if self.qualifier != null {
			self.qualifier.set_parent_node(null);
		}
	
		self.qualifier = qualifier;
		self.set_as_parent_node_of(qualifier);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.qualifier != null {
			if node == self.qualifier {
				self.remove_qualifier();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_qualifier(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.set_qualifier(null as Name);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.accept(CloneVisitor::new(), null) as Name;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::name_meta_model::NameMetaModel {
		return JavaParserMetaModel::nameMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.qualifier != null {
			if node == self.qualifier {
				self.set_qualifier(replacement_node as Name);
				return true;
			}
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_top_level(&self) -> bool {
		return !self.is_internal();
	}

	pub fn is_internal(&self) -> bool {
		return self.get_parent_node().filter(|parent|parent instanceof Name).isPresent();
	}
}

impl com::github::javaparser::ast::node_types::node_with_identifier::NodeWithIdentifier for Name {}

impl /* Java */ java::lang::Cloneable /**/ for Name {}

impl com::github::javaparser::has_parent_node::HasParentNode for Name {}

impl com::github::javaparser::ast::observer::observable::Observable for Name {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for Name {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for Name {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for Name {}