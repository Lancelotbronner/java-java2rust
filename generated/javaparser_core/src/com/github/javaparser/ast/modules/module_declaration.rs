use crate::com::github::javaparser::StaticJavaParser::parseModuleDirective;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Name;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ModuleDeclarationMetaModel;

pub struct ModuleDeclaration {
	name: com::github::javaparser::ast::expr::name::Name,
	annotations: com::github::javaparser::ast::node_list::NodeList,
	is_open: bool,
	directives: com::github::javaparser::ast::node_list::NodeList,
}

impl ModuleDeclaration {
	pub fn new() -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		this(null, NodeList<>::new(), Name::new(), false, NodeList<>::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, is_open: bool) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		this(null, NodeList<>::new(), name, is_open, NodeList<>::new());
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::name::Name, is_open: bool, directives: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		this(null, annotations, name, is_open, directives);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::name::Name, is_open: bool, directives: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		super(token_range);
		self.set_annotations(annotations);
		self.set_name(name);
		self.set_open(is_open);
		self.set_directives(directives);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
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

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
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
	
		 {
			let i: i32 = 0;
			while i < self.directives.size() {
				{
					if self.directives.get(i) == node {
						self.directives.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn is_open(&self) -> bool {
		return self.is_open;
	}

	pub fn set_open(&mut self, is_open: bool) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		if is_open == self.isOpen {
			return self;
		}
		self.notify_property_change(ObservableProperty::OPEN, self.isOpen, is_open);
		self.isOpen = is_open;
		return self;
	}

	pub fn get_directives(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.directives;
	}

	pub fn set_directives(&mut self, directives: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(directives)?;
		if directives == self.directives {
			return self;
		}
		self.notify_property_change(ObservableProperty::DIRECTIVES, self.directives, directives);
		if self.directives != null {
			self.directives.set_parent_node(null);
		}
	
		self.directives = directives;
		self.set_as_parent_node_of(directives);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		return self.accept(CloneVisitor::new(), null) as ModuleDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::module_declaration_meta_model::ModuleDeclarationMetaModel {
		return JavaParserMetaModel::moduleDeclarationMetaModel;
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
	
		 {
			let i: i32 = 0;
			while i < self.directives.size() {
				{
					if self.directives.get(i) == node {
						self.directives.set(i, replacement_node as ModuleDirective)?;
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
		return super.replace(node, replacement_node);
	}

	pub fn add_directive(&self, directive: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		return self.add_directive(&com::github::javaparser::static_java_parser::StaticJavaParser::parse_module_directive(directive));
	}

	pub fn add_directive(&self, directive: &com::github::javaparser::ast::modules::module_directive::ModuleDirective) -> com::github::javaparser::ast::modules::module_declaration::ModuleDeclaration {
		self.get_directives().add(directive);
		return self;
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for ModuleDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ModuleDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for ModuleDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for ModuleDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for ModuleDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ModuleDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ModuleDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ModuleDeclaration {}