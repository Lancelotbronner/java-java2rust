use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithImplements;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::EnumDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedEnumDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct EnumDeclaration {
	implemented_types: com::github::javaparser::ast::node_list::NodeList,
	entries: com::github::javaparser::ast::node_list::NodeList,
}

impl EnumDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		this(null, modifiers, NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, implemented_types: &com::github::javaparser::ast::node_list::NodeList, entries: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		this(null, modifiers, annotations, name, implemented_types, entries, members);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, implemented_types: &com::github::javaparser::ast::node_list::NodeList, entries: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		super(token_range, modifiers, annotations, name, members);
		self.set_implemented_types(implemented_types);
		self.set_entries(entries);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_entries(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.entries;
	}

	pub fn get_entry(&self, i: i32) -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		return self.get_entries().get(i);
	}

	pub fn set_entry(&self, i: i32, element: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		self.get_entries().set(i, element)?;
		return self;
	}

	pub fn add_entry(&self, element: &com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		self.get_entries().add(element);
		return self;
	}

	pub fn get_implemented_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.implemented_types;
	}

	pub fn set_entries(&mut self, entries: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(entries)?;
		if entries == self.entries {
			return self;
		}
		self.notify_property_change(ObservableProperty::ENTRIES, self.entries, entries);
		if self.entries != null {
			self.entries.set_parent_node(null);
		}
	
		self.entries = entries;
		self.set_as_parent_node_of(entries);
		return self;
	}

	pub fn set_implemented_types(&mut self, implemented_types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(implemented_types)?;
		if implemented_types == self.implementedTypes {
			return self;
		}
		self.notify_property_change(ObservableProperty::IMPLEMENTED_TYPES, self.implementedTypes, implemented_types);
		if self.implementedTypes != null {
			self.implementedTypes.set_parent_node(null);
		}
	
		self.implementedTypes = implemented_types;
		self.set_as_parent_node_of(implemented_types);
		return self;
	}

	pub fn add_enum_constant(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::enum_constant_declaration::EnumConstantDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_non_empty(name)?;
		let enum_constant: EnumConstantDeclaration = EnumConstantDeclaration::new(name);
		self.get_entries().add(enum_constant);
		return enum_constant;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.entries.size() {
				{
					if self.entries.get(i) == node {
						self.entries.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		return self.accept(CloneVisitor::new(), null) as EnumDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::enum_declaration_meta_model::EnumDeclarationMetaModel {
		return JavaParserMetaModel::enumDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.entries.size() {
				{
					if self.entries.get(i) == node {
						self.entries.set(i, replacement_node as EnumConstantDeclaration)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_enum_declaration(&self) -> bool {
		return true;
	}

	pub fn as_enum_declaration(&self) -> com::github::javaparser::ast::body::enum_declaration::EnumDeclaration {
		return self;
	}

	pub fn if_enum_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_enum_declaration::ResolvedEnumDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedEnumDeclaration.class);
	}

	pub fn to_enum_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_implements::NodeWithImplements for EnumDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for EnumDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for EnumDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for EnumDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for EnumDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_members::NodeWithMembers for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for EnumDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for EnumDeclaration {}