use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::IntegerLiteralExpr;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ArrayCreationLevelMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;

pub struct ArrayCreationLevel {
	dimension: com::github::javaparser::ast::expr::expression::Expression,
	annotations: com::github::javaparser::ast::node_list::NodeList = NodeList<>::new(),
}

impl ArrayCreationLevel {
	pub fn new() -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		this(null, null, NodeList<>::new());
	}

	pub fn new(dimension: i32) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		this(null, IntegerLiteralExpr::new("" + dimension), NodeList<>::new());
	}

	pub fn new(dimension: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		this(null, dimension, NodeList<>::new());
	}

	pub fn new(dimension: &com::github::javaparser::ast::expr::expression::Expression, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		this(null, dimension, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, dimension: &com::github::javaparser::ast::expr::expression::Expression, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		super(token_range);
		self.set_dimension(dimension);
		self.set_annotations(annotations);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn set_dimension(&mut self, dimension: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		if dimension == self.dimension {
			return self;
		}
		self.notify_property_change(ObservableProperty::DIMENSION, self.dimension, dimension);
		if self.dimension != null {
			self.dimension.set_parent_node(null);
		}
	
		self.dimension = dimension;
		self.set_as_parent_node_of(dimension);
		return self;
	}

	pub fn get_dimension(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.dimension);
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		com::github::javaparser::utils::utils::Utils::assert_not_null(annotations)?;
		if annotations == self.annotations {
			return self;
		}
		self.notify_property_change(ObservableProperty::ANNOTATIONS, self.annotations, annotations);
		if self.annotations != null {
			self.annotations.set_parent_node(null);
		}
	
		self.annotations = annotations;
		self.set_as_parent_node_of(annotations);
		return self;
	}

	pub fn remove_dimension(&self) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		return self.set_dimension(null as Expression);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.dimension != null {
			if node == self.dimension {
				self.remove_dimension();
				return true;
			}
		}
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::array_creation_level::ArrayCreationLevel {
		return self.accept(CloneVisitor::new(), null) as ArrayCreationLevel;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::array_creation_level_meta_model::ArrayCreationLevelMetaModel {
		return JavaParserMetaModel::arrayCreationLevelMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.annotations.size() {
				{
					if self.annotations.get(i) == node {
						self.annotations.set(i, replacement_node as AnnotationExpr)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.dimension != null {
			if node == self.dimension {
				self.set_dimension(replacement_node as Expression);
				return true;
			}
		}
		return super.replace(node, replacement_node);
	}
}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ArrayCreationLevel {}

impl /* Java */ java::lang::Cloneable /**/ for ArrayCreationLevel {}

impl com::github::javaparser::has_parent_node::HasParentNode for ArrayCreationLevel {}

impl com::github::javaparser::ast::observer::observable::Observable for ArrayCreationLevel {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ArrayCreationLevel {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ArrayCreationLevel {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ArrayCreationLevel {}