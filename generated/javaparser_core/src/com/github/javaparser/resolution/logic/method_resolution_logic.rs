use crate::com::github::javaparser::resolution::MethodAmbiguityException;
use crate::com::github::javaparser::resolution::MethodUsage;
use crate::com::github::javaparser::resolution::TypeSolver;
use crate::com::github::javaparser::resolution::declarations;
use crate::com::github::javaparser::resolution::model::LambdaArgumentTypePlaceholder;
use crate::com::github::javaparser::resolution::model::SymbolReference;
use crate::com::github::javaparser::resolution::model::typesystem::ReferenceTypeImpl;
use crate::com::github::javaparser::resolution::types;
use java::util;
use java::util::concurrent::ConcurrentHashMap;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::stream::Collectors;

pub struct MethodResolutionLogic;

impl MethodResolutionLogic {
	static JAVA_LANG_OBJECT: /* Java */ java::lang::String /**/ = Object.class.getCanonicalName();

	fn group_variadic_param_values(&self, arguments_types: &/* Java */ java::util::List /**/, start_variadic: i32, variadic_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> /* Java */ java::util::List /**/ {
		let res: List<ResolvedType> = ArrayList<>::new(&arguments_types.subList(0, start_variadic));
		let variadic_values: List<ResolvedType> = arguments_types.subList(start_variadic, &arguments_types.size());
		if variadic_values.isEmpty() {
			// TODO if there are no variadic values we should default to the bound of the formal type
			res.add(variadic_type);
		} else {
			let component_type: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::find_common_type(variadic_values)?;
			res.add(&com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::convert_to_variadic_parameter(component_type));
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

	pub fn is_applicable(&self, method: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) -> bool {
		return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_applicable(method, name, arguments_types, type_solver, false);
	}

	fn is_conflicting_lambda_type(&self, lambda_placeholder: &com::github::javaparser::resolution::model::lambda_argument_type_placeholder::LambdaArgumentTypePlaceholder, expected_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		//  to duplicated work or maybe infinite recursion.
		if !expected_type.is_reference_type() {
			return false;
		}
		let maybe_functional_interface: Optional<MethodUsage> = FunctionalInterfaceLogic::get_functional_method(expected_type);
		if maybe_functional_interface.isPresent() {
			let functional_interface: MethodUsage = maybe_functional_interface.get();
			// method, the lambda cannot implement that interface.
			if lambda_placeholder.get_parameter_count().isPresent() && functional_interface.get_no_params() != lambda_placeholder.get_parameter_count().get() {
				return true;
			}
			//      implement void methods.
			if lambda_placeholder.body_block_has_explicit_non_void_return().isPresent() {
				let lambda_return_is_void: bool = !lambda_placeholder.body_block_has_explicit_non_void_return().get();
				if lambda_return_is_void && !functional_interface.return_type().is_void() {
					return true;
				}
				if !lambda_return_is_void && functional_interface.return_type().is_void() {
					return true;
				}
			}
		}
		return false;
	}

	fn is_applicable(&self, method_declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, needle_name: &/* Java */ java::lang::String /**/, mut needle_argument_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver, with_wildcard_tolerance: bool) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if !method_declaration.get_name().equals(needle_name) {
			return false;
		}
		// Create MethodUsage for type variable substitution
		let method_usage_for_substitution: MethodUsage = MethodUsage::new(method_declaration);
		method_usage_for_substitution = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_declaring_type_parameters(method_usage_for_substitution, type_solver);
		// Map substituted parameters back to the argument list we'll use
		// This ensures inherited generic method signatures use the correct type variables
		// The index of the final method parameter (on the method declaration).
		let count_of_method_parameters_declared: i32 = method_declaration.get_number_of_params();
		// The index of the final argument passed (on the method usage).
		let count_of_needle_arguments_passed: i32 = needle_argument_types.size();
		let method_is_declared_with_variadic_parameter: bool = method_declaration.has_variadic_parameter();
		if !method_is_declared_with_variadic_parameter && (count_of_needle_arguments_passed != count_of_method_parameters_declared) {
			// If it is not variadic, and the number of parameters/arguments are unequal -- this is not a match.
			return false;
		}
		if method_is_declared_with_variadic_parameter {
			if count_of_needle_arguments_passed <= (count_of_method_parameters_declared - 2) {
				//  (thus being short of only 1 argument is fine, but being short of 2 or more is not).
				return false;
			}
			// If the method declaration we're considering has a variadic parameter,
			// attempt to convert the given list of arguments to fit this pattern
			// e.g. foo(String s, String... s2) {} --- consider the first argument, then group the remainder as an array
			let expected_variadic_parameter_type: ResolvedType = method_declaration.get_last_param()?.get_type();
			for tp in method_declaration.get_type_parameters() {
				expected_variadic_parameter_type = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::replace_type_param(expected_variadic_parameter_type, tp, type_solver)?;
			}
			if count_of_needle_arguments_passed > count_of_method_parameters_declared {
				// valid type. (Maybe this is also done later..?)
				 {
					let variadic_argument_index: i32 = count_of_method_parameters_declared;
					while variadic_argument_index < count_of_needle_arguments_passed {
						{
							let current_argument_type: ResolvedType = needle_argument_types.get(variadic_argument_index);
							let variadic_component_type: ResolvedType = expected_variadic_parameter_type.as_array_type()?.get_component_type();
							let argument_is_assignable_to_variadic_component_type: bool = variadic_component_type.is_assignable_by(current_argument_type);
							// Check boxing/unboxing for varargs
							if !argument_is_assignable_to_variadic_component_type {
								argument_is_assignable_to_variadic_component_type = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(variadic_component_type, current_argument_type, type_solver);
							}
							if !argument_is_assignable_to_variadic_component_type {
								return false;
							}
						}
						variadic_argument_index += 1;
					 }
				 }
	
			}
			needle_argument_types = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::group_trailing_arguments_into_array(method_declaration, needle_argument_types, expected_variadic_parameter_type);
		}
		// The index of the final argument passed (on the method usage).
		let count_of_needle_arguments_passed_after_grouping: i32 = needle_argument_types.size();
		// At this point, therefore, the number of arguments must be equal -- if they're not, then there is no match.
		if count_of_needle_arguments_passed_after_grouping != count_of_method_parameters_declared {
			return false;
		}
		let matched_parameters: Map<String, ResolvedType> = HashMap<>::new();
		let need_for_wild_card_tolerance: bool = false;
		 {
			let i: i32 = 0;
			while i < count_of_method_parameters_declared {
				{
					let expected_declared_type: ResolvedType = method_declaration.get_param(i).get_type();
					let actual_argument_type: ResolvedType = needle_argument_types.get(i);
					if actual_argument_type instanceof LambdaArgumentTypePlaceholder && com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_conflicting_lambda_type(actual_argument_type as LambdaArgumentTypePlaceholder, expected_declared_type) {
						return false;
					}
					if (expected_declared_type.is_type_variable() && !(expected_declared_type.is_wildcard())) && expected_declared_type.as_type_parameter()?.declared_on_method() {
						matched_parameters.put(&expected_declared_type.as_type_parameter()?.get_name(), actual_argument_type);
						continue;
					}
					// even if an array of primitive type cannot be assigned to an array of Object
					if method_declaration.get_param(i).is_variadic() && (i == count_of_method_parameters_declared - 1) && com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_array_of_object(expected_declared_type) && actual_argument_type.is_array() {
						continue;
					}
					let is_assignable_without_substitution: bool = expected_declared_type.is_assignable_by(actual_argument_type) || (method_declaration.get_param(i).is_variadic() && com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::convert_to_variadic_parameter(expected_declared_type).is_assignable_by(actual_argument_type));
					if !is_assignable_without_substitution && expected_declared_type.is_reference_type() && actual_argument_type.is_reference_type() {
						is_assignable_without_substitution = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters(&expected_declared_type.as_reference_type()?, &actual_argument_type.as_reference_type()?, matched_parameters);
					}
					if !is_assignable_without_substitution {
						let type_parameters: List<ResolvedTypeParameterDeclaration> = method_declaration.get_type_parameters();
						type_parameters.addAll(&method_declaration.declaring_type().get_type_parameters());
						for tp in type_parameters {
							expected_declared_type = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::replace_type_param(expected_declared_type, tp, type_solver)?;
						}
						if !expected_declared_type.is_assignable_by(actual_argument_type) {
							// Check boxing/unboxing compatibility using TypeSolver
							if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_declared_type, actual_argument_type, type_solver) {
								// This parameter is compatible via boxing/unboxing
								continue;
							}
							if actual_argument_type.is_wildcard() && with_wildcard_tolerance && !expected_declared_type.is_primitive() {
								need_for_wild_card_tolerance = true;
								continue;
							}
							// we want to keep this method for future resolution
							if actual_argument_type.is_constraint() && with_wildcard_tolerance && (actual_argument_type.as_constraint_type()?.get_bound().is_type_variable() || (!actual_argument_type.as_constraint_type()?.get_bound().is_type_variable() && expected_declared_type.is_assignable_by(&actual_argument_type.as_constraint_type()?.get_bound()))) {
								need_for_wild_card_tolerance = true;
								continue;
							}
							if method_is_declared_with_variadic_parameter && i == count_of_method_parameters_declared - 1 {
								if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::convert_to_variadic_parameter(expected_declared_type).is_assignable_by(actual_argument_type) {
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

	fn is_array_of_object(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return type.is_array() && type.as_array_type()?.get_component_type().is_reference_type() && type.as_array_type()?.get_component_type().as_reference_type()?.is_java_lang_object();
	}

	fn convert_to_variadic_parameter(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::types::resolved_array_type::ResolvedArrayType {
		return  if type.is_array() { type.as_array_type()? } else { ResolvedArrayType::new(type) };
	}

	fn get_last_parameter_index(&self, count_of_method_parameters_declared: i32) -> i32 {
		return Math::max(0, count_of_method_parameters_declared - 1);
	}

	fn group_trailing_arguments_into_array(&self, method_declaration: &com::github::javaparser::resolution::declarations::resolved_method_declaration::ResolvedMethodDeclaration, mut needle_argument_types: &/* Java */ java::util::List /**/, expected_variadic_parameter_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::util::List /**/ {
		// The index of the final method parameter (on the method declaration).
		let count_of_method_parameters_declared: i32 = method_declaration.get_number_of_params();
		let last_method_parameter_index: i32 = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_last_parameter_index(count_of_method_parameters_declared);
		// The index of the final argument passed (on the method usage).
		let count_of_needle_arguments_passed: i32 = needle_argument_types.size();
		let last_needle_argument_index: i32 = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_last_parameter_index(count_of_needle_arguments_passed);
		if count_of_needle_arguments_passed > count_of_method_parameters_declared {
			// If it is variadic, and we have an "excess" of arguments, group the "trailing" arguments into an array.
			// Here we are sure that all of these grouped "trailing" arguments have the required type
			needle_argument_types = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::group_variadic_param_values(needle_argument_types, last_method_parameter_index, &method_declaration.get_last_param()?.get_type());
		}
		if count_of_needle_arguments_passed == (count_of_method_parameters_declared - 1) {
			// If it is variadic and we are short of **exactly one** parameter, this is a match.
			// Note that omitting the variadic parameter is treated as an empty array
			//  (thus being short of only 1 argument is fine, but being short of 2 or more is not).
			// thus group the "empty" value into an empty array...
			needle_argument_types = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::group_variadic_param_values(needle_argument_types, last_method_parameter_index, &method_declaration.get_last_param()?.get_type());
		} else if count_of_needle_arguments_passed == count_of_method_parameters_declared {
			let actual_argument_type: ResolvedType = needle_argument_types.get(last_needle_argument_index);
			let final_argument_is_array: bool = actual_argument_type.is_array() && expected_variadic_parameter_type.is_assignable_by(&actual_argument_type.as_array_type()?.get_component_type());
			if final_argument_is_array {
			// Treat as an array of values -- in which case the expected parameter type is the common type of this
			// array.
			// no need to do anything
			// expectedVariadicParameterType = actualArgumentType.asArrayType().getComponentType();
			} else {
				// Treat as a single value -- in which case, the expected parameter type is the same as the single
				// value.
				needle_argument_types = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::group_variadic_param_values(needle_argument_types, last_method_parameter_index, &method_declaration.get_last_param()?.get_type());
			}
		} else {
		// Should be unreachable.
		}
		return needle_argument_types;
	}

	pub fn is_assignable_match_type_parameters(&self, expected: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, actual: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, matched_parameters: &/* Java */ java::util::Map /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if expected.is_reference_type() && actual.is_reference_type() {
			return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters(&expected.as_reference_type()?, &actual.as_reference_type()?, matched_parameters);
		}
		if expected.is_reference_type() && ResolvedPrimitiveType::is_box_type(expected) && actual.is_primitive() {
			let expected_type: ResolvedPrimitiveType = ResolvedPrimitiveType::by_box_typeq_name(&expected.as_reference_type()?.get_qualified_name()).get().as_primitive()?;
			return expected.is_assignable_by(actual);
		}
		if expected.is_type_variable() {
			matched_parameters.put(&expected.as_type_parameter()?.get_name(), actual);
			return true;
		}
		if expected.is_array() {
			matched_parameters.put(&expected.as_array_type()?.get_component_type().toString(), actual);
			return true;
		}
		return Err(UnsupportedOperationException::new(expected.getClass().getCanonicalName() + " " + actual.getClass().getCanonicalName()));
	}

	pub fn is_assignable_match_type_parameters(&self, expected: &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType, actual: &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType, matched_parameters: &/* Java */ java::util::Map /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if actual.get_qualified_name().equals(&expected.get_qualified_name()) {
			return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters_matchingq_name(expected, actual, matched_parameters)?;
		} else {
			let ancestors: List<ResolvedReferenceType> = actual.get_all_ancestors();
			for ancestor in ancestors {
				if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters_matchingq_name(expected, ancestor, matched_parameters)? {
					return true;
				}
			}
		}
		return false;
	}

	fn is_assignable_match_type_parameters_matchingq_name(&self, expected: &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType, actual: &com::github::javaparser::resolution::types::resolved_reference_type::ResolvedReferenceType, matched_parameters: &/* Java */ java::util::Map /**/) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> bool {
		if !expected.get_qualified_name().equals(&actual.get_qualified_name()) {
			return false;
		}
		if expected.type_parameters_values().size() != actual.type_parameters_values().size() {
			return Err(UnsupportedOperationException::new());
		// return true;
		}
		 {
			let i: i32 = 0;
			while i < expected.type_parameters_values().size() {
				{
					let expected_param: ResolvedType = expected.type_parameters_values().get(i);
					let actual_param: ResolvedType = actual.type_parameters_values().get(i);
					// we should peel off one layer and ensure R <-> Integer
					if expected_param.is_reference_type() && actual_param.is_reference_type() {
						let r1: ResolvedReferenceType = expected_param.as_reference_type()?;
						let r2: ResolvedReferenceType = actual_param.as_reference_type()?;
						// in this case we want to verify expected parameter from the actual parameter ancestors
						return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters(r1, r2, matched_parameters)?;
					}
					if expected_param.is_array() && actual_param.is_array() {
						let r1: ResolvedType = expected_param.as_array_type()?.get_component_type();
						let r2: ResolvedType = actual_param.as_array_type()?.get_component_type();
						// try to verify the component type of each array
						return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters(r1, r2, matched_parameters)?;
					}
					if expected_param.is_type_variable() {
						let expected_param_name: String = expected_param.as_type_parameter()?.get_name();
						if !actual_param.is_type_variable() || !actual_param.as_type_parameter()?.get_name().equals(expected_param_name) {
							return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::match_type_variable(&expected_param.as_type_variable()?, actual_param, matched_parameters);
						}
						// We should definitely consider that types are assignable
						return true;
					} else if expected_param.is_reference_type() {
						if actual_param.is_type_variable() {
							return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::match_type_variable(&actual_param.as_type_variable()?, expected_param, matched_parameters);
						}
						if !expected_param.equals(actual_param) {
							return false;
						}
					}
					if expected_param.is_wildcard() {
						if expected_param.as_wildcard()?.is_extends() {
							// trying to compare with unbounded wildcard type parameter <?>
							if actual_param.is_wildcard() && !actual_param.as_wildcard()?.is_bounded() {
								return true;
							}
							if actual_param.is_type_variable() {
								return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::match_type_variable(&actual_param.as_type_variable()?, &expected_param.as_wildcard()?.get_bounded_type()?, matched_parameters);
							}
							return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_assignable_match_type_parameters(&expected_param.as_wildcard()?.get_bounded_type()?, actual_param, matched_parameters)?;
						}
						// TODO verify super bound
						return true;
					}
					return Err(UnsupportedOperationException::new(&expected_param.describe()));
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	fn match_type_variable(&self, type_variable: &com::github::javaparser::resolution::types::resolved_type_variable::ResolvedTypeVariable, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, matched_parameters: &/* Java */ java::util::Map /**/) -> bool {
		let type_parameter_name: String = type_variable.as_type_parameter().get_name();
		if matched_parameters.containsKey(type_parameter_name) {
			let matched_parameter: ResolvedType = matched_parameters.get(type_parameter_name);
			if matched_parameter.is_assignable_by(type) {
				return true;
			}
			if type.is_assignable_by(matched_parameter) {
				// update matchedParameters to contain the more general type
				matched_parameters.put(type_parameter_name, type);
				return true;
			}
			return false;
		} else {
			matched_parameters.put(type_parameter_name, type);
		}
		return true;
	}

	pub fn replace_type_param(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(java.lang.UnsupportedOperationException | com.github.javaparser.resolution.UnsolvedSymbolException) */ -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		if type.is_type_variable() || type.is_wildcard() {
			if type.describe().equals(&tp.get_name()) {
				let bounds: List<ResolvedTypeParameterDeclaration.Bound> = tp.get_bounds();
				if bounds.size() > 1 {
					return Err(UnsupportedOperationException::new());
				}
				if bounds.size() == 1 {
					return bounds.get(0).get_type();
				}
				return ReferenceTypeImpl::new(&type_solver.solve_type(self.JAVA_LANG_OBJECT)?);
			}
			return type;
		}
		if type.is_primitive() {
			return type;
		}
		if type.is_array() {
			return ResolvedArrayType::new(&com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::replace_type_param(&type.as_array_type()?.get_component_type(), tp, type_solver)?);
		}
		if type.is_reference_type() {
			let result: ResolvedReferenceType = type.as_reference_type()?;
			result = result.transform_type_parameters(|type_param|com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::replace_type_param(type_param, tp, type_solver)?).as_reference_type()?;
			return result;
		}
		return Err(UnsupportedOperationException::new("Replacing " + type + ", param " + tp + " with " + type.getClass().getCanonicalName()));
	}

	pub fn is_applicable(&self, mut method_usage: &com::github::javaparser::resolution::method_usage::MethodUsage, needle_name: &/* Java */ java::lang::String /**/, needle_parameter_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(java.lang.UnsupportedOperationException | com.github.javaparser.resolution.UnsolvedSymbolException) */ -> bool {
		if !method_usage.get_name().equals(needle_name) {
			return false;
		}
		// Before checking parameter compatibility, we need to substitute
		// type variables from the declaring type into the method signature.
		//
		// Context: When a method is inherited from a generic ancestor interface/class,
		// the method signature may contain type variables from that ancestor.
		// For example:
		// - Interface Iterable<T> declares: forEach(Consumer<? super T>)
		// - Interface List<E> extends Collection<E> which extends Iterable<E>
		// - When we retrieve forEach() from List<E>, the signature still references
		// Iterable's type variable 'T' instead of List's type variable 'E'
		//
		// This substitution ensures that:
		// - forEach(Consumer<? super T>) becomes forEach(Consumer<? super E>)
		// - When List<E> is instantiated as List<String>, it becomes forEach(Consumer<?
		// super String>)
		//
		// Without this substitution, type compatibility checks fail because we're
		// comparing the wrong type variables.
		method_usage = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_declaring_type_parameters(method_usage, type_solver);
		// The index of the final method parameter (on the method declaration).
		let count_of_method_usage_arguments_passed: i32 = method_usage.get_no_params();
		let last_method_usage_argument_index: i32 = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_last_parameter_index(count_of_method_usage_arguments_passed);
		// The index of the final argument passed (on the method usage).
		let needle_parameter_count: i32 = needle_parameter_types.size();
		// TODO: Does the method usage have a declaration at this point..?
		let method_is_declared_with_variadic_parameter: bool = method_usage.get_declaration().has_variadic_parameter();
		// If the counts do not match and the method is not variadic, this is not a match.
		if !method_is_declared_with_variadic_parameter && !(needle_parameter_count == count_of_method_usage_arguments_passed) {
			return false;
		}
		// is not a match.
		if !(needle_parameter_count == count_of_method_usage_arguments_passed) && needle_parameter_count < last_method_usage_argument_index {
			return false;
		}
		// parameter types
		 {
			let i: i32 = 0;
			while i < needle_parameter_count {
				{
					let actual_argument_type: ResolvedType = needle_parameter_types.get(i);
					let expected_argument_type: ResolvedType;
					let reached_variadic_param: bool = method_is_declared_with_variadic_parameter && i >= last_method_usage_argument_index;
					if !reached_variadic_param {
						// Not yet reached the variadic parameters -- the expected type is just whatever is at that position.
						expected_argument_type = method_usage.get_param_type(i);
					} else {
						// We have reached the variadic parameters -- the expected type is the type of the last declared
						// parameter.
						expected_argument_type = method_usage.get_param_type(last_method_usage_argument_index);
						// Note that the given variadic value might be an array - if so, use the array's component type rather.
						// This is only valid if ONE argument has been given to the vararg parameter.
						// Example: {@code void test(String... s) {}} and {@code test(stringArray)} -- {@code String... is
						// assignable by stringArray}
						// Example: {@code void test(String[]... s) {}} and {@code test(stringArrayArray)} -- {@code String[]...
						// is assignable by stringArrayArray}
						let argument_is_array: bool = (needle_parameter_count == count_of_method_usage_arguments_passed) && expected_argument_type.is_assignable_by(actual_argument_type);
						if !argument_is_array {
							// Get the component type of the declared parameter type.
							expected_argument_type = expected_argument_type.as_array_type()?.get_component_type();
						}
					}
					// Consider type parameters directly on the method declaration, and ALSO on the enclosing type (e.g. a
					// class)
					let type_parameters: List<ResolvedTypeParameterDeclaration> = method_usage.get_declaration().get_type_parameters();
					type_parameters.addAll(&method_usage.declaring_type().get_type_parameters());
					let expected_type_without_substitutions: ResolvedType = expected_argument_type;
					let expected_type_with_inference: ResolvedType = expected_argument_type;
					let derived_values: Map<ResolvedTypeParameterDeclaration, ResolvedType> = HashMap<>::new();
					// For each declared parameter, infer the types that will replace generics (type parameters)
					 {
						let j: i32 = 0;
						while j < count_of_method_usage_arguments_passed {
							{
								let parameter: ResolvedParameterDeclaration = method_usage.get_declaration().get_param(j);
								let parameter_type: ResolvedType = parameter.get_type();
								if parameter.is_variadic() {
									// Don't continue if a vararg parameter is reached and there are no arguments left
									if needle_parameter_count == j {
										break;
									}
									parameter_type = parameter_type.as_array_type()?.get_component_type();
								}
								com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&needle_parameter_types.get(j), parameter_type, derived_values);
							}
							j += 1;
						 }
					 }
	
					for entry in derived_values.entrySet() {
						let tp: ResolvedTypeParameterDeclaration = entry.getKey();
						expected_type_with_inference = expected_type_with_inference.replace_type_variables(tp, &entry.getValue());
					}
					// Consider cases where type variables can be replaced (e.g. add(E element) vs add(String element))
					for tp in type_parameters {
						if tp.get_bounds().isEmpty() {
							// expectedArgumentType = expectedArgumentType.replaceTypeVariables(tp.getName(), new
							// ReferenceTypeUsageImpl(typeSolver.solveType(JAVA_LANG_OBJECT), typeSolver));
							expected_argument_type = expected_argument_type.replace_type_variables(tp, &ResolvedWildcard::extends_bound(ReferenceTypeImpl::new(&type_solver.solve_type(self.JAVA_LANG_OBJECT)?)));
						} else if tp.get_bounds().size() == 1 {
							let bound: ResolvedTypeParameterDeclaration.Bound = tp.get_bounds().get(0);
							if bound.is_extends() {
								// expectedArgumentType = expectedArgumentType.replaceTypeVariables(tp.getName(),
								// bound.getType());
								expected_argument_type = expected_argument_type.replace_type_variables(tp, &ResolvedWildcard::extends_bound(&bound.get_type()));
							} else {
								// expectedArgumentType = expectedArgumentType.replaceTypeVariables(tp.getName(), new
								// ReferenceTypeUsageImpl(typeSolver.solveType(JAVA_LANG_OBJECT), typeSolver));
								expected_argument_type = expected_argument_type.replace_type_variables(tp, &ResolvedWildcard::super_bound(&bound.get_type()));
							}
						} else {
							return Err(UnsupportedOperationException::new());
						}
					}
					// Consider cases where type variables involve bounds e.g. super/extends
					let expected_type_with_substitutions: ResolvedType = expected_type_without_substitutions;
					for tp in type_parameters {
						if tp.get_bounds().isEmpty() {
							expected_type_with_substitutions = expected_type_with_substitutions.replace_type_variables(tp, ReferenceTypeImpl::new(&type_solver.solve_type(self.JAVA_LANG_OBJECT)?));
						} else if tp.get_bounds().size() == 1 {
							let bound: ResolvedTypeParameterDeclaration.Bound = tp.get_bounds().get(0);
							if bound.is_extends() {
								expected_type_with_substitutions = expected_type_with_substitutions.replace_type_variables(tp, &bound.get_type());
							} else {
								expected_type_with_substitutions = expected_type_with_substitutions.replace_type_variables(tp, ReferenceTypeImpl::new(&type_solver.solve_type(self.JAVA_LANG_OBJECT)?));
							}
						} else {
							return Err(UnsupportedOperationException::new());
						}
					}
					// If the given argument still isn't applicable even after considering type arguments/generics, this is not
					// a match.
					// Check if the given argument is applicable, considering:
					// 1. Direct type assignability
					// 2. Type substitutions with bounds
					// 3. Type inference
					// 4. Boxing/unboxing conversions (especially important for varargs)
					let is_applicable: bool = expected_argument_type.is_assignable_by(actual_argument_type) || expected_type_with_substitutions.is_assignable_by(actual_argument_type) || expected_type_with_inference.is_assignable_by(actual_argument_type) || expected_type_without_substitutions.is_assignable_by(actual_argument_type);
					//   Should succeed via unboxing conversion
					if !is_applicable {
						// Check boxing/unboxing compatibility with all type variations
						is_applicable = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_argument_type, actual_argument_type, type_solver) || com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_type_with_substitutions, actual_argument_type, type_solver) || com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_type_with_inference, actual_argument_type, type_solver) || com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_type_without_substitutions, actual_argument_type, type_solver);
					}
					if !is_applicable {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		// If the checks above haven't failed, then we've found a match.
		return true;
	}

	fn is_boxing_compatible_with_type_solver(&self, expected_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, actual_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ -> bool {
		// Handle null types
		if expected_type == null || actual_type == null {
			return false;
		}
		// Handle wildcard types (e.g., ? extends Number, ? super Integer)
		if expected_type.is_wildcard() {
			let wildcard: ResolvedWildcard = expected_type.as_wildcard()?;
			if wildcard.is_bounded() {
				// Check compatibility with the wildcard bound
				return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(&wildcard.get_bounded_type()?, actual_type, type_solver)?;
			}
			// Unbounded wildcard (?) - can accept anything via boxing
			return actual_type.is_primitive();
		}
		// Handle array types (for variadic parameters)
		if expected_type.is_array() && actual_type.is_array() {
			let expected_component: ResolvedType = expected_type.as_array_type()?.get_component_type();
			let actual_component: ResolvedType = actual_type.as_array_type()?.get_component_type();
			// Check if component types are boxing compatible
			return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_component, actual_component, type_solver)?;
		}
		// Boxing (reference type expected, primitive provided)
		if expected_type.is_reference_type() && actual_type.is_primitive() {
			let expected_ref: ResolvedReferenceType = expected_type.as_reference_type()?;
			let primitive: ResolvedPrimitiveType = actual_type.as_primitive()?;
			// Get boxed type for the primitive (e.g., Integer for int)
			let boxed_type_q_name: String = primitive.get_box_typeq_name();
			let r0 = 'try0: {
				// Resolve the boxed type using TypeSolver
				let boxed_type_decl: ResolvedReferenceTypeDeclaration = match type_solver.solve_type(boxed_type_q_name) {
					Err(e) => break 'try0 Err(e),
					Ok(s) => s,
				};
				let boxed_type: ResolvedReferenceType = ReferenceTypeImpl::new(boxed_type_decl);
				// Example: Integer is assignable to Number
				return expected_ref.is_assignable_by(boxed_type);
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Exception) => {
					// If we can't resolve the type, try a fallback check
					return false;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		// Unboxing (primitive expected, reference type provided)
		if expected_type.is_primitive() && actual_type.is_reference_type() {
			let expected_primitive: ResolvedPrimitiveType = match expected_type.as_primitive() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			let actual_ref: ResolvedReferenceType = match actual_type.as_reference_type() {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			// Check if actual type is a direct box type for the expected primitive
			if ResolvedPrimitiveType::is_box_type(actual_ref) {
				let unboxed: Optional<ResolvedType> = ResolvedPrimitiveType::by_box_typeq_name(&actual_ref.get_qualified_name());
				return unboxed.isPresent() && unboxed.get().equals(expected_primitive);
			}
			// For other reference types, try to see if they can be assigned to the boxed type
			let expected_boxed_type_q_name: String = expected_primitive.get_box_typeq_name();
			let r1 = 'try1: {
				let expected_boxed_type_decl: ResolvedReferenceTypeDeclaration = match type_solver.solve_type(expected_boxed_type_q_name) {
					Err(e) => break 'try1 Err(e),
					Ok(s) => s,
				};
				let expected_boxed_type: ResolvedReferenceType = ReferenceTypeImpl::new(expected_boxed_type_decl);
				// Check if actual type is assignable to the expected boxed type
				if expected_boxed_type.is_assignable_by(actual_ref) {
					// Then we can unbox
					return true;
				}
				break 'try1 Ok(());
			};
			match r1 {
				Err(e @ Exception) => {
					return false;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		// Unboxing (primitive expected, reference type provided)
		if expected_type.is_primitive() && actual_type.is_primitive() {
			let expected_primitive: ResolvedPrimitiveType = match expected_type.as_primitive() {
				Err(e) => break 'try1 Err(e),
				Ok(s) => s,
			};
			let actual_primitive: ResolvedPrimitiveType = match actual_type.as_primitive() {
				Err(e) => break 'try1 Err(e),
				Ok(s) => s,
			};
			return expected_primitive.is_assignable_by(actual_primitive);
		}
		// This can happen after type variable substitution
		if expected_type.is_reference_type() && actual_type.is_reference_type() {
			// Let the main isApplicable logic handle this
			return false;
		}
		// Constraint types (e.g., LambdaConstraintType)
		if actual_type.is_constraint() {
			// Check compatibility with the constraint bound
			return match com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_boxing_compatible_with_type_solver(expected_type, &match actual_type.as_constraint_type() {
				Err(e) => break 'try1 Err(e),
				Ok(s) => s,
			}.get_bound(), type_solver) {
				Err(e) => break 'try1 Err(e),
				Ok(s) => s,
			};
		}
		return false;
	}

	fn substitute_declaring_type_parameters(&self, mut method_usage: &com::github::javaparser::resolution::method_usage::MethodUsage, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::method_usage::MethodUsage {
		// Get the declaring type declaration from the method usage
		// Note: methodUsage.declaringType() returns a ResolvedReferenceTypeDeclaration
		// which is the type declaration without specific type arguments
		let declaring_type_declaration: ResolvedReferenceTypeDeclaration = method_usage.declaring_type();
		// Build a ResolvedReferenceType from the declaration with its type parameters
		// For a generic type like List<E>, this creates a reference to List with E as type variable
		let declaring_type: ResolvedReferenceType = ReferenceTypeImpl::undetermined_parameters(declaring_type_declaration);
		// Get the type parameter declarations from the declaring type
		// For List<E>, this gets the declaration of E
		let type_params: List<ResolvedTypeParameterDeclaration> = declaring_type_declaration.get_type_parameters();
		// Non-generic types (like String, Integer) don't need any substitution
		if !type_params.isEmpty() {
			// Get the type variables as ResolvedTypes for substitution
			// For List<E>, this creates a list containing [E as ResolvedTypeVariable]
			let type_args: List<ResolvedType> = declaring_type.type_parameters_values();
			// Verify that we have matching counts of parameters and arguments
			if type_params.size() == type_args.size() {
				// Substitute type variables in each method parameter
				 {
					let i: i32 = 0;
					while i < method_usage.get_no_params() {
						{
							let param_type: ResolvedType = method_usage.get_param_type(i);
							// Recursively substitute type variables throughout the parameter type structure
							// This handles nested generics like Consumer<? super T>, List<List<T>>, etc.
							let substituted_type: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_type_variables(param_type, type_params, type_args);
							// Only update the method usage if the type actually changed
							if !substituted_type.equals(param_type) {
								method_usage = method_usage.replace_param_type(i, substituted_type)?;
							}
						}
						i += 1;
					 }
				 }
	
				// Substitute type variables in the return type
				let return_type: ResolvedType = method_usage.return_type();
				let substituted_return_type: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_type_variables(return_type, type_params, type_args);
				if !substituted_return_type.equals(return_type) {
					method_usage = method_usage.replace_return_type(substituted_return_type);
				}
			}
		}
		return method_usage;
	}

	fn substitute_type_variables(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, type_params: &/* Java */ java::util::List /**/, type_args: &/* Java */ java::util::List /**/) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		// Replace it with the corresponding type argument from the declaring type
		if type.is_type_variable() {
			let var_name: String = type.as_type_variable()?.as_type_parameter().get_name();
			// Search for this type variable in the declaring type's parameters
			 {
				let j: i32 = 0;
				while j < type_params.size() {
					{
						if type_params.get(j).get_name().equals(var_name) {
							// For example: if T maps to String, return String
							return type_args.get(j);
						}
					}
					j += 1;
				 }
			 }
	
		// If no match found, the type variable is from a different scope
		// (e.g., method type parameter, not class type parameter)
		// Return it unchanged
		}
		// Need to recursively substitute type variables in the wildcard's bound
		if type.is_wildcard() {
			let wildcard: ResolvedWildcard = type.as_wildcard()?;
			// Unbounded wildcards (?) don't need substitution
			if wildcard.is_bounded() {
				let bounded_type: ResolvedType = wildcard.get_bounded_type()?;
				// Recursively substitute within the bound
				// For example: ? super T -> ? super String
				let substituted_bound: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_type_variables(bounded_type, type_params, type_args);
				// If the bound changed, create a new wildcard with the substituted bound
				if !substituted_bound.equals(bounded_type) {
					if wildcard.is_super() {
						// Preserve the super bound: ? super T -> ? super String
						return ResolvedWildcard::super_bound(substituted_bound);
					} else {
						// Preserve the extends bound: ? extends T -> ? extends String
						return ResolvedWildcard::extends_bound(substituted_bound);
					}
				}
			}
		}
		// Need to recursively substitute type variables in all type arguments
		if type.is_reference_type() {
			let ref_type: ResolvedReferenceType = type.as_reference_type()?;
			let original_type_params: List<ResolvedType> = ref_type.type_parameters_values();
			let substituted_type_params: List<ResolvedType> = ArrayList<>::new();
			let changed: bool = false;
			// For example, in Map<K, V>, process both K and V
			for type_param in original_type_params {
				// Recursively substitute within each type argument
				// This handles nested cases like List<List<T>>
				let substituted: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::substitute_type_variables(type_param, type_params, type_args);
				substituted_type_params.add(substituted);
				// Track if any substitution actually occurred
				if !substituted.equals(type_param) {
					changed = true;
				}
			}
			// with the substituted type arguments
			if changed && ref_type.get_type_declaration().isPresent() {
				return ReferenceTypeImpl::new(&ref_type.get_type_declaration().get(), substituted_type_params);
			}
		}
		// - Array types (could be enhanced to handle T[] -> String[] if needed)
		return type;
	}

	fn distinct_by_key<T>(&self, key_extractor: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::function::Predicate /**/ {
		let seen: Set<Object> = ConcurrentHashMap::newKeySet();
		return |t|seen.add(&key_extractor.apply(t));
	}

	pub fn find_most_applicable(&self, methods: &/* Java */ java::util::List /**/, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(com.github.javaparser.resolution.MethodAmbiguityException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		let res: SymbolReference<ResolvedMethodDeclaration> = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::find_most_applicable(methods, name, arguments_types, type_solver, false)?;
		if res.is_solved() {
			return res;
		}
		return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::find_most_applicable(methods, name, arguments_types, type_solver, true)?;
	}

	pub fn find_most_applicable(&self, methods: &/* Java */ java::util::List /**/, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver, wildcard_tolerance: bool) /* thrown(com.github.javaparser.resolution.MethodAmbiguityException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		// Only consider methods with a matching name
		// Filters out duplicate ResolvedMethodDeclaration by their signature.
		// Checks if ResolvedMethodDeclaration is applicable to argumentsTypes.
		let applicable_methods: List<ResolvedMethodDeclaration> = methods.stream().filter(|m|m.get_name().equals(name)).filter(&com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::distinct_by_key(ResolvedMethodDeclaration::getQualifiedSignature)).filter(|(m)|com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_applicable(m, name, arguments_types, type_solver, wildcard_tolerance)?).collect(&Collectors::toList());
		// If no applicable methods found, return as unsolved.
		if applicable_methods.isEmpty() {
			return SymbolReference::unsolved();
		}
		// If there are multiple possible methods found, null arguments can help to eliminate some matches.
		if applicable_methods.size() > 1 {
			let null_param_indexes: List<Integer> = ArrayList<>::new();
			 {
				let i: i32 = 0;
				while i < arguments_types.size() {
					{
						if arguments_types.get(i).is_null() {
							null_param_indexes.add(i);
						}
					}
					i += 1;
				 }
			 }
	
			// If some null arguments have been provided, use this to eliminate some opitons.
			if !null_param_indexes.isEmpty() {
				// filter method with array param if a non array exists and arg is null
				// For example, if we define 2 methods
				// {@code void get(String str0, Object ... objects)}
				// {@code void get(String str0, String str1, Object ... objects)}
				// and want to determine the most specific method invoked by this expression
				// {@code foo.get("", null , new Object());}
				let remove_candidates: Set<ResolvedMethodDeclaration> = HashSet<>::new();
				for null_param_index in null_param_indexes {
					for meth_decl in applicable_methods {
						// methDecl.getParam(nullParamIndex).getType().isArray()) {
						if meth_decl.get_param(null_param_index).get_type().is_array() {
							remove_candidates.add(meth_decl);
						}
					}
				}
				// Where candidiates for removal are found, remove them.
				if !remove_candidates.isEmpty() && remove_candidates.size() < applicable_methods.size() {
					applicable_methods.removeAll(remove_candidates);
				}
			}
		}
		// If only one applicable method found, short-circuit and return it here.
		if applicable_methods.size() == 1 {
			return SymbolReference::solved(&applicable_methods.get(0));
		}
		// Examine the applicable methods found, and evaluate each to determine the "best" one
		let winning_candidate: ResolvedMethodDeclaration = applicable_methods.get(0);
		let other: ResolvedMethodDeclaration = null;
		let possible_ambiguity: bool = false;
		 {
			let i: i32 = 1;
			while i < applicable_methods.size() {
				{
					other = applicable_methods.get(i);
					if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_more_specific(winning_candidate, other, arguments_types) {
						possible_ambiguity = false;
					} else if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_more_specific(other, winning_candidate, arguments_types) {
						possible_ambiguity = false;
						winning_candidate = other;
					} else {
						// ...
						if winning_candidate.is_generic() && !other.is_generic() {
							winning_candidate = other;
						} else if !winning_candidate.is_generic() && other.is_generic() {
						// nothing to do at this stage winningCandidate is the winner
						} else if winning_candidate.declaring_type().get_qualified_name().equals(&other.declaring_type().get_qualified_name()) {
							possible_ambiguity = true;
						} else {
						// we expect the methods to be ordered such that inherited methods are later in the list
						}
					}
				}
				i += 1;
			 }
		 }
	
		if possible_ambiguity {
			// pick the first exact match if it exists
			if !com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_exact_match(winning_candidate, arguments_types) {
				if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_exact_match(other, arguments_types) {
					winning_candidate = other;
				} else {
					return Err(MethodAmbiguityException::new("Ambiguous method call: cannot find a most applicable method: " + winning_candidate + ", " + other));
				}
			}
		}
		return SymbolReference::solved(winning_candidate);
	}

	fn is_exact_match(&self, method: &com::github::javaparser::resolution::declarations::resolved_method_like_declaration::ResolvedMethodLikeDeclaration, arguments_types: &/* Java */ java::util::List /**/) -> bool {
		 {
			let i: i32 = 0;
			while i < method.get_number_of_params() {
				{
					let param_type: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_methods_explicit_and_variadic_parameter_type(method, i);
					if param_type == null {
						return false;
					}
					if i >= arguments_types.size() {
						return false;
					}
					if !param_type.equals(&arguments_types.get(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn get_methods_explicit_and_variadic_parameter_type(&self, method: &com::github::javaparser::resolution::declarations::resolved_method_like_declaration::ResolvedMethodLikeDeclaration, i: i32) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let number_of_params: i32 = method.get_number_of_params();
		if i < number_of_params {
			return method.get_param(i).get_type();
		}
		if method.has_variadic_parameter() {
			return method.get_param(number_of_params - 1).get_type();
		}
		return null;
	}

	pub fn get_method_usage_explicit_and_variadic_parameter_type(&self, method: &com::github::javaparser::resolution::method_usage::MethodUsage, i: i32) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let number_of_params: i32 = method.get_no_params();
		if i < number_of_params {
			return method.get_param_type(i);
		}
		if method.get_declaration().has_variadic_parameter() {
			return method.get_param_type(number_of_params - 1);
		}
		return null;
	}

	fn is_more_specific(&self, methoda: &com::github::javaparser::resolution::declarations::resolved_method_like_declaration::ResolvedMethodLikeDeclaration, methodb: &com::github::javaparser::resolution::declarations::resolved_method_like_declaration::ResolvedMethodLikeDeclaration, argument_types: &/* Java */ java::util::List /**/) -> bool {
		/* final */ let a_variadic: bool = method_a.has_variadic_parameter();
		/* final */ let b_variadic: bool = method_b.has_variadic_parameter();
		/* final */ let a_number_of_params: i32 = method_a.get_number_of_params();
		/* final */ let b_number_of_params: i32 = method_b.get_number_of_params();
		/* final */ let number_of_args: i32 = argument_types.size();
		/* final */ let last_arg_type: ResolvedType =  if number_of_args > 0 { argument_types.get(number_of_args - 1) } else { null };
		/* final */ let is_last_arg_array: bool = last_arg_type != null && last_arg_type.is_array();
		let omitted_args: i32 = 0;
		let is_method_a_more_specific: bool = false;
		// preferred to a declaration that is variadic (and hence possibly also has a different amount of parameters).
		if !a_variadic && a_number_of_params == number_of_args && (b_variadic && (b_number_of_params != number_of_args || !is_last_arg_array)) {
			return true;
		}
		if !b_variadic && b_number_of_params == number_of_args && (a_variadic && (a_number_of_params != number_of_args || !is_last_arg_array)) {
			return false;
		}
		// ensure the varargs type is considered when determining which method is more specific
		if a_variadic && b_variadic && a_number_of_params == b_number_of_params && number_of_args == a_number_of_params - 1 {
			omitted_args += 1;
		}
		// Either both methods are variadic or neither is. So we must compare the parameter types.
		 {
			let i: i32 = 0;
			while i < number_of_args + omitted_args {
				{
					let param_type_a: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_methods_explicit_and_variadic_parameter_type(method_a, i);
					let param_type_b: ResolvedType = com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::get_methods_explicit_and_variadic_parameter_type(method_b, i);
					let arg_type: ResolvedType = null;
					if i < argument_types.size() {
						arg_type = argument_types.get(i);
					}
					// This should not happen but it also means that this signature is immediately disqualified.
					if param_type_a == null {
						return false;
					}
					if param_type_b == null {
						return true;
					}
					// This is what we check here.
					if arg_type != null && param_type_a.is_primitive() == arg_type.is_primitive() && param_type_b.is_primitive() != arg_type.is_primitive() && param_type_a.is_assignable_by(arg_type) {
						return true;
					}
					if arg_type != null && param_type_b.is_primitive() == arg_type.is_primitive() && param_type_a.is_primitive() != arg_type.is_primitive() && param_type_b.is_assignable_by(arg_type) {
						return false;
					// if paramA and paramB are not the last parameters
					// and the type of paramA or paramB (which are not more specific at this stage) is java.lang.Object
					// then we have to consider others parameters before concluding
					}
					if (i < number_of_args - 1) && (com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_java_lang_object(param_type_b) || (com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_java_lang_object(param_type_a))) {
						// consider others parameters
						// but eventually mark the method A as more specific if the methodB has an argument of type
						// java.lang.Object
						is_method_a_more_specific = is_method_a_more_specific || com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_java_lang_object(param_type_b);
					} else {
						// If we get to this point then we check whether one of the methods contains a parameter type that is
						// more specific. If it does, we can assume the entire declaration is more specific as we would
						// otherwise have a situation where the declarations are ambiguous in the given context.
						// Note: This does not account for the case where one parameter is variadic (and therefore an array
						// type) and the other is not, since these will never be assignable by each other. This case is checked
						// below.
						let a_assignable_from_b: bool = param_type_a.is_assignable_by(param_type_b);
						let b_assignable_from_a: bool = param_type_b.is_assignable_by(param_type_a);
						if b_assignable_from_a && !a_assignable_from_b {
							// A's parameter is more specific
							return true;
						}
						if a_assignable_from_b && !b_assignable_from_a {
							// B's parameter is more specific
							return false;
						}
					}
					// a check in case this changes in the future.
					if method_a.get_number_of_params() > i && method_b.get_number_of_params() > i {
						let param_a_variadic: bool = method_a.get_param(i).is_variadic();
						let param_b_variadic: bool = method_b.get_param(i).is_variadic();
						// foo(String s, Object... o) is preferred over foo(Object... o)
						if !param_a_variadic && param_b_variadic {
							return true;
						}
					}
				}
				i += 1;
			 }
		 }
	
		if a_variadic && !b_variadic {
			// if the last argument is an array then m1 is more specific
			return is_last_arg_array;
		}
		if !a_variadic && b_variadic {
			// it is not more specific
			return !is_last_arg_array;
		}
		return is_method_a_more_specific;
	}

	fn is_java_lang_object(&self, param_type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		return param_type.is_reference_type() && param_type.as_reference_type()?.get_qualified_name().equals("java.lang.Object");
	}

	fn is_more_specific(&self, methoda: &com::github::javaparser::resolution::method_usage::MethodUsage, methodb: &com::github::javaparser::resolution::method_usage::MethodUsage, argument_types: &/* Java */ java::util::List /**/) -> bool {
		return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_more_specific(&method_a.get_declaration(), &method_b.get_declaration(), argument_types);
	}

	pub fn find_most_applicable_usage(&self, methods: &/* Java */ java::util::List /**/, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/, type_solver: &com::github::javaparser::resolution::type_solver::TypeSolver) /* thrown(com.github.javaparser.resolution.MethodAmbiguityException) */ -> /* Java */ java::util::Optional /**/ {
		let applicable_methods: List<MethodUsage> = methods.stream().filter(|(m)|com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_applicable(m, name, arguments_types, type_solver)?).collect(&Collectors::toList());
		if applicable_methods.isEmpty() {
			return Optional::empty();
		}
		if applicable_methods.size() == 1 {
			return Optional::of(&applicable_methods.get(0));
		}
		let winning_candidate: MethodUsage = applicable_methods.get(0);
		 {
			let i: i32 = 1;
			while i < applicable_methods.size() {
				{
					let other: MethodUsage = applicable_methods.get(i);
					if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_more_specific(winning_candidate, other, arguments_types) {
					// nothing to do
					} else if com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::is_more_specific(other, winning_candidate, arguments_types) {
						winning_candidate = other;
					} else {
						if winning_candidate.declaring_type().get_qualified_name().equals(&other.declaring_type().get_qualified_name()) {
							if !com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::are_override(winning_candidate, other) {
								return Err(MethodAmbiguityException::new("Ambiguous method call: cannot find a most applicable method: " + winning_candidate + ", " + other + ". First declared in " + winning_candidate.declaring_type().get_qualified_name()));
							}
						} else {
						// we expect the methods to be ordered such that inherited methods are later in the list
						// throw new UnsupportedOperationException();
						}
					}
				}
				i += 1;
			 }
		 }
	
		return Optional::of(winning_candidate);
	}

	fn are_override(&self, winning_candidate: &com::github::javaparser::resolution::method_usage::MethodUsage, other: &com::github::javaparser::resolution::method_usage::MethodUsage) -> bool {
		if !winning_candidate.get_name().equals(&other.get_name()) {
			return false;
		}
		if winning_candidate.get_no_params() != other.get_no_params() {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < winning_candidate.get_no_params() {
				{
					if !winning_candidate.get_param_types().get(i).equals(&other.get_param_types().get(i)) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn solve_method_in_type(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_type_declaration::ResolvedTypeDeclaration, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		return com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::solve_method_in_type(type_declaration, name, arguments_types, false)?;
	}

	pub fn solve_method_in_type(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_type_declaration::ResolvedTypeDeclaration, name: &/* Java */ java::lang::String /**/, arguments_types: &/* Java */ java::util::List /**/, static_only: bool) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::resolution::model::symbol_reference::SymbolReference {
		if type_declaration instanceof MethodResolutionCapability {
			return (type_declaration as MethodResolutionCapability).solve_method(name, arguments_types, static_only);
		}
		return Err(UnsupportedOperationException::new(&type_declaration.getClass().getCanonicalName()));
	}

	pub fn infer_types(&self, source: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, target: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, mappings: &/* Java */ java::util::Map /**/) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ {
		if source.equals(target) {
			return;
		}
		if source.is_reference_type() && target.is_reference_type() {
			let source_ref_type: ResolvedReferenceType = source.as_reference_type()?;
			let target_ref_type: ResolvedReferenceType = target.as_reference_type()?;
			if source_ref_type.get_qualified_name().equals(&target_ref_type.get_qualified_name()) {
				if !source_ref_type.is_raw_type() && !target_ref_type.is_raw_type() {
					 {
						let i: i32 = 0;
						while i < source_ref_type.type_parameters_values().size() {
							{
								com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&source_ref_type.type_parameters_values().get(i), &target_ref_type.type_parameters_values().get(i), mappings)?;
							}
							i += 1;
						 }
					 }
	
				}
			}
			return;
		}
		if source.is_reference_type() && target.is_wildcard() {
			if target.as_wildcard()?.is_bounded() {
				com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(source, &target.as_wildcard()?.get_bounded_type()?, mappings)?;
				return;
			}
			return;
		}
		if source.is_wildcard() && target.is_wildcard() {
			if source.as_wildcard()?.is_bounded() && target.as_wildcard()?.is_bounded() {
				com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&source.as_wildcard()?.get_bounded_type()?, &target.as_wildcard()?.get_bounded_type()?, mappings)?;
			}
			return;
		}
		if source.is_reference_type() && target.is_type_variable() {
			mappings.put(&target.as_type_parameter()?, source);
			return;
		}
		if source.is_wildcard() && target.is_type_variable() {
			mappings.put(&target.as_type_parameter()?, source);
			return;
		}
		if source.is_array() && target.is_array() {
			let source_component_type: ResolvedType = source.as_array_type()?.get_component_type();
			let target_component_type: ResolvedType = target.as_array_type()?.get_component_type();
			com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(source_component_type, target_component_type, mappings)?;
			return;
		}
		if source.is_array() && target.is_wildcard() {
			if target.as_wildcard()?.is_bounded() {
				com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(source, &target.as_wildcard()?.get_bounded_type()?, mappings)?;
				return;
			}
			return;
		}
		if source.is_array() && target.is_type_variable() {
			mappings.put(&target.as_type_parameter()?, source);
			return;
		}
		if source.is_wildcard() && target.is_reference_type() {
			if source.as_wildcard()?.is_bounded() {
				com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&source.as_wildcard()?.get_bounded_type()?, target, mappings)?;
			}
			return;
		}
		if source.is_constraint() && target.is_reference_type() {
			com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&source.as_constraint_type()?.get_bound(), target, mappings)?;
			return;
		}
		if source.is_constraint() && target.is_type_variable() {
			com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(&source.as_constraint_type()?.get_bound(), target, mappings)?;
			return;
		}
		if source.is_type_variable() && target.is_type_variable() {
			mappings.put(&target.as_type_parameter()?, source);
			return;
		}
		if source.is_type_variable() {
			com::github::javaparser::resolution::logic::method_resolution_logic::MethodResolutionLogic::infer_types(target, source, mappings)?;
			return;
		}
		if source.is_primitive() || target.is_primitive() {
			return;
		}
		if source.is_null() {
			return;
		}
		if target.is_reference_type() {
			let formal_type_as_reference: ResolvedReferenceType = target.as_reference_type()?;
			if formal_type_as_reference.is_java_lang_object() {
				return;
			}
		}
	}
}