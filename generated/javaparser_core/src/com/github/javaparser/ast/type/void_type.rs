use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::VoidTypeMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedVoidType;
use java::util::Optional;
use java::util::function::Consumer;

pub struct VoidType;

impl VoidType {
	pub fn new() -> com::github::javaparser::ast::type::void_type::VoidType {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::type::void_type::VoidType {
		super(token_range);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::void_type::VoidType {
		return super.set_annotations(annotations)? as VoidType;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return "void";
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return "V";
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::void_type::VoidType {
		return self.accept(CloneVisitor::new(), null) as VoidType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::void_type_meta_model::VoidTypeMetaModel {
		return JavaParserMetaModel::voidTypeMetaModel;
	}

	pub fn is_void_type(&self) -> bool {
		return true;
	}

	pub fn as_void_type(&self) -> com::github::javaparser::ast::type::void_type::VoidType {
		return self;
	}

	pub fn if_void_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_void_type::ResolvedVoidType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedVoidType.class);
	}

	pub fn to_void_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return ResolvedVoidType::INSTANCE;
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for VoidType {}

impl /* Java */ java::lang::Cloneable /**/ for VoidType {}

impl com::github::javaparser::has_parent_node::HasParentNode for VoidType {}

impl com::github::javaparser::ast::observer::observable::Observable for VoidType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for VoidType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for VoidType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for VoidType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for VoidType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for VoidType {}