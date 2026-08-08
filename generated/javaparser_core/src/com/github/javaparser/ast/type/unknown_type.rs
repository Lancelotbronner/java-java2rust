use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::UnknownTypeMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedReferenceType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::Optional;
use java::util::function::Consumer;

pub struct UnknownType;

impl UnknownType {
	pub fn new() -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		this(null);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange) -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		super(token_range);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError | java.lang.IllegalStateException) */ -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		if annotations.size() > 0 {
			return Err(IllegalStateException::new("Inferred lambda types cannot be annotated."));
		}
		return super.set_annotations(annotations)? as UnknownType;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return "";
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		return self.accept(CloneVisitor::new(), null) as UnknownType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::unknown_type_meta_model::UnknownTypeMetaModel {
		return JavaParserMetaModel::unknownTypeMetaModel;
	}

	pub fn is_unknown_type(&self) -> bool {
		return true;
	}

	pub fn as_unknown_type(&self) -> com::github::javaparser::ast::type::unknown_type::UnknownType {
		return self;
	}

	pub fn if_unknown_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedReferenceType.class);
	}

	pub fn to_unknown_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_phantom(&self) -> bool {
		return true;
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return Err(IllegalArgumentException::new("Inferred lambda parameter type"));
	}
}

impl /* Java */ java::lang::Cloneable /**/ for UnknownType {}

impl com::github::javaparser::has_parent_node::HasParentNode for UnknownType {}

impl com::github::javaparser::ast::observer::observable::Observable for UnknownType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for UnknownType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for UnknownType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for UnknownType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for UnknownType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for UnknownType {}