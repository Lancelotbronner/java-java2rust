use crate::com::github::javaparser::StaticJavaParser::parseName;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::MarkerAnnotationExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct MarkerAnnotationExpr;

impl MarkerAnnotationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		this(null, Name::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		this(null, &com::github::javaparser::static_java_parser::StaticJavaParser::parse_name(name));
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		this(null, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name) -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		super(token_range, name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		return self.accept(CloneVisitor::new(), null) as MarkerAnnotationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::marker_annotation_expr_meta_model::MarkerAnnotationExprMetaModel {
		return JavaParserMetaModel::markerAnnotationExprMetaModel;
	}

	pub fn is_marker_annotation_expr(&self) -> bool {
		return true;
	}

	pub fn as_marker_annotation_expr(&self) -> com::github::javaparser::ast::expr::marker_annotation_expr::MarkerAnnotationExpr {
		return self;
	}

	pub fn if_marker_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_marker_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for MarkerAnnotationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for MarkerAnnotationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for MarkerAnnotationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for MarkerAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for MarkerAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for MarkerAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for MarkerAnnotationExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for MarkerAnnotationExpr {}