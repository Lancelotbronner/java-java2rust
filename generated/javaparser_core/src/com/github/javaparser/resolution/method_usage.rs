use crate::com::github::javaparser::resolution::declarations::ResolvedMethodDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParametersMap;
use crate::com::github::javaparser::resolution::types::parametrization::ResolvedTypeParametrized;
use java::util;

pub struct MethodUsage {
	declaration: com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration,
	param_types: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	exception_types: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	return_type: com::github::javaparser::resolution::types::resolved_type::ResolvedType,
	type_parameters_map: com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap,
}

impl MethodUsage {
	pub fn new(declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration) -> com::github::javaparser::resolution::method_usage::MethodUsage {
		self.typeParametersMap = ResolvedTypeParametersMap::empty();
		self.declaration = declaration;
		self.param_types.addAll(&declaration.formal_parameter_types());
		self.exception_types.addAll(&declaration.get_specified_exceptions());
		self.return_type = declaration.get_return_type();
	}

	pub fn new(declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, param_types: &/* Java */ java::util::List /**/, return_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::method_usage::MethodUsage {
		this(declaration, param_types, return_type, &declaration.get_specified_exceptions(), &ResolvedTypeParametersMap::empty());
	}

	pub fn new(declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, param_types: &/* Java */ java::util::List /**/, return_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, exception_types: &/* Java */ java::util::List /**/) -> com::github::javaparser::resolution::method_usage::MethodUsage {
		this(declaration, param_types, return_type, exception_types, &ResolvedTypeParametersMap::empty());
	}

	fn new(declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, param_types: &/* Java */ java::util::List /**/, return_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, exception_types: &/* Java */ java::util::List /**/, type_parameters_map: &com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap) -> com::github::javaparser::resolution::method_usage::MethodUsage {
		self.declaration = declaration;
		self.paramTypes = param_types;
		self.returnType = return_type;
		self.exceptionTypes = exception_types;
		self.typeParametersMap = type_parameters_map;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "MethodUsage{" + "declaration=" + self.declaration + ", paramTypes=" + self.param_types + '}';
	}

	pub fn get_declaration(&self) -> com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration {
		return self.declaration;
	}

	pub fn get_name(&self) -> /* Java */ java::lang::String /**/ {
		return self.declaration.get_name();
	}

	pub fn declaring_type(&self) -> com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration {
		return self.declaration.declaring_type();
	}

	pub fn return_type(&self) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.return_type;
	}

	pub fn get_param_types(&self) -> /* Java */ java::util::List /**/ {
		return self.param_types;
	}

	pub fn replace_param_type(&self, i: i32, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::method_usage::MethodUsage {
		if i < 0 || i >= self.get_no_params() {
			return Err(IllegalArgumentException::new());
		}
		if self.param_types.get(i) == replaced {
			return self;
		}
		let new_params: List<ResolvedType> = LinkedList<>::new(self.param_types);
		new_params.set(i, replaced);
		return MethodUsage::new(self.declaration, new_params, self.return_type, self.exception_types, self.type_parameters_map);
	}

	pub fn replace_exception_type(&self, i: i32, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::method_usage::MethodUsage {
		if i < 0 || i >= self.exception_types.size() {
			return Err(IllegalArgumentException::new());
		}
		if self.exception_types.get(i) == replaced {
			return self;
		}
		let new_types: List<ResolvedType> = LinkedList<>::new(self.exception_types);
		new_types.set(i, replaced);
		return MethodUsage::new(self.declaration, self.param_types, self.return_type, new_types, self.type_parameters_map);
	}

	pub fn replace_return_type(&self, return_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> com::github::javaparser::resolution::method_usage::MethodUsage {
		if return_type == self.returnType {
			return self;
		}
		return MethodUsage::new(self.declaration, self.param_types, return_type, self.exception_types, self.type_parameters_map);
	}

	pub fn get_no_params(&self) -> i32 {
		return self.param_types.size();
	}

	pub fn get_param_type(&self, i: i32) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.param_types.get(i);
	}

	pub fn replace_type_parameter(&self, type_parameter: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::method_usage::MethodUsage {
		if type == null {
			return Err(IllegalArgumentException::new());
		}
		// TODO if the method declaration has a type param with that name ignore this call
		let res: MethodUsage = MethodUsage::new(self.declaration, self.param_types, self.return_type, self.exception_types, &self.type_parameters_map.to_builder().set_value(type_parameter, type).build());
		let inferred_types: Map<ResolvedTypeParameterDeclaration, ResolvedType> = HashMap<>::new();
		 {
			let i: i32 = 0;
			while i < self.param_types.size() {
				{
					let original_param_type: ResolvedType = self.param_types.get(i);
					let new_param_type: ResolvedType = original_param_type.replace_type_variables(type_parameter, type, inferred_types);
					res = res.replace_param_type(i, new_param_type)?;
				}
				i += 1;
			 }
		 }
	
		 {
			let i: i32 = 0;
			while i < self.exception_types.size() {
				{
					let original_type: ResolvedType = self.exception_types.get(i);
					let new_type: ResolvedType = original_type.replace_type_variables(type_parameter, type, inferred_types);
					res = res.replace_exception_type(i, new_type)?;
				}
				i += 1;
			 }
		 }
	
		let old_return_type: ResolvedType = res.returnType;
		let new_return_type: ResolvedType = old_return_type.replace_type_variables(type_parameter, type, inferred_types);
		res = res.replace_return_type(new_return_type);
		return res;
	}

	pub fn type_parameters_map(&self) -> com::github::javaparser::resolution::types::parametrization::resolved_type_parameters_map::ResolvedTypeParametersMap {
		return self.type_parameters_map;
	}

	pub fn get_qualified_signature(&self) -> /* Java */ java::lang::String /**/ {
		return self.get_declaration().declaring_type().get_qualified_name() + "." + self.get_signature();
	}

	pub fn get_signature(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append(&self.get_name());
		sb.append("(");
		 {
			let i: i32 = 0;
			while i < self.get_no_params() {
				{
					if i != 0 {
						sb.append(", ");
					}
					let type: ResolvedType = self.get_param_type(i);
					if type.is_array() && self.get_declaration().get_param(i).is_variadic() {
						sb.append(&type.as_array_type()?.get_component_type().describe()).append("...");
					} else {
						sb.append(&type.describe());
					}
				}
				i += 1;
			 }
		 }
	
		sb.append(")");
		return sb.toString();
	}

	pub fn get_erased_signature(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new();
		sb.append(&self.get_name());
		sb.append("(");
		 {
			let i: i32 = 0;
			while i < self.get_no_params() {
				{
					if i != 0 {
						sb.append(", ");
					}
					let type: ResolvedType = self.get_param_type(i).erasure();
					if type.is_array() && self.get_declaration().get_param(i).is_variadic() {
						sb.append(&type.as_array_type()?.get_component_type().describe()).append("...");
					} else {
						sb.append(&type.describe());
					}
				}
				i += 1;
			 }
		 }
	
		sb.append(")");
		return sb.toString();
	}

	pub fn exception_types(&self) -> /* Java */ java::util::List /**/ {
		return self.exception_types;
	}

	pub fn is_same_signature(&self, other_method_usage: &com::github::javaparser::resolution::method_usage::MethodUsage) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return self.get_signature()?.equals(&other_method_usage.get_signature()?);
	}

	pub fn is_sub_signature(&self, other_method_usage: &com::github::javaparser::resolution::method_usage::MethodUsage) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return self.get_erased_signature()?.equals(&other_method_usage.get_erased_signature()?);
	}

	pub fn is_return_type_substituable(&self, other_method_usage: &com::github::javaparser::resolution::method_usage::MethodUsage) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return self.get_declaration().is_return_type_substituable(&other_method_usage.get_declaration().get_return_type())?;
	}
}

impl com::github::javaparser::resolution::types::parametrization::resolved_type_parametrized::ResolvedTypeParametrized for MethodUsage {}