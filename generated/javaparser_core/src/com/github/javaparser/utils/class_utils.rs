use java::util::HashMap;
use java::util::Map;

pub struct ClassUtils;

impl ClassUtils {
	static primitiveWrapperMap: /* Java */ java::util::Map /**/ = HashMap<>::new();

	static wrapperPrimitiveMap: /* Java */ java::util::Map /**/ = HashMap<>::new();

	init {
	    primitiveWrapperMap.put(Boolean.TYPE, Boolean.class);
	    primitiveWrapperMap.put(Byte.TYPE, Byte.class);
	    primitiveWrapperMap.put(Character.TYPE, Character.class);
	    primitiveWrapperMap.put(Short.TYPE, Short.class);
	    primitiveWrapperMap.put(Integer.TYPE, Integer.class);
	    primitiveWrapperMap.put(Long.TYPE, Long.class);
	    primitiveWrapperMap.put(Double.TYPE, Double.class);
	    primitiveWrapperMap.put(Float.TYPE, Float.class);
	    primitiveWrapperMap.put(Void.TYPE, Void.TYPE);
	}

	init {
	    for (final Class<?> primitiveClass : primitiveWrapperMap.keySet()) {
	        final Class<?> wrapperClass = primitiveWrapperMap.get(primitiveClass);
	        if (!primitiveClass.equals(wrapperClass)) {
	            wrapperPrimitiveMap.put(wrapperClass, primitiveClass);
	        }
	    }
	}

	pub fn is_primitive_or_wrapper(&self, type: &/* Java */ java::lang::Class /**/) -> bool {
		if type == null {
			return false;
		}
		return type.isPrimitive() || com::github::javaparser::utils::class_utils::ClassUtils::is_primitive_wrapper(type);
	}

	pub fn is_primitive_wrapper(&self, type: &/* Java */ java::lang::Class /**/) -> bool {
		return self.wrapper_primitive_map.containsKey(type);
	}
}