use java::lang::reflect::Method;
use java::lang::reflect::Modifier;
use java::util::ArrayList;
use java::util::Collections;
use java::util::Comparator;
use java::util::HashMap;
use java::util::HashSet;
use java::util::Iterator;
use java::util::LinkedHashSet;
use java::util::List;
use java::util::Map;
use java::util::Objects;
use java::util::Set;
use java::util::concurrent::atomic::AtomicReference;
use java::util::stream::Collectors;

pub struct ClassUtils {
	interfaces: /* Java */ java::util::Iterator /**/ = Collections::emptyIterator(),
}

impl ClassUtils {
	static MAX_DIMENSIONS: i32 = 255;

	static COMPARATOR: /* Java */ java::util::Comparator /**/ = |(o1, o2)|Objects::compare(&org::apache::commons::lang3::class_utils::ClassUtils::get_name(o1), &org::apache::commons::lang3::class_utils::ClassUtils::get_name(o2), String::compareTo);

	pub static PACKAGE_SEPARATOR_CHAR: u16 = '.';

	pub static PACKAGE_SEPARATOR: /* Java */ java::lang::String /**/ = String::valueOf(PACKAGE_SEPARATOR_CHAR);

	pub static INNER_CLASS_SEPARATOR_CHAR: u16 = '$';

	pub static INNER_CLASS_SEPARATOR: /* Java */ java::lang::String /**/ = String::valueOf(INNER_CLASS_SEPARATOR_CHAR);

	static NAME_PRIMITIVE_MAP: /* Java */ java::util::Map /**/ = HashMap<>::new();

	static PRIMITIVE_WRAPPER_MAP: /* Java */ java::util::Map /**/ = HashMap<>::new();

	static WRAPPER_PRIMITIVE_MAP: /* Java */ java::util::Map /**/ = HashMap<>::new();

	static ABBREVIATION_MAP: /* Java */ java::util::Map /**/;

	static REVERSE_ABBREVIATION_MAP: /* Java */ java::util::Map /**/;

	init {
	    NAME_PRIMITIVE_MAP.put(Boolean.TYPE.getName(), Boolean.TYPE);
	    NAME_PRIMITIVE_MAP.put(Byte.TYPE.getName(), Byte.TYPE);
	    NAME_PRIMITIVE_MAP.put(Character.TYPE.getName(), Character.TYPE);
	    NAME_PRIMITIVE_MAP.put(Double.TYPE.getName(), Double.TYPE);
	    NAME_PRIMITIVE_MAP.put(Float.TYPE.getName(), Float.TYPE);
	    NAME_PRIMITIVE_MAP.put(Integer.TYPE.getName(), Integer.TYPE);
	    NAME_PRIMITIVE_MAP.put(Long.TYPE.getName(), Long.TYPE);
	    NAME_PRIMITIVE_MAP.put(Short.TYPE.getName(), Short.TYPE);
	    NAME_PRIMITIVE_MAP.put(Void.TYPE.getName(), Void.TYPE);
	}

	init {
	    PRIMITIVE_WRAPPER_MAP.put(Boolean.TYPE, Boolean.class);
	    PRIMITIVE_WRAPPER_MAP.put(Byte.TYPE, Byte.class);
	    PRIMITIVE_WRAPPER_MAP.put(Character.TYPE, Character.class);
	    PRIMITIVE_WRAPPER_MAP.put(Short.TYPE, Short.class);
	    PRIMITIVE_WRAPPER_MAP.put(Integer.TYPE, Integer.class);
	    PRIMITIVE_WRAPPER_MAP.put(Long.TYPE, Long.class);
	    PRIMITIVE_WRAPPER_MAP.put(Double.TYPE, Double.class);
	    PRIMITIVE_WRAPPER_MAP.put(Float.TYPE, Float.class);
	    PRIMITIVE_WRAPPER_MAP.put(Void.TYPE, Void.TYPE);
	}

	init {
	    PRIMITIVE_WRAPPER_MAP.forEach((primitiveClass, wrapperClass) -> {
	        if (!primitiveClass.equals(wrapperClass)) {
	            WRAPPER_PRIMITIVE_MAP.put(wrapperClass, primitiveClass);
	        }
	    });
	}

	init {
	    final Map<String, String> map = new HashMap<>();
	    map.put(Integer.TYPE.getName(), "I");
	    map.put(Boolean.TYPE.getName(), "Z");
	    map.put(Float.TYPE.getName(), "F");
	    map.put(Long.TYPE.getName(), "J");
	    map.put(Short.TYPE.getName(), "S");
	    map.put(Byte.TYPE.getName(), "B");
	    map.put(Double.TYPE.getName(), "D");
	    map.put(Character.TYPE.getName(), "C");
	    ABBREVIATION_MAP = Collections.unmodifiableMap(map);
	    REVERSE_ABBREVIATION_MAP = Collections.unmodifiableMap(map.entrySet().stream().collect(Collectors.toMap(Map.Entry::getValue, Map.Entry::getKey)));
	}

	pub fn comparator(&self) -> /* Java */ java::util::Comparator /**/ {
		return self.COMPARATOR;
	}

	pub fn convert_classes_to_class_names(&self, classes: &/* Java */ java::util::List /**/) -> /* Java */ java::util::List /**/ {
		return  if classes == null { null } else { classes.stream().map(|e|org::apache::commons::lang3::class_utils::ClassUtils::get_name(e, null)).collect(&Collectors::toList()) };
	}

	pub fn convert_class_names_to_classes(&self, class_names: &/* Java */ java::util::List /**/) -> /* Java */ java::util::List /**/ {
		if class_names == null {
			return null;
		}
		/* final */ let classes: List<Class<?>> = ArrayList<>::new(&class_names.size());
		class_names.forEach(|class_name|{
			let r0 = 'try0: {
				classes.add(&Class::forName(class_name));
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ Exception) => {
					classes.add(null);
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		});
		return classes;
	}

	pub fn get_abbreviated_name(&self, cls: &/* Java */ java::lang::Class /**/, length_hint: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if cls == null {
			return StringUtils::EMPTY;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_abbreviated_name(&cls.getName(), length_hint)?;
	}

	pub fn get_abbreviated_name(&self, class_name: &/* Java */ java::lang::String /**/, length_hint: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		if length_hint <= 0 {
			return Err(IllegalArgumentException::new("len must be > 0"));
		}
		if class_name == null {
			return StringUtils::EMPTY;
		}
		if class_name.length() <= length_hint {
			return class_name;
		}
		/* final */ let abbreviated: Vec<char> = class_name.toCharArray();
		let target: i32 = 0;
		let source: i32 = 0;
		while source < abbreviated.length {
			// copy the next part
			let run_ahead_target: i32 = target;
			while source < abbreviated.length && abbreviated[source] != '.' {
				abbreviated[run_ahead_target += 1 !!!check!!! post increment] = abbreviated[source += 1 !!!check!!! post increment];
			}
			target += 1;
			if org::apache::commons::lang3::class_utils::ClassUtils::use_full(run_ahead_target, source, abbreviated.length, length_hint) || target > run_ahead_target {
				target = run_ahead_target;
			}
			// copy the '.' unless it was the last part
			if source < abbreviated.length {
				abbreviated[target += 1 !!!check!!! post increment] = abbreviated[source += 1 !!!check!!! post increment];
			}
		}
		return String::new(abbreviated, 0, target);
	}

	pub fn get_all_interfaces(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		if cls == null {
			return null;
		}
		/* final */ let interfaces_found: LinkedHashSet<Class<?>> = LinkedHashSet<>::new();
		org::apache::commons::lang3::class_utils::ClassUtils::get_all_interfaces(cls, interfaces_found);
		return ArrayList<>::new(interfaces_found);
	}

	fn get_all_interfaces(&self, mut cls: &/* Java */ java::lang::Class /**/, interfaces_found: &/* Java */ java::util::HashSet /**/) {
		while cls != null {
			/* final */ let interfaces: Vec<Class<?>> = cls.getInterfaces();
			for /* final */ i in interfaces {
				if interfaces_found.add(i) {
					org::apache::commons::lang3::class_utils::ClassUtils::get_all_interfaces(i, interfaces_found);
				}
			}
			cls = cls.getSuperclass();
		}
	}

	pub fn get_all_superclasses(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::List /**/ {
		if cls == null {
			return null;
		}
		/* final */ let classes: List<Class<?>> = ArrayList<>::new();
		let superclass: Class<?> = cls.getSuperclass();
		while superclass != null {
			classes.add(superclass);
			superclass = superclass.getSuperclass();
		}
		return classes;
	}

	pub fn get_canonical_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_canonical_name(cls, StringUtils::EMPTY);
	}

	pub fn get_canonical_name(&self, cls: &/* Java */ java::lang::Class /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if cls == null {
			return value_if_null;
		}
		/* final */ let canonical_name: String = cls.getCanonicalName();
		return  if canonical_name == null { value_if_null } else { canonical_name };
	}

	pub fn get_canonical_name(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_canonical_name(object, StringUtils::EMPTY);
	}

	pub fn get_canonical_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if object == null {
			return value_if_null;
		}
		/* final */ let canonical_name: String = object.getClass().getCanonicalName();
		return  if canonical_name == null { value_if_null } else { canonical_name };
	}

	fn get_canonical_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		let class_name: String = StringUtils::delete_whitespace(name);
		if class_name == null {
			return null;
		}
		let dim: i32 = 0;
		/* final */ let len: i32 = class_name.length();
		while dim < len && class_name.charAt(dim) == '[' {
			dim += 1;
			if dim > self.MAX_DIMENSIONS {
				return Err(IllegalArgumentException::new(&String::format("Maximum array dimension %d exceeded", self.MAX_DIMENSIONS)));
			}
		}
		if dim >= len {
			return Err(IllegalArgumentException::new(&String::format("Invalid class name %s", name)));
		}
		if dim < 1 {
			return class_name;
		}
		class_name = class_name.substring(dim);
		if class_name.startsWith("L") {
			if !class_name.endsWith(";") || class_name.length() < 3 {
				return Err(IllegalArgumentException::new(&String::format("Invalid class name %s", name)));
			}
			class_name = class_name.substring(1, class_name.length() - 1);
		} else if class_name.length() == 1 {
			/* final */ let primitive: String = self.REVERSE_ABBREVIATION_MAP.get(&class_name.substring(0, 1));
			if primitive == null {
				return Err(IllegalArgumentException::new(&String::format("Invalid class name %s", name)));
			}
			class_name = primitive;
		} else {
			return Err(IllegalArgumentException::new(&String::format("Invalid class name %s", name)));
		}
		/* final */ let canonical_class_name_buffer: StringBuilder = StringBuilder::new(class_name.length() + dim * 2);
		canonical_class_name_buffer.append(class_name);
		 {
			let i: i32 = 0;
			while i < dim {
				{
					canonical_class_name_buffer.append("[]");
				}
				i += 1;
			 }
		 }
	
		return canonical_class_name_buffer.toString();
	}

	pub fn get_class(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/, class_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.ClassNotFoundException) */ -> /* Java */ java::lang::Class /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_class(class_loader, class_name, true)?;
	}

	pub fn get_class(&self, class_loader: &/* Java */ java::lang::ClassLoader /**/, class_name: &/* Java */ java::lang::String /**/, initialize: bool) /* thrown(java.lang.ClassNotFoundException) */ -> /* Java */ java::lang::Class /**/ {
		// This method was re-written to avoid recursion and stack overflows found by fuzz testing.
		let next: String = class_name;
		let last_dot_index: i32 = -1;
		loop { {
			let r0 = 'try0: {
				/* final */ let clazz: Class<?> = org::apache::commons::lang3::class_utils::ClassUtils::get_primitive_class(next);
				return  if clazz != null { clazz } else { Class::forName(&org::apache::commons::lang3::class_utils::ClassUtils::to_canonical_name(next), initialize, class_loader) };
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ ClassNotFoundException) => {
					last_dot_index = next.lastIndexOf(self.PACKAGE_SEPARATOR_CHAR);
					if last_dot_index != -1 {
						next = next.substring(0, last_dot_index) + self.INNER_CLASS_SEPARATOR_CHAR + next.substring(last_dot_index + 1);
					}
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}if !(last_dot_index != -1) break;}
		break 'try0 Err(ClassNotFoundException::new(next));
	}

	pub fn get_class(&self, class_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.ClassNotFoundException) */ -> /* Java */ java::lang::Class /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_class(class_name, true);
	}

	pub fn get_class(&self, class_name: &/* Java */ java::lang::String /**/, initialize: bool) /* thrown(java.lang.ClassNotFoundException) */ -> /* Java */ java::lang::Class /**/ {
		/* final */ let context_c_l: ClassLoader = Thread::currentThread().getContextClassLoader();
		/* final */ let loader: ClassLoader =  if context_c_l == null { ClassUtils.class.getClassLoader() } else { context_c_l };
		return org::apache::commons::lang3::class_utils::ClassUtils::get_class(loader, class_name, initialize)?;
	}

	pub fn get_component_type<T>(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::Class /**/ {
		return  if cls == null { null } else { cls.getComponentType() as Class<T> };
	}

	pub fn get_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_name(cls, StringUtils::EMPTY);
	}

	pub fn get_name(&self, cls: &/* Java */ java::lang::Class /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_name(cls, value_if_null, false);
	}

	fn get_name(&self, cls: &/* Java */ java::lang::Class /**/, value_if_null: &/* Java */ java::lang::String /**/, simple: bool) -> /* Java */ java::lang::String /**/ {
		return  if cls == null { value_if_null } else {  if simple { cls.getSimpleName() } else { cls.getName() } };
	}

	pub fn get_name(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_name(object, StringUtils::EMPTY);
	}

	pub fn get_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if object == null { value_if_null } else { object.getClass().getName() };
	}

	pub fn get_package_canonical_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		if cls == null {
			return StringUtils::EMPTY;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_package_canonical_name(&cls.getName());
	}

	pub fn get_package_canonical_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if object == null {
			return value_if_null;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_package_canonical_name(&object.getClass().getName());
	}

	pub fn get_package_canonical_name(&self, name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_package_name(&org::apache::commons::lang3::class_utils::ClassUtils::get_canonical_name(name)?);
	}

	pub fn get_package_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		if cls == null {
			return StringUtils::EMPTY;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_package_name(&cls.getName());
	}

	pub fn get_package_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if object == null {
			return value_if_null;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_package_name(&object.getClass());
	}

	pub fn get_package_name(&self, mut class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(class_name) {
			return StringUtils::EMPTY;
		}
		let i: i32 = 0;
		// Strip array encoding
		while class_name.charAt(i) == '[' {
			i += 1;
		}
		class_name = class_name.substring(i);
		// Strip Object type encoding
		if class_name.charAt(0) == 'L' && class_name.charAt(class_name.length() - 1) == ';' {
			class_name = class_name.substring(1);
		}
		i = class_name.lastIndexOf(self.PACKAGE_SEPARATOR_CHAR);
		if i == -1 {
			return StringUtils::EMPTY;
		}
		return class_name.substring(0, i);
	}

	fn get_primitive_class(&self, class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Class /**/ {
		return self.NAME_PRIMITIVE_MAP.get(class_name);
	}

	pub fn get_public_method(&self, cls: &/* Java */ java::lang::Class /**/, method_name: &/* Java */ java::lang::String /**/, parameter_types: &/* Java */ java::lang::Class /**/) /* thrown(java.lang.NoSuchMethodException) */ -> /* Java */ java::lang::reflect::Method /**/ {
		/* final */ let declared_method: Method = cls.getMethod(method_name, parameter_types);
		if org::apache::commons::lang3::class_utils::ClassUtils::is_public(&declared_method.getDeclaringClass()) {
			return declared_method;
		}
		/* final */ let candidate_classes: List<Class<?>> = ArrayList<>::new(&org::apache::commons::lang3::class_utils::ClassUtils::get_all_interfaces(cls));
		candidate_classes.addAll(&org::apache::commons::lang3::class_utils::ClassUtils::get_all_superclasses(cls));
		for /* final */ candidate_class in candidate_classes {
			if !org::apache::commons::lang3::class_utils::ClassUtils::is_public(candidate_class) {
				continue;
			}
			/* final */ let candidate_method: Method;
			let r0 = 'try0: {
				candidate_method = candidate_class.getMethod(method_name, parameter_types);
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ NoSuchMethodException) => {
					continue;
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
			if Modifier::isPublic(&candidate_method.getDeclaringClass().getModifiers()) {
				return candidate_method;
			}
		}
		break 'try0 Err(NoSuchMethodException::new("Can't find a public method for " + method_name + " " + ArrayUtils::to_string(parameter_types)));
	}

	pub fn get_short_canonical_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return  if cls == null { StringUtils::EMPTY } else { org::apache::commons::lang3::class_utils::ClassUtils::get_short_canonical_name(&cls.getCanonicalName()) };
	}

	pub fn get_short_canonical_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if object == null { value_if_null } else { org::apache::commons::lang3::class_utils::ClassUtils::get_short_canonical_name(&object.getClass()) };
	}

	pub fn get_short_canonical_name(&self, canonical_name: &/* Java */ java::lang::String /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_short_class_name(&org::apache::commons::lang3::class_utils::ClassUtils::get_canonical_name(canonical_name)?);
	}

	pub fn get_short_class_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		if cls == null {
			return StringUtils::EMPTY;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_short_class_name(&cls.getName());
	}

	pub fn get_short_class_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if object == null {
			return value_if_null;
		}
		return org::apache::commons::lang3::class_utils::ClassUtils::get_short_class_name(&object.getClass());
	}

	pub fn get_short_class_name(&self, mut class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		if StringUtils::is_empty(class_name) {
			return StringUtils::EMPTY;
		}
		/* final */ let array_prefix: StringBuilder = StringBuilder::new();
		// Handle array encoding
		if class_name.startsWith("[") {
			while class_name.charAt(0) == '[' {
				class_name = class_name.substring(1);
				array_prefix.append("[]");
			}
			// Strip Object type encoding
			if class_name.charAt(0) == 'L' && class_name.charAt(class_name.length() - 1) == ';' {
				class_name = class_name.substring(1, class_name.length() - 1);
			}
			if self.REVERSE_ABBREVIATION_MAP.containsKey(class_name) {
				class_name = self.REVERSE_ABBREVIATION_MAP.get(class_name);
			}
		}
		/* final */ let last_dot_idx: i32 = class_name.lastIndexOf(self.PACKAGE_SEPARATOR_CHAR);
		/* final */ let inner_idx: i32 = class_name.indexOf(self.INNER_CLASS_SEPARATOR_CHAR,  if last_dot_idx == -1 { 0 } else { last_dot_idx + 1 });
		let out: String = class_name.substring(last_dot_idx + 1);
		if inner_idx != -1 {
			out = out.replace(self.INNER_CLASS_SEPARATOR_CHAR, self.PACKAGE_SEPARATOR_CHAR);
		}
		return out + array_prefix;
	}

	pub fn get_simple_name(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_simple_name(cls, StringUtils::EMPTY);
	}

	pub fn get_simple_name(&self, cls: &/* Java */ java::lang::Class /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if cls == null { value_if_null } else { cls.getSimpleName() };
	}

	pub fn get_simple_name(&self, object: &/* Java */ java::lang::Object /**/) -> /* Java */ java::lang::String /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::get_simple_name(object, StringUtils::EMPTY);
	}

	pub fn get_simple_name(&self, object: &/* Java */ java::lang::Object /**/, value_if_null: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		return  if object == null { value_if_null } else { object.getClass().getSimpleName() };
	}

	pub fn hierarchy(&self, type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::Iterable /**/ {
		return org::apache::commons::lang3::class_utils::ClassUtils::hierarchy(type, Interfaces::EXCLUDE);
	}

	pub fn hierarchy(&self, type: &/* Java */ java::lang::Class /**/, interfaces_behavior: &org::apache::commons::lang3::class_utils::Interfaces) -> /* Java */ java::lang::Iterable /**/ {
		/* final */ let classes: Iterable<Class<?>> = |()|{
			/* final */ let next: AtomicReference<Class<?>> = AtomicReference<>::new(type);
			return Iterator<Class<?>>::new() {
				pub fn has_next(&self) -> bool {
					return next.get() != null;
				}
	
				pub fn next(&self) -> Class<?> {
					return next.getAndUpdate(Class::getSuperclass);
				}
	
				pub fn remove(&self) {
					return Err(UnsupportedOperationException::new());
				}
	
			};
		};
		if interfaces_behavior != Interfaces::INCLUDE {
			return classes;
		}
		return |()|{
			/* final */ let seen_interfaces: Set<Class<?>> = HashSet<>::new();
			/* final */ let wrapped: Iterator<Class<?>> = classes.iterator();
			return Iterator<Class<?>>::new() {
				let interfaces: Iterator<Class<?>> = Collections::emptyIterator(),
				pub fn has_next(&self) -> bool {
					return self.interfaces.hasNext() || wrapped.hasNext();
				}
	
				pub fn next(&self) -> Class<?> {
					if self.interfaces.hasNext() {
						/* final */ let next_interface: Class<?> = self.interfaces.next();
						seen_interfaces.add(next_interface);
						return next_interface;
					}
					/* final */ let next_superclass: Class<?> = wrapped.next();
					/* final */ let current_interfaces: Set<Class<?>> = LinkedHashSet<>::new();
					self.walkInterfaces(current_interfaces, next_superclass);
					self.interfaces = current_interfaces.iterator();
					return next_superclass;
				}
	
				pub fn remove(&self) {
					return Err(UnsupportedOperationException::new());
				}
	
				fn walk_interfaces(&self, /* final */ add_to: &Set<Class<?>>, /* final */ c: &Class<?>) {
					for /* final */ iface in c.getInterfaces() {
						if !seen_interfaces.contains(iface) {
							add_to.add(iface);
						}
						self.walkInterfaces(add_to, iface);
					}
				}
	
			};
		};
	}

	pub fn has_next(&self) -> bool {
		return self.interfaces.hasNext() || wrapped.hasNext();
	}

	pub fn next(&mut self) -> /* Java */ java::lang::Class /**/ {
		if self.interfaces.hasNext() {
			/* final */ let next_interface: Class<?> = self.interfaces.next();
			seen_interfaces.add(next_interface);
			return next_interface;
		}
		/* final */ let next_superclass: Class<?> = wrapped.next();
		/* final */ let current_interfaces: Set<Class<?>> = LinkedHashSet<>::new();
		self.walkInterfaces(current_interfaces, next_superclass);
		self.interfaces = current_interfaces.iterator();
		return next_superclass;
	}

	pub fn remove(&self) /* thrown(java.lang.UnsupportedOperationException) */ {
		return Err(UnsupportedOperationException::new());
	}

	fn walk_interfaces(&self, add_to: &/* Java */ java::util::Set /**/, c: &/* Java */ java::lang::Class /**/) {
		for /* final */ iface in c.getInterfaces() {
			if !seen_interfaces.contains(iface) {
				add_to.add(iface);
			}
			self.walkInterfaces(add_to, iface);
		}
	}

	pub fn is_assignable(&self, cls: &/* Java */ java::lang::Class /**/, to_class: &/* Java */ java::lang::Class /**/) -> bool {
		return org::apache::commons::lang3::class_utils::ClassUtils::is_assignable(cls, to_class, true);
	}

	pub fn is_assignable(&self, mut cls: &/* Java */ java::lang::Class /**/, to_class: &/* Java */ java::lang::Class /**/, autoboxing: bool) -> bool {
		if to_class == null {
			return false;
		}
		// have to check for null, as isAssignableFrom doesn't
		if cls == null {
			return !to_class.isPrimitive();
		}
		// autoboxing:
		if autoboxing {
			if cls.isPrimitive() && !to_class.isPrimitive() {
				cls = org::apache::commons::lang3::class_utils::ClassUtils::primitive_to_wrapper(cls);
				if cls == null {
					return false;
				}
			}
			if to_class.isPrimitive() && !cls.isPrimitive() {
				cls = org::apache::commons::lang3::class_utils::ClassUtils::wrapper_to_primitive(cls);
				if cls == null {
					return false;
				}
			}
		}
		if cls.equals(to_class) {
			return true;
		}
		if cls.isPrimitive() {
			if !to_class.isPrimitive() {
				return false;
			}
			if Integer::TYPE.equals(cls) {
				return Long::TYPE.equals(to_class) || Float::TYPE.equals(to_class) || Double::TYPE.equals(to_class);
			}
			if Long::TYPE.equals(cls) {
				return Float::TYPE.equals(to_class) || Double::TYPE.equals(to_class);
			}
			if Boolean::TYPE.equals(cls) {
				return false;
			}
			if Double::TYPE.equals(cls) {
				return false;
			}
			if Float::TYPE.equals(cls) {
				return Double::TYPE.equals(to_class);
			}
			if Character::TYPE.equals(cls) || Short::TYPE.equals(cls) {
				return Integer::TYPE.equals(to_class) || Long::TYPE.equals(to_class) || Float::TYPE.equals(to_class) || Double::TYPE.equals(to_class);
			}
			if Byte::TYPE.equals(cls) {
				return Short::TYPE.equals(to_class) || Integer::TYPE.equals(to_class) || Long::TYPE.equals(to_class) || Float::TYPE.equals(to_class) || Double::TYPE.equals(to_class);
			}
			// should never get here
			return false;
		}
		return to_class.isAssignableFrom(cls);
	}

	pub fn is_assignable(&self, class_array: &&[/* Java */ java::lang::Class /**/], to_class_array: &/* Java */ java::lang::Class /**/) -> bool {
		return org::apache::commons::lang3::class_utils::ClassUtils::is_assignable(class_array, to_class_array, true);
	}

	pub fn is_assignable(&self, mut class_array: &&[/* Java */ java::lang::Class /**/], mut to_class_array: &&[/* Java */ java::lang::Class /**/], autoboxing: bool) -> bool {
		if !ArrayUtils::is_same_length(class_array, to_class_array) {
			return false;
		}
		class_array = ArrayUtils::null_to_empty(class_array);
		to_class_array = ArrayUtils::null_to_empty(to_class_array);
		 {
			let i: i32 = 0;
			while i < class_array.length {
				{
					if !org::apache::commons::lang3::class_utils::ClassUtils::is_assignable(class_array[i], to_class_array[i], autoboxing) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	pub fn is_inner_class(&self, cls: &/* Java */ java::lang::Class /**/) -> bool {
		return cls != null && cls.getEnclosingClass() != null;
	}

	pub fn is_primitive_or_wrapper(&self, type: &/* Java */ java::lang::Class /**/) -> bool {
		return type != null && type.isPrimitive() || org::apache::commons::lang3::class_utils::ClassUtils::is_primitive_wrapper(type);
	}

	pub fn is_primitive_wrapper(&self, type: &/* Java */ java::lang::Class /**/) -> bool {
		return self.WRAPPER_PRIMITIVE_MAP.containsKey(type);
	}

	pub fn is_public(&self, cls: &/* Java */ java::lang::Class /**/) -> bool {
		return Modifier::isPublic(&cls.getModifiers());
	}

	pub fn primitives_to_wrappers(&self, classes: &/* Java */ java::lang::Class /**/) -> &[/* Java */ java::lang::Class /**/] {
		if classes == null {
			return null;
		}
		if classes.length == 0 {
			return classes;
		}
		return ArrayUtils::set_all(: [Option<Class>; classes.length] = [None; classes.length], |i|org::apache::commons::lang3::class_utils::ClassUtils::primitive_to_wrapper(classes[i]));
	}

	pub fn primitive_to_wrapper(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::Class /**/ {
		return  if cls != null && cls.isPrimitive() { self.PRIMITIVE_WRAPPER_MAP.get(cls) } else { cls };
	}

	fn to_canonical_name(&self, class_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::String /**/ {
		let canonical_name: String = StringUtils::delete_whitespace(class_name);
		Objects::requireNonNull(canonical_name, "className");
		/* final */ let array_marker: String = "[]";
		if canonical_name.endsWith(array_marker) {
			/* final */ let class_name_buffer: StringBuilder = StringBuilder::new();
			while canonical_name.endsWith(array_marker) {
				canonical_name = canonical_name.substring(0, canonical_name.length() - 2);
				class_name_buffer.append("[");
			}
			/* final */ let abbreviation: String = self.ABBREVIATION_MAP.get(canonical_name);
			if abbreviation != null {
				class_name_buffer.append(abbreviation);
			} else {
				class_name_buffer.append("L").append(canonical_name).append(";");
			}
			canonical_name = class_name_buffer.toString();
		}
		return canonical_name;
	}

	pub fn to_class(&self, array: &/* Java */ java::lang::Object /**/) -> &[/* Java */ java::lang::Class /**/] {
		if array == null {
			return null;
		}
		if array.length == 0 {
			return ArrayUtils::EMPTY_CLASS_ARRAY;
		}
		return ArrayUtils::set_all(: [Option<Class>; array.length] = [None; array.length], |i| if array[i] == null { null } else { array[i].getClass() });
	}

	fn use_full(&self, run_ahead_target: i32, source: i32, original_length: i32, desired_length: i32) -> bool {
		return source >= original_length || run_ahead_target + original_length - source <= desired_length;
	}

	pub fn wrappers_to_primitives(&self, classes: &/* Java */ java::lang::Class /**/) -> &[/* Java */ java::lang::Class /**/] {
		if classes == null {
			return null;
		}
		if classes.length == 0 {
			return classes;
		}
		return ArrayUtils::set_all(: [Option<Class>; classes.length] = [None; classes.length], |i|org::apache::commons::lang3::class_utils::ClassUtils::wrapper_to_primitive(classes[i]));
	}

	pub fn wrapper_to_primitive(&self, cls: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::Class /**/ {
		return self.WRAPPER_PRIMITIVE_MAP.get(cls);
	}

	pub fn new() -> org::apache::commons::lang3::class_utils::ClassUtils {
	// empty
	}
}

pub enum Interfaces;