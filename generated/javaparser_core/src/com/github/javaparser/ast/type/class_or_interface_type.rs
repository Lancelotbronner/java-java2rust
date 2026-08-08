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
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::ClassOrInterfaceTypeMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::resolution::Context;
use crate::com::github::javaparser::resolution::UnsolvedSymbolException;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeDeclaration;
use crate::com::github::javaparser::resolution::model::SymbolReference;
use crate::com::github::javaparser::resolution::model::typesystem::ReferenceTypeImpl;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedTypeVariable;
use java::util::Collections;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;
use java::util::stream::Collectors;

pub struct ClassOrInterfaceType {
	scope: com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	type_arguments: com::github::javaparser::ast::node_list::NodeList,
}

impl ClassOrInterfaceType {
	pub fn new() -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		this(null, null, SimpleName::new(), null, NodeList<>::new());
	}

	pub fn new(name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		this(null, null, SimpleName::new(name), null, NodeList<>::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		this(null, scope, SimpleName::new(name), null, NodeList<>::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		this(null, scope, name, type_arguments, NodeList<>::new());
	}

	pub fn new(scope: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_arguments: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		this(null, scope, name, type_arguments, annotations);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, scope: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, type_arguments: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		super(token_range, annotations);
		self.set_scope(scope);
		self.set_name(name);
		self.set_type_arguments(type_arguments);
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

	pub fn get_name_with_scope(&self) -> /* Java */ java::lang::String /**/ {
		let str: StringBuilder = StringBuilder::new();
		self.get_scope().ifPresent(|s|str.append(&s.get_name_with_scope()).append("."));
		str.append(&self.name.as_string());
		return str.toString();
	}

	pub fn get_scope(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.scope);
	}

	pub fn is_boxed_type(&self) -> bool {
		return PrimitiveType::unboxMap.containsKey(&self.name.get_identifier());
	}

	pub fn to_unboxed_type(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::ast::type::primitive_type::PrimitiveType {
		if !self.is_boxed_type() {
			return Err(UnsupportedOperationException::new(self.name + " isn't a boxed type."));
		}
		return PrimitiveType::new(&PrimitiveType::unboxMap.get(&self.name.get_identifier()));
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
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

	pub fn set_scope(&mut self, scope: &com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		if scope == self.scope {
			return self;
		}
		self.notify_property_change(ObservableProperty::SCOPE, self.scope, scope);
		if self.scope != null {
			self.scope.set_parent_node(null);
		}
	
		self.scope = scope;
		self.set_as_parent_node_of(scope);
		return self;
	}

	pub fn get_type_arguments(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.type_arguments);
	}

	pub fn set_type_arguments(&mut self, type_arguments: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		if type_arguments == self.typeArguments {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_ARGUMENTS, self.typeArguments, type_arguments);
		if self.typeArguments != null {
			self.typeArguments.set_parent_node(null);
		}
	
		self.typeArguments = type_arguments;
		self.set_as_parent_node_of(type_arguments);
		return self;
	}

	pub fn set_annotations(&self, annotations: &com::github::javaparser::ast::node_list::NodeList) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return super.set_annotations(annotations) as ClassOrInterfaceType;
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		if self.scope != null {
			if node == self.scope {
				self.remove_scope();
				return true;
			}
		}
		if self.type_arguments != null {
			 {
				let i: i32 = 0;
				while i < self.type_arguments.size() {
					{
						if self.type_arguments.get(i) == node {
							self.type_arguments.remove(i);
							return true;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return super.remove(node);
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		let str: StringBuilder = StringBuilder::new();
		self.get_scope().ifPresent(|s|str.append(&s.as_string()).append("."));
		str.append(&self.name.as_string());
		self.get_type_arguments().ifPresent(|ta|str.append(&ta.stream().map(Type::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining(",", "<", ">"))));
		return str.toString();
	}

	pub fn to_descriptor(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		return String::format("L%s;", &self.resolve().erasure().as_reference_type()?.get_qualified_name().replace(".", "/"));
	}

	pub fn remove_scope(&self) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self.set_scope(null as ClassOrInterfaceType);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self.accept(CloneVisitor::new(), null) as ClassOrInterfaceType;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::class_or_interface_type_meta_model::ClassOrInterfaceTypeMetaModel {
		return JavaParserMetaModel::classOrInterfaceTypeMetaModel;
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.AssertionError | java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		if node == self.name {
			self.set_name(replacement_node as SimpleName)?;
			return true;
		}
		if self.scope != null {
			if node == self.scope {
				self.set_scope(replacement_node as ClassOrInterfaceType);
				return true;
			}
		}
		if self.type_arguments != null {
			 {
				let i: i32 = 0;
				while i < self.type_arguments.size() {
					{
						if self.type_arguments.get(i) == node {
							self.type_arguments.set(i, replacement_node as Type)?;
							return true;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return super.replace(node, replacement_node);
	}

	pub fn is_class_or_interface_type(&self) -> bool {
		return true;
	}

	pub fn as_class_or_interface_type(&self) -> com::github::javaparser::ast::type::class_or_interface_type::ClassOrInterfaceType {
		return self;
	}

	pub fn if_class_or_interface_type(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.get_symbol_resolver().to_resolved_type(self, ResolvedType.class);
	}

	pub fn to_class_or_interface_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn convert_to_usage(&self, context: &com::github::javaparser::resolution::context::Context) /* thrown(com.github.javaparser.resolution.UnsolvedSymbolException | java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let name: String = self.get_name_with_scope();
		let ref: SymbolReference<ResolvedTypeDeclaration> = context.solve_type(name);
		if !ref.is_solved() {
			return Err(UnsolvedSymbolException::new(name));
		}
		let type_declaration: ResolvedTypeDeclaration = ref.get_corresponding_declaration();
		let type_parameters: List<ResolvedType> = Collections::emptyList();
		if self.get_type_arguments().isPresent() {
			type_parameters = self.get_type_arguments().get().stream().map(|(pt)|pt.convert_to_usage(context)).collect(&Collectors::toList());
		}
		if type_declaration.is_type_parameter() {
			return ResolvedTypeVariable::new(&type_declaration.as_type_parameter()?);
		}
		return ReferenceTypeImpl::new(type_declaration as ResolvedReferenceTypeDeclaration, type_parameters);
	}
}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for ClassOrInterfaceType {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for ClassOrInterfaceType {}

impl com::github::javaparser::ast::node_types::node_with_type_arguments::NodeWithTypeArguments for ClassOrInterfaceType {}

impl /* Java */ java::lang::Cloneable /**/ for ClassOrInterfaceType {}

impl com::github::javaparser::has_parent_node::HasParentNode for ClassOrInterfaceType {}

impl com::github::javaparser::ast::observer::observable::Observable for ClassOrInterfaceType {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for ClassOrInterfaceType {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for ClassOrInterfaceType {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for ClassOrInterfaceType {}

impl com::github::javaparser::resolution::resolvable::Resolvable for ClassOrInterfaceType {}

impl com::github::javaparser::ast::type::convertible_to_usage::ConvertibleToUsage for ClassOrInterfaceType {}