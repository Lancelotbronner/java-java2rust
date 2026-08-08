use crate::com::github::javaparser::resolution::MethodUsage;
use crate::com::github::javaparser::resolution::declarations::ResolvedFieldDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::logic::FunctionalInterfaceLogic;
use crate::com::github::javaparser::resolution::model::LambdaArgumentTypePlaceholder;
use crate::com::github::javaparser::resolution::types::ResolvedReferenceType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedTypeTransformer;
use crate::com::github::javaparser::resolution::types::ResolvedTypeVariable;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParametersMap;
use java::util;
use java::util::function::Function;
use java::util::stream::Collectors;
use java::util::stream::Stream;

pub struct ReferenceTypeImpl;

impl ReferenceTypeImpl {
	static ASSIGNABLE_REFERENCE_TYPE: &[/* Java */ java::lang::String /**/] = vec!["java.lang.Object", "java.lang.Cloneable", "java.io.Serializable", ]
	;

	pub fn undetermined_parameters(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return ReferenceTypeImpl::new(type_declaration, &type_declaration.get_type_parameters().stream().map(ResolvedTypeVariable::new).collect(&Collectors::toList()));
	}

	fn create(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration, type_parameters_corrected: &/* Java */ java::util::List /**/) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return ReferenceTypeImpl::new(type_declaration, type_parameters_corrected);
	}

	fn create(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return ReferenceTypeImpl::new(type_declaration);
	}

	pub fn new(type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> com::github::javaparser::resolution::model::typesystem::reference_type_impl::ReferenceTypeImpl {
		super(type_declaration);
	}

	pub fn new(type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration, type_arguments: &/* Java */ java::util::List /**/) -> com::github::javaparser::resolution::model::typesystem::reference_type_impl::ReferenceTypeImpl {
		super(type_declaration, type_arguments);
	}

	pub fn as_type_parameter(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration {
		return self.typeDeclaration.as_type_parameter()?;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> bool {
		if other instanceof NullType {
			return !self.is_primitive();
		}
		// everything is assignable to Object except void
		if !other.is_void() && self.is_java_lang_object() {
			return true;
		}
		// consider boxing
		if other.is_primitive() {
			if self.is_java_lang_object() {
				return true;
			}
			// Check if 'other' can be boxed to match this type
			if self.is_corresponding_boxing_type(&other.describe()) {
				return true;
			}
	
			// All numeric types extend Number
			return other.is_numeric_type() && self.is_reference_type() && self.as_reference_type().get_qualified_name().equals(&Number.class.getCanonicalName());
		}
		if other instanceof LambdaArgumentTypePlaceholder {
			return FunctionalInterfaceLogic::is_functional_interface_type(self);
		}
		if other.is_reference_type() {
			let other_ref: ResolvedReferenceType = other.as_reference_type()?;
			if self.compare_considering_type_parameters(other_ref)? {
				return true;
			}
			for other_ancestor in other_ref.get_all_ancestors() {
				if self.compare_considering_type_parameters(other_ancestor)? {
					return true;
				}
			}
			return false;
		}
		if other.is_type_variable() {
			for bound in other.as_type_variable()?.as_type_parameter().get_bounds() {
				if bound.is_extends() {
					if self.is_assignable_by(&bound.get_type())? {
						return true;
					}
				}
			}
			return false;
		}
		if other.is_constraint() {
			return self.is_assignable_by(&other.as_constraint_type()?.get_bound())?;
		}
		if other.is_wildcard() {
			if self.is_java_lang_object() {
				return true;
			}
			if other.as_wildcard()?.is_extends() {
				return self.is_assignable_by(&other.as_wildcard()?.get_bounded_type()?)?;
			}
			return false;
		}
		if other.is_union_type() {
			let common: Optional<ResolvedReferenceType> = other.as_union_type()?.get_common_ancestor();
			return common.map(|ancestor|self.is_assignable_by(ancestor)?).orElse(false);
		}
		// or to a variable of type Object, Cloneable or java.io.Serializable.
		if other.is_array() {
			return self.is_assignable_by_reference_type(&self.get_qualified_name());
		}
		return false;
	}

	fn is_assignable_by_reference_type(&self, qname: &/* Java */ java::lang::String /**/) -> bool {
		return Stream::of(self.ASSIGNABLE_REFERENCE_TYPE).anyMatch(|ref|ref.equals(qname));
	}

	pub fn get_declared_methods(&self) -> /* Java */ java::util::Set /**/ {
		// TODO replace variables
		let methods: Set<MethodUsage> = HashSet<>::new();
		self.get_type_declaration().ifPresent(|reference_type_declaration|{
			for method_declaration in reference_type_declaration.get_declared_methods() {
				let method_usage: MethodUsage = MethodUsage::new(method_declaration);
				methods.add(method_usage);
			}
		});
		return methods;
	}

	pub fn to_raw_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if self.is_raw_type() {
			return self;
		}
		return ReferenceTypeImpl::new(, &Collections::emptyList());
	}

	pub fn mention(&self, type_parameters: &/* Java */ java::util::List /**/) -> bool {
		return self.type_parameters_values().stream().anyMatch(|tp|tp.mention(type_parameters)?);
	}

	pub fn transform_type_parameters(&self, transformer: &com::github::javaparser::resolution::types::resolved_type_transformer::ResolvedTypeTransformer) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let result: ResolvedType = self;
		let i: i32 = 0;
		for tp in self.type_parameters_values() {
			let transformed_tp: ResolvedType = transformer.transform(tp);
			// Identity comparison on purpose
			if transformed_tp != tp {
				let type_parameters_corrected: List<ResolvedType> = result.as_reference_type()?.type_parameters_values();
				type_parameters_corrected.set(i, transformed_tp);
				result = self.create(, type_parameters_corrected);
			}
			i += 1;
		}
		return result;
	}

	pub fn get_all_ancestors(&self) -> /* Java */ java::util::List /**/ {
		return self.get_all_ancestors(ResolvedReferenceTypeDeclaration.depthFirstFunc);
	}

	pub fn get_all_ancestors(&self, traverser: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::List /**/ {
		// We need to go through the inheritance line and propagate the type parameters
		let ancestors: List<ResolvedReferenceType> = .get_all_ancestors(traverser);
		ancestors = ancestors.stream().map(|a|self.type_parameters_map().replace_all(a).as_reference_type()?).collect(&Collectors::toList());
		return ancestors;
	}

	pub fn get_direct_ancestors(&self) -> /* Java */ java::util::List /**/ {
		// We need to go through the inheritance line and propagate the type parameters
		let ancestors: List<ResolvedReferenceType> = .get_ancestors();
		ancestors = ancestors.stream().map(|a|self.type_parameters_map().replace_all(a).as_reference_type()?).collect(&Collectors::toList());
		// Conditionally re-insert java.lang.Object as an ancestor.
		if self.get_type_declaration().isPresent() {
			let this_type_declaration: ResolvedReferenceTypeDeclaration = self.get_type_declaration().get();
			// The superclass of interfaces is always null
			if this_type_declaration.is_class() {
				let optional_super_class: Optional<ResolvedReferenceType> = this_type_declaration.as_class()?.get_super_class();
				let super_class_is_java_lang_object: bool = optional_super_class.isPresent() && optional_super_class.get().is_java_lang_object();
				let this_is_java_lang_object: bool = this_type_declaration.as_class()?.is_java_lang_object();
				if super_class_is_java_lang_object && !this_is_java_lang_object {
					ancestors.add(&optional_super_class.get());
				}
			}
		}
		return ancestors;
	}

	pub fn derive_type_parameters(&self, type_parameters_map: &com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return self.create(, type_parameters_map);
	}

	pub fn get_declared_fields(&self) -> /* Java */ java::util::Set /**/ {
		let all_fields: Set<ResolvedFieldDeclaration> = LinkedHashSet<>::new();
		if self.get_type_declaration().isPresent() {
			all_fields.addAll(&self.get_type_declaration().get().get_declared_fields());
		}
		return all_fields;
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ReferenceTypeImpl {}

impl com::github::javaparser::resolution::types::parametrization::resolved_type_parametrized::ResolvedTypeParametrized for ReferenceTypeImpl {}

impl com::github::javaparser::resolution::types::parametrization::resolved_type_parameter_value_provider::ResolvedTypeParameterValueProvider for ReferenceTypeImpl {}