use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAccessModifiers;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::type::ReferenceType;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::CompactConstructorDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedConstructorDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct CompactConstructorDeclaration {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	body: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
	type_parameters: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	thrown_exceptions: com::github::javaparser::ast::node_list::NodeList,
}

impl CompactConstructorDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new(), BlockStmt::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		this(null, NodeList<>::new(Modifier::new()), NodeList<>::new(), NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), BlockStmt::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		this(null, modifiers, NodeList<>::new(), NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), BlockStmt::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		this(null, modifiers, annotations, type_parameters, name, thrown_exceptions, body);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		super(token_range, annotations);
		self.set_modifiers(modifiers);
		self.set_type_parameters(type_parameters);
		self.set_name(name);
		self.set_thrown_exceptions(thrown_exceptions);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_body(&self) -> com::github::javaparser::ast::stmt::block_stmt::BlockStmt {
		return self.body;
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
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

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
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

	pub fn get_thrown_exceptions(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.thrown_exceptions;
	}

	pub fn set_thrown_exceptions(&mut self, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(thrown_exceptions)?;
		if thrown_exceptions == self.thrownExceptions {
			return self;
		}
		self.notify_property_change(ObservableProperty::THROWN_EXCEPTIONS, self.thrownExceptions, thrown_exceptions);
		if self.thrownExceptions != null {
			self.thrownExceptions.set_parent_node(null);
		}
	
		self.thrownExceptions = thrown_exceptions;
		self.set_as_parent_node_of(thrown_exceptions);
		return self;
	}

	pub fn get_type_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.type_parameters;
	}

	pub fn set_type_parameters(&mut self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type_parameters)?;
		if type_parameters == self.typeParameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_PARAMETERS, self.typeParameters, type_parameters);
		if self.typeParameters != null {
			self.typeParameters.set_parent_node(null);
		}
	
		self.typeParameters = type_parameters;
		self.set_as_parent_node_of(type_parameters);
		return self;
	}

	pub fn get_declaration_as_string(&self, including_modifiers: bool, including_throws: bool, including_parameter_name: bool) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if including_modifiers {
			let access_specifier: AccessSpecifier = self.get_access_specifier();
			sb.append(&access_specifier.as_string()).append(" ");
		}
		sb.append(&self.get_name());
		sb.append("(");
		let first_param: bool = true;
		sb.append(")");
		sb.append(&self.append_throws_if_requested(including_throws));
		return sb.toString();
	}

	fn append_throws_if_requested(&self, including_throws: bool) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if including_throws {
			let first_throw: bool = true;
			for thr in self.get_thrown_exceptions() {
				if first_throw {
					first_throw = false;
					sb.append(" throws ");
				} else {
					sb.append(", ");
				}
				sb.append(&thr.to_string());
			}
		}
		return sb.toString();
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
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
	
		 {
			let i: i32 = 0;
			while i < self.thrown_exceptions.size() {
				{
					if self.thrown_exceptions.get(i) == node {
						self.thrown_exceptions.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		return self.accept(CloneVisitor::new(), null) as CompactConstructorDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::compact_constructor_declaration_meta_model::CompactConstructorDeclarationMetaModel {
		return JavaParserMetaModel::compactConstructorDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as BlockStmt)?;
			return true;
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
		 {
			let i: i32 = 0;
			while i < self.thrown_exceptions.size() {
				{
					if self.thrown_exceptions.get(i) == node {
						self.thrown_exceptions.set(i, replacement_node as ReferenceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.set(i, replacement_node as TypeParameter)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_compact_constructor_declaration(&self) -> bool {
		return true;
	}

	pub fn as_compact_constructor_declaration(&self) -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		return self;
	}

	pub fn if_compact_constructor_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedConstructorDeclaration.class);
	}

	pub fn to_compact_constructor_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::compact_constructor_declaration::CompactConstructorDeclaration {
		super(token_range, annotations);
		self.set_body(body)?;
		self.custom_initialization();
	}
}

impl com::github::javaparser::ast::node_types::node_with_block_stmt::NodeWithBlockStmt for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for CompactConstructorDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for CompactConstructorDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for CompactConstructorDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CompactConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for CompactConstructorDeclaration {}