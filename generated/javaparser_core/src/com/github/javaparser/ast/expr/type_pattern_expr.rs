use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TypePatternExprMetaModel;
use java::util::Optional;
use java::util::function::Consumer;

pub struct TypePatternExpr {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
}

impl TypePatternExpr {
	pub fn new() -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		this(null, NodeList<>::new(), ClassOrInterfaceType::new(), SimpleName::new());
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		this(null, modifiers, type, name);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		super(token_range, type);
		self.set_modifiers(modifiers);
		self.set_name(name);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
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

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.set(i, replacement_node as Modifier)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		return super.replace(node, replacement_node)?;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		return self.accept(CloneVisitor::new(), null) as TypePatternExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::type_pattern_expr_meta_model::TypePatternExprMetaModel {
		return JavaParserMetaModel::typePatternExprMetaModel;
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.modifiers.size() {
				{
					if self.modifiers.get(i) == node {
						self.modifiers.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn is_type_pattern_expr(&self) -> bool {
		return true;
	}

	pub fn as_type_pattern_expr(&self) -> com::github::javaparser::ast::expr::type_pattern_expr::TypePatternExpr {
		return self;
	}

	pub fn to_type_pattern_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_type_pattern_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for TypePatternExpr {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for TypePatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for TypePatternExpr {}

impl /* Java */ java::lang::Cloneable /**/ for TypePatternExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for TypePatternExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for TypePatternExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for TypePatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TypePatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TypePatternExpr {}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for TypePatternExpr {}