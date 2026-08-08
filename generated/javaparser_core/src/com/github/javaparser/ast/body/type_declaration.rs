use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithJavadoc;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithMembers;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAccessModifiers;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithStaticModifier;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithStrictfpModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TypeDeclarationMetaModel;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct TypeDeclaration<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	members: com::github::javaparser::ast::node_list::NodeList,
}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> TypeDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		this(null, modifiers, NodeList<>::new(), SimpleName::new(name), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		this(null, modifiers, annotations, name, members);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		super(token_range, annotations);
		self.set_modifiers(modifiers);
		self.set_name(name);
		self.set_members(members);
		self.custom_initialization();
	}

	pub fn add_member(&self, decl: &com::github::javaparser::ast::body::body_declaration::BodyDeclaration) -> T {
		let members: NodeList<BodyDeclaration<?>> = self.get_members();
		members.add(decl);
		return self as T;
	}

	pub fn get_members(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.members;
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_members(&mut self, members: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(members)?;
		if members == self.members {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::MEMBERS, self.members, members);
		if self.members != null {
			self.members.set_parent_node(null);
		}
	
		self.members = members;
		self.set_as_parent_node_of(members);
		return self as T;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self as T;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self as T;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.members.size() {
				{
					if self.members.get(i) == node {
						self.members.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn is_top_level_type(&self) -> bool {
		return self.get_parent_node().map(|p|p instanceof CompilationUnit).orElse(false);
	}

	pub fn get_callables_with_signature(&self, signature: &com::github::javaparser::ast::body::callable_declaration::Signature) -> /* Java */ java::util::List /**/ {
		return self.get_members().stream().filter(|m|m instanceof CallableDeclaration).map(|m|(m as CallableDeclaration<?>)).filter(|m|m.get_signature().equals(signature)).collect(&/* Java */ java::util::stream::Collectors /**/::toList());
	}

	pub fn get_fully_qualified_name(&self) -> /* Java */ java::util::Optional /**/ {
		if self.is_top_level_type() {
			return self.find_compilation_unit().map(|cu|cu.get_package_declaration().map(|pd|pd.get_name_as_string()).map(|pkg|pkg + "." + self.get_name_as_string()).orElseGet(|()|self.get_name_as_string()));
		}
		return .findAncestor(TypeDeclaration.class).map(|td|td as TypeDeclaration<?>).flatMap(|td|td.getFullyQualifiedName().map(|fqn|fqn + "." + self.get_name_as_string()));
	}

	pub fn is_nested_type(&self) -> bool {
		return self.get_parent_node().map(|p|p instanceof TypeDeclaration).orElse(false);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		return self.accept(CloneVisitor::new(), null) as TypeDeclaration<?>;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::type_declaration_meta_model::TypeDeclarationMetaModel {
		return JavaParserMetaModel::typeDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.members.size() {
				{
					if self.members.get(i) == node {
						self.members.set(i, replacement_node as BodyDeclaration)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.set(i, replacement_node as Modifier)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_type_declaration(&self) -> bool {
		return true;
	}

	pub fn as_type_declaration(&self) -> com::github::javaparser::ast::body::type_declaration::TypeDeclaration {
		return self;
	}

	pub fn if_type_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_type_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration ;
}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_members::NodeWithMembers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> /* Java */ java::lang::Cloneable /**/ for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::has_parent_node::HasParentNode for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::observer::observable::Observable for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::visitor::visitable::Visitable for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TypeDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::type_declaration::TypeDeclaration> com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for TypeDeclaration<T> {}