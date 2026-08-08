use crate::com::github::javaparser::utils::Utils::assertNotNull;
use java::util::stream::Collectors::joining;
use java::util::stream::Collectors::toList;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::ast;
use crate::com::github::javaparser::ast::expr::AnnotationExpr;
use crate::com::github::javaparser::ast::expr::SimpleName;
use crate::com::github::javaparser::ast::nodeTypes;
use crate::com::github::javaparser::ast::nodeTypes::modifiers;
use crate::com::github::javaparser::ast::observer::ObservableProperty;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::ast::type::ReferenceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::ast::type::TypeParameter;
use crate::com::github::javaparser::ast::visitor::CloneVisitor;
use crate::com::github::javaparser::metamodel::CallableDeclarationMetaModel;
use crate::com::github::javaparser::metamodel::JavaParserMetaModel;
use crate::com::github::javaparser::metamodel::OptionalProperty;
use java::util::List;
use java::util::Optional;
use java::util::function::Consumer;

pub struct CallableDeclaration<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> {
	modifiers: com::github::javaparser::ast::node_list::NodeList,
	type_parameters: com::github::javaparser::ast::node_list::NodeList,
	name: com::github::javaparser::ast::expr::simple_name::SimpleName,
	parameters: com::github::javaparser::ast::node_list::NodeList,
	thrown_exceptions: com::github::javaparser::ast::node_list::NodeList,
	receiver_parameter: com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter,
}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> CallableDeclaration {
	fn new(modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		this(null, modifiers, annotations, type_parameters, name, parameters, thrown_exceptions, receiver_parameter);
	}

	pub fn new(token_range: &com::github::javaparser::token_range::TokenRange, modifiers: &com::github::javaparser::ast::node_list::NodeList, annotations: &com::github::javaparser::ast::node_list::NodeList, type_parameters: &com::github::javaparser::ast::node_list::NodeList, name: &com::github::javaparser::ast::expr::simple_name::SimpleName, parameters: &com::github::javaparser::ast::node_list::NodeList, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		super(token_range, annotations);
		self.set_modifiers(modifiers);
		self.set_type_parameters(type_parameters);
		self.set_name(name);
		self.set_parameters(parameters);
		self.set_thrown_exceptions(thrown_exceptions);
		self.set_receiver_parameter(receiver_parameter);
		self.custom_initialization();
	}

	pub fn get_modifiers(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.modifiers;
	}

	pub fn set_modifiers(&mut self, modifiers: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(modifiers)?;
		if modifiers == self.modifiers {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::MODIFIERS, self.modifiers, modifiers);
		if self.modifiers != null {
			self.modifiers.set_parent_node(null);
		}
	
		self.modifiers = modifiers;
		self.set_as_parent_node_of(modifiers);
		return self as T;
	}

	pub fn get_name(&self) -> com::github::javaparser::ast::expr::simple_name::SimpleName {
		return self.name;
	}

	pub fn set_name(&mut self, name: &com::github::javaparser::ast::expr::simple_name::SimpleName) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(name)?;
		if name == self.name {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::NAME, self.name, name);
		if self.name != null {
			self.name.set_parent_node(null);
		}
	
		self.name = name;
		self.set_as_parent_node_of(name);
		return self as T;
	}

	pub fn get_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.parameters;
	}

	pub fn set_parameters(&mut self, parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(parameters)?;
		if parameters == self.parameters {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::PARAMETERS, self.parameters, parameters);
		if self.parameters != null {
			self.parameters.set_parent_node(null);
		}
	
		self.parameters = parameters;
		self.set_as_parent_node_of(parameters);
		return self as T;
	}

	pub fn get_thrown_exceptions(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.thrown_exceptions;
	}

	pub fn set_thrown_exceptions(&mut self, thrown_exceptions: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(thrown_exceptions)?;
		if thrown_exceptions == self.thrownExceptions {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::THROWN_EXCEPTIONS, self.thrownExceptions, thrown_exceptions);
		if self.thrownExceptions != null {
			self.thrownExceptions.set_parent_node(null);
		}
	
		self.thrownExceptions = thrown_exceptions;
		self.set_as_parent_node_of(thrown_exceptions);
		return self as T;
	}

	pub fn get_type_parameters(&self) -> com::github::javaparser::ast::node_list::NodeList {
		return self.type_parameters;
	}

	pub fn set_type_parameters(&mut self, type_parameters: &com::github::javaparser::ast::node_list::NodeList) /* thrown(java.lang.AssertionError) */ -> T {
		com::github::javaparser::utils::utils::Utils::assert_not_null(type_parameters)?;
		if type_parameters == self.typeParameters {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::TYPE_PARAMETERS, self.typeParameters, type_parameters);
		if self.typeParameters != null {
			self.typeParameters.set_parent_node(null);
		}
	
		self.typeParameters = type_parameters;
		self.set_as_parent_node_of(type_parameters);
		return self as T;
	}

	fn append_throws_if_requested(&self, including_throws: bool) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if including_throws {
			let first_throw: bool = true;
			for thr in self.get_thrown_exceptions() {
				if first_throw {
					first_throw = false;
					sb.append(" throws ");
				} else {
					sb.append(", ");
				}
				sb.append(&thr.to_string());
			}
		}
		return sb.toString();
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
			while i < self.thrown_exceptions.size() {
				{
					if self.thrown_exceptions.get(i) == node {
						self.thrown_exceptions.remove(i);
						return true;
					}
				}
				i += 1;
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

	pub fn get_signature(&self) -> com::github::javaparser::ast::body::callable_declaration::Signature {
		return Signature::new(&self.get_name().get_identifier(), &self.get_parameters().stream().map(self::getTypeWithVarargsAsArray).map(self::stripGenerics).map(self::stripAnnotations).collect(&/* Java */ java::util::stream::Collectors /**/::toList()));
	}

	fn strip_annotations(&self, type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::ast::type::type::Type {
		if type instanceof NodeWithAnnotations {
			(type as NodeWithAnnotations).set_annotations(NodeList<>::new());
		}
		return type;
	}

	fn strip_generics(&self, type: &com::github::javaparser::ast::type::type::Type) -> com::github::javaparser::ast::type::type::Type {
		if type instanceof NodeWithTypeArguments {
			(type as NodeWithTypeArguments).set_type_arguments(null as NodeList<Type>);
		}
		return type;
	}

	fn get_type_with_varargs_as_array(&self, p: &com::github::javaparser::ast::body::parameter::Parameter) -> com::github::javaparser::ast::type::type::Type {
		/*  A signature includes the varargs ellipsis.
	        This is a field on parameter which we lose when we only get the type,
	        so we represent it as an additional [] on the type. */ 
		let t: Type = p.get_type().clone();
		if p.is_var_args() {
			t = ArrayType::new(t);
		}
		return t;
	}

	pub fn clone(&self) -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		return self.accept(CloneVisitor::new(), null) as CallableDeclaration<?>;
	}

	pub fn get_meta_model(&self) -> com::github::javaparser::metamodel::callable_declaration_meta_model::CallableDeclarationMetaModel {
		return JavaParserMetaModel::callableDeclarationMetaModel;
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
			while i < self.thrown_exceptions.size() {
				{
					if self.thrown_exceptions.get(i) == node {
						self.thrown_exceptions.set(i, replacement_node as ReferenceType)?;
						return true;
					}
				}
				i += 1;
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
	
		return super.replace(node, replacement_node)?;
	}

	pub fn is_callable_declaration(&self) -> bool {
		return true;
	}

	pub fn as_callable_declaration(&self) -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		return self;
	}

	pub fn if_callable_declaration(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		action.accept(self);
	}

	pub fn get_receiver_parameter(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::ofNullable(self.receiver_parameter);
	}

	pub fn set_receiver_parameter(&mut self, receiver_parameter: &com::github::javaparser::ast::body::receiver_parameter::ReceiverParameter) -> T {
		if receiver_parameter == self.receiverParameter {
			return self as T;
		}
		self.notify_property_change(ObservableProperty::RECEIVER_PARAMETER, self.receiverParameter, receiver_parameter);
		if self.receiverParameter != null {
			self.receiverParameter.set_parent_node(null);
		}
	
		self.receiverParameter = receiver_parameter;
		self.set_as_parent_node_of(receiver_parameter);
		return self as T;
	}

	pub fn remove_receiver_parameter(&self) -> com::github::javaparser::ast::body::callable_declaration::CallableDeclaration {
		return self.set_receiver_parameter(null as ReceiverParameter);
	}

	pub fn to_callable_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self);
	}

	pub fn is_variable_arity_method(&self) -> bool {
		return self.get_parameters().size() > 0 && self.get_parameters().get_last().get().is_var_args();
	}

	pub fn is_fixed_arity_method(&self) -> bool {
		return !self.is_variable_arity_method();
	}
}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_access_modifiers::NodeWithAccessModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_public_modifier::NodeWithPublicModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_private_modifier::NodeWithPrivateModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_protected_modifier::NodeWithProtectedModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_declaration::NodeWithDeclaration for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_simple_name::NodeWithSimpleName for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_parameters::NodeWithParameters for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_thrown_exceptions::NodeWithThrownExceptions for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_type_parameters::NodeWithTypeParameters for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_javadoc::NodeWithJavadoc for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_abstract_modifier::NodeWithAbstractModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_static_modifier::NodeWithStaticModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_final_modifier::NodeWithFinalModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::modifiers::node_with_strictfp_modifier::NodeWithStrictfpModifier for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_modifiers::NodeWithModifiers for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> /* Java */ java::lang::Cloneable /**/ for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::has_parent_node::HasParentNode for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::observer::observable::Observable for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::visitor::visitable::Visitable for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_range::NodeWithRange for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_token_range::NodeWithTokenRange for CallableDeclaration<T> {}

impl<T: com::github::javaparser::ast::body::callable_declaration::CallableDeclaration> com::github::javaparser::ast::node_types::node_with_annotations::NodeWithAnnotations for CallableDeclaration<T> {}

pub struct Signature {
	name: /* Java */ java::lang::String /**/,
	parameter_types: /* Java */ java::util::List /**/,
}

impl Signature {
	fn new(name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::util::List /**/) -> com::github::javaparser::ast::body::callable_declaration::Signature {
		self.name = name;
		self.parameterTypes = parameter_types;
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.name;
	}

	pub fn get_parameter_types(&self) -> /* Java */ java::util::List /**/ {
		return self.parameter_types;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let signature: Signature = o as Signature;
		if !self.name.equals(signature.name) {
			return false;
		}
	
		if !self.parameter_types.equals(signature.parameterTypes) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.name.hashCode();
		result = 31 * result + self.parameter_types.hashCode();
		return result;
	}

	pub fn as_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.parameter_types.stream().map(Type::asString).collect(&/* Java */ java::util::stream::Collectors /**/::joining(", ", self.name + "(", ")"));
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.as_string();
	}
}