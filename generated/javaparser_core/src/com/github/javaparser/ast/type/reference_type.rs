use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::ReferenceTypeMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ReferenceType;

impl ReferenceType {
	pub fn new() -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		this(null, NodeList<>::new());
	}

	pub fn new(annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		this(null, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		super(token_range, annotations);
		self.custom_initialization();
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		return self.accept(CloneVisitor::new(), null) as ReferenceType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::reference_type_meta_model::ReferenceTypeMetaModel {
		return JavaParserMetaModel::referenceTypeMetaModel;
	}

	pub fn is_reference_type(&self) -> bool {
		return true;
	}

	pub fn as_reference_type(&self) -> com::github::javaparser::ast::type::reference_type::ReferenceType {
		return self;
	}

	pub fn if_reference_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_reference_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ ;
}

impl /* Java */ java::lang::Cloneable /**/ for ReferenceType {}

impl com::github::javaparser::has_parent_node::HasParentNode for ReferenceType {}

impl com::github::javaparser::ast::observer::observable::Observable for ReferenceType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ReferenceType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ReferenceType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ReferenceType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ReferenceType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for ReferenceType {}