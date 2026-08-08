use crate::com::github::javaparser::resolution::declarations::ResolvedTypeParameterDeclaration;
use java::util;
use java::util::stream::Collectors;

pub struct ResolvedIntersectionType {
	elements: /* Java */ java::util::List /**/,
}

impl ResolvedIntersectionType {
	pub fn new(elements: &/* Java */ java::util::Collection /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_intersection_type::ResolvedIntersectionType {
		if elements.size() < 2 {
			return Err(IllegalArgumentException::new("An intersection type should have at least two elements. This has " + elements.size()));
		}
		self.elements = LinkedList<>::new(elements);
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ResolvedIntersectionType = o as ResolvedIntersectionType;
		return HashSet<>::new(self.elements).equals(HashSet<>::new(that.elements));
	}

	pub fn hash_code(&self) -> i32 {
		return HashSet<>::new(self.elements).hashCode();
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return String::join(" & ", &self.elements.stream().map(ResolvedType::describe).collect(&Collectors::toList()));
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		return self.elements.stream().allMatch(|e|e.is_assignable_by(other));
	}

	pub fn replace_type_variables(&self, tp: &com::github::javaparser::resolution::declarations::resolved_type_parameter_declaration::ResolvedTypeParameterDeclaration, replaced: &com::github::javaparser::resolution::types::resolved_type::ResolvedType, inferred_types: &/* Java */ java::util::Map /**/) -> com::github::javaparser::resolution::types::resolved_type::ResolvedType {
		let elements_replaced: List<ResolvedType> = self.elements.stream().map(|e|e.replace_type_variables(tp, replaced, inferred_types)).collect(&Collectors::toList());
		if elements_replaced.equals(self.elements) {
			return self;
		}
		return ResolvedIntersectionType::new(elements_replaced);
	}

	pub fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedIntersectionType {}