use crate::com::github::javaparser::ast::type::PrimitiveType::Primitive;
use crate::com::github::javaparser::ast::type::VoidType;
use java::lang::reflect::Method;
use java::util::Optional;

pub struct TypeUtils;

impl TypeUtils {
	pub fn get_method_descriptor(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::lang::String /**/ {
		let string_builder: StringBuilder = StringBuilder::new();
		string_builder.append("(");
		for parameter in method.getParameterTypes() {
			com::github::javaparser::utils::type_utils::TypeUtils::append_descriptor(parameter, string_builder);
		}
		string_builder.append(")");
		com::github::javaparser::utils::type_utils::TypeUtils::append_descriptor(&method.getReturnType(), string_builder);
		return string_builder.toString();
	}

	fn append_descriptor(&self, clazz: &/* Java */ java::lang::Class /**/, string_builder: &/* Java */ java::lang::StringBuilder /**/) {
		let current_class: Class<?> = clazz;
		while current_class.isArray() {
			string_builder.append("[");
			current_class = current_class.getComponentType();
		}
		if current_class.isPrimitive() {
			let descriptor: String = com::github::javaparser::utils::type_utils::TypeUtils::get_primitive_type_descriptor(current_class);
			string_builder.append(descriptor);
		} else {
			string_builder.append("L").append(&current_class.getName().replace(".", "/")).append(";");
		}
	}

	pub fn get_primitive_type_descriptor(&self, clazz: &/* Java */ java::lang::Class /**/) -> /* Java */ java::lang::String /**/ {
		if clazz == Void::TYPE || clazz == Void.class {
			return VoidType::new().to_descriptor();
		}
		let class_name: String = clazz.getSimpleName();
		let prim: Optional<Primitive> = Primitive::by_type_name(class_name);
		if prim.isPresent() {
			return prim.get().to_descriptor();
		}
		prim = Primitive::by_boxed_type_name(class_name);
		return prim.map(|p_type|p_type.to_descriptor()).orElseThrow(|()|IllegalArgumentException::new(&String::format("Unknown primitive type \"%s\"", class_name)));
	}
}