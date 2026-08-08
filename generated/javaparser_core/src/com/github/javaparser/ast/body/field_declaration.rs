use crate::com::github::javaparser::ast::Modifier::Keyword::STATIC;
use crate::com::github::javaparser::ast::NodeList::nodeList;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::Modifier::Keyword;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::AssignExpr;
use crate::com::github::javaparser::ast::expr::AssignExpr::Operator;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithJavadoc;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithVariables;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAccessModifiers;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithStaticModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::stmt::ReturnStmt;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::VoidType;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::FieldDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedFieldDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct FieldDeclaration {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	variables: com::github::javaparser::ast::node_list::NodeList,
}

impl FieldDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, variable: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		this(null, modifiers, NodeList<>::new(), &com::github::javaparser::ast::node_list::NodeList::node_list(variable));
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		this(null, modifiers, NodeList<>::new(), variables);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		this(null, modifiers, annotations, variables);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		super(token_range, annotations);
		self.set_modifiers(modifiers);
		self.set_variables(variables);
		self.custom_initialization();
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		this(&com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?, VariableDeclarator::new(type, &com::github::javaparser::utils::utils::Utils::assert_not_null(name)?));
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn get_variables(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.variables;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
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

	pub fn set_variables(&mut self, variables: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(variables)?;
		if variables == self.variables {
			return self;
		}
		self.notify_property_change(ObservableProperty::VARIABLES, self.variables, variables);
		if self.variables != null {
			self.variables.set_parent_node(null);
		}
	
		self.variables = variables;
		self.set_as_parent_node_of(variables);
		return self;
	}

	pub fn create_getter(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		if self.get_variables().size() != 1 {
			return Err(IllegalStateException::new("You can use this only when the field declares only 1 variable name"));
		}
	
		let parent_class: Optional<ClassOrInterfaceDeclaration> = .findAncestor(ClassOrInterfaceDeclaration.class);
		let parent_enum: Optional<EnumDeclaration> = .findAncestor(EnumDeclaration.class);
		if !(parent_class.isPresent() || parent_enum.isPresent()) || (parent_class.isPresent() && parent_class.get().is_interface()) {
			return Err(IllegalStateException::new("You can use this only when the field is attached to a class or an enum"));
		}
	
		let variable: VariableDeclarator = self.get_variable(0);
		let field_name: String = variable.get_name_as_string();
		let field_name_upper: String = field_name.toUpperCase().substring(0, 1) + field_name.substring(1, &field_name.length());
		/* final */ let getter: MethodDeclaration;
		getter = parent_class.map(|clazz|clazz.add_method("get" + field_name_upper, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC)).orElseGet(|()|parent_enum.get().add_method("get" + field_name_upper, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC));
		getter.set_type(&variable.get_type());
		let block_stmt: BlockStmt = BlockStmt::new();
		getter.set_body(block_stmt);
		block_stmt.add_statement(ReturnStmt::new(field_name));
		return getter;
	}

	pub fn create_setter(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		if self.get_variables().size() != 1 {
			return Err(IllegalStateException::new("You can use this only when the field declares only 1 variable name"));
		}
	
		let parent_class: Optional<ClassOrInterfaceDeclaration> = .findAncestor(ClassOrInterfaceDeclaration.class);
		let parent_enum: Optional<EnumDeclaration> = .findAncestor(EnumDeclaration.class);
		if !(parent_class.isPresent() || parent_enum.isPresent()) || (parent_class.isPresent() && parent_class.get().is_interface()) {
			return Err(IllegalStateException::new("You can use this only when the field is attached to a class or an enum"));
		}
	
		let variable: VariableDeclarator = self.get_variable(0);
		let field_name: String = variable.get_name_as_string();
		let field_name_upper: String = field_name.toUpperCase().substring(0, 1) + field_name.substring(1, &field_name.length());
		/* final */ let setter: MethodDeclaration;
		setter = parent_class.map(|clazz|clazz.add_method("set" + field_name_upper, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC)).orElseGet(|()|parent_enum.get().add_method("set" + field_name_upper, Modifier::com::github::javaparser::ast::modifier::Keyword::PUBLIC));
		setter.set_type(VoidType::new());
		setter.get_parameters().add(Parameter::new(&variable.get_type(), field_name));
		let block_stmt2: BlockStmt = BlockStmt::new();
		setter.set_body(block_stmt2);
		block_stmt2.add_statement(AssignExpr::new(NameExpr::new("this." + field_name), NameExpr::new(field_name), Operator::ASSIGN));
		return setter;
	}

	pub fn is_transient(&self) -> bool {
		return self.has_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::TRANSIENT);
	}

	pub fn is_volatile(&self) -> bool {
		return self.has_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::VOLATILE);
	}

	pub fn set_transient(&self, set: bool) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return self.set_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::TRANSIENT, set);
	}

	pub fn set_volatile(&self, set: bool) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return self.set_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::VOLATILE, set);
	}

	pub fn is_static(&self) -> bool {
		return self.has_modifier(STATIC) || self.is_declared_in_interface();
	}

	pub fn is_final(&self) -> bool {
		return self.has_modifier(Keyword::FINAL) || self.is_declared_in_interface();
	}

	pub fn is_public(&self) -> bool {
		return self.has_modifier(Keyword::PUBLIC) || self.is_declared_in_interface();
	}

	fn is_declared_in_interface(&self) -> bool {
		let parent_type: Optional<TypeDeclaration> = .findAncestor(TypeDeclaration.class);
		return parent_type.filter(BodyDeclaration::isClassOrInterfaceDeclaration).map(BodyDeclaration::asClassOrInterfaceDeclaration).map(ClassOrInterfaceDeclaration::isInterface).orElse(false);
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
			while i < self.variables.size() {
				{
					if self.variables.get(i) == node {
						self.variables.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return self.accept(CloneVisitor::new(), null) as FieldDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::field_declaration_meta_model::FieldDeclarationMetaModel {
		return JavaParserMetaModel::fieldDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
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
	
		 {
			let i: i32 = 0;
			while i < self.variables.size() {
				{
					if self.variables.get(i) == node {
						self.variables.set(i, replacement_node as VariableDeclarator)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_field_declaration(&self) -> bool {
		return true;
	}

	pub fn as_field_declaration(&self) -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return self;
	}

	pub fn if_field_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_field_declaration::ResolvedFieldDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedFieldDeclaration.class);
	}

	pub fn to_field_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_variables::NodeWithVariables for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for FieldDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for FieldDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for FieldDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for FieldDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for FieldDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for FieldDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for FieldDeclaration {}