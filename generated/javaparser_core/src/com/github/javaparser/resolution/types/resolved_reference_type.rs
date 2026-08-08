use crate::com::github::javaparser::ast::AccessSpecifier;
use crate::com::github::javaparser::resolution::MethodUsage;
use crate::com::github::javaparser::resolution::declarations::ResolvedFieldDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration::Bound;
use crate::com::github::javaparser::resolution::model::typesystem::LazyType;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParameterValueProvider;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParametersMap;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParametrized;
use crate::com::github::javaparser::utils::Pair;
use java::util;
use java::util::function::Function;
use java::util::stream::Collectors;

pub struct ResolvedReferenceType {
	type_declaration: com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration,
	type_parameters_map: com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap,
}

impl ResolvedReferenceType {
	static JAVA_LANG_ENUM: /* Java */ java::lang::String /**/ = java.lang.Enum.class.getCanonicalName();

	static JAVA_LANG_OBJECT: /* Java */ java::lang::String /**/ = java.lang.Object.class.getCanonicalName();

	static JAVA_LANG_RECORD: /* Java */ java::lang::String /**/ = "java.lang.Record";

	pub fn new(type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) /* thrown(java.lang.RuntimeException | java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		this(type_declaration, &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType::derive_params(type_declaration)?);
	}

	pub fn new(type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration, type_arguments: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		if type_declaration == null {
			return Err(IllegalArgumentException::new("TypeDeclaration is not expected to be null"));
		}
		if type_declaration.is_type_parameter() {
			return Err(IllegalArgumentException::new("You should use only Classes, Interfaces and enums"));
		}
		if type_arguments.size() > 0 && type_arguments.size() != type_declaration.get_type_parameters().size() {
			return Err(IllegalArgumentException::new(&String::format("expected either zero type arguments or has many as defined in the declaration (%d). Found %d", &type_declaration.get_type_parameters().size(), &type_arguments.size())));
		}
		let type_parameters_map_builder: ResolvedTypeParametersMap.Builder = ResolvedTypeParametersMap.Builder::new();
		 {
			let i: i32 = 0;
			while i < type_arguments.size() {
				{
					type_parameters_map_builder.set_value(&type_declaration.get_type_parameters().get(i), &type_arguments.get(i));
				}
				i += 1;
			 }
		 }
	
		self.typeParametersMap = type_parameters_map_builder.build();
		self.typeDeclaration = type_declaration;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if self == o {
			return true;
		}
	
		if o == null {
			return false;
		}
	
		if o instanceof LazyType {
			/* final */ let lazy_type: LazyType = o as LazyType;
			if !lazy_type.is_reference_type() {
				return false;
			}
	
			return self.equals(&lazy_type.as_reference_type()?)?;
		}
		if self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ResolvedReferenceType = o as ResolvedReferenceType;
		if !self.type_declaration.equals(that.typeDeclaration) {
			return false;
		}
	
		if !self.type_parameters_map.equals(that.typeParametersMap) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.type_declaration.hashCode();
		result = 31 * result + self.type_parameters_map.hash_code();
		return result;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "ReferenceType{" + self.get_qualified_name() + ", typeParametersMap=" + self.type_parameters_map + '}';
	}

	pub fn is_reference_type(&self) -> bool {
		return true;
	}

	pub fn as_reference_type(&self) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return self;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		if self.has_name() {
			sb.append(&self.type_declaration.get_qualified_name());
		} else {
			sb.append("<anonymous class>");
		}
		if !self.type_parameters_map().is_empty() {
			sb.append("<");
			sb.append(&String::join(", ", &self.type_declaration.get_type_parameters().stream().map(|tp|self.type_parameters_map().get_value(tp).describe()).collect(&Collectors::toList())));
			sb.append(">");
		}
		return sb.toString();
	}

	pub fn transform_type_parameters(&self, transformer: &com::github::javaparser::resolution::types::resolved_type_transformer::ResolvedTypeTransformer) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType ;

	pub fn replace_type_variables(&self, tp_to_replace: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if replaced == null {
			return Err(IllegalArgumentException::new());
		}
		let result: ResolvedReferenceType = self;
		let i: i32 = 0;
		for tp in self.type_parameters_values() {
			let transformed_tp: ResolvedType = tp.replace_type_variables(tp_to_replace, replaced, inferred_types);
			// Identity comparison on purpose
			if tp.is_type_variable() && tp.as_type_variable()?.describe().equals(&tp_to_replace.get_name()) {
				inferred_types.put(&tp.as_type_parameter()?, replaced);
			}
			// FIXME
			if true {
				let type_parameters_corrected: List<ResolvedType> = result.as_reference_type().type_parameters_values();
				type_parameters_corrected.set(i, transformed_tp);
				result = self.create(self.type_declaration, type_parameters_corrected);
			}
			i += 1;
		}
		let values: List<ResolvedType> = result.type_parameters_values();
		// FIXME
		if values.contains(tp_to_replace) {
			let index: i32 = values.indexOf(tp_to_replace);
			values.set(index, replaced);
			if result.get_type_declaration().isPresent() {
				return self.create(&result.get_type_declaration().get(), values);
			}
		}
		return result;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool ;

	pub fn get_all_ancestors(&self) -> /* Java */ java::util::List /**/ ;

	pub fn get_all_ancestors(&self, traverser: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::List /**/ ;

	pub fn get_direct_ancestors(&self) -> /* Java */ java::util::List /**/ ;

	pub fn get_all_interfaces_ancestors(&self) -> /* Java */ java::util::List /**/ {
		return self.get_all_ancestors().stream().filter(|it|it.get_type_declaration().isPresent()).filter(|it|it.get_type_declaration().get().is_interface()).collect(&Collectors::toList());
	}

	pub fn get_all_classes_ancestors(&self) -> /* Java */ java::util::List /**/ {
		return self.get_all_ancestors().stream().filter(|it|it.get_type_declaration().isPresent()).filter(|it|it.get_type_declaration().get().is_class()).collect(&Collectors::toList());
	}

	pub fn get_generic_parameter_by_name(&self, name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		for tp in self.type_declaration.get_type_parameters() {
			if tp.get_name().equals(name) {
				return Optional::of(&self.type_parameters_map().get_value(tp));
			}
		}
		return Optional::empty();
	}

	pub fn type_parameters_values(&self) -> /* Java */ java::util::List /**/ {
		return  if self.typeParametersMap.is_empty() { Collections::emptyList() } else { self.type_declaration.get_type_parameters().stream().map(|tp|self.type_parameters_map.get_value(tp)).collect(&Collectors::toList()) };
	}

	pub fn get_type_parameters_map(&self) -> /* Java */ java::util::List /**/ {
		let type_parameters_map: List<Pair<ResolvedTypeParameterDeclaration, ResolvedType>> = ArrayList<>::new();
		if !self.is_raw_type() {
			 {
				let i: i32 = 0;
				while i < self.type_declaration.get_type_parameters().size() {
					{
						type_parameters_map.add(Pair<>::new(&self.type_declaration.get_type_parameters().get(i), &self.type_parameters_values().get(i)));
					}
					i += 1;
				 }
			 }
	
		}
		return type_parameters_map;
	}

	pub fn type_parameters_map(&self) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap {
		return self.type_parameters_map;
	}

	pub fn get_type_declaration(&self) -> /* Java */ java::util::Optional /**/ {
		return Optional::of(self.type_declaration);
	}

	pub fn get_field_type(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ -> /* Java */ java::util::Optional /**/ {
		if !self.type_declaration.has_field(name) {
			return Optional::empty();
		}
		let type: ResolvedType = self.type_declaration.get_field(name)?.get_type();
		type = self.use_this_type_parameters_on_the_given_type(type)?;
		return Optional::of(type);
	}

	pub fn has_name(&self) -> bool {
		return self.type_declaration.has_name();
	}

	pub fn get_qualified_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.type_declaration.get_qualified_name();
	}

	pub fn get_id(&self) -> /* Java */ java::lang::String /**/ {
		return self.type_declaration.get_id();
	}

	pub fn get_declared_methods(&self) -> /* Java */ java::util::Set /**/ ;

	pub fn get_declared_fields(&self) -> /* Java */ java::util::Set /**/ ;

	pub fn is_raw_type(&self) -> bool {
		if !self.type_declaration.get_type_parameters().isEmpty() {
			if self.type_parameters_map().is_empty() {
				return true;
			}
		}
		return false;
	}

	pub fn type_param_value(&self, type_parameter_declaration: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Optional /**/ {
		if type_parameter_declaration.declared_on_method() {
			return Err(IllegalArgumentException::new());
		}
		if !self.get_type_declaration().isPresent() {
			// TODO: Consider IllegalStateException or similar
			return Optional::empty();
		}
		let type_id: String = self.get_type_declaration().get().get_id();
		if type_id.equals(&type_parameter_declaration.get_container_id()) {
			return Optional::of(&self.type_parameters_map().get_value(type_parameter_declaration));
		}
		for ancestor in self.get_all_ancestors() {
			if ancestor.get_id().equals(&type_parameter_declaration.get_container_id()) {
				return Optional::of(&ancestor.type_parameters_map().get_value(type_parameter_declaration));
			}
		}
		return Optional::empty();
	}

	pub fn to_raw_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType ;

	pub fn get_all_methods(&self) -> /* Java */ java::util::List /**/ {
		if !self.get_type_declaration().isPresent() {
			// empty list -- consider IllegalStateException or similar
			return ArrayList<>::new();
		}
		// Get the methods declared directly on this.
		let all_methods: List<ResolvedMethodDeclaration> = LinkedList<>::new(&self.get_type_declaration().get().get_declared_methods());
		// Also get methods inherited from ancestors.
		self.get_direct_ancestors().forEach(|a|all_methods.addAll(&a.get_all_methods()));
		return all_methods;
	}

	pub fn get_all_fields_visible_to_inheritors(&self) -> /* Java */ java::util::List /**/ {
		let res: List<ResolvedFieldDeclaration> = LinkedList<>::new(&self.get_declared_fields().stream().filter(|f|f.access_specifier() != AccessSpecifier::PRIVATE).collect(&Collectors::toList()));
		self.get_direct_ancestors().forEach(|a|res.addAll(&a.get_all_fields_visible_to_inheritors()));
		return res;
	}

	pub fn get_all_methods_visible_to_inheritors(&self) -> /* Java */ java::util::List /**/ {
		return self.get_all_methods().stream().filter(|m|m.access_specifier() != AccessSpecifier::PRIVATE).collect(&Collectors::toList());
	}

	fn create(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration, type_parameters: &/* Java */ java::util::List /**/) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType ;

	fn create(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration, type_parameters_map: &com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType {
		return self.create(type_declaration, &type_declaration.get_type_parameters().stream().map(typeParametersMap::getValue).collect(&Collectors::toList()));
	}

	fn create(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType ;

	fn is_corresponding_boxing_type(&self, type_name: &/* Java */ java::lang::String /**/) -> bool {
		let resolved_primitive_type: ResolvedPrimitiveType = ResolvedPrimitiveType::by_name(type_name)? as ResolvedPrimitiveType;
		return self.get_qualified_name().equals(&resolved_primitive_type.get_box_typeq_name());
	}

	fn compare_considering_type_parameters(&self, other: &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType) /* thrown(java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ -> bool {
		if other.equals(self)? {
			return true;
		}
		if self.get_qualified_name().equals(&other.get_qualified_name()) {
			if self.is_raw_type() || other.is_raw_type() {
				return true;
			}
			let type_parameters_values: List<ResolvedType> = self.type_parameters_values();
			if type_parameters_values.size() != other.type_parameters_values().size() {
				return Err(IllegalStateException::new());
			}
			 {
				let i: i32 = 0;
				while i < type_parameters_values.size() {
					{
						let this_param: ResolvedType = type_parameters_values.get(i);
						let other_param: ResolvedType = other.type_parameters_values().get(i);
						if !this_param.equals(other_param) {
							if this_param instanceof ResolvedWildcard {
								let this_param_as_wildcard: ResolvedWildcard = this_param as ResolvedWildcard;
								if this_param_as_wildcard.is_super() && other_param.is_assignable_by(&this_param_as_wildcard.get_bounded_type()?) {
								// ok
								} else if this_param_as_wildcard.is_extends() && this_param_as_wildcard.get_bounded_type()?.is_assignable_by(other_param) {
								// ok
								} else if !this_param_as_wildcard.is_bounded() {
								// ok
								} else {
									return false;
								}
							} else {
								if this_param instanceof ResolvedTypeVariable && other_param instanceof ResolvedTypeVariable {
									// Here we want to compare something like @{code C extends Comparable<C>} with @{code K
									// extends Comparable<K>}
									// we have to compare the type of the erased bound (in this example the type @{code
									// Comparable}).
									let this_bounds: List<ResolvedType> = this_param.as_type_variable()?.as_type_parameter().get_bounds().stream().map(ResolvedTypeParameterDeclaration.Bound::getType).map(|type|type.erasure()).collect(&Collectors::toList());
									let other_bounds: List<ResolvedType> = other_param.as_type_variable()?.as_type_parameter().get_bounds().stream().map(ResolvedTypeParameterDeclaration.Bound::getType).map(|type|type.erasure()).collect(&Collectors::toList());
									return this_bounds.size() == other_bounds.size() && other_bounds.containsAll(this_bounds);
								}
								if !(this_param instanceof ResolvedTypeVariable) && other_param instanceof ResolvedTypeVariable {
									return self.compare_considering_variable_type_parameters(this_param, other_param as ResolvedTypeVariable);
								}
								if this_param instanceof ResolvedTypeVariable && !(other_param instanceof ResolvedTypeVariable) {
									return self.compare_considering_variable_type_parameters(other_param, this_param as ResolvedTypeVariable);
								}
								return false;
							}
						}
					}
					i += 1;
				 }
			 }
	
			return true;
		}
		return false;
	}

	fn compare_considering_variable_type_parameters(&self, reference_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, type_variable: &com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable) /* thrown(java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ -> bool {
		// verify if the ResolvedTypeVariable has only one type variable and the bound is
		// not a reference type with a bound parameter
		// for example EnumSet<E> noneOf(Class<E> elementType)
		let bounds: List<Bound> = type_variable.as_type_variable().as_type_parameter().get_bounds();
		if bounds.size() == 1 {
			let bound_type: ResolvedType = bounds.get(0).get_type();
			let has_type_parameter: bool = bound_type.is_reference_type() && !bound_type.as_reference_type()?.typeParametersMap.is_empty();
			return  if has_type_parameter { self.compare_considering_type_parameters(&bound_type.as_reference_type()?)? } else { bound_type.is_assignable_by(reference_type) };
		}
		return false;
	}

	fn derive_params(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) /* thrown(java.lang.IllegalArgumentException | java.lang.RuntimeException) */ -> /* Java */ java::util::List /**/ {
		if type_declaration == null {
			return Err(IllegalArgumentException::new("TypeDeclaration is not expected to be null"));
		}
		let type_parameters: List<ResolvedTypeParameterDeclaration> = type_declaration.get_type_parameters();
		if type_parameters == null {
			return Err(RuntimeException::new("Type parameters are not expected to be null"));
		}
		return type_parameters.stream().map(ResolvedTypeVariable::new).collect(&Collectors::toList());
	}

	pub fn derive_type_parameters(&self, type_parameters_map: &com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap) -> com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType ;

	pub fn is_java_lang_object(&self) -> bool {
		return // Consider anonymous classes
		self.is_reference_type() && self.has_name() && self.get_qualified_name().equals(self.JAVA_LANG_OBJECT);
	}

	pub fn is_java_lang_enum(&self) -> bool {
		return // Consider anonymous classes
		self.is_reference_type() && self.has_name() && self.get_qualified_name().equals(self.JAVA_LANG_ENUM);
	}

	pub fn is_java_lang_record(&self) -> bool {
		return // Consider anonymous classes
		self.is_reference_type() && self.has_name() && self.get_qualified_name().equals(self.JAVA_LANG_RECORD);
	}

	pub fn is_unboxable(&self) -> bool {
		return Arrays::stream(&ResolvedPrimitiveType::values()).anyMatch(|pt|self.get_qualified_name().equals(&pt.get_box_typeq_name()));
	}

	pub fn is_unboxable_to(&self, primitive_type: &com::github::javaparser::resolution::types::resolved_primitive_type::ResolvedPrimitiveType) -> bool {
		return primitive_type.get_box_typeq_name().equals(&self.as_reference_type().describe());
	}

	pub fn to_unboxed_type(&self) -> /* Java */ java::util::Optional /**/ {
		return Arrays::stream(&ResolvedPrimitiveType::values()).filter(|pt|self.as_reference_type().get_qualified_name().equals(&pt.get_box_typeq_name())).findFirst();
	}

	pub fn erasure(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if !self.type_declaration.is_generic() {
			return self;
		}
	
		return self.create(self.type_declaration, &self.erasure_of_paramaters(self.type_parameters_map));
	}

	fn erasure_of_paramaters(&self, type_parameters_map: &com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap) -> /* Java */ java::util::List /**/ {
		return ArrayList<ResolvedType>::new();
	}

	fn is_java_object(&self, rt: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return rt != null && rt.is_reference_type() && rt.as_reference_type()?.is_java_lang_object();
	}

	pub fn to_descriptor(&self) -> /* Java */ java::lang::String /**/ {
		return String::format("L%s;", &self.get_qualified_name().replace(".", "/"));
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedReferenceType {}

impl com::github::javaparser::resolution::types::parametrization::resolved_type_parametrized::ResolvedTypeParametrized for ResolvedReferenceType {}

impl com::github::javaparser::resolution::types::parametrization::resolved_type_parameter_value_provider::ResolvedTypeParameterValueProvider for ResolvedReferenceType {}