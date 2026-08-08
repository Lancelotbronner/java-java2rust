use crate::com::github::javaparser::StaticJavaParser::parseType;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ArrayCreationExprMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::Optional;
use java::util::function::Consumer;

pub struct ArrayCreationExpr {
	levels: com::github::javaparser::ast::node_list::NodeList,
	element_type: com::github::javaparser::ast::type::type::Type,
	initializer: com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr,
}

impl ArrayCreationExpr {
	pub fn new() -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		this(null, ClassOrInterfaceType::new(), NodeList<>::new(ArrayCreationLevel::new()), ArrayInitializerExpr::new());
	}

	pub fn new(element_type: &com::github::javaparser::ast::type::type::Type, levels: &com::github::javaparser::ast::node_list::NodeList, initializer: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		this(null, element_type, levels, initializer);
	}

	pub fn new(element_type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		this(null, element_type, NodeList<>::new(ArrayCreationLevel::new()), ArrayInitializerExpr::new());
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, element_type: &com::github::javaparser::ast::type::type::Type, levels: &com::github::javaparser::ast::node_list::NodeList, initializer: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		super(token_range);
		self.set_element_type(element_type);
		self.set_levels(levels);
		self.set_initializer(initializer);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_initializer(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.initializer);
	}

	pub fn get_element_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.element_type;
	}

	pub fn set_initializer(&mut self, initializer: &com::github::javaparser::ast::expr::array_initializer_expr::ArrayInitializerExpr) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		if initializer == self.initializer {
			return self;
		}
		self.notify_property_change(ObservableProperty::INITIALIZER, self.initializer, initializer);
		if self.initializer != null {
			self.initializer.set_parent_node(null);
		}
	
		self.initializer = initializer;
		self.set_as_parent_node_of(initializer);
		return self;
	}

	pub fn set_element_type(&mut self, element_type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(element_type)?;
		if element_type == self.elementType {
			return self;
		}
		self.notify_property_change(ObservableProperty::ELEMENT_TYPE, self.elementType, element_type);
		if self.elementType != null {
			self.elementType.set_parent_node(null);
		}
	
		self.elementType = element_type;
		self.set_as_parent_node_of(element_type);
		return self;
	}

	pub fn get_levels(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.levels;
	}

	pub fn set_levels(&mut self, levels: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		com::github::javaparser::utils::utils::Utils::assert_not_null(levels)?;
		if levels == self.levels {
			return self;
		}
		self.notify_property_change(ObservableProperty::LEVELS, self.levels, levels);
		if self.levels != null {
			self.levels.set_parent_node(null);
		}
	
		self.levels = levels;
		self.set_as_parent_node_of(levels);
		return self;
	}

	pub fn created_type(&self) -> com::github::javaparser::ast::type::type::Type {
		let result: Type = self.element_type;
		 {
			let i: i32 = 0;
			while i < self.levels.size() {
				{
					result = ArrayType::new(result, ArrayType::com::github::javaparser::ast::type::array_type::Origin::TYPE, NodeList<>::new());
				}
				i += 1;
			 }
		 }
	
		return result;
	}

	pub fn set_element_type(&self, type_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		self.try_add_import_to_parent_compilation_unit(type_class);
		return self.set_element_type(&com::github::javaparser::static_java_parser::StaticJavaParser::parse_type(&type_class.getSimpleName()))?;
	}

	pub fn set_element_type(&self, type: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		return self.set_element_type(&com::github::javaparser::static_java_parser::StaticJavaParser::parse_type(type))?;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.initializer != null {
			if node == self.initializer {
				self.remove_initializer();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.levels.size() {
				{
					if self.levels.get(i) == node {
						self.levels.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn remove_initializer(&self) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		return self.set_initializer(null as ArrayInitializerExpr);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		return self.accept(CloneVisitor::new(), null) as ArrayCreationExpr;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::array_creation_expr_meta_model::ArrayCreationExprMetaModel {
		return JavaParserMetaModel::arrayCreationExprMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.element_type {
			self.set_element_type(replacement_node as Type)?;
			return true;
		}
		if self.initializer != null {
			if node == self.initializer {
				self.set_initializer(replacement_node as ArrayInitializerExpr);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.levels.size() {
				{
					if self.levels.get(i) == node {
						self.levels.set(i, replacement_node as ArrayCreationLevel)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn is_array_creation_expr(&self) -> bool {
		return true;
	}

	pub fn as_array_creation_expr(&self) -> com::github::javaparser::ast::expr::array_creation_expr::ArrayCreationExpr {
		return self;
	}

	pub fn if_array_creation_expr(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn to_array_creation_expr(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}
}

impl /* Java */ java::lang::Cloneable /**/ for ArrayCreationExpr {}

impl com::github::javaparser::has_parent_node::HasParentNode for ArrayCreationExpr {}

impl com::github::javaparser::ast::observer::observable::Observable for ArrayCreationExpr {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ArrayCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ArrayCreationExpr {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ArrayCreationExpr {}