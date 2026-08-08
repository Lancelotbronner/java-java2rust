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
use crate::com::github::javaparser::metamodel::ConstructorDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedConstructorDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ConstructorDeclaration {
	body: com::github::javaparser::ast::stmt::block_stmt::BlockStmt,
}

impl ConstructorDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new(), NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		this(null, NodeList<>::new(Modifier::new()), NodeList<>::new(), NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		this(null, modifiers, NodeList<>::new(), NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), BlockStmt::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		this(null, modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, body, null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		this(null, modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, body, receiver_parameter);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		super(token_range, modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, receiver_parameter);
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

	pub fn set_body(&mut self, body: &com::github::javaparser::ast::stmt::block_stmt::BlockStmt) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
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

	pub fn set_modifiers(&self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return super.set_modifiers(modifiers)?;
	}

	pub fn set_name(&self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return super.set_name(name)?;
	}

	pub fn set_parameters(&self, parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return super.set_parameters(parameters)?;
	}

	pub fn set_thrown_exceptions(&self, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return super.set_thrown_exceptions(thrown_exceptions)?;
	}

	pub fn set_type_parameters(&self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return super.set_type_parameters(type_parameters)?;
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
			}
		}
		sb.append(")");
		sb.append(&self.append_throws_if_requested(including_throws));
		return sb.toString();
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append('(');
		 {
			let i: i32 = 0;
			while i < self.get_parameters().size() {
				{
					sb.append(&self.get_parameter(i).get_type().to_descriptor());
				}
				i += 1;
			 }
		 }
	
		return sb.append(")V").toString();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return self.accept(CloneVisitor::new(), null) as ConstructorDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::constructor_declaration_meta_model::ConstructorDeclarationMetaModel {
		return JavaParserMetaModel::constructorDeclarationMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.body {
			self.set_body(replacement_node as BlockStmt)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_constructor_declaration(&self) -> bool {
		return true;
	}

	pub fn as_constructor_declaration(&self) -> com::github::javaparser::ast::body::constructor_declaration::ConstructorDeclaration {
		return self;
	}

	pub fn if_constructor_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedConstructorDeclaration.class);
	}

	pub fn to_constructor_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_block_stmt::NodeWithBlockStmt for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for ConstructorDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ConstructorDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for ConstructorDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for ConstructorDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for ConstructorDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_declaration::NodeWithDeclaration for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for ConstructorDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for ConstructorDeclaration {}