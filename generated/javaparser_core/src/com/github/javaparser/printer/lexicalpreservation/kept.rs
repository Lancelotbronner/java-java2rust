use crate::com::github::javaparser::TokenTypes;
use crate::com::github::javaparser::ast::type::PrimitiveType;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmElement;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmIndent;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmToken;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmUnindent;

pub struct Kept {
	element: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
}

impl Kept {
	fn new(element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::lexicalpreservation::kept::Kept {
		self.element = element;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Kept{" + self.element + '}';
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let kept: Kept = o as Kept;
		return self.element.equals(kept.element);
	}

	pub fn hash_code(&self) -> i32 {
		return self.element.hashCode();
	}

	pub fn get_element(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.element;
	}

	pub fn get_token_type(&self) /* thrown(java.lang.IllegalStateException) */ -> i32 {
		if self.is_token() {
			let csm_token: CsmToken = self.element as CsmToken;
			return csm_token.get_token_type();
		}
		return Err(IllegalStateException::new("Kept is not a " + CsmToken.class.getSimpleName()));
	}

	pub fn is_added(&self) -> bool {
		return false;
	}

	pub fn is_removed(&self) -> bool {
		return false;
	}

	pub fn is_kept(&self) -> bool {
		return true;
	}

	pub fn is_indent(&self) -> bool {
		return self.element instanceof CsmIndent;
	}

	pub fn is_unindent(&self) -> bool {
		return self.element instanceof CsmUnindent;
	}

	pub fn is_token(&self) -> bool {
		return self.element instanceof CsmToken;
	}

	pub fn is_primitive_type(&self) -> bool {
		if self.is_child() {
			let csm_child: LexicalDifferenceCalculator.CsmChild = self.element as LexicalDifferenceCalculator.CsmChild;
			return csm_child.get_child() instanceof PrimitiveType;
		}
		return false;
	}

	pub fn is_white_space(&self) -> bool {
		if self.is_token() {
			let csm_token: CsmToken = self.element as CsmToken;
			return csm_token.is_white_space();
		}
		return false;
	}

	pub fn is_white_space_or_comment(&self) -> bool {
		if self.is_token() {
			let csm_token: CsmToken = self.element as CsmToken;
			return TokenTypes::is_whitespace_or_comment(&csm_token.get_token_type());
		}
		return false;
	}

	pub fn is_new_line(&self) -> bool {
		if self.is_token() {
			let csm_token: CsmToken = self.element as CsmToken;
			return csm_token.is_new_line();
		}
		return false;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement for Kept {}