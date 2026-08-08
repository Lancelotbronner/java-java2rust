use crate::com::github::javaparser::resolution::MethodUsage;
use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use crate::com::github::javaparser::resolution::types::ResolvedType;
use java::lang::reflect::Method;
use java::lang::reflect::Modifier;
use java::lang::reflect::Parameter;
use java::util;
use java::util::stream::Collectors;

pub struct FunctionalInterfaceLogic;

impl FunctionalInterfaceLogic {
	static JAVA_LANG_FUNCTIONAL_INTERFACE: /* Java */ java::lang::String /**/ = FunctionalInterface.class.getCanonicalName();

	static OBJECT_PUBLIC_METHODS_SIGNATURES: /* Java */ java::util::List /**/ = Arrays::stream(&Object.class.getDeclaredMethods()).filter(|m|Modifier::isPublic(&m.getModifiers())).map(|method|com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic::get_signature(method)).collect(&Collectors::toList());

	fn new() -> com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic {
	// prevent instantiation
	}

	pub fn get_functional_method(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> /* Java */ java::util::Optional /**/ {
		let optional_type_declaration: Optional<ResolvedReferenceTypeDeclaration> = type.as_reference_type()?.get_type_declaration();
		if !optional_type_declaration.isPresent() {
			return Optional::empty();
		}
		let type_declaration: ResolvedReferenceTypeDeclaration = optional_type_declaration.get();
		if type.is_reference_type() && type_declaration.is_interface() {
			return com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic::get_functional_method(type_declaration);
		}
		return Optional::empty();
	}

	pub fn get_functional_method(&self, type_declaration: &com::github::javaparser::resolution::declarations::resolved_reference_type_declaration::ResolvedReferenceTypeDeclaration) -> /* Java */ java::util::Optional /**/ {
		// We need to find all abstract methods
		// Remove methods inherited by Object:
		// Consider the case of Comparator which define equals. It would be considered a functional method.
	let 	methods: Set<MethodUsage> = type_declaration.get_all_methods().stream().filter(|m|m.get_declaration().is_abstract()).filter(|m|!com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic::is_public_member_of_object(m)).collect(&Collectors::toSet());
		// see https://docs.oracle.com/javase/specs/jls/se8/html/jls-9.html#jls-9.8
		if methods.size() == 0 {
			return Optional::empty();
		}
		let iterator: Iterator<MethodUsage> = methods.iterator();
		let method_usage: MethodUsage = iterator.next();
		while iterator.hasNext() {
			let other_method_usage: MethodUsage = iterator.next();
			if !(method_usage.is_same_signature(other_method_usage) || method_usage.is_sub_signature(other_method_usage) || other_method_usage.is_sub_signature(method_usage)) {
				method_usage = null;
				break;
			}
			if !(method_usage.is_return_type_substituable(other_method_usage)) {
				method_usage = null;
				break;
			}
		}
		return Optional::ofNullable(method_usage);
	}

	pub fn is_functional_interface_type(&self, type: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		if type.is_reference_type() {
			let optional_type_declaration: Optional<ResolvedReferenceTypeDeclaration> = type.as_reference_type()?.get_type_declaration();
			if optional_type_declaration.isPresent() && optional_type_declaration.get().has_annotation(self.JAVA_LANG_FUNCTIONAL_INTERFACE) {
				return true;
			}
		}
		return com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic::get_functional_method(type).isPresent();
	}

	fn get_signature(&self, m: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::lang::String /**/ {
		return String::format("%s(%s)", &m.getName(), &String::join(", ", &Arrays::stream(&m.getParameters()).map(|p|com::github::javaparser::resolution::logic::functional_interface_logic::FunctionalInterfaceLogic::to_signature(p)).collect(&Collectors::toList())));
	}

	fn to_signature(&self, p: &/* Java */ java::lang::reflect::Parameter /**/) -> /* Java */ java::lang::String /**/ {
		return p.getType().getCanonicalName();
	}

	fn is_public_member_of_object(&self, m: &com::github::javaparser::resolution::method_usage::MethodUsage) -> bool {
		return self.OBJECT_PUBLIC_METHODS_SIGNATURES.contains(&m.get_declaration().get_signature());
	}
}