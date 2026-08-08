use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NormalAnnotationExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct NormalAnnotationExpr {
	pairs: com::github::javaparser::ast::node_list::NodeList,
}

impl NormalAnnotationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		this(null, Name::new(), NodeList<>::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, pairs: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		this(null, name, pairs);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, pairs: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		super(token_range, name);
		self.set_pairs(pairs);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_pairs(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.pairs;
	}

	pub fn set_pairs(&mut self, pairs: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(pairs)?;
		if pairs == self.pairs {
			return self;
		}
		self.notify_property_change(ObservableProperty::PAIRS, self.pairs, pairs);
		if self.pairs != null {
			self.pairs.set_parent_node(null);
		}
	
		self.pairs = pairs;
		self.set_as_parent_node_of(pairs);
		return self;
	}

	pub fn add_pair(&self, key: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		return self.add_pair(key, NameExpr::new(value));
	}

	pub fn add_pair(&self, key: &/* Java */ java::lang::String /**/, value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		let member_value_pair: MemberValuePair = MemberValuePair::new(key, value);
		self.get_pairs().add(member_value_pair);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.pairs.size() {
				{
					if self.pairs.get(i) == node {
						self.pairs.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		return self.accept(CloneVisitor::new(), null) as NormalAnnotationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::normal_annotation_expr_meta_model::NormalAnnotationExprMetaModel {
		return JavaParserMetaModel::normalAnnotationExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.pairs.size() {
				{
					if self.pairs.get(i) == node {
						self.pairs.set(i, replacement_node as MemberValuePair)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_normal_annotation_expr(&self) -> bool {
		return true;
	}

	pub fn as_normal_annotation_expr(&self) -> com::github::javaparser::ast::expr::normal_annotation_expr::NormalAnnotationExpr {
		return self;
	}

	pub fn if_normal_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_normal_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for NormalAnnotationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for NormalAnnotationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for NormalAnnotationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for NormalAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for NormalAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for NormalAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for NormalAnnotationExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for NormalAnnotationExpr {}