use crate::com::github::javaparser::resolution::declarations::ResolvedReferenceTypeDeclaration;
use java::util;
use java::util::stream::Collectors;

pub struct ResolvedUnionType {
	elements: /* Java */ java::util::List /**/,
}

impl ResolvedUnionType {
	pub fn new(elements: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::resolution::types::resolved_union_type::ResolvedUnionType {
		if elements.size() < 2 {
			return Err(IllegalArgumentException::new("An union type should have at least two elements. This has " + elements.size()));
		}
		self.elements = LinkedList<>::new(elements);
	}

	pub fn get_common_ancestor(&self) -> /* Java */ java::util::Optional /**/ {
		let reduce: Optional<List<ResolvedReferenceType>> = self.elements.stream().map(ResolvedType::asReferenceType).map(|rt|rt.get_all_ancestors(ResolvedReferenceTypeDeclaration.breadthFirstFunc)).reduce(|(a, b)|{
			let common: ArrayList<ResolvedReferenceType> = ArrayList<>::new(a);
			common.retainAll(b);
			return common;
		});
		return reduce.orElse(ArrayList<>::new()).stream().findFirst();
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: ResolvedUnionType = o as ResolvedUnionType;
		return HashSet<>::new(self.elements).equals(HashSet<>::new(that.elements));
	}

	pub fn hash_code(&self) -> i32 {
		return HashSet<>::new(self.elements).hashCode();
	}

	pub fn describe(&self) -> /* Java */ java::lang::String /**/ {
		return String::join(" | ", &self.elements.stream().map(ResolvedType::describe).collect(&Collectors::toList()));
	}

	pub fn is_assignable_by(&self, other: &com::github::javaparser::resolution::types::resolved_type::ResolvedType) -> bool {
		return self.elements.stream().allMatch(|e|e.is_assignable_by(other));
	}

	pub fn is_union_type(&self) -> bool {
		return true;
	}

	pub fn as_union_type(&self) -> com::github::javaparser::resolution::types::resolved_union_type::ResolvedUnionType {
		return self;
	}

	pub fn get_elements(&self) -> /* Java */ java::util::List /**/ {
		return self.elements;
	}
}

impl com::github::javaparser::resolution::types::resolved_type::ResolvedType for ResolvedUnionType {}