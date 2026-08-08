use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::joining;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast::AllFieldsConstructor;
use crate::com::github::javaparser::ast::Generated;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithAnnotations;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithSimpleName;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::TypeParameterMetaModel;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedTypeVariable;
use java::util::Optional;
use java::util::function::Consumer;

pub struct TypeParameter {
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	type_bound: com::github::javaparser::ast::node_list::NodeList,
}

impl TypeParameter {
	pub fn new() -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		this(null, SimpleName::new(), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		this(null, SimpleName::new(name), NodeList<>::new(), NodeList<>::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/, type_bound: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		this(null, SimpleName::new(name), type_bound, NodeList<>::new());
	}

	pub fn new(name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_bound: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		this(null, name, type_bound, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_bound: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		super(token_range, annotations);
		self.set_name(name);
		self.set_type_bound(type_bound);
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

	pub fn get_type_bound(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.type_bound;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
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

	pub fn set_type_bound(&mut self, type_bound: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type_bound)?;
		if type_bound == self.typeBound {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_BOUND, self.typeBound, type_bound);
		if self.typeBound != null {
			self.typeBound.set_parent_node(null);
		}
	
		self.typeBound = type_bound;
		self.set_as_parent_node_of(type_bound);
		return self;
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		super.set_annotations(annotations)?;
		return self;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.type_bound.size() {
				{
					if self.type_bound.get(i) == node {
						self.type_bound.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		let str: StringBuilder = StringBuilder::new(&self.get_name_as_string());
		self.get_type_bound().if_non_empty(|l|str.append(&l.stream().map(ClassOrInterfaceType::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining("&", " extends ", ""))));
		return str.toString();
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("L%s;", &self.resolve().qualified_name());
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		return self.accept(CloneVisitor::new(), null) as TypeParameter;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::type_parameter_meta_model::TypeParameterMetaModel {
		return JavaParserMetaModel::typeParameterMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		 {
			let i: i32 = 0;
			while i < self.type_bound.size() {
				{
					if self.type_bound.get(i) == node {
						self.type_bound.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_type_parameter(&self) -> bool {
		return true;
	}

	pub fn as_type_parameter(&self) -> com::github::javaparser::ast::type::type_parameter::TypeParameter {
		return self;
	}

	pub fn if_type_parameter(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedTypeVariable.class);
	}

	pub fn to_type_parameter(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return Err(UnsupportedOperationException::new(&self.getClass().getCanonicalName()));
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for TypeParameter {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for TypeParameter {}

impl /* Java */ java::lang::Cloneable /**/ for TypeParameter {}

impl com::github::javaparser::has_parent_node::HasParentNode for TypeParameter {}

impl com::github::javaparser::ast::observer::observable::Observable for TypeParameter {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for TypeParameter {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for TypeParameter {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for TypeParameter {}

impl com::github::javaparser::resolution::resolvable::Resolvable for TypeParameter {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for TypeParameter {}