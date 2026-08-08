use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::AnnotationExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedAnnotationDeclaration;
use java::util::Optional;
use java::util::function::Consumer;

pub struct AnnotationExpr {
	name: com::github::javaparser::ast::expr::name::Name,
}

impl AnnotationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		this(null, Name::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		this(null, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		super(token_range);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::name::Name {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::name::Name) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
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

	pub fn clone(&self) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		return self.accept(CloneVisitor::new(), null) as AnnotationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::annotation_expr_meta_model::AnnotationExprMetaModel {
		return JavaParserMetaModel::annotationExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as Name)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_annotation_expr(&self) -> bool {
		return true;
	}

	pub fn as_annotation_expr(&self) -> com::github::javaparser::ast::expr::annotation_expr::AnnotationExpr {
		return self;
	}

	pub fn if_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_annotation_declaration::ResolvedAnnotationDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedAnnotationDeclaration.class);
	}

	pub fn to_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for AnnotationExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for AnnotationExpr {}

impl /* Java */ java::lang::Cloneable /**/ for AnnotationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for AnnotationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for AnnotationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for AnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for AnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for AnnotationExpr {}