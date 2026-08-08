use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmElement;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmIndent;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmToken;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmUnindent;

pub struct Added {
	element: com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement,
}

impl Added {
	fn new(element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::lexicalpreservation::added::Added {
		self.element = element;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Added{" + self.element + '}';
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let added: Added = o as Added;
		return self.element.equals(added.element);
	}

	pub fn hash_code(&self) -> i32 {
		return self.element.hashCode();
	}

	pub fn get_element(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement {
		return self.element;
	}

	pub fn is_added(&self) -> bool {
		return true;
	}

	pub fn is_removed(&self) -> bool {
		return false;
	}

	pub fn is_kept(&self) -> bool {
		return false;
	}

	pub fn is_indent(&self) -> bool {
		return self.element instanceof CsmIndent;
	}

	pub fn is_unindent(&self) -> bool {
		return self.element instanceof CsmUnindent;
	}

	fn is_token(&self) -> bool {
		return self.element instanceof CsmToken;
	}

	pub fn to_text_element(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> com::github::javaparser::printer::lexicalpreservation::text_element::TextElement {
		if self.element instanceof LexicalDifferenceCalculator.CsmChild {
			return ChildTextElement::new(&(self.element as LexicalDifferenceCalculator.CsmChild).get_child());
		}
		if self.element instanceof CsmToken {
			return TokenTextElement::new(&(self.element as CsmToken).get_token_type(), &(self.element as CsmToken).get_content());
		}
		return Err(UnsupportedOperationException::new("Unsupported element type: " + self.element.getClass().getSimpleName()));
	}

	pub fn replace_eol_tokens(&self, line_separator: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement {
		return  if self.is_new_line() { Added::new(line_separator) } else { self };
	}

	pub fn is_new_line(&self) -> bool {
		return self.is_token() && (self.element as CsmToken).is_new_line();
	}
}

impl com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement for Added {}