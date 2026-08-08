use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::PackageDeclarationMetaModel;

pub struct PackageDeclaration {
	annotations: com::github::javaparser::ast::node_list::NodeList = NodeList<>::new(),
	name: com::github::javaparser::ast::expr::name::Name,
}

impl PackageDeclaration {
	pub fn new() -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		this(null, NodeList<>::new(), Name::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		this(null, NodeList<>::new(), name);
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		this(null, annotations, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		super(token_range);
		self.set_annotations(annotations);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		if annotations == self.annotations {
			return self;
		}
		self.notify_property_change(ObservableProperty::ANNOTATIONS, self.annotations, annotations);
		if self.annotations != null {
			self.annotations.set_parent_node(null);
		}
	
		self.annotations = annotations;
		self.set_as_parent_node_of(annotations);
		return self;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
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

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::package_declaration::PackageDeclaration {
		return self.accept(CloneVisitor::new(), null) as PackageDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::package_declaration_meta_model::PackageDeclarationMetaModel {
		return JavaParserMetaModel::packageDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.set(i, replacement_node as AnnotationExpr)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as Name)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for PackageDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for PackageDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for PackageDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for PackageDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for PackageDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for PackageDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for PackageDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for PackageDeclaration {}