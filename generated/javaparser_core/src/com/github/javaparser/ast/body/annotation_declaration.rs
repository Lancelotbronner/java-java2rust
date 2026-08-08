use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Modifier;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithAbstractModifier;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::AnnotationDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedAnnotationDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct AnnotationDeclaration;

impl AnnotationDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		this(null, modifiers, NodeList<>::new(), SimpleName::new(name), NodeList<>::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		this(null, modifiers, annotations, name, members);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, members: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		super(token_range, modifiers, annotations, name, members);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		return self.accept(CloneVisitor::new(), null) as AnnotationDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::annotation_declaration_meta_model::AnnotationDeclarationMetaModel {
		return JavaParserMetaModel::annotationDeclarationMetaModel;
	}

	pub fn is_annotation_declaration(&self) -> bool {
		return true;
	}

	pub fn as_annotation_declaration(&self) -> com::github::javaparser::ast::body::annotation_declaration::AnnotationDeclaration {
		return self;
	}

	pub fn if_annotation_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_annotation_declaration::ResolvedAnnotationDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedAnnotationDeclaration.class);
	}

	pub fn to_annotation_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn add_field(&self, type: &com::github::javaparser::ast::type::type::Type, name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Keyword) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::ast::body::field_declaration::FieldDeclaration {
		return Err(IllegalStateException::new("Cannot add a field to an annotation declaration."));
	}
}

impl com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for AnnotationDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for AnnotationDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for AnnotationDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for AnnotationDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_members::NodeWithMembers for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for AnnotationDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for AnnotationDeclaration {}