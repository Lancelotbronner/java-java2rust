use crate::com::github::javaparser::resolution::TypeSolver;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::model::typesystem::ReferenceTypeImpl;
use crate::com::github::javaparser::resolution::types;
use java::util;
use java::util::stream::Collectors;
use java::util::stream::Stream;

pub struct InferenceContext {
	next_inference_variable_id: i32 = 0,
	type_solver: com::github::javaparser::resolution::type_solver::TypeSolver,
	inference_variable_types: /* Java */ java::util::List /**/ = ArrayList<>::new(),
	inference_variable_type_map: /* Java */ java::util::Map /**/ = HashMap<>::new(),
}

impl InferenceContext {
	pub fn new(type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) -> com::github::javaparser::resolution::logic::inference_context::InferenceContext {
		self.typeSolver = type_solver;
	}

	fn inference_variable_type_for_tp(&self, tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration) -> com::github::javaparser::resolution::logic::inference_variable_type::InferenceVariableType {
		if !self.inference_variable_type_map.containsKey(&tp.get_name()) {
			let inference_variable_type: InferenceVariableType = InferenceVariableType::new(self.next_inference_variable_id += 1 !!!check!!! post increment, self.type_solver);
			self.inference_variable_types.add(inference_variable_type);
			inference_variable_type.set_corresponding_tp(tp);
			self.inference_variable_type_map.put(&tp.get_name(), inference_variable_type);
		}
		return self.inference_variable_type_map.get(&tp.get_name());
	}

	pub fn add_pair(&self, mut target: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, mut actual: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(com.github.javaparser.resolution.logic.ConflictingGenericTypesException | java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		target = self.place_inference_variables(target)?;
		actual = self.place_inference_variables(actual)?;
		self.register_correspondance(target, actual)?;
		return target;
	}

	pub fn add_single(&self, actual: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		return self.place_inference_variables(actual)?;
	}

	fn register_correspondance(&self, formal_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, actual_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(com.github.javaparser.resolution.logic.ConflictingGenericTypesException | java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ {
		if formal_type.is_reference_type() && actual_type.is_reference_type() {
			let formal_type_as_reference: ResolvedReferenceType = formal_type.as_reference_type()?;
			let actual_type_as_reference: ResolvedReferenceType = actual_type.as_reference_type()?;
			if !formal_type_as_reference.get_qualified_name().equals(&actual_type_as_reference.get_qualified_name()) {
				let ancestors: List<ResolvedReferenceType> = actual_type_as_reference.get_all_ancestors();
				/* final */ let formal_param_type_q_name: String = formal_type_as_reference.get_qualified_name();
				// Interfaces do not extend the class Object,
				// which means that if the formal parameter is of type Object,
				// all types can match including the actual type.
				let corresponding_formal_type: List<ResolvedType> =  if "java.lang.Object".equals(formal_param_type_q_name) { Stream::concat(&ArrayList<ResolvedType>::new(&Arrays::asList(actual_type)).stream(), &ancestors.stream().map(|ancestor|ancestor.as_reference_type()).collect(&Collectors::toList()).stream()).collect(&Collectors::toList()) } else { ancestors.stream().filter(|(a)|a.get_qualified_name().equals(formal_param_type_q_name)).collect(&Collectors::toList()) };
				if corresponding_formal_type.isEmpty() {
					ancestors = formal_type_as_reference.get_all_ancestors();
					/* final */ let actual_param_type_qname: String = actual_type_as_reference.get_qualified_name();
					let corresponding_actual_type: List<ResolvedType> = ancestors.stream().filter(|a|a.get_qualified_name().equals(actual_param_type_qname)).collect(&Collectors::toList());
					if corresponding_actual_type.isEmpty() {
						return Err(ConflictingGenericTypesException::new(formal_type, actual_type));
					}
					corresponding_formal_type = corresponding_actual_type;
				}
				actual_type_as_reference = corresponding_formal_type.get(0).as_reference_type()?;
			}
			if formal_type_as_reference.get_qualified_name().equals(&actual_type_as_reference.get_qualified_name()) {
				if !formal_type_as_reference.type_parameters_values().isEmpty() {
					if actual_type_as_reference.is_raw_type() {
					// nothing to do
					} else {
						let i: i32 = 0;
						for formal_type_parameter in formal_type_as_reference.type_parameters_values() {
							self.register_correspondance(formal_type_parameter, &actual_type_as_reference.type_parameters_values().get(i))?;
							i += 1;
						}
					}
				}
			}
		} else if formal_type instanceof InferenceVariableType && !actual_type.is_primitive() {
			(formal_type as InferenceVariableType).register_equivalent_type(actual_type);
			if actual_type instanceof InferenceVariableType {
				(actual_type as InferenceVariableType).register_equivalent_type(formal_type);
			}
		} else if actual_type.is_null() {
		// nothing to do
		} else if actual_type.equals(formal_type) {
		// nothing to do
		} else if actual_type.is_array() && formal_type.is_array() {
			self.register_correspondance(&formal_type.as_array_type()?.get_component_type(), &actual_type.as_array_type()?.get_component_type())?;
		} else if formal_type.is_wildcard() {
			// nothing to do
			if (actual_type instanceof InferenceVariableType) && formal_type.as_wildcard()?.is_bounded() {
				(actual_type as InferenceVariableType).register_equivalent_type(&formal_type.as_wildcard()?.get_bounded_type()?);
				if formal_type.as_wildcard()?.get_bounded_type()? instanceof InferenceVariableType {
					(formal_type.as_wildcard()?.get_bounded_type()? as InferenceVariableType).register_equivalent_type(actual_type);
				}
			}
			if actual_type.is_wildcard() {
				let formal_wildcard: ResolvedWildcard = formal_type.as_wildcard()?;
				let actual_wildcard: ResolvedWildcard = actual_type.as_wildcard()?;
				if formal_wildcard.is_bounded() && formal_wildcard.get_bounded_type()? instanceof InferenceVariableType {
					if formal_wildcard.is_super() && actual_wildcard.is_super() {
						(formal_type.as_wildcard()?.get_bounded_type()? as InferenceVariableType).register_equivalent_type(&actual_wildcard.get_bounded_type()?);
					} else if formal_wildcard.is_extends() && actual_wildcard.is_extends() {
						(formal_type.as_wildcard()?.get_bounded_type()? as InferenceVariableType).register_equivalent_type(&actual_wildcard.get_bounded_type()?);
					}
				}
			}
			if actual_type.is_reference_type() {
				if formal_type.as_wildcard()?.is_bounded() {
					self.register_correspondance(&formal_type.as_wildcard()?.get_bounded_type()?, actual_type)?;
				}
			}
		} else if actual_type instanceof InferenceVariableType {
			if formal_type instanceof ResolvedReferenceType {
				(actual_type as InferenceVariableType).register_equivalent_type(formal_type);
			} else if formal_type instanceof InferenceVariableType {
				(actual_type as InferenceVariableType).register_equivalent_type(formal_type);
			}
		} else if actual_type.is_constraint() {
			let constraint_type: ResolvedLambdaConstraintType = actual_type.as_constraint_type()?;
			if constraint_type.get_bound() instanceof InferenceVariableType {
				(constraint_type.get_bound() as InferenceVariableType).register_equivalent_type(formal_type);
			}
		} else if actual_type.is_primitive() {
			if formal_type.is_primitive() {
			// nothing to do
			} else {
				let resolved_typedeclaration: ResolvedReferenceTypeDeclaration = self.type_solver.solve_type(&actual_type.as_primitive()?.get_box_typeq_name())?;
				self.register_correspondance(formal_type, ReferenceTypeImpl::new(resolved_typedeclaration))?;
			}
		} else if actual_type.is_reference_type() {
			if formal_type.is_primitive() {
				if formal_type.as_primitive()?.get_box_typeq_name().equals(&actual_type.describe()) {
					let resolved_typedeclaration: ResolvedReferenceTypeDeclaration = self.type_solver.solve_type(&formal_type.as_primitive()?.get_box_typeq_name())?;
					self.register_correspondance(ReferenceTypeImpl::new(resolved_typedeclaration), actual_type)?;
				} else {
				// nothing to do
				}
			} else {
			// nothing to do
			}
		} else if formal_type.is_reference_type() {
			let formal_type_as_reference: ResolvedReferenceType = formal_type.as_reference_type()?;
			if formal_type_as_reference.is_java_lang_object() {
			// nothing to do
			} else {
				return Err(UnsupportedOperationException::new(formal_type.describe() + " " + actual_type.describe()));
			}
		} else {
			return Err(UnsupportedOperationException::new(formal_type.describe() + " " + actual_type.describe()));
		}
	}

	fn place_inference_variables(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if type.is_wildcard() {
			if type.as_wildcard()?.is_extends() {
				return ResolvedWildcard::extends_bound(&self.place_inference_variables(&type.as_wildcard()?.get_bounded_type()?)?);
			}
			if type.as_wildcard()?.is_super() {
				return ResolvedWildcard::super_bound(&self.place_inference_variables(&type.as_wildcard()?.get_bounded_type()?)?);
			}
			return type;
		}
		if type.is_type_variable() {
			return self.inference_variable_type_for_tp(&type.as_type_parameter()?);
		}
		if type.is_reference_type() {
			return type.as_reference_type()?.transform_type_parameters(|tp|self.place_inference_variables(tp)?);
		}
		if type.is_array() {
			return ResolvedArrayType::new(&self.place_inference_variables(&type.as_array_type()?.get_component_type())?);
		}
		if type.is_null() || type.is_primitive() || type.is_void() {
			return type;
		}
		if type.is_constraint() {
			return ResolvedLambdaConstraintType::bound(&self.place_inference_variables(&type.as_constraint_type()?.get_bound())?);
		}
		if type instanceof InferenceVariableType {
			return type;
		}
		return Err(UnsupportedOperationException::new(&type.describe()));
	}

	pub fn resolve(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.IllegalStateException | java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if type instanceof InferenceVariableType {
			let inference_variable_type: InferenceVariableType = type as InferenceVariableType;
			return inference_variable_type.equivalent_type()?;
		}
		if type.is_reference_type() {
			return type.as_reference_type()?.transform_type_parameters(|tp|self.resolve(tp)?);
		}
		if type.is_null() || type.is_primitive() || type.is_void() {
			return type;
		}
		if type.is_array() {
			return ResolvedArrayType::new(&self.resolve(&type.as_array_type()?.get_component_type())?);
		}
		if type.is_wildcard() {
			if type.as_wildcard()?.is_extends() {
				return ResolvedWildcard::extends_bound(&self.resolve(&type.as_wildcard()?.get_bounded_type()?)?);
			}
			if type.as_wildcard()?.is_super() {
				return ResolvedWildcard::super_bound(&self.resolve(&type.as_wildcard()?.get_bounded_type()?)?);
			}
			return type;
		}
		return Err(UnsupportedOperationException::new(&type.describe()));
	}
}