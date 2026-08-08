use crate::com::github::javaparser::ast::NodeList::nodeList;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithVariables;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::VariableDeclarationExprMetaModel;
use java::util::Arrays;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::stream::Collectors;

pub struct VariableDeclarationExpr {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	annotations: com::github::javaparser::ast::node_list::NodeList,
	variables: com::github::javaparser::ast::node_list::NodeList,
}

impl VariableDeclarationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, NodeList<>::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, variable_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, NodeList<>::new(), NodeList<>::new(), &com::github::javaparser::ast::node_list::NodeList::node_list(VariableDeclarator::new(type, variable_name)));
	}

	pub fn new(var: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, NodeList<>::new(), NodeList<>::new(), &com::github::javaparser::ast::node_list::NodeList::node_list(var));
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, variable_name: &/* Java */ java::lang::String /**/, modifiers: &com::github::javaparser::ast::modifier::Modifier) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, &Arrays::stream(modifiers).collect(&Collectors::toCollection(|()|NodeList<>::new())), NodeList<>::new(), &com::github::javaparser::ast::node_list::NodeList::node_list(VariableDeclarator::new(type, variable_name)));
	}

	pub fn new(var: &com::github::javaparser::ast::body::variable_declarator::VariableDeclarator, modifiers: &com::github::javaparser::ast::modifier::Modifier) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, &Arrays::stream(modifiers).collect(&Collectors::toCollection(|()|NodeList<>::new())), NodeList<>::new(), &com::github::javaparser::ast::node_list::NodeList::node_list(var));
	}

	pub fn new(variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, NodeList<>::new(), NodeList<>::new(), variables);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, modifiers, NodeList<>::new(), variables);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		this(null, modifiers, annotations, variables);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, variables: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		super(token_range);
		self.set_modifiers(modifiers);
		self.set_annotations(annotations);
		self.set_variables(variables);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_annotations(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.annotations;
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn get_variables(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.variables;
	}

	pub fn set_annotations(&mut self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
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

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
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

	pub fn set_variables(&mut self, variables: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(variables)?;
		if variables == self.variables {
			return self;
		}
		self.notify_property_change(ObservableProperty::VARIABLES, self.variables, variables);
		if self.variables != null {
			self.variables.set_parent_node(null);
		}
	
		self.variables = variables;
		self.set_as_parent_node_of(variables);
		return self;
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
	
		 {
			let i: i32 = 0;
			while i < self.variables.size() {
				{
					if self.variables.get(i) == node {
						self.variables.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		return self.accept(CloneVisitor::new(), null) as VariableDeclarationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::variable_declaration_expr_meta_model::VariableDeclarationExprMetaModel {
		return JavaParserMetaModel::variableDeclarationExprMetaModel;
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
	
		 {
			let i: i32 = 0;
			while i < self.variables.size() {
				{
					if self.variables.get(i) == node {
						self.variables.set(i, replacement_node as VariableDeclarator)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_variable_declaration_expr(&self) -> bool {
		return true;
	}

	pub fn as_variable_declaration_expr(&self) -> com::github::javaparser::ast::expr::variable_declaration_expr::VariableDeclarationExpr {
		return self;
	}

	pub fn if_variable_declaration_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_variable_declaration_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for VariableDeclarationExpr {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for VariableDeclarationExpr {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for VariableDeclarationExpr {}

impl com::github::javaparser::ast::node_types::node_with_variables::NodeWithVariables for VariableDeclarationExpr {}

impl /* Java */ java::lang::Cloneable /**/ for VariableDeclarationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for VariableDeclarationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for VariableDeclarationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for VariableDeclarationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for VariableDeclarationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for VariableDeclarationExpr {}