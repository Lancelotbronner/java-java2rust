use crate::com::github::javaparser::ast::Modifier::Keyword::PUBLIC;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::Modifier::Keyword;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes;
use crate::com::github::javaparser::ast::nodeTypes::modifiers;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::BlockStmt;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::ReferenceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MethodDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use java::util::Arrays;
use java::util::Optional;
use java::util::function::Consumer;

pub struct MethodDeclaration {
	type: com::github::javaparser::ast::type::type::Type,
	body: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
}

impl MethodDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), ClassOrInterfaceType::new(), SimpleName::new(), NodeList<>::new(), NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		this(null, modifiers, NodeList<>::new(), NodeList<>::new(), type, SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/, type: &com::github::javaparser::ast::type::type::Type, parameters: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		this(null, modifiers, NodeList<>::new(), NodeList<>::new(), type, SimpleName::new(name), parameters, NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		this(null, modifiers, annotations, type_parameters, type, name, parameters, thrown_exceptions, body, null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		this(null, modifiers, annotations, type_parameters, type, name, parameters, thrown_exceptions, body, receiver_parameter);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		super(token_range, modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, receiver_parameter);
		self.set_type(type);
		self.set_body(body);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_body(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.body);
	}

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
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

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		if self.type != null {
			self.type.set_parent_node(null);
		}
	
		self.type = type;
		self.set_as_parent_node_of(type);
		return self;
	}

	pub fn set_modifiers(&self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return super.set_modifiers(modifiers)?;
	}

	pub fn set_name(&self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return super.set_name(name)?;
	}

	pub fn set_parameters(&self, parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return super.set_parameters(parameters)?;
	}

	pub fn set_thrown_exceptions(&self, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return super.set_thrown_exceptions(thrown_exceptions)?;
	}

	pub fn set_type_parameters(&self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return super.set_type_parameters(type_parameters)?;
	}

	pub fn get_declaration_as_string(&self, including_modifiers: bool, including_throws: bool, including_parameter_name: bool) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if including_modifiers {
			let access_specifier: AccessSpecifier = self.get_access_specifier();
			sb.append(&access_specifier.as_string()).append(" ");
			if self.is_static() {
				sb.append("static ");
			}
			if self.is_abstract() {
				sb.append("abstract ");
			}
			if self.is_final() {
				sb.append("final ");
			}
			if self.is_native() {
				sb.append("native ");
			}
			if self.is_synchronized() {
				sb.append("synchronized ");
			}
		}
		sb.append(&self.get_type().to_string());
		sb.append(" ");
		sb.append(&self.get_name().to_string());
		sb.append("(");
		let first_param: bool = true;
		for param in self.get_parameters() {
			if first_param {
				first_param = false;
			} else {
				sb.append(", ");
			}
			if including_parameter_name {
				sb.append(&param.to_string());
			} else {
				sb.append(&param.get_type().to_string());
				if param.is_var_args() {
					sb.append("...");
				}
			}
		}
		sb.append(")");
		sb.append(&self.append_throws_if_requested(including_throws));
		return sb.toString();
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append("(");
		 {
			let i: i32 = 0;
			while i < self.get_parameters().size() {
				{
					sb.append(&self.get_parameter(i).get_type().to_descriptor());
				}
				i += 1;
			 }
		 }
	
		sb.append(")");
		sb.append(&self.get_type().to_descriptor());
		return sb.toString();
	}

	pub fn is_public(&self) -> bool {
		return self.has_modifier(PUBLIC) || self.is_implicitly_public();
	}

	fn is_implicitly_public(&self) -> bool {
		return self.get_access_specifier() == AccessSpecifier::NONE && self.has_parent_node() && self.get_parent_node().get() instanceof ClassOrInterfaceDeclaration && (self.get_parent_node().get() as ClassOrInterfaceDeclaration).is_interface();
	}

	pub fn is_abstract(&self) -> bool {
		return super.is_abstract() || self.is_implicitly_abstract();
	}

	fn is_implicitly_abstract(&self) -> bool {
		return self.has_parent_node() && self.get_parent_node().get() instanceof ClassOrInterfaceDeclaration && (self.get_parent_node().get() as ClassOrInterfaceDeclaration).is_interface() && Arrays::asList(Keyword::STATIC, Keyword::DEFAULT, Keyword::PRIVATE).stream().noneMatch(|modifier|self.has_modifier(modifier));
	}

	pub fn is_native(&self) -> bool {
		return self.has_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::NATIVE);
	}

	pub fn is_synchronized(&self) -> bool {
		return self.has_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::SYNCHRONIZED);
	}

	pub fn is_default(&self) -> bool {
		return self.has_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::DEFAULT);
	}

	pub fn set_native(&self, set: bool) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.set_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::NATIVE, set);
	}

	pub fn set_synchronized(&self, set: bool) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.set_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::SYNCHRONIZED, set);
	}

	pub fn set_default(&self, set: bool) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.set_modifier(Modifier::com::github::javaparser::ast::modifier::Keyword::DEFAULT, set);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.body != null {
			if node == self.body {
				self.remove_body();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn remove_body(&self) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.set_body(null as BlockStmt);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self.accept(CloneVisitor::new(), null) as MethodDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::method_declaration_meta_model::MethodDeclarationMetaModel {
		return JavaParserMetaModel::methodDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if self.body != null {
			if node == self.body {
				self.set_body(replacement_node as BlockStmt);
				return true;
			}
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_method_declaration(&self) -> bool {
		return true;
	}

	pub fn as_method_declaration(&self) -> com::github::javaparser::ast::body::method_declaration::MethodDeclaration {
		return self;
	}

	pub fn if_method_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedMethodDeclaration.class);
	}

	pub fn to_method_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_optional_block_stmt::NodeWithOptionalBlockStmt for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_declaration::NodeWithDeclaration for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for MethodDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for MethodDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for MethodDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for MethodDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_declaration::NodeWithDeclaration for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for MethodDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for MethodDeclaration {}