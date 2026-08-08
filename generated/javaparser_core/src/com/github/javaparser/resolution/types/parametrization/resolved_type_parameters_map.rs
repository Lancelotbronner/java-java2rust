use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedTypeVariable;
use java::util;

pub struct ResolvedTypeParametersMap {
	name_to_value: /* Java */ java::util::Map /**/,
	name_to_declaration: /* Java */ java::util::Map /**/,
}

impl ResolvedTypeParametersMap {
	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if !(o instanceof ResolvedTypeParametersMap) {
			return false;
		}
	
		let that: ResolvedTypeParametersMap = o as ResolvedTypeParametersMap;
		return self.name_to_value.equals(that.nameToValue) && self.name_to_declaration.equals(that.nameToDeclaration);
	}

	pub fn hash_code(&self) -> i32 {
		return self.name_to_value.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "TypeParametersMap{" + "nameToValue=" + self.name_to_value + '}';
	}

	pub fn empty(&self) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap {
		return Builder::new().build();
	}

	fn new(name_to_value: &/* Java */ java::util::Map /**/, name_to_declaration: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap {
		self.nameToValue = HashMap<>::new();
		self.nameToValue.putAll(name_to_value);
		self.nameToDeclaration = HashMap<>::new();
		self.nameToDeclaration.putAll(name_to_declaration);
	}

	pub fn get_value(&self, type_parameter: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let qualified_name: String = type_parameter.get_qualified_name();
		if self.name_to_value.containsKey(qualified_name) {
			return self.name_to_value.get(qualified_name);
		}
		return ResolvedTypeVariable::new(type_parameter);
	}

	pub fn get_value_by_signature(&self, signature: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Optional /**/ {
		if self.name_to_value.containsKey(signature) {
			return Optional::of(&self.name_to_value.get(signature));
		}
		return Optional::empty();
	}

	pub fn get_names(&self) -> /* Java */ java::util::List /**/ {
		return ArrayList<>::new(&self.name_to_value.keySet());
	}

	pub fn get_types(&self) -> /* Java */ java::util::List /**/ {
		return ArrayList<>::new(&self.name_to_value.values());
	}

	pub fn to_builder(&self) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::Builder {
		return Builder::new(self.name_to_value, self.name_to_declaration);
	}

	pub fn is_empty(&self) -> bool {
		return self.name_to_value.isEmpty();
	}

	pub fn replace_all(&self, mut type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let inferred_types: Map<ResolvedTypeParameterDeclaration, ResolvedType> = HashMap<>::new();
		for type_parameter_declaration in self.nameToDeclaration.values() {
			type = type.replace_type_variables(type_parameter_declaration, &self.get_value(type_parameter_declaration), inferred_types);
		}
		return type;
	}
}

pub struct Builder {
	name_to_value: /* Java */ java::util::Map /**/,
	name_to_declaration: /* Java */ java::util::Map /**/,
}

impl Builder {
	pub fn new() -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::Builder {
		self.name_to_value = HashMap<>::new();
		self.name_to_declaration = HashMap<>::new();
	}

	fn new(name_to_value: &/* Java */ java::util::Map /**/, name_to_declaration: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::Builder {
		self.nameToValue = HashMap<>::new();
		self.nameToValue.putAll(name_to_value);
		self.nameToDeclaration = HashMap<>::new();
		self.nameToDeclaration.putAll(name_to_declaration);
	}

	pub fn build(&self) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap {
		return ResolvedTypeParametersMap::new(self.name_to_value, self.name_to_declaration);
	}

	pub fn set_value(&self, type_parameter: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, value: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::Builder {
		// TODO: we shouldn't just silently overwrite existing types!
		let qualified_name: String = type_parameter.get_qualified_name();
		self.name_to_value.put(qualified_name, value);
		self.name_to_declaration.put(qualified_name, type_parameter);
		return self;
	}
}