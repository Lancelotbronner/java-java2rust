use java::lang::reflect::Constructor;
use java::lang::reflect::InvocationTargetException;
use java::util::Objects;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;

pub struct ConstructorUtils;

impl ConstructorUtils {
	pub fn get_accessible_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Constructor /**/ {
		Objects::requireNonNull(cls, "cls");
		let r0 = 'try0: {
			return org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::get_accessible_constructor(&cls.getConstructor(parameter_types));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchMethodException) => {
				return null;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn get_accessible_constructor<T>(&self, ctor: &/* Java */ java::lang::reflect::Constructor /**/) -> /* Java */ java::lang::reflect::Constructor /**/ {
		Objects::requireNonNull(ctor, "ctor");
		return  if MemberUtils::is_accessible(ctor) && org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::is_accessible(&ctor.getDeclaringClass()) { ctor } else { null };
	}

	pub fn get_matching_accessible_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, parameter_types: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::reflect::Constructor /**/ {
		Objects::requireNonNull(cls, "cls");
		// most of the time this works and it's much faster
		let r0 = 'try0: {
			return MemberUtils::set_accessible_workaround(&cls.getConstructor(parameter_types));
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ NoSuchMethodException) => {
			// ignore
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		let result: Constructor<T> = null;
		/* 
	         * (1) Class.getConstructors() is documented to return Constructor<T> so as long as the array is not subsequently modified, everything's fine.
	         */ 
		/* final */ let ctors: Vec<Constructor<?>> = cls.getConstructors();
		// return best match:
		for ctor in ctors {
			// compare parameters
			if MemberUtils::is_matching_constructor(ctor, parameter_types) {
				// get accessible version of constructor
				ctor = org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::get_accessible_constructor(ctor);
				if ctor != null {
					MemberUtils::set_accessible_workaround(ctor);
					if result == null || MemberUtils::compare_constructor_fit(ctor, result, parameter_types) < 0 {
						// temporary variable for annotation, see comment above (1)
						/* final */ let constructor: Constructor<T> = ctor as Constructor<T>;
						result = constructor;
					}
				}
			}
		}
		return result;
	}

	pub fn invoke_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.NoSuchMethodException | java.lang.InstantiationException | java.lang.IllegalAccessException | java.lang.reflect.InvocationTargetException) */ -> T {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::invoke_constructor(cls, actuals, &ClassUtils::to_class(actuals))?;
	}

	pub fn invoke_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.NoSuchMethodException | java.lang.InstantiationException | java.lang.IllegalAccessException | java.lang.reflect.InvocationTargetException) */ -> T {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		/* final */ let ctor: Constructor<T> = org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::get_matching_accessible_constructor(cls, &ArrayUtils::null_to_empty(parameter_types));
		if ctor == null {
			return Err(NoSuchMethodException::new("No such accessible constructor on object: " + cls.getName()));
		}
		return ctor.newInstance(&MethodUtils::to_var_args(ctor, actuals));
	}

	pub fn invoke_exact_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, args: &/* Java */ java::lang::Object /**/) /* thrown(java.lang.NoSuchMethodException | java.lang.InstantiationException | java.lang.IllegalAccessException | java.lang.reflect.InvocationTargetException) */ -> T {
		/* final */ let actuals: Vec<Object> = ArrayUtils::null_to_empty(args);
		return org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::invoke_exact_constructor(cls, actuals, &ClassUtils::to_class(actuals))?;
	}

	pub fn invoke_exact_constructor<T>(&self, cls: &/* Java */ java::lang::Class /**/, args: &&[/* Java */ java::lang::Object /**/], parameter_types: &&[/* Java */ java::lang::Class /**/]) /* thrown(java.lang.NoSuchMethodException | java.lang.InstantiationException | java.lang.IllegalAccessException | java.lang.reflect.InvocationTargetException) */ -> T {
		/* final */ let ctor: Constructor<T> = org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils::get_accessible_constructor(cls, &ArrayUtils::null_to_empty(parameter_types));
		if ctor == null {
			return Err(NoSuchMethodException::new("No such accessible constructor on object: " + cls.getName()));
		}
		return ctor.newInstance(&ArrayUtils::null_to_empty(args));
	}

	fn is_accessible(&self, type: &/* Java */ java::lang::Class /**/) -> bool {
		let cls: Class<?> = type;
		while cls != null {
			if !ClassUtils::is_public(cls) {
				return false;
			}
			cls = cls.getEnclosingClass();
		}
		return true;
	}

	pub fn new() -> org::apache::commons::lang3::reflect::constructor_utils::ConstructorUtils {
	// empty
	}
}