use java::lang::reflect::AccessibleObject;
use java::lang::reflect::Constructor;
use java::lang::reflect::Member;
use java::lang::reflect::Method;
use java::lang::reflect::Modifier;
use crate::org::apache::commons::lang3::ClassUtils;

struct MemberUtils;

impl MemberUtils {
	static ACCESS_TEST: i32 = Modifier::PUBLIC | Modifier::PROTECTED | Modifier::PRIVATE;

	static WIDENING_PRIMITIVE_TYPES: &[/* Java */ java::lang::Class /**/] = vec![// byte
	Byte::TYPE, // short
	Short::TYPE, // char
	Character::TYPE, // int
	Integer::TYPE, // long
	Long::TYPE, // float
	Float::TYPE, // double
	Double::TYPE, ]
	;

	fn compare_constructor_fit(&self, left: &/* Java */ java::lang::reflect::Constructor /**/, right: &/* Java */ java::lang::reflect::Constructor /**/, actual: &&[/* Java */ java::lang::Class /**/]) -> i32 {
		return org::apache::commons::lang3::reflect::member_utils::MemberUtils::compare_parameter_types(&Executable::of(left), &Executable::of(right), actual);
	}

	fn compare_method_fit(&self, left: &/* Java */ java::lang::reflect::Method /**/, right: &/* Java */ java::lang::reflect::Method /**/, actual: &&[/* Java */ java::lang::Class /**/]) -> i32 {
		return org::apache::commons::lang3::reflect::member_utils::MemberUtils::compare_parameter_types(&Executable::of(left), &Executable::of(right), actual);
	}

	fn compare_parameter_types(&self, left: &org::apache::commons::lang3::reflect::member_utils::Executable, right: &org::apache::commons::lang3::reflect::member_utils::Executable, actual: &&[/* Java */ java::lang::Class /**/]) -> i32 {
		/* final */ let left_cost: f32 = org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_total_transformation_cost(actual, left);
		/* final */ let right_cost: f32 = org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_total_transformation_cost(actual, right);
		return Float::compare(left_cost, right_cost);
	}

	fn get_object_transformation_cost(&self, mut src_class: &/* Java */ java::lang::Class /**/, dest_class: &/* Java */ java::lang::Class /**/) -> f32 {
		if dest_class.isPrimitive() {
			return org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_primitive_promotion_cost(src_class, dest_class);
		}
		let cost: f32 = 0.0f;
		while src_class != null && !dest_class.equals(src_class) {
			if dest_class.isInterface() && ClassUtils::is_assignable(src_class, dest_class) {
				// slight penalty for interface match.
				// we still want an exact match to override an interface match,
				// but
				// an interface match should override anything where we have to
				// get a superclass.
				cost += 0.25f;
				break;
			}
			cost += 1;
			src_class = src_class.getSuperclass();
		}
		/* 
	         * If the destination class is null, we've traveled all the way up to an Object match. We'll penalize this by adding 1.5 to the cost.
	         */ 
		if src_class == null {
			cost += 1.5f;
		}
		return cost;
	}

	fn get_primitive_promotion_cost(&self, src_class: &/* Java */ java::lang::Class /**/, dest_class: &/* Java */ java::lang::Class /**/) -> f32 {
		if src_class == null {
			return 1.5f;
		}
		let cost: f32 = 0.0f;
		let cls: Class<?> = src_class;
		if !cls.isPrimitive() {
			// slight unwrapping penalty
			cost += 0.1f;
			cls = ClassUtils::wrapper_to_primitive(cls);
		}
		// Increase the cost as the loop widens the type.
		 {
			let i: i32 = 0;
			while cls != dest_class && i < self.WIDENING_PRIMITIVE_TYPES.length {
				{
					if cls == self.WIDENING_PRIMITIVE_TYPES[i] {
						cost += 0.1f;
						if i < self.WIDENING_PRIMITIVE_TYPES.length - 1 {
							cls = self.WIDENING_PRIMITIVE_TYPES[i + 1];
						}
					}
				}
				i += 1;
			 }
		 }
	
		return cost;
	}

	fn get_total_transformation_cost(&self, src_args: &&[/* Java */ java::lang::Class /**/], executable: &org::apache::commons::lang3::reflect::member_utils::Executable) -> f32 {
		/* final */ let dest_args: Vec<Class<?>> = executable.get_parameter_types();
		/* final */ let is_var_args: bool = executable.is_var_args();
		// "source" and "destination" are the actual and declared args respectively.
		let total_cost: f32 = 0.0f;
		/* final */ let normal_args_len: i64 =  if is_var_args { dest_args.length - 1 } else { dest_args.length };
		if src_args.length < normal_args_len {
			return Float::MAX_VALUE;
		}
		 {
			let i: i32 = 0;
			while i < normal_args_len {
				{
					total_cost += org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_object_transformation_cost(src_args[i], dest_args[i]);
				}
				i += 1;
			 }
		 }
	
		if is_var_args {
			// When isVarArgs is true, srcArgs and dstArgs may differ in length.
			// There are two special cases to consider:
			/* final */ let no_var_args_passed: bool = src_args.length < dest_args.length;
			/* final */ let explicit_array_for_varargs: bool = src_args.length == dest_args.length && src_args[src_args.length - 1] != null && src_args[src_args.length - 1].isArray();
			/* final */ let var_args_cost: f32 = 0.001f;
			/* final */ let dest_class: Class<?> = dest_args[dest_args.length - 1].getComponentType();
			if no_var_args_passed {
				// When no varargs passed, the best match is the most generic matching type, not the most specific.
				total_cost += org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_object_transformation_cost(dest_class, Object.class) + var_args_cost;
			} else if explicit_array_for_varargs {
				/* final */ let source_class: Class<?> = src_args[src_args.length - 1].getComponentType();
				total_cost += org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_object_transformation_cost(source_class, dest_class) + var_args_cost;
			} else {
				// This is typical varargs case.
				 {
					let i: i32 = dest_args.length - 1;
					while i < src_args.length {
						{
							/* final */ let src_class: Class<?> = src_args[i];
							total_cost += org::apache::commons::lang3::reflect::member_utils::MemberUtils::get_object_transformation_cost(src_class, dest_class) + var_args_cost;
						}
						i += 1;
					 }
				 }
	
			}
		}
		return total_cost;
	}

	fn is_accessible(&self, member: &/* Java */ java::lang::reflect::Member /**/) -> bool {
		return org::apache::commons::lang3::reflect::member_utils::MemberUtils::is_public(member) && !member.isSynthetic();
	}

	fn is_matching_constructor(&self, method: &/* Java */ java::lang::reflect::Constructor /**/, parameter_types: &&[/* Java */ java::lang::Class /**/]) -> bool {
		return org::apache::commons::lang3::reflect::member_utils::MemberUtils::is_matching_executable(&Executable::of(method), parameter_types);
	}

	fn is_matching_executable(&self, method: &org::apache::commons::lang3::reflect::member_utils::Executable, parameter_types: &&[/* Java */ java::lang::Class /**/]) -> bool {
		/* final */ let method_parameter_types: Vec<Class<?>> = method.get_parameter_types();
		if ClassUtils::is_assignable(parameter_types, method_parameter_types, true) {
			return true;
		}
		if method.is_var_args() {
			let i: i32;
			 {
				i = 0;
				while i < method_parameter_types.length - 1 && i < parameter_types.length {
					{
						if !ClassUtils::is_assignable(parameter_types[i], method_parameter_types[i], true) {
							return false;
						}
					}
					i += 1;
				 }
			 }
	
			/* final */ let var_arg_parameter_type: Class<?> = method_parameter_types[method_parameter_types.length - 1].getComponentType();
			while i < parameter_types.length {
				{
					if !ClassUtils::is_assignable(parameter_types[i], var_arg_parameter_type, true) {
						return false;
					}
				}
				i += 1;
			 }
	
			return true;
		}
		return false;
	}

	fn is_matching_method(&self, method: &/* Java */ java::lang::reflect::Method /**/, parameter_types: &&[/* Java */ java::lang::Class /**/]) -> bool {
		return org::apache::commons::lang3::reflect::member_utils::MemberUtils::is_matching_executable(&Executable::of(method), parameter_types);
	}

	fn is_package(&self, modifiers: i32) -> bool {
		return (modifiers & self.ACCESS_TEST) == 0;
	}

	fn is_public(&self, member: &/* Java */ java::lang::reflect::Member /**/) -> bool {
		return member != null && Modifier::isPublic(&member.getModifiers());
	}

	fn is_static(&self, member: &/* Java */ java::lang::reflect::Member /**/) -> bool {
		return member != null && Modifier::isStatic(&member.getModifiers());
	}

	fn set_accessible_workaround<T: /* Java */ java::lang::reflect::AccessibleObject /**/>(&self, obj: &T) -> T {
		if AccessibleObjects::is_accessible(obj) {
			return obj;
		}
		/* final */ let m: Member = obj as Member;
		if org::apache::commons::lang3::reflect::member_utils::MemberUtils::is_public(m) && org::apache::commons::lang3::reflect::member_utils::MemberUtils::is_package(&m.getDeclaringClass().getModifiers()) {
			let r0 = 'try0: {
				obj.setAccessible(true);
				return obj;
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ SecurityException) => {
				// Ignore in favor of subsequent IllegalAccessException
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		return obj;
	}
}

struct Executable {
	parameter_types: &[/* Java */ java::lang::Class /**/],
	is_var_args: bool,
}

impl Executable {
	fn of(&self, constructor: &/* Java */ java::lang::reflect::Constructor /**/) -> org::apache::commons::lang3::reflect::member_utils::Executable {
		return Executable::new(constructor);
	}

	fn of(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::reflect::member_utils::Executable {
		return Executable::new(method);
	}

	fn new(constructor: &/* Java */ java::lang::reflect::Constructor /**/) -> org::apache::commons::lang3::reflect::member_utils::Executable {
		self.parameter_types = constructor.getParameterTypes();
		self.is_var_args = constructor.isVarArgs();
	}

	fn new(method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::reflect::member_utils::Executable {
		self.parameter_types = method.getParameterTypes();
		self.is_var_args = method.isVarArgs();
	}

	pub fn get_parameter_types(&self) -> &[/* Java */ java::lang::Class /**/] {
		return self.parameter_types;
	}

	pub fn is_var_args(&self) -> bool {
		return self.is_var_args;
	}
}