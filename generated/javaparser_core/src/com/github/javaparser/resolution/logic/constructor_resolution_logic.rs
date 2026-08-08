use crate::com::github::javaparser::resolution::MethodAmbiguityException;
use crate::com::github::javaparser::resolution::TypeSolver;
use crate::com::github::javaparser::resolution::declarations::ResolvedConstructorDeclaration;
use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use crate::com::github::javaparser::resolution::model::SymbolReference;
use crate::com::github::javaparser::resolution::types::ResolvedArrayType;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::util::ArrayList;
use java::util::HashMap;
use java::util::List;
use java::util::Map;
use java::util::stream::Collectors;

pub struct ConstructorResolutionLogic;

impl ConstructorResolutionLogic {
	fn group_variadic_param_values(&self, arguments_types: &/* Java */ java::util::List /**/, start_variadic: i32, variadic_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> /* Java */ java::util::List /**/ {
		let res: List<ResolvedType> = ArrayList<>::new(&arguments_types.subList(0, start_variadic));
		let variadic_values: List<ResolvedType> = arguments_types.subList(start_variadic, &arguments_types.size());
		if variadic_values.isEmpty() {
			// TODO if there are no variadic values we should default to the bound of the formal type
			res.add(variadic_type);
		} else {
			let component_type: ResolvedType = com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::find_common_type(variadic_values)?;
			res.add(ResolvedArrayType::new(component_type));
		}
		return res;
	}

	fn find_common_type(&self, variadic_values: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if variadic_values.isEmpty() {
			return Err(IllegalArgumentException::new());
		}
		// TODO implement this decently
		return variadic_values.get(0);
	}

	pub fn is_applicable(&self, constructor: &com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) -> bool {
		return com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::is_applicable(constructor, arguments_types, type_solver, false);
	}

	fn is_applicable(&self, constructor: &com::github::javaparser::resolution::declarations::resolved_constructor_declaration::ResolvedConstructorDeclaration, mut arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver, with_wildcard_tolerance: bool) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if constructor.has_variadic_parameter() {
			let pos: i32 = constructor.get_number_of_params() - 1;
			if constructor.get_number_of_params() == arguments_types.size() {
				// check if the last value is directly assignable as an array
				let expected_type: ResolvedType = constructor.get_last_param()?.get_type();
				let actual_type: ResolvedType = arguments_types.get(pos);
				if !expected_type.is_assignable_by(actual_type) {
					for tp in constructor.get_type_parameters() {
						expected_type = MethodResolutionLogic::replace_type_param(expected_type, tp, type_solver)?;
					}
					if !expected_type.is_assignable_by(actual_type) {
						if actual_type.is_array() && expected_type.is_assignable_by(&actual_type.as_array_type()?.get_component_type()) {
							arguments_types.set(pos, &actual_type.as_array_type()?.get_component_type());
						} else {
							arguments_types = com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::group_variadic_param_values(arguments_types, pos, &constructor.get_last_param()?.get_type());
						}
					}
				}
			// else it is already assignable, nothing to do
			} else {
				if pos > arguments_types.size() {
					return false;
				}
				arguments_types = com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::group_variadic_param_values(arguments_types, pos, &constructor.get_last_param()?.get_type());
			}
		}
		if constructor.get_number_of_params() != arguments_types.size() {
			return false;
		}
		let matched_parameters: Map<String, ResolvedType> = HashMap<>::new();
		let need_for_wild_card_tolerance: bool = false;
		 {
			let i: i32 = 0;
			while i < constructor.get_number_of_params() {
				{
					let expected_type: ResolvedType = constructor.get_param(i).get_type();
					let actual_type: ResolvedType = arguments_types.get(i);
					if (expected_type.is_type_variable() && !(expected_type.is_wildcard())) && expected_type.as_type_parameter()?.declared_on_method() {
						matched_parameters.put(&expected_type.as_type_parameter()?.get_name(), actual_type);
						continue;
					}
					let is_assignable_without_substitution: bool = expected_type.is_assignable_by(actual_type) || (constructor.get_param(i).is_variadic() && ResolvedArrayType::new(expected_type).is_assignable_by(actual_type));
					if !is_assignable_without_substitution && expected_type.is_reference_type() && actual_type.is_reference_type() {
						is_assignable_without_substitution = MethodResolutionLogic::is_assignable_match_type_parameters(&expected_type.as_reference_type()?, &actual_type.as_reference_type()?, matched_parameters);
					}
					if !is_assignable_without_substitution {
						for tp in constructor.get_type_parameters() {
							expected_type = MethodResolutionLogic::replace_type_param(expected_type, tp, type_solver)?;
						}
						for tp in constructor.declaring_type().get_type_parameters() {
							expected_type = MethodResolutionLogic::replace_type_param(expected_type, tp, type_solver)?;
						}
						if !expected_type.is_assignable_by(actual_type) {
							if actual_type.is_wildcard() && with_wildcard_tolerance && !expected_type.is_primitive() {
								need_for_wild_card_tolerance = true;
								continue;
							}
							if constructor.has_variadic_parameter() && i == constructor.get_number_of_params() - 1 {
								if ResolvedArrayType::new(expected_type).is_assignable_by(actual_type) {
									continue;
								}
							}
							return false;
						}
					}
				}
				i += 1;
			 }
		 }
	
		return !with_wildcard_tolerance || need_for_wild_card_tolerance;
	}

	pub fn find_most_applicable(&self, constructors: &/* Java */ java::util::List /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(com.github.javaparser.resolution.MethodAmbiguityException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let res: SymbolReference<ResolvedConstructorDeclaration> = com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::find_most_applicable(constructors, arguments_types, type_solver, false)?;
		if res.is_solved() {
			return res;
		}
		return com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::find_most_applicable(constructors, arguments_types, type_solver, true)?;
	}

	pub fn find_most_applicable(&self, constructors: &/* Java */ java::util::List /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver, wildcard_tolerance: bool) /* thrown(com.github.javaparser.resolution.MethodAmbiguityException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let applicable_constructors: List<ResolvedConstructorDeclaration> = constructors.stream().filter(|(m)|com::github::javaparser::resolution::logic::constructor_resolution_logic::ConstructorResolutionLogic::is_applicable(m, arguments_types, type_solver, wildcard_tolerance)?).collect(&Collectors::toList());
		if applicable_constructors.isEmpty() {
			return SymbolReference::unsolved();
		}
		if applicable_constructors.size() == 1 {
			return SymbolReference::solved(&applicable_constructors.get(0));
		}
		let winning_candidate: ResolvedConstructorDeclaration = applicable_constructors.get(0);
		let other: ResolvedConstructorDeclaration = null;
		let possible_ambiguity: bool = false;
		 {
			let i: i32 = 1;
			while i < applicable_constructors.size() {
				{
					other = applicable_constructors.get(i);
					if MethodResolutionLogic::is_more_specific(winning_candidate, other, arguments_types) {
						possible_ambiguity = false;
					} else if MethodResolutionLogic::is_more_specific(other, winning_candidate, arguments_types) {
						possible_ambiguity = false;
						winning_candidate = other;
					} else {
						if winning_candidate.declaring_type().get_qualified_name().equals(&other.declaring_type().get_qualified_name()) {
							possible_ambiguity = true;
						} else {
						// we expect the methods to be ordered such that inherited methods are later in the list
						}
					}
					if possible_ambiguity {
						// pick the first exact match if it exists
						if !MethodResolutionLogic::is_exact_match(winning_candidate, arguments_types) {
							if MethodResolutionLogic::is_exact_match(other, arguments_types) {
								winning_candidate = other;
							} else {
								return Err(MethodAmbiguityException::new("Ambiguous constructor call: cannot find a most applicable constructor: " + winning_candidate + ", " + other));
							}
						}
					}
				}
				i += 1;
			 }
		 }
	
		return SymbolReference::solved(winning_candidate);
	}
}