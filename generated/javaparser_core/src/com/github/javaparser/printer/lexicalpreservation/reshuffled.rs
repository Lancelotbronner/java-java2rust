use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmElement;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmMix;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmToken;
use java::util::List;
use java::util::stream::Collectors;

pub struct Reshuffled {
	previous_order: com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix,
	next_order: com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix,
}

impl Reshuffled {
	fn new(previous_order: &com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix, next_order: &com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix) -> com::github::javaparser::printer::lexicalpreservation::reshuffled::Reshuffled {
		self.previousOrder = previous_order;
		self.nextOrder = next_order;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Reshuffled{" + self.next_order + ", previous=" + self.previous_order + '}';
	}

	pub fn equals(&self, o: &/* Java */ java::lang::Object /**/) -> bool {
		if self == o {
			return true;
		}
	
		if o == null || self.getClass() != o.getClass() {
			return false;
		}
	
		let that: Reshuffled = o as Reshuffled;
		if !self.previous_order.equals(that.previousOrder) {
			return false;
		}
	
		return self.next_order.equals(that.nextOrder);
	}

	pub fn hash_code(&self) -> i32 {
		let result: i32 = self.previous_order.hash_code();
		result = 31 * result + self.next_order.hash_code();
		return result;
	}

	pub fn get_element(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix {
		return self.next_order;
	}

	pub fn get_previous_order(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix {
		return self.previous_order;
	}

	pub fn get_next_order(&self) -> com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix {
		return self.next_order;
	}

	pub fn is_added(&self) -> bool {
		return false;
	}

	pub fn is_removed(&self) -> bool {
		return false;
	}

	pub fn is_kept(&self) -> bool {
		return false;
	}

	pub fn replace_eol_tokens(&self, line_separator: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement {
		let modified_next_order: CsmMix = CsmMix::new(&self.replace_tokens(&self.next_order.get_elements(), line_separator));
		let modified_previous_order: CsmMix = CsmMix::new(&self.replace_tokens(&self.previous_order.get_elements(), line_separator));
		return Reshuffled::new(modified_previous_order, modified_next_order);
	}

	fn replace_tokens(&self, elements: &/* Java */ java::util::List /**/, line_separator: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> /* Java */ java::util::List /**/ {
		return elements.stream().map(|element| if self.is_new_line_token(element) { line_separator } else { element }).collect(&Collectors::toList());
	}

	fn is_new_line_token(&self, element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> bool {
		return self.is_token(element) && (element as CsmToken).is_new_line();
	}

	fn is_token(&self, element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> bool {
		return element instanceof CsmToken;
	}
}

impl com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement for Reshuffled {}