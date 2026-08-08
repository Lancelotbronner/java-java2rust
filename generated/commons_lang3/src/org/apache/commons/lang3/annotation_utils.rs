use java::lang::annotation::Annotation;
use java::lang::reflect::Method;
use java::util::Arrays;
use crate::org::apache::commons::lang3::builder::ToStringBuilder;
use crate::org::apache::commons::lang3::builder::ToStringStyle;
use crate::org::apache::commons::lang3::exception::UncheckedException;

pub struct AnnotationUtils;

impl AnnotationUtils {
	static TO_STRING_STYLE: org::apache::commons::lang3::builder::to_string_style::ToStringStyle = ToStringStyle::new() {
		/* TraditionalJavadocComment/**
	 * Serialization version
	 */
	
		*/ /* static final */ let serial_version_u_i_d: i64 = 1,
		{
			self.set_default_full_detail(true);
			self.set_array_content_detail(true);
			self.set_use_class_name(true);
			self.set_use_short_class_name(true);
			self.set_use_identity_hash_code(false);
			self.set_content_start("(");
			self.set_content_end(")");
			self.set_field_separator(", ");
			self.set_array_start("[");
			self.set_array_end("]");
		}/* TraditionalJavadocComment/**
	 * {@inheritDoc}
	 */
	
		*/ /* protected */ fn append_detail(&self, /* final */ buffer: &StringBuffer, /* final */ field_name: &String, value: &Object) {
			if value instanceof Annotation {
				value = AnnotationUtils::to_string(value as Annotation)?;
			}
			super.append_detail(buffer, field_name, value);
		}
	
		/* TraditionalJavadocComment/**
	 * {@inheritDoc}
	 */
	
		*/ /* protected */ fn get_short_class_name(&self, /* final */ cls: &Class<?>) -> String {
			// formatter:off
			return ClassUtils::get_all_interfaces(cls).stream().filter(Annotation.class::isAssignableFrom).findFirst().map(|iface|"@" + iface.getName()).orElse(StringUtils::EMPTY);
		// formatter:on
		}
	
	};

	fn annotation_array_member_equals(&self, a1: &&[/* Java */ java::lang::annotation::Annotation /**/], a2: &&[/* Java */ java::lang::annotation::Annotation /**/]) -> bool {
		if a1.length != a2.length {
			return false;
		}
		 {
			let i: i32 = 0;
			while i < a1.length {
				{
					if !org::apache::commons::lang3::annotation_utils::AnnotationUtils::equals(a1[i], a2[i]) {
						return false;
					}
				}
				i += 1;
			 }
		 }
	
		return true;
	}

	fn array_member_equals(&self, component_type: &/* Java */ java::lang::Class /**/, o1: &/* Java */ java::lang::Object /**/, o2: &/* Java */ java::lang::Object /**/) -> bool {
		if component_type.isAnnotation() {
			return org::apache::commons::lang3::annotation_utils::AnnotationUtils::annotation_array_member_equals(o1 as Vec<Annotation>, o2 as Vec<Annotation>);
		}
		if component_type.equals(Byte::TYPE) {
			return Arrays::equals(o1 as Vec<i8>, o2 as Vec<i8>);
		}
		if component_type.equals(Short::TYPE) {
			return Arrays::equals(o1 as Vec<i16>, o2 as Vec<i16>);
		}
		if component_type.equals(Integer::TYPE) {
			return Arrays.equals(o1 as Vec<i32>, o2 as Vec<i32>);
		}
		if component_type.equals(Character::TYPE) {
			return Arrays::equals(o1 as Vec<char>, o2 as Vec<char>);
		}
		if component_type.equals(Long::TYPE) {
			return Arrays::equals(o1 as Vec<i64>, o2 as Vec<i64>);
		}
		if component_type.equals(Float::TYPE) {
			return Arrays.equals(o1 as Vec<f32>, o2 as Vec<f32>);
		}
		if component_type.equals(Double::TYPE) {
			return Arrays::equals(o1 as Vec<f64>, o2 as Vec<f64>);
		}
		if component_type.equals(Boolean::TYPE) {
			return Arrays::equals(o1 as Vec<bool>, o2 as Vec<bool>);
		}
		return Arrays::equals(o1 as Vec<Object>, o2 as Vec<Object>);
	}

	fn array_member_hash(&self, component_type: &/* Java */ java::lang::Class /**/, o: &/* Java */ java::lang::Object /**/) -> i32 {
		if component_type.equals(Byte::TYPE) {
			return Arrays::hashCode(o as Vec<i8>);
		}
		if component_type.equals(Short::TYPE) {
			return Arrays::hashCode(o as Vec<i16>);
		}
		if component_type.equals(Integer::TYPE) {
			return Arrays.hashCode(o as Vec<i32>);
		}
		if component_type.equals(Character::TYPE) {
			return Arrays::hashCode(o as Vec<char>);
		}
		if component_type.equals(Long::TYPE) {
			return Arrays::hashCode(o as Vec<i64>);
		}
		if component_type.equals(Float::TYPE) {
			return Arrays.hashCode(o as Vec<f32>);
		}
		if component_type.equals(Double::TYPE) {
			return Arrays::hashCode(o as Vec<f64>);
		}
		if component_type.equals(Boolean::TYPE) {
			return Arrays::hashCode(o as Vec<bool>);
		}
		return Arrays::hashCode(o as Vec<Object>);
	}

	pub fn equals(&self, a1: &/* Java */ java::lang::annotation::Annotation /**/, a2: &/* Java */ java::lang::annotation::Annotation /**/) -> bool {
		if a1 == a2 {
			return true;
		}
		if a1 == null || a2 == null {
			return false;
		}
		/* final */ let type1: Class<? extends Annotation> = a1.annotationType();
		/* final */ let type2: Class<? extends Annotation> = a2.annotationType();
		Validate::not_null(type1, "Annotation %s with null annotationType()", a1);
		Validate::not_null(type2, "Annotation %s with null annotationType()", a2);
		if !type1.equals(type2) {
			return false;
		}
		let r0 = 'try0: {
			for /* final */ m in type1.getDeclaredMethods() {
				if m.getParameterTypes().length == 0 && org::apache::commons::lang3::annotation_utils::AnnotationUtils::is_valid_annotation_member_type(&m.getReturnType()) {
					/* final */ let v1: Object = m.invoke(a1);
					/* final */ let v2: Object = m.invoke(a2);
					if !org::apache::commons::lang3::annotation_utils::AnnotationUtils::member_equals(&m.getReturnType(), v1, v2) {
						return false;
					}
				}
			}
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ ReflectiveOperationException) => {
				return false;
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
		return true;
	}

	pub fn hash_code(&self, a: &/* Java */ java::lang::annotation::Annotation /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedException | java.lang.IllegalStateException) */ -> i32 {
		let result: i32 = 0;
		/* final */ let type: Class<? extends Annotation> = a.annotationType();
		for /* final */ m in type.getDeclaredMethods() {
			let r0 = 'try0: {
				/* final */ let value: Object = m.invoke(a);
				if value == null {
					break 'try0 Err(IllegalStateException::new(&String::format("Annotation method %s returned null", m)));
				}
				result += org::apache::commons::lang3::annotation_utils::AnnotationUtils::hash_member(&m.getName(), value);
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ ReflectiveOperationException) => {
					return Err(UncheckedException::new(ex));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		return result;
	}

	fn hash_member(&self, name: &/* Java */ java::lang::String /**/, value: &/* Java */ java::lang::Object /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedException | java.lang.IllegalStateException) */ -> i32 {
		/* final */ let part1: i32 = name.hashCode() * 127;
		if ObjectUtils::is_array(value) {
			return part1 ^ org::apache::commons::lang3::annotation_utils::AnnotationUtils::array_member_hash(&value.getClass().getComponentType(), value);
		}
		if value instanceof Annotation {
			return part1 ^ org::apache::commons::lang3::annotation_utils::AnnotationUtils::hash_code(value as Annotation)?;
		}
		return part1 ^ value.hashCode();
	}

	pub fn is_valid_annotation_member_type(&self, mut type: &/* Java */ java::lang::Class /**/) -> bool {
		if type == null {
			return false;
		}
		if type.isArray() {
			type = type.getComponentType();
		}
		return type.isPrimitive() || type.isEnum() || type.isAnnotation() || String.class.equals(type) || Class.class.equals(type);
	}

	fn member_equals(&self, type: &/* Java */ java::lang::Class /**/, o1: &/* Java */ java::lang::Object /**/, o2: &/* Java */ java::lang::Object /**/) -> bool {
		if o1 == o2 {
			return true;
		}
		if o1 == null || o2 == null {
			return false;
		}
		if type.isArray() {
			return org::apache::commons::lang3::annotation_utils::AnnotationUtils::array_member_equals(&type.getComponentType(), o1, o2);
		}
		if type.isAnnotation() {
			return org::apache::commons::lang3::annotation_utils::AnnotationUtils::equals(o1 as Annotation, o2 as Annotation);
		}
		return o1.equals(o2);
	}

	pub fn to_string(&self, a: &/* Java */ java::lang::annotation::Annotation /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::String /**/ {
		/* final */ let builder: ToStringBuilder = ToStringBuilder::new(a, self.TO_STRING_STYLE);
		for /* final */ m in a.annotationType().getDeclaredMethods() {
			if m.getParameterTypes().length > 0 {
				// what?
				continue;
			}
			let r0 = 'try0: {
				builder.append(&m.getName(), &m.invoke(a));
				break 'try0 Ok(());
			};
			match r0 {
				Err(e @ ReflectiveOperationException) => {
					break 'try0 Err(UncheckedException::new(ex));
				},
				Err(e) => Err(e)?,
				Ok => (),
			}
		}
		return builder.build();
	}

	pub fn new() -> org::apache::commons::lang3::annotation_utils::AnnotationUtils {
	// empty
	}
}