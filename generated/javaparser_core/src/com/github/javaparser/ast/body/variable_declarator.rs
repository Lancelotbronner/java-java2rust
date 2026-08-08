use crate::com::github::javaparser::utils::Utils::assertNonEmpty;
use crate::com::github::javaparser::utils::Utils::assertNotNull;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::expr::Expression;
use crate::com::github::javaparser::ast::expr::NameExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithType;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithVariables;
use crate::com::github::javaparser::ast::observer::AstObserverAdapter;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::NonEmptyProperty;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::VariableDeclaratorMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedValueDeclaration;
use java::util::LinkedList;
use java::util::List;
use java::util::Optional;

pub struct VariableDeclarator {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	initializer: com::github::javaparser::ast::expr::expression::Expression,
	type: com::github::javaparser::ast::type::type::Type,
}

impl VariableDeclarator {
	pub fn new() -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		this(null, ClassOrInterfaceType::new(), SimpleName::new(), null);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, variable_name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		this(null, type, SimpleName::new(variable_name), null);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		this(null, type, name, null);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, variable_name: &/* Java */ java::lang::String /**/, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		this(null, type, SimpleName::new(variable_name), initializer);
	}

	pub fn new(type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		this(null, type, name, initializer);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, type: &com::github::javaparser::ast::type::type::Type, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		super(token_range);
		self.set_type(type);
		self.set_name(name);
		self.set_initializer(initializer);
		self.custom_initialization();
	}

	fn custom_initialization(&self) {
		// We register an observer on the type property. When it is changed the MaximumCommonType is changes as well,
		// because it is derived from the type of the variables it contains, for this reason we notify about the change
		self.register(AstObserverAdapter::new() {
			pub fn property_change(&self, observed_node: &Node, property: &ObservableProperty, old_value: &Object, new_value: &Object) {
				if property == ObservableProperty::TYPE {
					let vd: VariableDeclarator = VariableDeclarator;
					if vd.get_parent_node().isPresent() && vd.get_parent_node().get() instanceof NodeWithVariables {
						let node_with_variables: NodeWithVariables<?> = vd.get_parent_node().get() as NodeWithVariables<?>;
						// We calculate the value the property will assume after the change will be completed
						let current_max_common_type: Optional<Type> = node_with_variables.get_maximum_common_type();
						let types: List<Type> = LinkedList<>::new();
						let index: i32 = node_with_variables.get_variables().index_of(vd);
						 {
							let i: i32 = 0;
							while i < node_with_variables.get_variables().size() {
								{
									if i == index {
										types.add(new_value as Type);
									} else {
										types.add(&node_with_variables.get_variable(i).get_type());
									}
								}
								i += 1;
							 }
						 }
	
						let new_max_common_type: Optional<Type> = NodeWithVariables::calculate_maximum_common_type(types);
						(node_with_variables as Node).notify_property_change(ObservableProperty::MAXIMUM_COMMON_TYPE, &current_max_common_type.orElse(null), &new_max_common_type.orElse(null));
					}
				}
			}
	
		});
	}

	pub fn property_change(&self, observed_node: &com::github::javaparser::ast::node::Node, property: &com::github::javaparser::ast::observer::observable_property::ObservableProperty, old_value: &/* Java */ java::lang::Object /**/, new_value: &/* Java */ java::lang::Object /**/) {
		if property == ObservableProperty::TYPE {
			let vd: VariableDeclarator = VariableDeclarator;
			if vd.get_parent_node().isPresent() && vd.get_parent_node().get() instanceof NodeWithVariables {
				let node_with_variables: NodeWithVariables<?> = vd.get_parent_node().get() as NodeWithVariables<?>;
				// We calculate the value the property will assume after the change will be completed
				let current_max_common_type: Optional<Type> = node_with_variables.get_maximum_common_type();
				let types: List<Type> = LinkedList<>::new();
				let index: i32 = node_with_variables.get_variables().index_of(vd);
				 {
					let i: i32 = 0;
					while i < node_with_variables.get_variables().size() {
						{
							if i == index {
								types.add(new_value as Type);
							} else {
								types.add(&node_with_variables.get_variable(i).get_type());
							}
						}
						i += 1;
					 }
				 }
	
				let new_max_common_type: Optional<Type> = NodeWithVariables::calculate_maximum_common_type(types);
				(node_with_variables as Node).notify_property_change(ObservableProperty::MAXIMUM_COMMON_TYPE, &current_max_common_type.orElse(null), &new_max_common_type.orElse(null));
			}
		}
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

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
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

	pub fn set_initializer(&mut self, initializer: &com::github::javaparser::ast::expr::expression::Expression) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
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

	pub fn set_initializer(&self, init: &/* Java */ java::lang::String /**/) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		return self.set_initializer(NameExpr::new(&com::github::javaparser::utils::utils::Utils::assert_non_empty(init)?));
	}

	pub fn get_type(&self) -> com::github::javaparser::ast::type::type::Type {
		return self.type;
	}

	pub fn set_type(&mut self, type: &com::github::javaparser::ast::type::type::Type) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type)?;
		if type == self.type {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE, self.type, type);
		if self.type != null {
			self.type.set_parent_node(null);
		}
	
		self.type = type;
		self.set_as_parent_node_of(type);
		return self;
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
		return super.remove(node);
	}

	pub fn remove_initializer(&self) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		return self.set_initializer(null as Expression);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::variable_declarator::VariableDeclarator {
		return self.accept(CloneVisitor::new(), null) as VariableDeclarator;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::variable_declarator_meta_model::VariableDeclaratorMetaModel {
		return JavaParserMetaModel::variableDeclaratorMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError) */ -> bool {
		if node == null {
			return false;
		}
		if self.initializer != null {
			if node == self.initializer {
				self.set_initializer(replacement_node as Expression);
				return true;
			}
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if node == self.type {
			self.set_type(replacement_node as Type)?;
			return true;
		}
		return super.replace(node, replacement_node);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_value_declaration::ResolvedValueDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedValueDeclaration.class);
	}
}

impl com::github::javaparser::ast::node_types::node_with_type::NodeWithType for VariableDeclarator {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for VariableDeclarator {}

impl com::github::javaparser::resolution::resolvable::Resolvable for VariableDeclarator {}

impl /* Java */ java::lang::Cloneable /**/ for VariableDeclarator {}

impl com::github::javaparser::has_parent_node::HasParentNode for VariableDeclarator {}

impl com::github::javaparser::ast::observer::observable::Observable for VariableDeclarator {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for VariableDeclarator {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for VariableDeclarator {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for VariableDeclarator {}