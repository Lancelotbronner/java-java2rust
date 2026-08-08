use java::lang::annotation::Annotation;
use java::lang::reflect::Array;
use java::lang::reflect::Executable;
use java::lang::reflect::InvocationTargetException;
use java::lang::reflect::Method;
use java::lang::reflect::Type;
use java::lang::reflect::TypeVariable;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::Collections;
use java::util::Comparator;
use java::util::Iterator;
use java::util::LinkedHashSet;
use java::util::List;
use java::util::Map;
use java::util::Objects;
use java::util::Set;
use java::util::TreeMap;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::ClassUtils::Interfaces;
use crate::org::apache::commons::lang3::Validate;
use crate::org::apache::commons::lang3::stream::LangCollectors;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct MethodUtils;

impl MethodUtils {
	static METHOD_BY_SIGNATURE: /* Java */ java::util::Comparator /**/ = Comparator::comparing(Method::toString);

	fn distance(&self, from_class_array: &&[/* Java */ java::lang::Class /**/], to_class_array: &&[/* Java */ java::lang::Class /**/]) -> i32 {
		let answer: i32 = 0;
		if !ClassUtils::is_assignable(from_class_array, to_class_array, true) {
			return -1;
		}
		 {
			let offset: i32 = 0;
			while offset < from_class_array.length {
				{
					// Note InheritanceUtils.distance() uses different scoring system.
					/* final */ let a_class: Class<?> = from_class_array[offset];
					/* final */ let to_class: Class<?> = to_class_array[offset];
					if a_class == null || a_class.equals(to_class) {
						continue;
					}
					if ClassUtils::is_assignable(a_class, to_class, true) && !ClassUtils::is_assignable(a_class, to_class, false) {
						answer += 1;
					} else {
						answer += 2;
					}
				}
				offset += 1;
			 }
		 }
	
		return answer;
	}

	pub fn get_accessible_method(&self, cls: &/* Java */ java::lang::Class /**/, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		if !MemberUtils::is_public(method) {
			return null;
		}
		// If the declaring class is public, we are done
		if ClassUtils::is_public(cls) {
			return method;
		}
		/* final */ let method_name: String = method.getName();
		/* final */ let parameter_types: Vec<Class<?>> = method.getParameterTypes();
		// Check the implemented interfaces and subinterfaces
		/* final */ let method2: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method_from_interface_nest(cls, method_name, parameter_types);
		// Check the superclass chain
		return  if method2 != null { method2 } else { org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method_from_superclass(cls, method_name, parameter_types) };
	}

	pub fn get_accessible_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method(&org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_method_object(cls, method_name, parameter_types));
	}

	pub fn get_accessible_method(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		return  if method != null { org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method(&method.getDeclaringClass(), method) } else { null };
	}

	fn get_accessible_method_from_interface_nest(&self, mut cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		// Search up the superclass chain
		while cls != null {
			{
				// Check the implemented interfaces of the parent class
				/* final */ let interfaces: Vec<Class<?>> = cls.getInterfaces();
				for /* final */ an_interface in interfaces {
					// Is this interface public?
					if !ClassUtils::is_public(an_interface) {
						continue;
					}
					// Does the method exist on this interface?
					let r0 = 'try0: {
						return an_interface.getDeclaredMethod(method_name, parameter_types);
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ NoSuchMethodException) => {
						/* 
	                     * Swallow, if no method is found after the loop then this method returns null.
	                     */ 
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
					// Recursively check our parent interfaces
					/* final */ let method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method_from_interface_nest(an_interface, method_name, parameter_types);
					if method != null {
						return method;
					}
				}
			}
			cls = cls.getSuperclass();
		 }
	
		return null;
	}

	fn get_accessible_method_from_superclass(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		let parent_class: Class<?> = cls.getSuperclass();
		while parent_class != null {
			if ClassUtils::is_public(parent_class) {
				return org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_method_object(parent_class, method_name, parameter_types);
			}
			parent_class = parent_class.getSuperclass();
		}
		return null;
	}

	fn get_all_superclasses_and_interfaces(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		if cls == null {
			return null;
		}
		/* final */ let all_super_classes_and_interfaces: List<Class<?>> = ArrayList<>::new();
		/* final */ let all_superclasses: List<Class<?>> = ClassUtils::get_all_superclasses(cls);
		let super_class_index: i32 = 0;
		/* final */ let all_interfaces: List<Class<?>> = ClassUtils::get_all_interfaces(cls);
		let interface_index: i32 = 0;
		while interface_index < all_interfaces.size() || super_class_index < all_superclasses.size() {
			/* final */ let acls: Class<?>;
			if interface_index >= all_interfaces.size() || super_class_index < all_superclasses.size() && super_class_index < interface_index {
				acls = all_superclasses.get(super_class_index += 1 !!!check!!! post increment);
			} else {
				acls = all_interfaces.get(interface_index += 1 !!!check!!! post increment);
			}
			all_super_classes_and_interfaces.add(acls);
		}
		return all_super_classes_and_interfaces;
	}

	pub fn get_annotation<A: /* Java */ java::lang::annotation::Annotation /**/>(&self, method: &/* Java */ java::lang::reflect::Method /**/, annotation_cls: &/* Java */ java::lang::Class /**/, search_supers: bool, ignore_access: bool) -> A {
		Objects::requireNonNull(method, "method");
		Objects::requireNonNull(annotation_cls, "annotationCls");
		if !ignore_access && !MemberUtils::is_accessible(method) {
			return null;
		}
		let annotation: A = method.getAnnotation(annotation_cls);
		if annotation == null && search_supers {
			/* final */ let mcls: Class<?> = method.getDeclaringClass();
			/* final */ let classes: List<Class<?>> = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_all_superclasses_and_interfaces(mcls);
			for /* final */ acls in classes {
				/* final */ let equivalent_method: Method =  if ignore_access { org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_method(acls, &method.getName(), &method.getParameterTypes()) } else { org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_accessible_method(acls, &method.getName(), &method.getParameterTypes()) };
				if equivalent_method != null {
					annotation = equivalent_method.getAnnotation(annotation_cls);
					if annotation != null {
						break;
					}
				}
			}
		}
		return annotation;
	}

	fn get_invoke_method(&self, force_access: bool, method_name: &/* Java */ java::lang::String /**/, parameter_types: &&[/* Java */ java::lang::Class /**/], cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		/* final */ let method: Method;
		if force_access {
			method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_method(cls, method_name, parameter_types);
			AccessibleObjects::set_accessible(method);
		} else {
			method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_accessible_method(cls, method_name, parameter_types);
		}
		return method;
	}

	pub fn get_matching_accessible_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, request_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		/* final */ let candidate: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_method_object(cls, method_name, request_types);
		if candidate != null {
			return MemberUtils::set_accessible_workaround(candidate);
		}
		// search through all methods
		/* final */ let methods: Vec<Method> = cls.getMethods();
		/* final */ let matching_methods: List<Method> = Stream::of(methods).filter(|method|method.getName().equals(method_name) && MemberUtils::is_matching_method(method, request_types)).collect(&Collectors::toList());
		// Sort methods by signature to force deterministic result
		matching_methods.sort(self.METHOD_BY_SIGNATURE);
		let best_match: Method = null;
		for /* final */ method in matching_methods {
			// get accessible version of method
			/* final */ let accessible_method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method(method);
			if accessible_method != null && (best_match == null || MemberUtils::compare_method_fit(accessible_method, best_match, request_types) < 0) {
				best_match = accessible_method;
			}
		}
		if best_match != null {
			MemberUtils::set_accessible_workaround(best_match);
			if best_match.isVarArgs() {
				/* final */ let best_match_parameter_types: Vec<Class<?>> = best_match.getParameterTypes();
				/* final */ let var_arg_type: Class<?> = best_match_parameter_types[best_match_parameter_types.length - 1].getComponentType();
				 {
					let param_idx: i32 = best_match_parameter_types.length - 1;
					while param_idx < request_types.length {
						{
							/* final */ let parameter_type: Class<?> = request_types[param_idx];
							if !ClassUtils::is_assignable(parameter_type, var_arg_type, true) {
								return null;
							}
						}
						param_idx += 1;
					 }
				 }
	
			}
		}
		return best_match;
	}

	pub fn get_matching_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::Method /**/ {
		Objects::requireNonNull(cls, "cls");
		Validate::not_empty(method_name, "methodName")?;
		/* final */ let methods: List<Method> = Stream::of(&cls.getDeclaredMethods()).filter(|method|method.getName().equals(method_name)).collect(&Collectors::toList());
		/* final */ let all_superclasses_and_interfaces: List<Class<?>> = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_all_superclasses_and_interfaces(cls);
		Collections::reverse(all_superclasses_and_interfaces);
		all_superclasses_and_interfaces.stream().map(Class::getDeclaredMethods).flatMap(Stream::of).filter(|method|method.getName().equals(method_name)).forEach(methods::add);
		for /* final */ method in methods {
			if Arrays::deepEquals(&method.getParameterTypes(), parameter_types) {
				return method;
			}
		}
		/* final */ let candidates: TreeMap<Integer, List<Method>> = TreeMap<>::new();
		methods.stream().filter(|method|ClassUtils::is_assignable(parameter_types, &method.getParameterTypes(), true)).forEach(|method|{
			/* final */ let distance: i32 = org::apache::commons::lang3::reflect::method_utils::MethodUtils::distance(parameter_types, &method.getParameterTypes());
			/* final */ let candidates_at_distance: List<Method> = candidates.computeIfAbsent(distance, |k|ArrayList<>::new());
			candidates_at_distance.add(method);
		});
		if candidates.isEmpty() {
			return null;
		}
		/* final */ let best_candidates: List<Method> = candidates.values().iterator().next();
		if best_candidates.size() == 1 || !Objects::equals(&best_candidates.get(0).getDeclaringClass(), &best_candidates.get(1).getDeclaringClass()) {
			return best_candidates.get(0);
		}
		return Err(IllegalStateException::new(&String::format("Found multiple candidates for method %s on class %s : %s", method_name + Stream::of(parameter_types).map(String::valueOf).collect(&Collectors::joining(",", "(", ")")), &cls.getName(), &best_candidates.stream().map(Method::toString).collect(&Collectors::joining(",", "[", "]")))));
	}

	pub fn get_method_object(&self, cls: &/* Java */ java::lang::Class /**/, name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		let r0 = 'try0: {
			return  if name != null && cls != null { cls.getMethod(name, parameter_types) } else { null };
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchMethodExceptionSecurityException | ) => {
				return null;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_methods_list_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_methods_list_with_annotation(cls, annotation_cls, false, false);
	}

	pub fn get_methods_list_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/, search_supers: bool, ignore_access: bool) -> /* Java */ java::util::List /**/ {
		Objects::requireNonNull(cls, "cls");
		Objects::requireNonNull(annotation_cls, "annotationCls");
		/* final */ let classes: List<Class<?>> =  if search_supers { org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_all_superclasses_and_interfaces(cls) } else { ArrayList<>::new() };
		classes.add(0, cls);
		/* final */ let annotated_methods: List<Method> = ArrayList<>::new();
		classes.forEach(|acls|{
			/* final */ let methods: Vec<Method> =  if ignore_access { acls.getDeclaredMethods() } else { acls.getMethods() };
			Stream::of(methods).filter(|method|method.isAnnotationPresent(annotation_cls)).forEachOrdered(annotatedMethods::add);
		});
		return annotated_methods;
	}

	pub fn get_methods_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/) -> &[/* Java */ java::lang::reflect::Method /**/] {
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_methods_with_annotation(cls, annotation_cls, false, false);
	}

	pub fn get_methods_with_annotation(&self, cls: &/* Java */ java::lang::Class /**/, annotation_cls: &/* Java */ java::lang::Class /**/, search_supers: bool, ignore_access: bool) -> &[/* Java */ java::lang::reflect::Method /**/] {
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_methods_list_with_annotation(cls, annotation_cls, search_supers, ignore_access).toArray(ArrayUtils::EMPTY_METHOD_ARRAY);
	}

	pub fn get_override_hierarchy(&self, method: &/* Java */ java::lang::reflect::Method /**/, interfaces_behavior: &org::apache::commons::lang3::class_utils::Interfaces) -> /* Java */ java::util::Set /**/ {
		Objects::requireNonNull(method, "method");
		/* final */ let result: Set<Method> = LinkedHashSet<>::new();
		result.add(method);
		/* final */ let parameter_types: Vec<Class<?>> = method.getParameterTypes();
		/* final */ let declaring_class: Class<?> = method.getDeclaringClass();
		/* final */ let hierarchy: Iterator<Class<?>> = ClassUtils::hierarchy(declaring_class, interfaces_behavior).iterator();
		//skip the declaring class :P
		hierarchy.next();
		'hierarchyTraversal: while hierarchy.hasNext() {
			/* final */ let c: Class<?> = hierarchy.next();
			/* final */ let m: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_accessible_method(c, &method.getName(), parameter_types);
			if m == null {
				continue;
			}
			if Arrays::equals(&m.getParameterTypes(), parameter_types) {
				// matches without generics
				result.add(m);
				continue;
			}
			// necessary to get arguments every time in the case that we are including interfaces
			/* final */ let type_arguments: Map<TypeVariable<?>, Type> = TypeUtils::get_type_arguments(declaring_class, &m.getDeclaringClass());
			 {
				let i: i32 = 0;
				while i < parameter_types.length {
					{
						/* final */ let child_type: Type = TypeUtils::unroll_variables(type_arguments, method.getGenericParameterTypes()[i]);
						/* final */ let parent_type: Type = TypeUtils::unroll_variables(type_arguments, m.getGenericParameterTypes()[i]);
						if !TypeUtils::equals(child_type, parent_type) {
							continue 'hierarchyTraversal;
						}
					}
					i += 1;
				 }
			 }
	
			result.add(m);
		}
		return result;
	}

	pub fn invoke_exact_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		return .invokeExactMethod(object, method_name, ArrayUtils::EMPTY_OBJECT_ARRAY, null);
	}

	pub fn invoke_exact_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_exact_method(object, method_name, actuals, &ClassUtils::to_class(actuals));
	}

	pub fn invoke_exact_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let cls: Class<?> = Objects::requireNonNull(object, "object").getClass();
		/* final */ let param_types: Vec<Class<?>> = ArrayUtils::null_to_empty(parameter_types);
		/* final */ let method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method(cls, method_name, param_types);
		org::apache::commons::lang3::reflect::method_utils::MethodUtils::require_non_null(method, cls, method_name, param_types)?;
		return method.invoke(object, &ArrayUtils::null_to_empty(args));
	}

	pub fn invoke_exact_static_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_exact_static_method(cls, method_name, actuals, &ClassUtils::to_class(actuals));
	}

	pub fn invoke_exact_static_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let param_types: Vec<Class<?>> = ArrayUtils::null_to_empty(parameter_types);
		/* final */ let method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_accessible_method(cls, method_name, &ArrayUtils::null_to_empty(param_types));
		org::apache::commons::lang3::reflect::method_utils::MethodUtils::require_non_null(method, cls, method_name, param_types)?;
		return method.invoke(null, &ArrayUtils::null_to_empty(args));
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, force_access: bool, method_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		return .invokeMethod(object, force_access, method_name, ArrayUtils::EMPTY_OBJECT_ARRAY, null);
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, force_access: bool, method_name: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_method(object, force_access, method_name, actuals, &ClassUtils::to_class(actuals));
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, force_access: bool, method_name: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let cls: Class<? extends Object> = Objects::requireNonNull(object, "object").getClass();
		/* final */ let param_types: Vec<Class<?>> = ArrayUtils::null_to_empty(parameter_types);
		/* final */ let method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_invoke_method(force_access, method_name, param_types, cls);
		org::apache::commons::lang3::reflect::method_utils::MethodUtils::require_non_null(method, cls, method_name, param_types)?;
		return method.invoke(object, &org::apache::commons::lang3::reflect::method_utils::MethodUtils::to_var_args(method, &ArrayUtils::null_to_empty(args)));
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		return .invokeMethod(object, method_name, ArrayUtils::EMPTY_OBJECT_ARRAY, null);
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_method(object, method_name, actuals, &ClassUtils::to_class(actuals));
	}

	pub fn invoke_method(&self, object: &/* Java */ java::lang::Object /**/, method_name: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_method(object, false, method_name, args, parameter_types)?;
	}

	pub fn invoke_static_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::method_utils::MethodUtils::invoke_static_method(cls, method_name, actuals, &ClassUtils::to_class(actuals));
	}

	pub fn invoke_static_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> /* Java */ java::lang::Object /**/ {
		/* final */ let param_types: Vec<Class<?>> = ArrayUtils::null_to_empty(parameter_types);
		/* final */ let method: Method = org::apache::commons::lang3::reflect::method_utils::MethodUtils::get_matching_accessible_method(cls, method_name, param_types);
		org::apache::commons::lang3::reflect::method_utils::MethodUtils::require_non_null(method, cls, method_name, param_types)?;
		return method.invoke(null, &org::apache::commons::lang3::reflect::method_utils::MethodUtils::to_var_args(method, &ArrayUtils::null_to_empty(args)));
	}

	fn require_non_null(&self, method: &/* Java */ java::lang::reflect::Method /**/, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.NoSuchMethodException) */ -> /* Java */ java::lang::reflect::Method /**/ {
		if method == null {
			return Err(NoSuchMethodException::new(&String::format("No method: %s.%s(%s)", &ClassUtils::get_name(cls), method_name, &Streams::of(parameter_types).map(ClassUtils::getName).collect(&LangCollectors::joining(", ")))));
		}
		return method;
	}

	fn to_var_args(&self, executable: &/* Java */ java::lang::reflect::Executable /**/, args: &&[/* Java */ java::lang::Object /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> &[/* Java */ java::lang::Object /**/] {
		return  if executable.isVarArgs() { org::apache::commons::lang3::reflect::method_utils::MethodUtils::to_var_args(args, &executable.getParameterTypes())? } else { args };
	}

	fn to_var_args(&self, args: &&[/* Java */ java::lang::Object /**/], method_parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.IllegalAccessException | java.lang.IllegalArgumentException | java.lang.NoSuchMethodException | java.lang.reflect.InvocationTargetException) */ -> &[/* Java */ java::lang::Object /**/] {
		/* final */ let mpt_length: i32 = method_parameter_types.length;
		if args.length == mpt_length {
			/* final */ let last_arg: Object = args[args.length - 1];
			if last_arg == null || last_arg.getClass().equals(method_parameter_types[mpt_length - 1]) {
				// The args array is already in the canonical form for the method.
				return args;
			}
		}
		// Construct a new array matching the method's declared parameter types.
		// Copy the normal (non-varargs) parameters
		/* final */ let new_args: Vec<Object> = ArrayUtils::arraycopy(args, 0, 0, mpt_length - 1, |()|: [Option<Object>; mpt_length] = [None; mpt_length]);
		// Construct a new array for the variadic parameters
		/* final */ let var_arg_component_type: Class<?> = method_parameter_types[mpt_length - 1].getComponentType();
		/* final */ let var_arg_component_wrapped_type: Class<?> = ClassUtils::primitive_to_wrapper(var_arg_component_type);
		/* final */ let var_arg_length: i32 = args.length - mpt_length + 1;
		// Copy the variadic arguments into the varargs array, converting types if needed.
		let var_args_array: Object = Array::newInstance(var_arg_component_wrapped_type, var_arg_length);
		/* final */ let primitive_or_wrapper: bool = ClassUtils::is_primitive_or_wrapper(var_arg_component_wrapped_type);
		 {
			let i: i32 = 0;
			while i < var_arg_length {
				{
					/* final */ let arg: Object = args[mpt_length - 1 + i];
					let r0 = 'try0: {
						Array::set(var_args_array, i,  if primitive_or_wrapper { var_arg_component_wrapped_type.getConstructor(&ClassUtils::wrapper_to_primitive(var_arg_component_wrapped_type)).newInstance(arg) } else { var_arg_component_wrapped_type.cast(arg) });
						break 'try0 Ok(());
					};
					match r0 {
						Err(e @ InstantiationException) => {
							break 'try0 Err(IllegalArgumentException::new("Cannot convert vararg #" + i, e));
						},
						Err(e) => Err(e)?,
						Ok => (),
					}
				}
				i += 1;
			 }
		 }
	
		if var_arg_component_type.isPrimitive() {
			// unbox from wrapper type to primitive type
			var_args_array = ArrayUtils::to_primitive(var_args_array);
		}
		// Store the varargs array in the last position of the array to return
		new_args[mpt_length - 1] = var_args_array;
		// Return the canonical varargs array.
		return new_args;
	}

	pub fn new() -> org::apache::commons::lang3::reflect::method_utils::MethodUtils {
	// empty
	}
}