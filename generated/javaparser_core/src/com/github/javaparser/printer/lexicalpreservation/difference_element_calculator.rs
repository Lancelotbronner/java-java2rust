use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::body::VariableDeclarator;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::printer::concretesyntaxmodel;
use crate::com::github::javaparser::printer::lexicalpreservation::LexicalDifferenceCalculator::CalculatedSyntaxModel;
use crate::com::github::javaparser::printer::lexicalpreservation::LexicalDifferenceCalculator::CsmChild;
use java::util;

struct DifferenceElementCalculator {
	cache: /* Java */ java::util::Map /**/,
}

impl DifferenceElementCalculator {
	fn matching(&self, a: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, b: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if a instanceof CsmChild {
			if b instanceof CsmChild {
				let child_a: CsmChild = a as CsmChild;
				let child_b: CsmChild = b as CsmChild;
				return child_a.get_child().equals(&child_b.get_child());
			}
			if b instanceof CsmToken {
				return false;
			}
			if b instanceof CsmIndent {
				return false;
			}
			if b instanceof CsmUnindent {
				return false;
			}
			return Err(UnsupportedOperationException::new(a.getClass().getSimpleName() + " " + b.getClass().getSimpleName()));
		}
		if a instanceof CsmToken {
			if b instanceof CsmToken {
				// fix #2382:
				// Tokens are described by their type AND their content
				// and TokenContentCalculator. By using .equals(), all
				// three values are compared.
				let child_a: CsmToken = a as CsmToken;
				let child_b: CsmToken = b as CsmToken;
				return child_a.equals(child_b);
			}
			if b instanceof CsmChild {
				return false;
			}
			if b instanceof CsmIndent {
				return false;
			}
			if b instanceof CsmUnindent {
				return false;
			}
			return Err(UnsupportedOperationException::new(a.getClass().getSimpleName() + " " + b.getClass().getSimpleName()));
		}
		if a instanceof CsmIndent {
			return b instanceof CsmIndent;
		}
		if a instanceof CsmUnindent {
			return b instanceof CsmUnindent;
		}
		return Err(UnsupportedOperationException::new(a.getClass().getSimpleName() + " " + b.getClass().getSimpleName()));
	}

	fn remove_indentation_elements(&self, elements: &/* Java */ java::util::List /**/) {
		elements.removeIf(|el|el.get_element() instanceof CsmIndent || el.get_element() instanceof CsmUnindent);
	}

	pub fn new() -> com::github::javaparser::printer::lexicalpreservation::difference_element_calculator::DifferenceElementCalculator {
		self.cache = HashMap<>::new();
	}

	fn replacement(&self, a: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, b: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		if a instanceof CsmIndent || b instanceof CsmIndent || a instanceof CsmUnindent || b instanceof CsmUnindent {
			return false;
		}
		if a instanceof CsmChild {
			if b instanceof CsmChild {
				let child_a: CsmChild = a as CsmChild;
				let child_b: CsmChild = b as CsmChild;
				return child_a.get_child().getClass().equals(&child_b.get_child().getClass());
			}
			if b instanceof CsmToken {
				return false;
			}
			return Err(UnsupportedOperationException::new(a.getClass().getSimpleName() + " " + b.getClass().getSimpleName()));
		}
		if a instanceof CsmToken {
			if b instanceof CsmToken {
				let child_a: CsmToken = a as CsmToken;
				let child_b: CsmToken = b as CsmToken;
				return child_a.get_token_type() == child_b.get_token_type();
			}
			if b instanceof CsmChild {
				return false;
			}
		}
		return Err(UnsupportedOperationException::new(a.getClass().getSimpleName() + " " + b.getClass().getSimpleName()));
	}

	fn find_children_positions(&self, calculated_syntax_model: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel) -> /* Java */ java::util::List /**/ {
		let positions: List<ChildPositionInfo> = ArrayList<>::new();
		 {
			let i: i32 = 0;
			while i < calculated_syntax_model.elements.size() {
				{
					let element: CsmElement = calculated_syntax_model.elements.get(i);
					if element instanceof CsmChild {
						positions.add(ChildPositionInfo::new(&(element as CsmChild).get_child(), i));
					}
				}
				i += 1;
			 }
		 }
	
		return positions;
	}

	fn calculate(&self, original: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel, after: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel) -> /* Java */ java::util::List /**/ {
		// For performance reasons we use the positions of matching children
		// to guide the calculation of the difference
		//
		// Suppose we have:
		// qwerty[A]uiop
		// qwer[A]uiop
		//
		// with [A] being a child and lowercase letters being tokens
		//
		// We would calculate the Difference between "qwerty" and "qwer" then we know the A is kept, and then we
		// would calculate the difference between "uiop" and "uiop"
		let children_in_original: List<ChildPositionInfo> = self.find_children_positions(original);
		let children_in_after: List<ChildPositionInfo> = self.find_children_positions(after);
		let common_children: List<ChildPositionInfo> = ArrayList<>::new(children_in_original);
		common_children.retainAll(children_in_after);
		let elements: List<DifferenceElement> = LinkedList<>::new();
		let original_index: i32 = 0;
		let after_index: i32 = 0;
		let common_children_index: i32 = 0;
		// undefined
		let pos_of_next_child_in_original: i32 = -1;
		// undefined
		let pos_of_next_child_in_after: i32 = -1;
		// following this child and the common element in the list.
		while common_children_index < common_children.size() {
			let child: ChildPositionInfo = common_children.get(common_children_index += 1 !!!check!!! post increment);
			// search the position of the node "child" in the original list of cms element
			/* final */ let current_pos_of_next_child_in_original: i32 = pos_of_next_child_in_original;
			/* final */ let current_pos_of_next_child_in_after: i32 = pos_of_next_child_in_after;
			pos_of_next_child_in_original = children_in_original.stream().filter(|i|i.equals(child)).map(|i|i.position).filter(|position|position > current_pos_of_next_child_in_original).findFirst().orElse(pos_of_next_child_in_original);
			// search the position of the node "child" in the modified list of cms element
			pos_of_next_child_in_after = children_in_after.stream().filter(|i|i.equals(child)).map(|i|i.position).filter(|position|position > current_pos_of_next_child_in_after).findFirst().orElse(pos_of_next_child_in_after);
			// modification but the previous position in the list was that of element 'a'.
			if original_index < pos_of_next_child_in_original || after_index < pos_of_next_child_in_after {
				// defines the sublist of elements located before the common element
				let original_sub: CalculatedSyntaxModel =  if original_index < pos_of_next_child_in_original { original.sub(original_index, pos_of_next_child_in_original) } else { CalculatedSyntaxModel::new(Collections::EMPTY_LIST) };
				let after_sub: CalculatedSyntaxModel =  if after_index < pos_of_next_child_in_after { after.sub(after_index, pos_of_next_child_in_after) } else { CalculatedSyntaxModel::new(Collections::EMPTY_LIST) };
				elements.addAll(&self.calculate_impl(original_sub, after_sub));
			}
			if after_index <= pos_of_next_child_in_after {
				// we need to keep the current common node
				elements.add(Kept::new(CsmChild::new(child.node)));
			} else {
				// In this case the current node was not found in the list after change
				// so we need to remove it.
				elements.add(Removed::new(CsmChild::new(child.node)));
			}
			original_index =  if original_index <= pos_of_next_child_in_original { pos_of_next_child_in_original + 1 } else { original_index };
			after_index =  if after_index <= pos_of_next_child_in_after { pos_of_next_child_in_after + 1 } else { after_index };
		}
		if original_index < original.elements.size() || after_index < after.elements.size() {
			let original_sub: CalculatedSyntaxModel =  if original_index < original.elements.size() { original.sub(original_index, &original.elements.size()) } else { CalculatedSyntaxModel::new(Collections::EMPTY_LIST) };
			let after_sub: CalculatedSyntaxModel =  if after_index < after.elements.size() { after.sub(after_index, &after.elements.size()) } else { CalculatedSyntaxModel::new(Collections::EMPTY_LIST) };
			elements.addAll(&self.calculate_impl(original_sub, after_sub));
		}
		return elements;
	}

	fn consider_removal(&self, node_text_for_child: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, elements: &/* Java */ java::util::List /**/) /* thrown(java.lang.UnsupportedOperationException) */ {
		for el in node_text_for_child.get_elements() {
			if el instanceof ChildTextElement {
				let cte: ChildTextElement = el as ChildTextElement;
				self.consider_removal(&LexicalPreservingPrinter::get_or_create_node_text(&cte.get_child()), elements)?;
			} else if el instanceof TokenTextElement {
				let tte: TokenTextElement = el as TokenTextElement;
				elements.add(Removed::new(CsmToken::new(&tte.get_token_kind(), &tte.get_text())));
			} else {
				return Err(UnsupportedOperationException::new(&el.toString()));
			}
		}
	}

	fn consider_removal(&self, removed_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement, original_index: i32, elements: &/* Java */ java::util::List /**/) /* thrown(java.lang.UnsupportedOperationException) */ -> i32 {
		let dealt_with: bool = false;
		if removed_element instanceof CsmChild {
			let removed_child: CsmChild = removed_element as CsmChild;
			if removed_child.get_child() instanceof Type && removed_child.get_child().get_parent_node().isPresent() && removed_child.get_child().get_parent_node().get() instanceof VariableDeclarator {
				let node_text_for_child: NodeText = LexicalPreservingPrinter::get_or_create_node_text(&removed_child.get_child());
				self.consider_removal(node_text_for_child, elements)?;
				original_index += 1;
				dealt_with = true;
			}
		}
		if !dealt_with {
			elements.add(Removed::new(removed_element));
			original_index += 1;
		}
		return original_index;
	}

	fn calculate_impl(&self, original: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel, after: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel) -> /* Java */ java::util::List /**/ {
		let key: String = original.hash_code() + "-" + after.hash_code();
		if self.cache.containsKey(key) {
			return self.cache.get(key);
		}
		let result: List<DifferenceElement> = self.calculate_impl2(original, after);
		self.cache.put(key, result);
		return result;
	}

	fn calculate_impl2(&self, original: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel, after: &com::github::javaparser::printer::lexicalpreservation::lexical_difference_calculator::CalculatedSyntaxModel) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::util::List /**/ {
		// This list will hold the final differences between the two models.
		let elements: List<DifferenceElement> = LinkedList<>::new();
		// Pointers to traverse both sequences (before and after).
		let original_index: i32 = 0;
		let after_index: i32 = 0;
		// and moving just one side forward when we have an element kept or removed
		loop { {
			// elements remain only in the original sequence everything left must be marked as removed.
			if original_index < original.elements.size() && after_index >= after.elements.size() {
				let removed_element: CsmElement = original.elements.get(original_index);
				original_index = self.consider_removal(removed_element, original_index, elements)?;
			// elements remain only in the "after" sequence everything left must be marked as added.
			} else if original_index >= original.elements.size() && after_index < after.elements.size() {
				elements.add(Added::new(&after.elements.get(after_index)));
				after_index += 1;
			} else {
				let next_original: CsmElement = original.elements.get(original_index);
				let next_after: CsmElement = after.elements.get(after_index);
				if (next_original instanceof CsmMix) && (next_after instanceof CsmMix) {
					// If sub-elements are identical, mark everything as kept
					if (next_after as CsmMix).get_elements().equals(&(next_original as CsmMix).get_elements()) {
						// No reason to deal with a reshuffled, we are just going to keep everything as it is
						(next_after as CsmMix).get_elements().forEach(|el|elements.add(Kept::new(el)));
					} else {
						// Otherwise, same type but with shuffled/reorganized content
						elements.add(Reshuffled::new(next_original as CsmMix, next_after as CsmMix));
					}
					original_index += 1;
					after_index += 1;
				} else if com::github::javaparser::printer::lexicalpreservation::difference_element_calculator::DifferenceElementCalculator::matching(next_original, next_after)? {
					// The two elements match according to a custom "matching" rule
					elements.add(Kept::new(next_original));
					original_index += 1;
					after_index += 1;
				} else if self.replacement(next_original, next_after)? {
					// The two elements represent a replacement: remove the old one and add the new one.
					original_index = self.consider_removal(next_original, original_index, elements)?;
					elements.add(Added::new(next_after));
					after_index += 1;
				// Ambiguous case: it could be either an addition or a removal.
				} else {
					// We can try to remove the element or add it and look which one leads to the lower difference
					// Try hypothesis A: treat "nextAfter" as an addition
					let adding_elements: List<DifferenceElement> = self.calculate(&original.from(original_index), &after.from(after_index + 1));
					let cost_adding_elements: i64 = self.cost(adding_elements);
					// Try hypothesis B: treat "nextOriginal" as a removal
					let removing_elements: List<DifferenceElement> = null;
					if cost_adding_elements > 0 {
						removing_elements = self.calculate(&original.from(original_index + 1), &after.from(after_index));
					}
					// Choose the cheaper option based on cost.
					if removing_elements == null || self.cost(removing_elements) > self.cost(adding_elements) {
						elements.add(Added::new(next_after));
						after_index += 1;
					} else {
						elements.add(Removed::new(next_original));
						original_index += 1;
					}
				}
			}
		}if !(original_index < original.elements.size() || after_index < after.elements.size()) break;}
		return elements;
	}

	fn cost(&self, elements: &/* Java */ java::util::List /**/) -> i64 {
		return elements.stream().filter(|e|!(e instanceof Kept)).count();
	}
}

pub struct ChildPositionInfo {
	node: com::github::javaparser::ast::node::Node,
	position: /* Java */ java::lang::Integer /**/,
}

impl ChildPositionInfo {
	fn new(node: &com::github::javaparser::ast::node::Node, position: &/* Java */ java::lang::Integer /**/) -> com::github::javaparser::printer::lexicalpreservation::difference_element_calculator::ChildPositionInfo {
		self.node = node;
		self.position = position;
	}

	pub fn equals(&self, other: &/* Java */ java::lang::Object /**/) -> bool {
		if other == null || !(other instanceof ChildPositionInfo) {
			return false;
		}
	
		let cpi: ChildPositionInfo = other as ChildPositionInfo;
		// If the nodes have no declared position they are considered equal.
		return self.node.equals(cpi.node) && (self.node.has_range() == false && cpi.node.has_range() == false || (self.node.has_range() && cpi.node.has_range() && self.node.get_range().get().contains(&cpi.node.get_range().get())));
	}

	pub fn hash_code(&self) -> i32 {
		return self.node.hash_code() + self.position.hashCode();
	}
}