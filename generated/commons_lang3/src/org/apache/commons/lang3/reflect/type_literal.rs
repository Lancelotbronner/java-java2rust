use java::lang::reflect::Type;
use java::lang::reflect::TypeVariable;
use crate::org::apache::commons::lang3::Validate;

pub struct TypeLiteral<T> {
	value: /* Java */ java::lang::reflect::Type /**/,
	to_string: /* Java */ java::lang::String /**/,
}

impl<T> TypeLiteral {
	static T: /* Java */ java::lang::reflect::TypeVariable /**/ = TypeLiteral.class.getTypeParameters()[0];

	fn new() /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::reflect::type_literal::TypeLiteral {
		self.value = Validate::not_null(&TypeUtils::get_type_arguments(&self.getClass(), TypeLiteral.class).get(self.T), "%s does not assign type parameter %s", &self.getClass(), &TypeUtils::to_long_string(self.T));
		self.toString = String::format("%s<%s>", &TypeLiteral.class.getSimpleName(), &TypeUtils::to_string(self.value)?);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == self {
			return true;
		}
		if !(obj instanceof TypeLiteral) {
			return false;
		}
		/* final */ let other: TypeLiteral<?> = obj as TypeLiteral<?>;
		return TypeUtils::equals(self.value, other.value);
	}

	pub fn get_type(&self) -> /* Java */ java::lang::reflect::Type /**/ {
		return self.value;
	}

	pub fn hash_code(&self) -> i32 {
		return 37 << 4 | self.value.hashCode();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.to_string;
	}
}

impl<T> org::apache::commons::lang3::reflect::typed::Typed for TypeLiteral<T> {}