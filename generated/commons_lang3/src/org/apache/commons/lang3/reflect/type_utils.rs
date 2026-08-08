use java::lang::reflect::Array;
use java::lang::reflect::GenericArrayType;
use java::lang::reflect::GenericDeclaration;
use java::lang::reflect::ParameterizedType;
use java::lang::reflect::Type;
use java::lang::reflect::TypeVariable;
use java::lang::reflect::WildcardType;
use java::util::Arrays;
use java::util::Collection;
use java::util::Collections;
use java::util::HashMap;
use java::util::HashSet;
use java::util::List;
use java::util::Map;
use java::util::Objects;
use java::util::Set;
use java::util::TreeSet;
use crate::org::apache::commons::lang3::AppendableJoiner;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ClassUtils;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::Validate;
use crate::org::apache::commons::lang3::builder::Builder;

pub struct TypeUtils;

impl TypeUtils {
	static AMP_JOINER: org::apache::commons::lang3::appendable_joiner::AppendableJoiner = AppendableJoiner<Type>::builder().set_delimiter(" & ").set_element_appender(|(a, e)|a.append(&.toString(e))).get();

	static CTJ_JOINER: org::apache::commons::lang3::appendable_joiner::AppendableJoiner = AppendableJoiner<TypeVariable<Class<?>>>::builder().set_delimiter(", ").set_element_appender(|(a, e)|a.append(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::any_to_string(e))).get();

	static GT_JOINER: org::apache::commons::lang3::appendable_joiner::AppendableJoiner = AppendableJoiner::builder().set_prefix("<").set_suffix(">").set_delimiter(", ").set_element_appender(|(a, e)|a.append(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::any_to_string(e))).get();

	pub static WILDCARD_ALL: /* Java */ java::lang::reflect::WildcardType /**/ = org::apache::commons::lang3::reflect::type_utils::TypeUtils::wildcard_type().with_upper_bounds(Object.class).build();

	fn any_to_string<T>(&self, object: &T) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return  if object instanceof Type { org::apache::commons::lang3::reflect::type_utils::TypeUtils::to_string(object as Type)? } else { object.toString() };
	}

	fn append_recursive_types(&self, builder: &/* Java */ java::lang::StringBuilder /**/, recursive_type_indexes: &&[i32], argument_types: &&[/* Java */ java::lang::reflect::Type /**/]) /* thrown(java.io.IOException | org.apache.commons.lang3.exception.UncheckedException) */ {
		 {
			let i: i32 = 0;
			while i < recursive_type_indexes.length {
				{
					// toString() or SO
					self.GT_JOINER.join(builder, &argument_types[i].toString())?;
				}
				i += 1;
			 }
		 }
	
		/* final */ let arguments_filtered: Vec<Type> = ArrayUtils::remove_all(argument_types, recursive_type_indexes)?;
		if arguments_filtered.length > 0 {
			self.GT_JOINER.join(builder, arguments_filtered as Vec<Object>)?;
		}
	}

	fn class_to_string<T>(&self, cls: &/* Java */ java::lang::Class /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		if cls.isArray() {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::to_string(&cls.getComponentType())? + "[]";
		}
		if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_cyclical(cls) {
			return cls.getSimpleName() + "(cycle)";
		}
		/* final */ let buf: StringBuilder = StringBuilder::new();
		if cls.getEnclosingClass() != null {
			buf.append(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::class_to_string(&cls.getEnclosingClass())?).append('.').append(&cls.getSimpleName());
		} else {
			buf.append(&cls.getName());
		}
		if cls.getTypeParameters().length > 0 {
			self.GT_JOINER.join(buf, cls.getTypeParameters() as Vec<TypeVariable>)?;
		}
		return buf.toString();
	}

	pub fn contains_type_variables(&self, type: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		if type instanceof TypeVariable<?> {
			return true;
		}
		if type instanceof Class<?> {
			return (type as Class<?>).getTypeParameters().length > 0;
		}
		if type instanceof ParameterizedType {
			for /* final */ arg in (type as ParameterizedType).getActualTypeArguments() {
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_type_variables(arg) {
					return true;
				}
			}
			return false;
		}
		if type instanceof WildcardType {
			/* final */ let wild: WildcardType = type as WildcardType;
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_type_variables(org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_lower_bounds(wild)[0]) || org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_type_variables(org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(wild)[0]);
		}
		if type instanceof GenericArrayType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_type_variables(&(type as GenericArrayType).getGenericComponentType());
		}
		return false;
	}

	fn contains_variable_type_same_parametrized_type_bound(&self, type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/) -> bool {
		return ArrayUtils::contains(&type_variable.getBounds(), parameterized_type);
	}

	pub fn determine_type_arguments(&self, cls: &/* Java */ java::lang::Class /**/, super_parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		Objects::requireNonNull(cls, "cls");
		Objects::requireNonNull(super_parameterized_type, "superParameterizedType");
		/* final */ let super_class: Class<?> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(super_parameterized_type)?;
		// compatibility check
		if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(cls, super_class)? {
			return null;
		}
		if cls.equals(super_class) {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(super_parameterized_type, super_class, null);
		}
		// get the next class in the inheritance hierarchy
		/* final */ let mid_type: Type = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_closest_parent_type(cls, super_class)?;
		// can only be a class or a parameterized type
		if mid_type instanceof Class<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::determine_type_arguments(mid_type as Class<?>, super_parameterized_type)?;
		}
		/* final */ let mid_parameterized_type: ParameterizedType = mid_type as ParameterizedType;
		/* final */ let mid_class: Class<?> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(mid_parameterized_type)?;
		// get the type variables of the mid class that map to the type
		// arguments of the super class
		/* final */ let type_var_assigns: Map<TypeVariable<?>, Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::determine_type_arguments(mid_class, super_parameterized_type)?;
		// map the arguments of the mid type to the class type variables
		org::apache::commons::lang3::reflect::type_utils::TypeUtils::map_type_variables_to_arguments(cls, mid_parameterized_type, type_var_assigns);
		return type_var_assigns;
	}

	fn equals(&self, generic_array_type: &/* Java */ java::lang::reflect::GenericArrayType /**/, type: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		return type instanceof GenericArrayType && org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&generic_array_type.getGenericComponentType(), &(type as GenericArrayType).getGenericComponentType());
	}

	fn equals(&self, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/, type: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		if type instanceof ParameterizedType {
			/* final */ let other: ParameterizedType = type as ParameterizedType;
			if org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&parameterized_type.getRawType(), &other.getRawType()) && org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&parameterized_type.getOwnerType(), &other.getOwnerType()) {
				return org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&parameterized_type.getActualTypeArguments(), &other.getActualTypeArguments());
			}
		}
		return false;
	}

	pub fn equals(&self, type1: &/* Java */ java::lang::reflect::Type /**/, type2: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		if Objects::equals(type1, type2) {
			return true;
		}
		if type1 instanceof ParameterizedType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(type1 as ParameterizedType, type2);
		}
		if type1 instanceof GenericArrayType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(type1 as GenericArrayType, type2);
		}
		if type1 instanceof WildcardType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(type1 as WildcardType, type2);
		}
		return false;
	}

	fn equals(&self, type1: &&[/* Java */ java::lang::reflect::Type /**/], type2: &&[/* Java */ java::lang::reflect::Type /**/]) -> bool {
		if type1.length == type2.length {
			 {
				let i: i32 = 0;
				while i < type1.length {
					{
						if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(type1[i], type2[i]) {
							return false;
						}
					}
					i += 1;
				 }
			 }
	
			return true;
		}
		return false;
	}

	fn equals(&self, wildcard_type: &/* Java */ java::lang::reflect::WildcardType /**/, type: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		if type instanceof WildcardType {
			/* final */ let other: WildcardType = type as WildcardType;
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_lower_bounds(wildcard_type), &org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_lower_bounds(other)) && org::apache::commons::lang3::reflect::type_utils::TypeUtils::equals(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(wildcard_type), &org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(other));
		}
		return false;
	}

	fn extract_type_arguments_from(&self, mappings: &/* Java */ java::util::Map /**/, variables: &&[/* Java */ java::lang::reflect::TypeVariable /**/]) /* thrown(java.lang.IllegalArgumentException) */ -> &[/* Java */ java::lang::reflect::Type /**/] {
		/* final */ let result: [Option<Type>; variables.length] = [None; variables.length];
		let index: i32 = 0;
		for /* final */ var in variables {
			Validate::is_true(&mappings.containsKey(var), |()|String::format("missing argument mapping for %s", &org::apache::commons::lang3::reflect::type_utils::TypeUtils::to_string(var)?))?;
			result[index += 1 !!!check!!! post increment] = mappings.get(var);
		}
		return result;
	}

	fn find_recursive_types(&self, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/) -> &[i32] {
		/* final */ let filtered_argument_types: Vec<Type> = Arrays::copyOf(&parameterized_type.getActualTypeArguments(), parameterized_type.getActualTypeArguments().length);
		let indexes_to_remove;
		 {
			let i: i32 = 0;
			while i < filtered_argument_types.length {
				{
					if filtered_argument_types[i] instanceof TypeVariable<?> && org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_variable_type_same_parametrized_type_bound(filtered_argument_types[i] as TypeVariable<?>, parameterized_type) {
						indexes_to_remove = ArrayUtils::add(indexes_to_remove, i);
					}
				}
				i += 1;
			 }
		 }
	
		return indexes_to_remove;
	}

	pub fn generic_array_type(&self, component_type: &/* Java */ java::lang::reflect::Type /**/) -> /* Java */ java::lang::reflect::GenericArrayType /**/ {
		return GenericArrayTypeImpl::new(&Objects::requireNonNull(component_type, "componentType"));
	}

	fn generic_array_type_to_string(&self, generic_array_type: &/* Java */ java::lang::reflect::GenericArrayType /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return String::format("%s[]", &org::apache::commons::lang3::reflect::type_utils::TypeUtils::to_string(&generic_array_type.getGenericComponentType())?);
	}

	pub fn get_array_component_type(&self, type: &/* Java */ java::lang::reflect::Type /**/) -> /* Java */ java::lang::reflect::Type /**/ {
		if type instanceof Class<?> {
			/* final */ let cls: Class<?> = type as Class<?>;
			return  if cls.isArray() { cls.getComponentType() } else { null };
		}
		if type instanceof GenericArrayType {
			return (type as GenericArrayType).getGenericComponentType();
		}
		return null;
	}

	fn get_closest_parent_type(&self, cls: &/* Java */ java::lang::Class /**/, super_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::reflect::Type /**/ {
		// only look at the interfaces if the super class is also an interface
		if super_class.isInterface() {
			// get the generic interfaces of the subject class
			/* final */ let interface_types: Vec<Type> = cls.getGenericInterfaces();
			// will hold the best generic interface match found
			let generic_interface: Type = null;
			// find the interface closest to the super class
			for /* final */ mid_type in interface_types {
				/* final */ let mid_class: Class<?>;
				if mid_type instanceof ParameterizedType {
					mid_class = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(mid_type as ParameterizedType)?;
				} else if mid_type instanceof Class<?> {
					mid_class = mid_type as Class<?>;
				} else {
					return Err(IllegalStateException::new("Unexpected generic interface type found: " + mid_type));
				}
				// than the previously found match
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(mid_class, super_class)? && org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(generic_interface, mid_class as Type) {
					generic_interface = mid_type;
				}
			}
			// found a match?
			if generic_interface != null {
				return generic_interface;
			}
		}
		// super class has to be one, instead
		return cls.getGenericSuperclass();
	}

	pub fn get_implicit_bounds(&self, type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/) -> &[/* Java */ java::lang::reflect::Type /**/] {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::normalize_upper_to_object(&Objects::requireNonNull(type_variable, "typeVariable").getBounds());
	}

	pub fn get_implicit_lower_bounds(&self, wildcard_type: &/* Java */ java::lang::reflect::WildcardType /**/) -> &[/* Java */ java::lang::reflect::Type /**/] {
		Objects::requireNonNull(wildcard_type, "wildcardType");
		/* final */ let bounds: Vec<Type> = wildcard_type.getLowerBounds();
		return  if bounds.length == 0 { : [Option<Type>; ] = [None; ] } else { bounds };
	}

	pub fn get_implicit_upper_bounds(&self, wildcard_type: &/* Java */ java::lang::reflect::WildcardType /**/) -> &[/* Java */ java::lang::reflect::Type /**/] {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::normalize_upper_to_object(&Objects::requireNonNull(wildcard_type, "wildcardType").getUpperBounds());
	}

	fn get_raw_type(&self, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::lang::Class /**/ {
		/* final */ let raw_type: Type = parameterized_type.getRawType();
		// rarely a bad idea.
		if !(raw_type instanceof Class<?>) {
			return Err(IllegalStateException::new("Type of rawType: " + raw_type));
		}
		return raw_type as Class<?>;
	}

	pub fn get_raw_type(&self, type: &/* Java */ java::lang::reflect::Type /**/, assigning_type: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::Class /**/ {
		if type instanceof Class<?> {
			// it is raw, no problem
			return type as Class<?>;
		}
		if type instanceof ParameterizedType {
			// simple enough to get the raw type of a ParameterizedType
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(type as ParameterizedType)?;
		}
		if type instanceof TypeVariable<?> {
			if assigning_type == null {
				return null;
			}
			// get the entity declaring this type variable
			/* final */ let generic_declaration: Object = (type as TypeVariable<?>).getGenericDeclaration();
			// variable
			if !(generic_declaration instanceof Class<?>) {
				return null;
			}
			// get the type arguments for the declaring class/interface based
			// on the enclosing type
			/* final */ let type_var_assigns: Map<TypeVariable<?>, Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(assigning_type, generic_declaration as Class<?>);
			// declaring type
			if type_var_assigns == null {
				return null;
			}
			// get the argument assigned to this type variable
			/* final */ let type_argument: Type = type_var_assigns.get(type);
			if type_argument == null {
				return null;
			}
			// get the argument for this type variable
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(type_argument, assigning_type)?;
		}
		if type instanceof GenericArrayType {
			// get raw component type
			/* final */ let raw_component_type: Class<?> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(&(type as GenericArrayType).getGenericComponentType(), assigning_type)?;
			// create array type from raw component type and return its class
			return  if raw_component_type != null { Array::newInstance(raw_component_type, 0).getClass() } else { null };
		}
		// (hand-waving) this is not the method you're looking for
		if type instanceof WildcardType {
			return null;
		}
		return Err(IllegalArgumentException::new("unknown type: " + type));
	}

	fn get_type_arguments(&self, mut cls: &/* Java */ java::lang::Class /**/, to_class: &/* Java */ java::lang::Class /**/, subtype_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		// make sure they're assignable
		if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(cls, to_class)? {
			return null;
		}
		// can't work with primitives
		if cls.isPrimitive() {
			// both classes are primitives?
			if to_class.isPrimitive() {
				// harvested with these two types.
				return HashMap<>::new();
			}
			// work with wrapper the wrapper class instead of the primitive
			cls = ClassUtils::primitive_to_wrapper(cls);
		}
		// create a copy of the incoming map, or an empty one if it's null
		/* final */ let type_var_assigns: HashMap<TypeVariable<?>, Type> =  if subtype_var_assigns == null { HashMap<>::new() } else { HashMap<>::new(subtype_var_assigns) };
		// has target class been reached?
		if to_class.equals(cls) {
			return type_var_assigns;
		}
		// walk the inheritance hierarchy until the target class is reached
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_closest_parent_type(cls, to_class)?, to_class, type_var_assigns)?;
	}

	pub fn get_type_arguments(&self, type: &/* Java */ java::lang::reflect::ParameterizedType /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(type, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(type)?, null);
	}

	fn get_type_arguments(&self, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/, to_class: &/* Java */ java::lang::Class /**/, subtype_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		/* final */ let cls: Class<?> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(parameterized_type)?;
		// make sure they're assignable
		if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(cls, to_class)? {
			return null;
		}
		/* final */ let owner_type: Type = parameterized_type.getOwnerType();
		/* final */ let type_var_assigns: Map<TypeVariable<?>, Type>;
		if owner_type instanceof ParameterizedType {
			// get the owner type arguments first
			/* final */ let parameterized_owner_type: ParameterizedType = owner_type as ParameterizedType;
			type_var_assigns = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(parameterized_owner_type, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(parameterized_owner_type)?, subtype_var_assigns)?;
		} else {
			// no owner, prep the type variable assignments map
			type_var_assigns =  if subtype_var_assigns == null { HashMap<>::new() } else { HashMap<>::new(subtype_var_assigns) };
		}
		// get the subject parameterized type's arguments
		/* final */ let type_args: Vec<Type> = parameterized_type.getActualTypeArguments();
		// and get the corresponding type variables from the raw class
		/* final */ let type_params: Vec<TypeVariable<?>> = cls.getTypeParameters();
		// map the arguments to their respective type variables
		 {
			let i: i32 = 0;
			while i < type_params.length {
				{
					/* final */ let type_arg: Type = type_args[i];
					type_var_assigns.put(type_params[i], &type_var_assigns.getOrDefault(type_arg, type_arg));
				}
				i += 1;
			 }
		 }
	
		if to_class.equals(cls) {
			// target class has been reached. Done.
			return type_var_assigns;
		}
		// walk the inheritance hierarchy until the target class is reached
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_closest_parent_type(cls, to_class)?, to_class, type_var_assigns)?;
	}

	pub fn get_type_arguments(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(type, to_class, null)?;
	}

	fn get_type_arguments(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_class: &/* Java */ java::lang::Class /**/, subtype_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> /* Java */ java::util::Map /**/ {
		if type instanceof Class<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(type as Class<?>, to_class, subtype_var_assigns)?;
		}
		if type instanceof ParameterizedType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(type as ParameterizedType, to_class, subtype_var_assigns)?;
		}
		if type instanceof GenericArrayType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(&(type as GenericArrayType).getGenericComponentType(),  if to_class.isArray() { to_class.getComponentType() } else { to_class }, subtype_var_assigns)?;
		}
		// return null?
		if type instanceof WildcardType {
			for /* final */ bound in org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(type as WildcardType) {
				// find the first bound that is assignable to the target class
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_class)? {
					return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(bound, to_class, subtype_var_assigns)?;
				}
			}
			return null;
		}
		if type instanceof TypeVariable<?> {
			for /* final */ bound in org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_bounds(type as TypeVariable<?>) {
				// find the first bound that is assignable to the target class
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_class)? {
					return org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(bound, to_class, subtype_var_assigns)?;
				}
			}
			return null;
		}
		return Err(IllegalStateException::new("found an unhandled type: " + type));
	}

	pub fn is_array_type(&self, type: &/* Java */ java::lang::reflect::Type /**/) -> bool {
		return type instanceof GenericArrayType || type instanceof Class<?> && (type as Class<?>).isArray();
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_class: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type == null {
			// consistency with ClassUtils.isAssignable() behavior
			return to_class == null || !to_class.isPrimitive();
		}
		// would have cause the previous to return true
		if to_class == null {
			return false;
		}
		// all types are assignable to themselves
		if to_class.equals(type) {
			return true;
		}
		if type instanceof Class<?> {
			// just comparing two classes
			return ClassUtils::is_assignable(type as Class<?>, to_class);
		}
		if type instanceof ParameterizedType {
			// only have to compare the raw type to the class
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(type as ParameterizedType)?, to_class)?;
		}
		// *
		if type instanceof TypeVariable<?> {
			// type is assignable to the class.
			for /* final */ bound in (type as TypeVariable<?>).getBounds() {
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_class)? {
					return true;
				}
			}
			return false;
		}
		// are class Object and array classes
		if type instanceof GenericArrayType {
			return to_class.equals(Object.class) || to_class.isArray() && org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&(type as GenericArrayType).getGenericComponentType(), &to_class.getComponentType())?;
		}
		// "? super Object" would be assignable to Object)
		if type instanceof WildcardType {
			return false;
		}
		return Err(IllegalStateException::new("found an unhandled type: " + type));
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_generic_array_type: &/* Java */ java::lang::reflect::GenericArrayType /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type == null {
			return true;
		}
		// would have cause the previous to return true
		if to_generic_array_type == null {
			return false;
		}
		// all types are assignable to themselves
		if to_generic_array_type.equals(type) {
			return true;
		}
		/* final */ let to_component_type: Type = to_generic_array_type.getGenericComponentType();
		if type instanceof Class<?> {
			/* final */ let cls: Class<?> = type as Class<?>;
			// compare the component types
			return cls.isArray() && org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&cls.getComponentType(), to_component_type, type_var_assigns)?;
		}
		if type instanceof GenericArrayType {
			// compare the component types
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&(type as GenericArrayType).getGenericComponentType(), to_component_type, type_var_assigns)?;
		}
		if type instanceof WildcardType {
			// so long as one of the upper bounds is assignable, it's good
			for /* final */ bound in org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(type as WildcardType) {
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_generic_array_type) {
					return true;
				}
			}
			return false;
		}
		if type instanceof TypeVariable<?> {
			// type variables cannot specify arrays as bounds.
			for /* final */ bound in org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_bounds(type as TypeVariable<?>) {
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_generic_array_type) {
					return true;
				}
			}
			return false;
		}
		if type instanceof ParameterizedType {
			// Collection[]< ? extends String > collection;
			return false;
		}
		return Err(IllegalStateException::new("found an unhandled type: " + type));
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type == null {
			return true;
		}
		// would have cause the previous to return true
		if to_parameterized_type == null {
			return false;
		}
		// cannot cast an array type to a parameterized type.
		if type instanceof GenericArrayType {
			return false;
		}
		// all types are assignable to themselves
		if to_parameterized_type.equals(type) {
			return true;
		}
		// get the target type's raw type
		/* final */ let to_class: Class<?> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(to_parameterized_type)?;
		// get the subject type's type arguments including owner type arguments
		// and supertype arguments up to and including the target class.
		/* final */ let from_type_var_assigns: Map<TypeVariable<?>, Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(type, to_class, null)?;
		// null means the two types are not compatible
		if from_type_var_assigns == null {
			return false;
		}
		// to parameterized types.
		if from_type_var_assigns.isEmpty() {
			return true;
		}
		// get the target type's type arguments including owner type arguments
		/* final */ let to_type_var_assigns: Map<TypeVariable<?>, Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(to_parameterized_type, to_class, type_var_assigns)?;
		// now to check each type argument
		for /* final */ var in to_type_var_assigns.keySet() {
			/* final */ let to_type_arg: Type = org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_variable_assignments(var, to_type_var_assigns);
			/* final */ let from_type_arg: Type = org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_variable_assignments(var, from_type_var_assigns);
			if to_type_arg == null && from_type_arg instanceof Class {
				continue;
			}
			// parameters of the target type.
			if from_type_arg != null && to_type_arg != null && !to_type_arg.equals(from_type_arg) && !(to_type_arg instanceof WildcardType && org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(from_type_arg, to_type_arg, type_var_assigns)?) {
				return false;
			}
		}
		return true;
	}

	pub fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_type: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type, null)?;
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_type: &/* Java */ java::lang::reflect::Type /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if to_type == null || to_type instanceof Class<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type as Class<?>)?;
		}
		if to_type instanceof ParameterizedType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type as ParameterizedType, type_var_assigns)?;
		}
		if to_type instanceof GenericArrayType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type as GenericArrayType, type_var_assigns)?;
		}
		if to_type instanceof WildcardType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type as WildcardType, type_var_assigns);
		}
		if to_type instanceof TypeVariable<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, to_type as TypeVariable<?>, type_var_assigns)?;
		}
		return Err(IllegalStateException::new("found an unhandled type: " + to_type));
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type == null {
			return true;
		}
		// would have cause the previous to return true
		if to_type_variable == null {
			return false;
		}
		// all types are assignable to themselves
		if to_type_variable.equals(type) {
			return true;
		}
		if type instanceof TypeVariable<?> {
			// a type variable is assignable to another type variable, if
			// and only if the former is the latter, extends the latter, or
			// is otherwise a descendant of the latter.
			/* final */ let bounds: Vec<Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_bounds(type as TypeVariable<?>);
			for /* final */ bound in bounds {
				if org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_type_variable, type_var_assigns)? {
					return true;
				}
			}
		}
		if type instanceof Class<?> || type instanceof ParameterizedType || type instanceof GenericArrayType || type instanceof WildcardType {
			return false;
		}
		return Err(IllegalStateException::new("found an unhandled type: " + type));
	}

	fn is_assignable(&self, type: &/* Java */ java::lang::reflect::Type /**/, to_wildcard_type: &/* Java */ java::lang::reflect::WildcardType /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> bool {
		if type == null {
			return true;
		}
		// would have cause the previous to return true
		if to_wildcard_type == null {
			return false;
		}
		// all types are assignable to themselves
		if to_wildcard_type.equals(type) {
			return true;
		}
		/* final */ let to_upper_bounds: Vec<Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(to_wildcard_type);
		/* final */ let to_lower_bounds: Vec<Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_lower_bounds(to_wildcard_type);
		if type instanceof WildcardType {
			/* final */ let wildcard_type: WildcardType = type as WildcardType;
			/* final */ let upper_bounds: Vec<Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_upper_bounds(wildcard_type);
			/* final */ let lower_bounds: Vec<Type> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_lower_bounds(wildcard_type);
			for to_bound in to_upper_bounds {
				// if there are assignments for unresolved type variables,
				// now's the time to substitute them.
				to_bound = org::apache::commons::lang3::reflect::type_utils::TypeUtils::substitute_type_variables(to_bound, type_var_assigns)?;
				// upper bound of the target type
				for /* final */ bound in upper_bounds {
					if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(bound, to_bound, type_var_assigns)? {
						return false;
					}
				}
			}
			for to_bound in to_lower_bounds {
				// if there are assignments for unresolved type variables,
				// now's the time to substitute them.
				to_bound = org::apache::commons::lang3::reflect::type_utils::TypeUtils::substitute_type_variables(to_bound, type_var_assigns)?;
				// lower bound of the subject type
				for /* final */ bound in lower_bounds {
					if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(to_bound, bound, type_var_assigns)? {
						return false;
					}
				}
			}
			return true;
		}
		for /* final */ to_bound in to_upper_bounds {
			// now's the time to substitute them.
			if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::substitute_type_variables(to_bound, type_var_assigns)?, type_var_assigns)? {
				return false;
			}
		}
		for /* final */ to_bound in to_lower_bounds {
			// now's the time to substitute them.
			if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::substitute_type_variables(to_bound, type_var_assigns)?, type, type_var_assigns)? {
				return false;
			}
		}
		return true;
	}

	fn is_cyclical(&self, cls: &/* Java */ java::lang::Class /**/) -> bool {
		for /* final */ type_parameter in cls.getTypeParameters() {
			for /* final */ bound in type_parameter.getBounds() {
				if bound.getTypeName().contains(&cls.getName()) {
					return true;
				}
			}
		}
		return false;
	}

	pub fn is_instance(&self, value: &/* Java */ java::lang::Object /**/, type: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.lang.IllegalStateException) */ -> bool {
		if type == null {
			return false;
		}
		return  if value == null { !(type instanceof Class<?>) || !(type as Class<?>).isPrimitive() } else { org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(&value.getClass(), type, null)? };
	}

	fn map_type_variables_to_arguments<T>(&self, cls: &/* Java */ java::lang::Class /**/, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/, type_var_assigns: &/* Java */ java::util::Map /**/) {
		// capture the type variables from the owner type that have assignments
		/* final */ let owner_type: Type = parameterized_type.getOwnerType();
		if owner_type instanceof ParameterizedType {
			// recursion to make sure the owner's owner type gets processed
			org::apache::commons::lang3::reflect::type_utils::TypeUtils::map_type_variables_to_arguments(cls, owner_type as ParameterizedType, type_var_assigns);
		}
		// parameterizedType is a generic interface/class (or it's in the owner
		// hierarchy of said interface/class) implemented/extended by the class
		// cls. Find out which type variables of cls are type arguments of
		// parameterizedType:
		/* final */ let type_args: Vec<Type> = parameterized_type.getActualTypeArguments();
		// of the cls's type variables that are arguments of parameterizedType,
		// find out which ones can be determined from the super type's arguments
		/* final */ let type_vars: Vec<TypeVariable<?>> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_raw_type(parameterized_type)?.getTypeParameters();
		// use List view of type parameters of cls so the contains() method can be used:
		/* final */ let type_var_list: List<TypeVariable<Class<T>>> = Arrays::asList(&cls.getTypeParameters());
		 {
			let i: i32 = 0;
			while i < type_args.length {
				{
					/* final */ let type_var: TypeVariable<?> = type_vars[i];
					/* final */ let type_arg: Type = type_args[i];
					// argument of parameterizedType is a type variable of cls
					if type_var_list.contains(type_arg) && // the super type.
					type_var_assigns.containsKey(type_var) {
						// map the assignment to the cls's type variable
						type_var_assigns.put(type_arg as TypeVariable<?>, &type_var_assigns.get(type_var));
					}
				}
				i += 1;
			 }
		 }
	
	}

	pub fn normalize_upper_bounds(&self, bounds: &&[/* Java */ java::lang::reflect::Type /**/]) /* thrown(java.lang.IllegalStateException) */ -> &[/* Java */ java::lang::reflect::Type /**/] {
		Objects::requireNonNull(bounds, "bounds");
		// don't bother if there's only one (or none) type
		if bounds.length < 2 {
			return bounds;
		}
		/* final */ let types: Set<Type> = HashSet<>::new(bounds.length);
		for /* final */ type1 in bounds {
			let subtype_found: bool = false;
			for /* final */ type2 in bounds {
				if type1 != type2 && org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type2, type1, null)? {
					subtype_found = true;
					break;
				}
			}
			if !subtype_found {
				types.add(type1);
			}
		}
		return types.toArray(ArrayUtils::EMPTY_TYPE_ARRAY);
	}

	fn normalize_upper_to_object(&self, bounds: &&[/* Java */ java::lang::reflect::Type /**/]) /* thrown(java.lang.IllegalStateException) */ -> &[/* Java */ java::lang::reflect::Type /**/] {
		return  if bounds.length == 0 { : [Option<Type>; ] = [None; ] } else { org::apache::commons::lang3::reflect::type_utils::TypeUtils::normalize_upper_bounds(bounds)? };
	}

	pub fn parameterize(&self, raw_class: &/* Java */ java::lang::Class /**/, type_variable_map: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::ParameterizedType /**/ {
		Objects::requireNonNull(raw_class, "rawClass");
		Objects::requireNonNull(type_variable_map, "typeVariableMap");
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::parameterize_with_owner(null, raw_class, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::extract_type_arguments_from(type_variable_map, &raw_class.getTypeParameters())?);
	}

	pub fn parameterize(&self, raw_class: &/* Java */ java::lang::Class /**/, type_arguments: &/* Java */ java::lang::reflect::Type /**/) -> /* Java */ java::lang::reflect::ParameterizedType /**/ {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::parameterize_with_owner(null, raw_class, type_arguments);
	}

	fn parameterized_type_to_string(&self, parameterized_type: &/* Java */ java::lang::reflect::ParameterizedType /**/) /* thrown(java.io.IOException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let builder: StringBuilder = StringBuilder::new();
		/* final */ let use_owner: Type = parameterized_type.getOwnerType();
		/* final */ let raw: Class<?> = parameterized_type.getRawType() as Class<?>;
		if use_owner == null {
			builder.append(&raw.getName());
		} else {
			if use_owner instanceof Class<?> {
				builder.append(&(use_owner as Class<?>).getName());
			} else {
				builder.append(use_owner);
			}
			builder.append('.').append(&raw.getSimpleName());
		}
		/* final */ let recursive_type_indexes: Vec<i32> = org::apache::commons::lang3::reflect::type_utils::TypeUtils::find_recursive_types(parameterized_type);
		if recursive_type_indexes.length > 0 {
			org::apache::commons::lang3::reflect::type_utils::TypeUtils::append_recursive_types(builder, recursive_type_indexes, &parameterized_type.getActualTypeArguments())?;
		} else {
			self.GT_JOINER.join(builder, parameterized_type.getActualTypeArguments() as Vec<Object>)?;
		}
		return builder.toString();
	}

	pub fn parameterize_with_owner(&self, owner: &/* Java */ java::lang::reflect::Type /**/, raw_class: &/* Java */ java::lang::Class /**/, type_variable_map: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::ParameterizedType /**/ {
		Objects::requireNonNull(raw_class, "rawClass");
		Objects::requireNonNull(type_variable_map, "typeVariableMap");
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::parameterize_with_owner(owner, raw_class, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::extract_type_arguments_from(type_variable_map, &raw_class.getTypeParameters())?);
	}

	pub fn parameterize_with_owner(&self, owner: &/* Java */ java::lang::reflect::Type /**/, raw_class: &/* Java */ java::lang::Class /**/, type_arguments: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> /* Java */ java::lang::reflect::ParameterizedType /**/ {
		Objects::requireNonNull(raw_class, "rawClass");
		/* final */ let use_owner: Type;
		if raw_class.getEnclosingClass() == null {
			Validate::is_true(owner == null, "no owner allowed for top-level %s", raw_class)?;
			use_owner = null;
		} else if owner == null {
			use_owner = raw_class.getEnclosingClass();
		} else {
			Validate::is_true(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(owner, &raw_class.getEnclosingClass())?, "%s is invalid owner type for parameterized %s", owner, raw_class)?;
			use_owner = owner;
		}
		Validate::no_null_elements(type_arguments, "null type argument at index %s")?;
		Validate::is_true(raw_class.getTypeParameters().length == type_arguments.length, "invalid number of type parameters specified: expected %d, got %d", raw_class.getTypeParameters().length, type_arguments.length)?;
		return ParameterizedTypeImpl::new(raw_class, use_owner, type_arguments);
	}

	fn substitute_type_variables(&self, type: &/* Java */ java::lang::reflect::Type /**/, type_var_assigns: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::reflect::Type /**/ {
		if type instanceof TypeVariable<?> && type_var_assigns != null {
			/* final */ let replacement_type: Type = type_var_assigns.get(type);
			if replacement_type == null {
				return Err(IllegalArgumentException::new("missing assignment type for type variable " + type));
			}
			return replacement_type;
		}
		return type;
	}

	pub fn to_long_string(&self, type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(type_variable, "typeVariable");
		/* final */ let buf: StringBuilder = StringBuilder::new();
		/* final */ let d: GenericDeclaration = type_variable.getGenericDeclaration();
		if d instanceof Class<?> {
			let c: Class<?> = d as Class<?>;
			while true {
				if c.getEnclosingClass() == null {
					buf.insert(0, &c.getName());
					break;
				}
				buf.insert(0, &c.getSimpleName()).insert(0, '.');
				c = c.getEnclosingClass();
			}
		} else if d instanceof Type {
			// not possible as of now
			buf.append(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::to_string(d as Type)?);
		} else {
			buf.append(d);
		}
		return buf.append(':').append(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::type_variable_to_string(type_variable)).toString();
	}

	pub fn to_string(&self, type: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.io.IOException | java.lang.IllegalArgumentException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		Objects::requireNonNull(type, "type");
		if type instanceof Class<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::class_to_string(type as Class<?>)?;
		}
		if type instanceof ParameterizedType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::parameterized_type_to_string(type as ParameterizedType)?;
		}
		if type instanceof WildcardType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::wildcard_type_to_string(type as WildcardType);
		}
		if type instanceof TypeVariable<?> {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::type_variable_to_string(type as TypeVariable<?>);
		}
		if type instanceof GenericArrayType {
			return org::apache::commons::lang3::reflect::type_utils::TypeUtils::generic_array_type_to_string(type as GenericArrayType)?;
		}
		return Err(IllegalArgumentException::new(&ObjectUtils::identity_to_string(type)));
	}

	pub fn types_satisfy_variables(&self, type_variable_map: &/* Java */ java::util::Map /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException) */ -> bool {
		Objects::requireNonNull(type_variable_map, "typeVariableMap");
		// type variable.
		for /* final */ entry in type_variable_map.entrySet() {
			/* final */ let type_var: TypeVariable<?> = entry.getKey();
			/* final */ let type: Type = entry.getValue();
			for /* final */ bound in org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_implicit_bounds(type_var) {
				if !org::apache::commons::lang3::reflect::type_utils::TypeUtils::is_assignable(type, &org::apache::commons::lang3::reflect::type_utils::TypeUtils::substitute_type_variables(bound, type_variable_map)?, type_variable_map)? {
					return false;
				}
			}
		}
		return true;
	}

	fn type_variable_to_string(&self, type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/) /* thrown(java.io.IOException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let builder: StringBuilder = StringBuilder::new(&type_variable.getName());
		/* final */ let bounds: Vec<Type> = type_variable.getBounds();
		if bounds.length > 0 && !(bounds.length == 1 && Object.class.equals(bounds[0])) {
			// https://issues.apache.org/jira/projects/LANG/issues/LANG-1698
			// There must be a better way to avoid a stack overflow on Java 17 and up.
			// Bounds are different in Java 17 and up where instead of Object you can get an interface like Comparable.
			/* final */ let bound: Type = bounds[0];
			let append: bool = true;
			if bound instanceof ParameterizedType {
				/* final */ let raw_type: Type = (bound as ParameterizedType).getRawType();
				if raw_type instanceof Class && (raw_type as Class<?>).isInterface() {
					// Avoid recursion and stack overflow on Java 17 and up.
					append = false;
				}
			}
			if append {
				builder.append(" extends ");
				self.AMP_JOINER.join(builder, bounds)?;
			}
		}
		return builder.toString();
	}

	fn unroll_bounds(&self, type_arguments: &/* Java */ java::util::Map /**/, bounds: &&[/* Java */ java::lang::reflect::Type /**/]) /* thrown(java.lang.IndexOutOfBoundsException) */ -> &[/* Java */ java::lang::reflect::Type /**/] {
		let result: Vec<Type> = bounds;
		let i: i32 = 0;
		while i < result.length {
			{
				/* final */ let unrolled: Type = org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_variables(type_arguments, result[i]);
				if unrolled == null {
					result = ArrayUtils::remove(result, i -= 1 !!!check!!! post decrement)?;
				} else {
					result[i] = unrolled;
				}
			}
			i += 1;
		 }
	
		return result;
	}

	fn unroll_variable_assignments(&self, mut type_variable: &/* Java */ java::lang::reflect::TypeVariable /**/, type_var_assigns: &/* Java */ java::util::Map /**/) -> /* Java */ java::lang::reflect::Type /**/ {
		let result: Type;
		loop { {
			result = type_var_assigns.get(type_variable);
			if !(result instanceof TypeVariable<?>) || result.equals(type_variable) {
				break;
			}
			type_variable = result as TypeVariable<?>;
		}if !(true) break;}
		return result;
	}

	pub fn unroll_variables(&self, mut type_arguments: &/* Java */ java::util::Map /**/, type: &/* Java */ java::lang::reflect::Type /**/) /* thrown(java.lang.IllegalArgumentException | java.lang.IllegalStateException | java.lang.IndexOutOfBoundsException) */ -> /* Java */ java::lang::reflect::Type /**/ {
		if type_arguments == null {
			type_arguments = Collections::emptyMap();
		}
		if org::apache::commons::lang3::reflect::type_utils::TypeUtils::contains_type_variables(type) {
			if type instanceof TypeVariable<?> {
				return org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_variables(type_arguments, &type_arguments.get(type))?;
			}
			if type instanceof ParameterizedType {
				/* final */ let p: ParameterizedType = type as ParameterizedType;
				/* final */ let parameterized_type_arguments: Map<TypeVariable<?>, Type>;
				if p.getOwnerType() == null {
					parameterized_type_arguments = type_arguments;
				} else {
					parameterized_type_arguments = HashMap<>::new(type_arguments);
					parameterized_type_arguments.putAll(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::get_type_arguments(p)?);
				}
				/* final */ let args: Vec<Type> = p.getActualTypeArguments();
				 {
					let i: i32 = 0;
					while i < args.length {
						{
							/* final */ let unrolled: Type = org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_variables(parameterized_type_arguments, args[i])?;
							if unrolled != null {
								args[i] = unrolled;
							}
						}
						i += 1;
					 }
				 }
	
				return org::apache::commons::lang3::reflect::type_utils::TypeUtils::parameterize_with_owner(&p.getOwnerType(), p.getRawType() as Class<?>, args)?;
			}
			if type instanceof WildcardType {
				/* final */ let wild: WildcardType = type as WildcardType;
				return org::apache::commons::lang3::reflect::type_utils::TypeUtils::wildcard_type().with_upper_bounds(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_bounds(type_arguments, &wild.getUpperBounds())?).with_lower_bounds(&org::apache::commons::lang3::reflect::type_utils::TypeUtils::unroll_bounds(type_arguments, &wild.getLowerBounds())?).build();
			}
		}
		return type;
	}

	pub fn wildcard_type(&self) -> org::apache::commons::lang3::reflect::type_utils::WildcardTypeBuilder {
		return WildcardTypeBuilder::new();
	}

	fn wildcard_type_to_string(&self, wildcard_type: &/* Java */ java::lang::reflect::WildcardType /**/) /* thrown(java.io.IOException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let builder: StringBuilder = StringBuilder::new().append('?');
		/* final */ let lower_bounds: Vec<Type> = wildcard_type.getLowerBounds();
		/* final */ let upper_bounds: Vec<Type> = wildcard_type.getUpperBounds();
		if lower_bounds.length > 1 || lower_bounds.length == 1 && lower_bounds[0] != null {
			self.AMP_JOINER.join(&builder.append(" super "), lower_bounds)?;
		} else if upper_bounds.length > 1 || upper_bounds.length == 1 && !Object.class.equals(upper_bounds[0]) {
			self.AMP_JOINER.join(&builder.append(" extends "), upper_bounds)?;
		}
		return builder.toString();
	}

	pub fn wrap<T>(&self, type: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::reflect::typed::Typed {
		return org::apache::commons::lang3::reflect::type_utils::TypeUtils::wrap(type as Type);
	}

	pub fn wrap<T>(&self, type: &/* Java */ java::lang::reflect::Type /**/) -> org::apache::commons::lang3::reflect::typed::Typed {
		return |()|type;
	}

	pub fn new() -> org::apache::commons::lang3::reflect::type_utils::TypeUtils {
	// empty
	}
}

struct GenericArrayTypeImpl {
	component_type: /* Java */ java::lang::reflect::Type /**/,
}

impl GenericArrayTypeImpl {
	fn new(component_type: &/* Java */ java::lang::reflect::Type /**/) -> org::apache::commons::lang3::reflect::type_utils::GenericArrayTypeImpl {
		self.componentType = component_type;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj == self || obj instanceof GenericArrayType && TypeUtils::equals(self, obj as GenericArrayType);
	}

	pub fn get_generic_component_type(&self) -> /* Java */ java::lang::reflect::Type /**/ {
		return self.component_type;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = 67 << 4;
		result |= self.component_type.hashCode();
		return result;
	}

	pub fn to_string(&self) /* thrown(java.io.IOException | java.lang.IllegalArgumentException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		return TypeUtils::to_string(self)?;
	}
}

impl /* Java */ java::lang::reflect::GenericArrayType /**/ for GenericArrayTypeImpl {}

impl /* Java */ java::lang::reflect::Type /**/ for GenericArrayTypeImpl {}

struct ParameterizedTypeImpl {
	raw: /* Java */ java::lang::Class /**/,
	use_owner: /* Java */ java::lang::reflect::Type /**/,
	type_arguments: &[/* Java */ java::lang::reflect::Type /**/],
}

impl ParameterizedTypeImpl {
	fn new(raw_class: &/* Java */ java::lang::Class /**/, use_owner: &/* Java */ java::lang::reflect::Type /**/, type_arguments: &&[/* Java */ java::lang::reflect::Type /**/]) -> org::apache::commons::lang3::reflect::type_utils::ParameterizedTypeImpl {
		self.raw = raw_class;
		self.useOwner = use_owner;
		self.typeArguments = Arrays::copyOf(type_arguments, type_arguments.length, Vec<Type>.class);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj == self || obj instanceof ParameterizedType && TypeUtils::equals(self, obj as ParameterizedType);
	}

	pub fn get_actual_type_arguments(&self) -> &[/* Java */ java::lang::reflect::Type /**/] {
		return self.type_arguments.clone();
	}

	pub fn get_owner_type(&self) -> /* Java */ java::lang::reflect::Type /**/ {
		return self.use_owner;
	}

	pub fn get_raw_type(&self) -> /* Java */ java::lang::reflect::Type /**/ {
		return self.raw;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = 71 << 4;
		result |= self.raw.hashCode();
		result <<= 4;
		result |= Objects::hashCode(self.use_owner);
		result <<= 8;
		result |= Arrays::hashCode(self.type_arguments);
		return result;
	}

	pub fn to_string(&self) /* thrown(java.io.IOException | java.lang.IllegalArgumentException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		return TypeUtils::to_string(self)?;
	}
}

impl /* Java */ java::lang::reflect::ParameterizedType /**/ for ParameterizedTypeImpl {}

impl /* Java */ java::lang::reflect::Type /**/ for ParameterizedTypeImpl {}

pub struct WildcardTypeBuilder {
	upper_bounds: &[/* Java */ java::lang::reflect::Type /**/],
	lower_bounds: &[/* Java */ java::lang::reflect::Type /**/],
}

impl WildcardTypeBuilder {
	fn new() -> org::apache::commons::lang3::reflect::type_utils::WildcardTypeBuilder {
	}

	pub fn build(&self) -> /* Java */ java::lang::reflect::WildcardType /**/ {
		return WildcardTypeImpl::new(self.upper_bounds, self.lower_bounds);
	}

	pub fn with_lower_bounds(&mut self, bounds: &/* Java */ java::lang::reflect::Type /**/) -> org::apache::commons::lang3::reflect::type_utils::WildcardTypeBuilder {
		self.lowerBounds = bounds;
		return self;
	}

	pub fn with_upper_bounds(&mut self, bounds: &/* Java */ java::lang::reflect::Type /**/) -> org::apache::commons::lang3::reflect::type_utils::WildcardTypeBuilder {
		self.upperBounds = bounds;
		return self;
	}
}

impl org::apache::commons::lang3::builder::builder::Builder for WildcardTypeBuilder {}

struct WildcardTypeImpl {
	upper_bounds: &[/* Java */ java::lang::reflect::Type /**/],
	lower_bounds: &[/* Java */ java::lang::reflect::Type /**/],
}

impl WildcardTypeImpl {
	fn new(upper_bounds: &&[/* Java */ java::lang::reflect::Type /**/], lower_bounds: &&[/* Java */ java::lang::reflect::Type /**/]) -> org::apache::commons::lang3::reflect::type_utils::WildcardTypeImpl {
		self.upperBounds = ObjectUtils::get_if_null(upper_bounds, ArrayUtils::EMPTY_TYPE_ARRAY);
		self.lowerBounds = ObjectUtils::get_if_null(lower_bounds, ArrayUtils::EMPTY_TYPE_ARRAY);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj == self || obj instanceof WildcardType && TypeUtils::equals(self, obj as WildcardType);
	}

	pub fn get_lower_bounds(&self) -> &[/* Java */ java::lang::reflect::Type /**/] {
		return self.lower_bounds.clone();
	}

	pub fn get_upper_bounds(&self) -> &[/* Java */ java::lang::reflect::Type /**/] {
		return self.upper_bounds.clone();
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = 73 << 8;
		result |= Arrays::hashCode(self.upper_bounds);
		result <<= 8;
		result |= Arrays::hashCode(self.lower_bounds);
		return result;
	}

	pub fn to_string(&self) /* thrown(java.io.IOException | java.lang.IllegalArgumentException | org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		return TypeUtils::to_string(self)?;
	}
}

impl /* Java */ java::lang::reflect::WildcardType /**/ for WildcardTypeImpl {}

impl /* Java */ java::lang::reflect::Type /**/ for WildcardTypeImpl {}