use crate::com::github::javaparser::utils::CodeGenerationUtils::f;
use java::util::Objects;

pub struct Pair<A, B> {
	a: A,
	b: B,
}

impl<A, B> Pair {
	pub fn new(a: &A, b: &B) -> com::github::javaparser::utils::pair::Pair {
		self.a = a;
		self.b = b;
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let pair: Pair<?, ?> = o as Pair<?, ?>;
		if !Objects::equals(self.a, pair.a) {
			return false;
		}
	
		if !Objects::equals(self.b, pair.b) {
			return false;
		}
	
		return true;
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 =  if self.a != null { self.a.hashCode() } else { 0 };
		return 31 * result + ( if self.b != null { self.b.hashCode() } else { 0 });
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return com::github::javaparser::utils::code_generation_utils::CodeGenerationUtils::f("<%s, %s>", self.a, self.b);
	}
}