use crate::com::github::javaparser::resolution::TypeSolver;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::model::typesystem::ReferenceTypeImpl;
use crate::com::github::javaparser::resolution::types::ResolvedReferenceType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use crate::com::github::javaparser::resolution::types::ResolvedTypeVariable;
use crate::com::github::javaparser::resolution::types::ResolvedWildcard;
use java::util::HashSet;
use java::util::Set;
use java::util::stream::Collectors;

pub struct InferenceVariableType {
	id: i32,
	corresponding_tp: com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration,
	equivalent_types: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	type_solver: com::github::javaparser::resolution::type_solver::TypeSolver,
	super_types: /* Java */ java::util::Set /**/ = HashSet<>::new(),
}

impl InferenceVariableType {
	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "InferenceVariableType{" + "id=" + self.id + '}';
	}

	pub fn set_corresponding_tp(&mut self, corresponding_tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration) {
		self.correspondingTp = corresponding_tp;
	}

	pub fn register_equivalent_type(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) {
		self.equivalentTypes.add(type);
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if !(o instanceof InferenceVariableType) {
			return false;
		}
	
		let that: InferenceVariableType = o as InferenceVariableType;
		return self.id == that.id;
	}

	pub fn hash_code(&self) -> i32 {
		return self.id;
	}

	pub fn new(id: i32, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) -> com::github::javaparser::resolution::logic::inference_variable_type::InferenceVariableType {
		self.id = id;
		self.typeSolver = type_solver;
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return "InferenceVariable_" + self.id;
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return Err(UnsupportedOperationException::new());
	}

	fn concrete_equivalent_types_also_indirectly(&self, considered: &/* Java */ java::util::Set /**/, inference_variable_type: &com::github::javaparser::resolution::logic::inference_variable_type::InferenceVariableType) -> /* Java */ java::util::Set /**/ {
		considered.add(inference_variable_type);
		let result: Set<ResolvedType> = HashSet<>::new();
		result.addAll(&inference_variable_type.equivalentTypes.stream().filter(|t|!t.is_type_variable() && !(t instanceof InferenceVariableType)).collect(&Collectors::toSet()));
		inference_variable_type.equivalentTypes.stream().filter(|t|t instanceof InferenceVariableType).forEach(|t|{
			let ivt: InferenceVariableType = t as InferenceVariableType;
			if !considered.contains(ivt) {
				result.addAll(&self.concrete_equivalent_types_also_indirectly(considered, ivt));
			}
		});
		return result;
	}

	pub fn equivalent_type(&self) /* thrown(java.lang.IllegalStateException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let concrete_equivalent: Set<ResolvedType> = self.concrete_equivalent_types_also_indirectly(HashSet<>::new(), self);
		if concrete_equivalent.isEmpty() {
			if self.corresponding_tp == null {
				return ReferenceTypeImpl::new(&self.type_solver.get_solved_java_lang_object());
			}
			return ResolvedTypeVariable::new(self.corresponding_tp);
		}
		if concrete_equivalent.size() == 1 {
			return concrete_equivalent.iterator().next();
		}
		let not_type_variables: Set<ResolvedType> = self.equivalent_types.stream().filter(|t|!t.is_type_variable() && !self.has_inference_variables(t)).collect(&Collectors::toSet());
		if not_type_variables.size() == 1 {
			return not_type_variables.iterator().next();
		}
		if not_type_variables.size() == 0 && !self.super_types.isEmpty() {
			if self.super_types.size() == 1 {
				return self.super_types.iterator().next();
			}
			return Err(IllegalStateException::new("Super types are: " + self.super_types));
		}
		return Err(IllegalStateException::new("Equivalent types are: " + self.equivalent_types));
	}

	fn has_inference_variables(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type instanceof InferenceVariableType {
			return true;
		}
		if type.is_reference_type() {
			let ref_type: ResolvedReferenceType = type.as_reference_type()?;
			for t in ref_type.type_parameters_values() {
				if self.has_inference_variables(t)? {
					return true;
				}
			}
			return false;
		}
		if type.is_wildcard() {
			let wildcard_type: ResolvedWildcard = type.as_wildcard()?;
			return self.has_inference_variables(&wildcard_type.get_bounded_type()?)?;
		}
		return false;
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for InferenceVariableType {}