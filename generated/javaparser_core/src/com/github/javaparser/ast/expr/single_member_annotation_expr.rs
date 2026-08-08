use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::SingleMemberAnnotationExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct SingleMemberAnnotationExpr {
	member_value: com::github::javaparser::ast::expr::expression::Expression,
}

impl SingleMemberAnnotationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		this(null, Name::new(), StringLiteralExpr::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::name::Name, member_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		this(null, name, member_value);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::name::Name, member_value: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		super(token_range, name);
		self.set_member_value(member_value);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_member_value(&self) -> com::github::javaparser::ast::expr::expression::Expression {
		return self.member_value;
	}

	pub fn set_member_value(&mut self, member_value: &com::github::javaparser::ast::expr::expression::Expression) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(member_value)?;
		if member_value == self.memberValue {
			return self;
		}
		self.notify_property_change(ObservableProperty::MEMBER_VALUE, self.memberValue, member_value);
		if self.memberValue != null {
			self.memberValue.set_parent_node(null);
		}
	
		self.memberValue = member_value;
		self.set_as_parent_node_of(member_value);
		return self;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		return self.accept(CloneVisitor::new(), null) as SingleMemberAnnotationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::single_member_annotation_expr_meta_model::SingleMemberAnnotationExprMetaModel {
		return JavaParserMetaModel::singleMemberAnnotationExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.member_value {
			self.set_member_value(replacement_node as Expression)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn is_single_member_annotation_expr(&self) -> bool {
		return true;
	}

	pub fn as_single_member_annotation_expr(&self) -> com::github::javaparser::ast::expr::single_member_annotation_expr::SingleMemberAnnotationExpr {
		return self;
	}

	pub fn if_single_member_annotation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_single_member_annotation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for SingleMemberAnnotationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for SingleMemberAnnotationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for SingleMemberAnnotationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for SingleMemberAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for SingleMemberAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for SingleMemberAnnotationExpr {}

impl com::github::javaparser::ast::node_types::node_with_name::NodeWithName for SingleMemberAnnotationExpr {}

impl com::github::javaparser::resolution::resolvable::Resolvable for SingleMemberAnnotationExpr {}