use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::Collections::unmodifiableList;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithImplements;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithParameters;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeParameters;
use crate::com::github::javaparser::ast::nodeTypes::modifiers::NodeWithFinalModifier;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::stmt::LocalRecordDeclarationStmt;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use crate::com::github::javaparser::metamodel::RecordDeclarationMetaModel;
use crate::com::github::javaparser::resolution::Resolvable;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct RecordDeclaration {
	type_parameters: com::github::javaparser::ast::node_list::NodeList,
	implemented_types: com::github::javaparser::ast::node_list::NodeList,
	receiver_parameter: com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter,
	parameters: com::github::javaparser::ast::node_list::NodeList,
}

impl RecordDeclaration {
	pub fn new() -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		this(null, NodeList<>::new(), NodeList<>::new(), SimpleName::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, name: &/* Java */ java::lang::String /**/) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		this(null, modifiers, NodeList<>::new(), SimpleName::new(name), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), NodeList<>::new(), null);
	}

	pub fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		this(null, modifiers, annotations, name, parameters, type_parameters, implemented_types, members, receiver_parameter);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, implemented_types: &com::github::javaparser::ast::node_list::NodeList, members: &com::github::javaparser::ast::node_list::NodeList, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		super(token_range, modifiers, annotations, name, members);
		self.set_parameters(parameters);
		self.set_type_parameters(type_parameters);
		self.set_implemented_types(implemented_types);
		self.set_receiver_parameter(receiver_parameter);
		self.custom_initialization();
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) -> R {
		return v.visit(self, arg);
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) {
		v.visit(self, arg);
	}

	pub fn get_implemented_types(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.implemented_types;
	}

	pub fn get_type_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.type_parameters;
	}

	pub fn set_implemented_types(&mut self, implemented_types: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(implemented_types)?;
		if implemented_types == self.implementedTypes {
			return self;
		}
		self.notify_property_change(ObservableProperty::IMPLEMENTED_TYPES, self.implementedTypes, implemented_types);
		if self.implementedTypes != null {
			self.implementedTypes.set_parent_node(null);
		}
	
		self.implementedTypes = implemented_types;
		self.set_as_parent_node_of(implemented_types);
		return self;
	}

	pub fn set_type_parameters(&mut self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type_parameters)?;
		if type_parameters == self.typeParameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::TYPE_PARAMETERS, self.typeParameters, type_parameters);
		if self.typeParameters != null {
			self.typeParameters.set_parent_node(null);
		}
	
		self.typeParameters = type_parameters;
		self.set_as_parent_node_of(type_parameters);
		return self;
	}

	pub fn is_local_record_declaration(&self) -> bool {
		return self.get_parent_node().map(|p|p instanceof LocalRecordDeclarationStmt).orElse(false);
	}

	pub fn get_fully_qualified_name(&self) -> /* Java */ java::util::Optional /**/ {
		if self.is_local_record_declaration() {
			return Optional::empty();
		}
		return super.get_fully_qualified_name();
	}

	pub fn resolve(&self) -> com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration {
		return self.get_symbol_resolver().resolve_declaration(self, ResolvedReferenceTypeDeclaration.class);
	}

	pub fn is_record_declaration(&self) -> bool {
		return true;
	}

	pub fn as_record_declaration(&self) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		return self;
	}

	pub fn to_record_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn if_record_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn remove(&self, node: &com::github::javaparser::ast::node::Node) -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.parameters.size() {
				{
					if self.parameters.get(i) == node {
						self.parameters.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.receiver_parameter != null {
			if node == self.receiver_parameter {
				self.remove_receiver_parameter();
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.remove(i);
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.remove(node);
	}

	pub fn replace(&self, node: &com::github::javaparser::ast::node::Node, replacement_node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.IllegalArgumentException) */ -> bool {
		if node == null {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < self.implemented_types.size() {
				{
					if self.implemented_types.get(i) == node {
						self.implemented_types.set(i, replacement_node as ClassOrInterfaceType)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.parameters.size() {
				{
					if self.parameters.get(i) == node {
						self.parameters.set(i, replacement_node as Parameter)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		if self.receiver_parameter != null {
			if node == self.receiver_parameter {
				self.set_receiver_parameter(replacement_node as ReceiverParameter);
				return true;
			}
		}
		 {
			let i: i32 = 0;
			while i < self.type_parameters.size() {
				{
					if self.type_parameters.get(i) == node {
						self.type_parameters.set(i, replacement_node as TypeParameter)?;
						return true;
					}
				}
				i += 1;
			 }
		 }
	
		return super.replace(node, replacement_node);
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		return self.accept(CloneVisitor::new(), null) as RecordDeclaration;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::record_declaration_meta_model::RecordDeclarationMetaModel {
		return JavaParserMetaModel::recordDeclarationMetaModel;
	}

	pub fn get_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.parameters;
	}

	pub fn set_parameters(&mut self, parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parameters)?;
		if parameters == self.parameters {
			return self;
		}
		self.notify_property_change(ObservableProperty::PARAMETERS, self.parameters, parameters);
		if self.parameters != null {
			self.parameters.set_parent_node(null);
		}
	
		self.parameters = parameters;
		self.set_as_parent_node_of(parameters);
		return self;
	}

	pub fn get_receiver_parameter(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.receiver_parameter);
	}

	pub fn set_receiver_parameter(&mut self, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		if receiver_parameter == self.receiverParameter {
			return self;
		}
		self.notify_property_change(ObservableProperty::RECEIVER_PARAMETER, self.receiverParameter, receiver_parameter);
		if self.receiverParameter != null {
			self.receiverParameter.set_parent_node(null);
		}
	
		self.receiverParameter = receiver_parameter;
		self.set_as_parent_node_of(receiver_parameter);
		return self;
	}

	pub fn remove_receiver_parameter(&self) -> com::github::javaparser::ast::body::record_declaration::RecordDeclaration {
		return self.set_receiver_parameter(null as ReceiverParameter);
	}

	pub fn is_final(&self) -> bool {
		return true;
	}

	pub fn is_static(&self) -> bool {
		if self.get_parent_node().isPresent() {
			let parent_node: Node = self.get_parent_node().get();
			if !(parent_node instanceof CompilationUnit) {
				return true;
			}
		}
		// Otherwise use the default method.
		return super.is_static();
	}

	pub fn get_compact_constructors(&self) -> /* Java */ java::util::List /**/ {
		return /* Java */ java::util::Collections /**/::unmodifiableList(&self.get_members().stream().filter(|m|m instanceof CompactConstructorDeclaration).map(|m|m as CompactConstructorDeclaration).collect(&/* Java */ java::util::stream::Collectors /**/::toList()));
	}
}

impl com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_implements::NodeWithImplements for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for RecordDeclaration {}

impl com::github::javaparser::resolution::resolvable::Resolvable for RecordDeclaration {}

impl /* Java */ java::lang::Cloneable /**/ for RecordDeclaration {}

impl com::github::javaparser::has_parent_node::HasParentNode for RecordDeclaration {}

impl com::github::javaparser::ast::observer::observable::Observable for RecordDeclaration {}

impl com::github::javaparser::ast::visitor::visitable::Visitable for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_members::NodeWithMembers for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for RecordDeclaration {}

impl com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for RecordDeclaration {}