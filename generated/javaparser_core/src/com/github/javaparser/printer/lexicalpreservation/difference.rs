use crate::com::github::javaparser::GeneratedJavaParserConstants::LBRACE;
use crate::com::github::javaparser::GeneratedJavaParserConstants::RBRACE;
use crate::com::github::javaparser::GeneratedJavaParserConstants::SPACE;
use crate::com::github::javaparser::GeneratedJavaParserConstants;
use crate::com::github::javaparser::JavaToken;
use crate::com::github::javaparser::JavaToken::Kind;
use crate::com::github::javaparser::TokenTypes;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::NodeList;
use crate::com::github::javaparser::ast::comments::Comment;
use crate::com::github::javaparser::ast::nodeTypes::NodeWithTypeArguments;
use crate::com::github::javaparser::ast::type::ArrayType;
use crate::com::github::javaparser::ast::type::ClassOrInterfaceType;
use crate::com::github::javaparser::ast::type::Type;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmElement;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmIndent;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmUnindent;
use crate::com::github::javaparser::printer::lexicalpreservation::LexicalDifferenceCalculator::CsmChild;
use java::util;
use java::util::function::Predicate;
use java::util::stream::IntStream;

pub struct Difference {
	node_text: com::github::javaparser::printer::lexicalpreservation::node_text::NodeText,
	node: com::github::javaparser::ast::node::Node,
	diff_elements: /* Java */ java::util::List /**/,
	original_elements: /* Java */ java::util::List /**/,
	original_index: i32 = 0,
	diff_index: i32 = 0,
	indentation: /* Java */ java::util::List /**/,
	added_indentation: bool = false,
}

impl Difference {
	pub static STANDARD_INDENTATION_SIZE: i32 = 4;

	fn new(diff_elements: &/* Java */ java::util::List /**/, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node: &com::github::javaparser::ast::node::Node) /* thrown(java.lang.NullPointerException) */ -> com::github::javaparser::printer::lexicalpreservation::difference::Difference {
		if node_text == null {
			return Err(NullPointerException::new("nodeText can not be null"));
		}
		self.nodeText = node_text;
		self.node = node;
		self.diffElements = diff_elements;
		self.originalElements = node_text.get_elements();
		self.indentation = LexicalPreservingPrinter::find_indentation(node);
	}

	fn process_indentation(&self, mut indentation: &/* Java */ java::util::List /**/, prev_elements: &/* Java */ java::util::List /**/) -> /* Java */ java::util::List /**/ {
		let eol_index: i32 = self.last_index_of_eol(prev_elements);
		// Return "indentation" as is if no EOL element was found
		if eol_index < 0 {
			return indentation;
		}
	
		// Find consecutive space characters after the EOL element
		indentation = self.take_while(&prev_elements.subList(eol_index + 1, &prev_elements.size()), |element|element.is_white_space());
		return indentation;
	}

	fn take_while(&self, prev_elements: &/* Java */ java::util::List /**/, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::List /**/ {
		let spaces: List<TextElement> = ArrayList<>::new();
		for element in prev_elements {
			if predicate.test(element) {
				spaces.add(element);
				continue;
			}
			break;
		}
		return spaces;
	}

	fn last_index_of_eol(&self, source: &/* Java */ java::util::List /**/) -> i32 {
		return IntStream::range(0, &source.size()).map(|i|source.size() - i - 1).filter(|i|source.get(i).is_newline()).findFirst().orElse(-1);
	}

	fn pos_of_next_comment(&self, from_index: i32, elements: &/* Java */ java::util::List /**/) -> i32 {
		if !self.is_valid_index(from_index, elements) {
			return -1;
		}
	
		let iterator: ArrayIterator<TextElement> = ArrayIterator<>::new(elements, from_index);
		// search for the next consecutive space characters
		while iterator.has_next() {
			let element: TextElement = iterator.next();
			if element.is_space_or_tab() {
				continue;
			}
			if element.is_comment() {
				return iterator.index();
			}
			break;
		}
		return -1;
	}

	fn is_followed_by_comment(&self, from_index: i32, elements: &/* Java */ java::util::List /**/) -> bool {
		return self.pos_of_next_comment(from_index, elements) != -1;
	}

	fn remove_elements(&self, from_index: i32, to_index: i32, elements: &/* Java */ java::util::List /**/) {
		if !(self.is_valid_index(from_index, elements) && self.is_valid_index(to_index, elements) && from_index <= to_index) {
			return;
		}
	
		let iterator: ListIterator<TextElement> = elements.listIterator(from_index);
		// removing elements
		let count: i32 = from_index;
		while iterator.hasNext() && count <= to_index {
			iterator.next();
			iterator.remove();
			count += 1;
		}
	}

	fn is_valid_index(&self, index: i32, elements: &/* Java */ java::util::List /**/) -> bool {
		return index >= 0 && index <= elements.size();
	}

	fn last_index_of_eol_withoutgpt(&self, source: &/* Java */ java::util::List /**/) -> i32 {
		let list_iterator: ListIterator<TextElement> = source.listIterator(&source.size());
		let last_index: i32 = source.size() - 1;
		while list_iterator.hasPrevious() {
			let elem: TextElement = list_iterator.previous();
			if elem.is_newline() {
				return last_index;
			}
			last_index -= 1;
		}
		return -1;
	}

	fn indentation_block(&self) -> /* Java */ java::util::List /**/ {
		let res: List<TextElement> = LinkedList<>::new();
		res.add(TokenTextElement::new());
		res.add(TokenTextElement::new());
		res.add(TokenTextElement::new());
		res.add(TokenTextElement::new());
		return res;
	}

	fn is_afterl_brace(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32) -> bool {
		if node_text_index > 0 && node_text.get_text_element(node_text_index - 1).is_token() {
			return true;
		}
		if node_text_index > 0 && node_text.get_text_element(node_text_index - 1).is_space_or_tab() {
			return self.is_afterl_brace(node_text, node_text_index - 1);
		}
		return false;
	}

	fn consider_enforcing_indentation(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32) /* thrown(java.lang.IllegalStateException) */ -> i32 {
		return self.consider_indentation(node_text, node_text_index, &self.indentation.size())?;
	}

	fn consider_removing_indentation(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32) /* thrown(java.lang.IllegalStateException) */ -> i32 {
		return self.consider_indentation(node_text, node_text_index, 0)?;
	}

	fn consider_indentation(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32, number_of_characters_to_preserve: i32) /* thrown(java.lang.IllegalStateException) */ -> i32 {
		let enforcing_indentation_context: EnforcingIndentationContext = self.define_enforcing_indentation_context(node_text, node_text_index);
		// the next position in the list (by default the current position)
		let res: i32 = node_text_index;
		if enforcing_indentation_context.extraCharacters > 0 {
			let extra_characters: i32 =  if enforcing_indentation_context.extraCharacters > number_of_characters_to_preserve { enforcing_indentation_context.extraCharacters - number_of_characters_to_preserve } else { 0 };
			res = self.remove_extra_characters(node_text, enforcing_indentation_context.start, extra_characters);
			// The next position must take into account the indentation
			res =  if extra_characters > 0 { res + number_of_characters_to_preserve } else { res };
		}
		if res < 0 {
			return Err(IllegalStateException::new());
		}
		return res;
	}

	fn is_enforcing_indentation_activable(&self, removed_group: &com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup) -> bool {
		return (self.is_last_element(self.diff_elements, self.diff_index) || !(self.next_diff_element(self.diff_elements, self.diff_index).is_added())) && self.original_index < self.original_elements.size() && !removed_group.isa_complete_line();
	}

	fn is_removing_indentation_activable(&self, removed_group: &com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup) -> bool {
		return (self.is_last_element(self.diff_elements, self.diff_index) || !(self.next_diff_element(self.diff_elements, self.diff_index).is_added())) && self.original_index < self.original_elements.size() && removed_group.isa_complete_line();
	}

	fn is_last_element(&self, list: &/* Java */ java::util::List /**/, index: i32) -> bool {
		return index + 1 >= list.size();
	}

	fn next_diff_element(&self, list: &/* Java */ java::util::List /**/, index: i32) -> com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement {
		return list.get(index + 1);
	}

	fn remove_extra_characters(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32, extra_characters: i32) -> i32 {
		let count: i32 = 0;
		while node_text_index >= 0 && node_text_index < node_text.number_of_elements() && count < extra_characters {
			node_text.remove_element(node_text_index);
			count += 1;
		}
		return node_text_index;
	}

	fn define_enforcing_indentation_context(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, start_index: i32) -> com::github::javaparser::printer::lexicalpreservation::difference::EnforcingIndentationContext {
		let ctx: EnforcingIndentationContext = EnforcingIndentationContext::new(start_index);
		// compute space before startIndex value
		if start_index < node_text.number_of_elements() && start_index > 0 {
			// at this stage startIndex points to the first element before the deleted one
			 {
				let i: i32 = start_index - 1;
				while i >= 0 && i < node_text.number_of_elements() {
					{
						if node_text.get_text_element(i).is_newline() {
							break;
						}
						if !self.is_space_or_tab_element(node_text, i) {
							ctx = EnforcingIndentationContext::new(start_index);
							break;
						}
						ctx.start = i;
						ctx.extraCharacters += 1;
					}
					i -= 1;
				 }
			 }
	
		}
		// compute space after the deleted element
		if start_index < node_text.number_of_elements() && self.is_space_or_tab_element(node_text, start_index) {
			//			int startingFromIndex = startIndex == 0 ? startIndex : startIndex + 1;
			 {
				let i: i32 = start_index;
				while i >= 0 && i < node_text.number_of_elements() {
					{
						if node_text.get_text_element(i).is_newline() {
							break;
						}
						if !self.is_space_or_tab_element(node_text, i) {
							break;
						}
						ctx.extraCharacters += 1;
					}
					i += 1;
				 }
			 }
	
		}
		return ctx;
	}

	fn is_inlined(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, start_index: i32) -> bool {
		let inlined: bool = false;
		if start_index < node_text.number_of_elements() && start_index >= 0 {
			// at this stage startIndex points to the first element before the deleted one
			 {
				let i: i32 = start_index;
				while i < node_text.number_of_elements() {
					{
						if node_text.get_text_element(i).is_newline() {
							break;
						}
						if node_text.get_text_element(i).is_child() {
							inlined = true;
							break;
						}
					}
					i += 1;
				 }
			 }
	
		}
		return inlined;
	}

	fn is_space_or_tab_element(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, i: i32) -> bool {
		return node_text.get_text_element(i).is_space_or_tab();
	}

	fn apply(&self) /* thrown(java.lang.UnsupportedOperationException) */ {
		ReshuffledDiffElementExtractor::of(self.node_text).extract(self.diff_elements);
		let removed_groups: Map<Removed, RemovedGroup> = self.combine_removed_elements_to_removed_groups();
		loop { {
			let is_left_over_diff_element: bool = self.apply_left_over_diff_elements();
			let is_left_over_original_element: bool = self.apply_left_over_original_elements()?;
			if !is_left_over_diff_element && !is_left_over_original_element {
				let diff_element: DifferenceElement = self.diff_elements.get(self.diff_index);
				if diff_element.is_added() {
					self.apply_added_diff_element(diff_element as Added);
				} else {
					let original_element: TextElement = self.original_elements.get(self.original_index);
					let original_element_is_child: bool = original_element instanceof ChildTextElement;
					let original_element_is_token: bool = original_element instanceof TokenTextElement;
					if diff_element.is_kept() {
						self.apply_kept_diff_element(diff_element as Kept, original_element, original_element_is_child, original_element_is_token)?;
					} else if diff_element.is_removed() {
						let removed: Removed = diff_element as Removed;
						self.apply_removed_diff_element(&removed_groups.get(removed), removed, original_element, original_element_is_child, original_element_is_token)?;
					} else {
						return Err(UnsupportedOperationException::new("Unable to apply operations from " + diff_element.getClass().getSimpleName() + " to " + original_element.getClass().getSimpleName()));
					}
				}
			}
		}if !(self.diff_index < self.diff_elements.size() || self.original_index < self.original_elements.size()) break;}
	}

	fn apply_left_over_original_elements(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		let is_left_over_element: bool = false;
		if self.diff_index >= self.diff_elements.size() && self.original_index < self.original_elements.size() {
			let original_element: TextElement = self.original_elements.get(self.original_index);
			if original_element.is_white_space_or_comment() {
				self.original_index += 1;
			} else {
				return Err(UnsupportedOperationException::new("NodeText: " + self.node_text + ". Difference: " + self + " " + original_element));
			}
			is_left_over_element = true;
		}
		return is_left_over_element;
	}

	fn apply_left_over_diff_elements(&self) /* thrown(java.lang.UnsupportedOperationException) */ -> bool {
		let is_left_over_element: bool = false;
		if self.diff_index < self.diff_elements.size() && self.original_index >= self.original_elements.size() {
			let diff_element: DifferenceElement = self.diff_elements.get(self.diff_index);
			if diff_element.is_kept() {
				self.diff_index += 1;
			} else if diff_element.is_added() {
				let added_element: Added = diff_element as Added;
				if added_element.is_indent() {
					self.add_indent();
				} else if added_element.is_unindent() {
					self.remove_indent();
				} else {
					self.node_text.add_element(self.original_index, &added_element.to_text_element()?);
					self.original_index += 1;
				}
				self.diff_index += 1;
			} else {
				// let's forget this element
				self.diff_index += 1;
			}
			is_left_over_element = true;
		}
		return is_left_over_element;
	}

	fn combine_removed_elements_to_removed_groups(&self) -> /* Java */ java::util::Map /**/ {
		let removed_elements_map: Map<Integer, List<Removed>> = self.group_consecutive_removed_elements();
		let removed_groups: List<RemovedGroup> = ArrayList<>::new();
		for entry in removed_elements_map.entrySet() {
			removed_groups.add(&RemovedGroup::of(&entry.getKey(), &entry.getValue()));
		}
		let map: Map<Removed, RemovedGroup> = HashMap<>::new();
		for removed_group in removed_groups {
			for index in removed_group {
				map.put(index, removed_group);
			}
		}
		return map;
	}

	fn group_consecutive_removed_elements(&self) -> /* Java */ java::util::Map /**/ {
		let removed_elements_map: Map<Integer, List<Removed>> = HashMap<>::new();
		let first_element: Integer = null;
		 {
			let i: i32 = 0;
			while i < self.diff_elements.size() {
				{
					let diff_element: DifferenceElement = self.diff_elements.get(i);
					if diff_element.is_removed() {
						if first_element == null {
							first_element = i;
						}
						removed_elements_map.computeIfAbsent(first_element, |key|ArrayList<>::new()).add(diff_element as Removed);
					} else {
						first_element = null;
					}
				}
				i += 1;
			 }
		 }
	
		return removed_elements_map;
	}

	fn apply_removed_diff_element(&mut self, removed_group: &com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup, removed: &com::github::javaparser::printer::lexicalpreservation::removed::Removed, original_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement, original_element_is_child: bool, original_element_is_token: bool) /* thrown(java.lang.UnsupportedOperationException | java.lang.AssertionError | java.lang.IllegalStateException) */ {
		if removed.is_child() && original_element_is_child {
			let original_element_child: ChildTextElement = original_element as ChildTextElement;
			if original_element_child.is_comment() {
				// We expected to remove a proper node but we found a comment in between.
				// If the comment is associated to the node we want to remove we remove it as well, otherwise we keep it
				let comment: Comment = original_element_child.get_child() as Comment;
				if !comment.is_orphan() && comment.get_commented_node().isPresent() && comment.get_commented_node().get().equals(&removed.get_child()?) {
					self.node_text.remove_element(self.original_index);
				} else {
					self.original_index += 1;
				}
			} else {
				// If we delete the first element, it is possible that there is an indentation to be deleted which is
				// stored in the parent node.
				let parent_node_text: NodeText = NodeText::new();
				let indentation_tokens: List<TextElement> = ArrayList<>::new();
				if self.original_index == 0 && removed.get_child()?.get_parent_node().isPresent() {
					let starting_node_for_finding_indentation: Node = removed.get_child()?;
					let parent_node: Node = removed.get_child()?.get_parent_node().get();
					parent_node_text = LexicalPreservingPrinter::get_or_create_node_text(parent_node);
					// so the previous indentation is defined in the parent node of the method declaration.
					if !parent_node_text.get_elements().isEmpty() && parent_node.get_parent_node().isPresent() && parent_node_text.get_text_element(0).equals(&self.node_text.get_text_element(self.original_index)) {
						starting_node_for_finding_indentation = parent_node;
						parent_node_text = LexicalPreservingPrinter::get_or_create_node_text(&parent_node.get_parent_node().get());
					}
					indentation_tokens = LexicalPreservingPrinter::find_indentation(starting_node_for_finding_indentation);
				}
				self.node_text.remove_element(self.original_index);
				// then we want to enforce the indentation.
				if self.is_enforcing_indentation_activable(removed_group) {
					// Since the element has been deleted we try to start the analysis from the element following the
					// one that was deleted
					self.original_index = self.consider_enforcing_indentation(self.node_text, self.original_index)?;
				}
				// If in front we have one space and before also we had space let's drop one space
				if self.original_elements.size() > self.original_index && self.original_index > 0 {
					if self.original_elements.get(self.original_index).is_white_space() && self.original_elements.get(self.original_index - 1).is_white_space() {
						// OR or if the next change is to keep the element
						if (self.diff_index + 1) == self.diff_elements.size() || (self.diff_elements.get(self.diff_index + 1).is_kept()) {
							self.original_elements.remove(self.original_index -= 1 !!!check!!! post decrement);
						}
					}
				}
				// If so, it should also be deleted.
				if self.is_followed_by_comment(self.original_index, self.original_elements) {
					let index_of_next_comment: i32 = self.pos_of_next_comment(self.original_index, self.original_elements);
					self.remove_elements(self.original_index, index_of_next_comment, self.original_elements);
				}
				if self.is_removing_indentation_activable(removed_group) {
					// Since the element has been deleted we try to start the analysis from the previous element
					self.original_index = self.consider_removing_indentation(self.node_text, self.original_index)?;
					// the same line as a method declaration.
					if self.original_index == 0 && !indentation_tokens.isEmpty() && !self.is_inlined(self.node_text, self.original_index) {
						for indentation_token in indentation_tokens {
							parent_node_text.remove_element(&parent_node_text.find_element(&indentation_token.and(&indentation_token.match_by_range())));
						}
					}
				}
				self.diff_index += 1;
			}
		} else if removed.is_child() && original_element.is_comment() {
			// removing the comment first
			self.node_text.remove_element(self.original_index);
			if self.is_removing_indentation_activable(removed_group) {
				self.original_index = self.consider_removing_indentation(self.node_text, self.original_index)?;
			}
		} else if removed.is_token() && original_element_is_token && (// element always has the current operating system's EOL as type
		removed.get_token_type()? == (original_element as TokenTextElement).get_token_kind() || ((original_element as TokenTextElement).get_token().get_category()?.is_end_of_line() && removed.is_new_line())) {
			self.node_text.remove_element(self.original_index);
			self.diff_index += 1;
		} else if (removed.is_white_space_not_eol() || removed.get_element() instanceof CsmIndent || removed.get_element() instanceof CsmUnindent) && original_element.is_space_or_tab() {
			// remove the current space
			self.node_text.remove_element(self.original_index);
		} else if original_element_is_token && original_element.is_white_space_or_comment() {
			self.original_index += 1;
			// skip the newline token which may be generated unnecessarily by the concrete syntax pattern
			if removed.is_new_line() {
				self.diff_index += 1;
			}
		} else if original_element.is_literal() {
			self.node_text.remove_element(self.original_index);
			self.diff_index += 1;
		} else if removed.is_primitive_type() {
			if original_element.is_primitive() {
				self.node_text.remove_element(self.original_index);
				self.diff_index += 1;
			} else {
				return Err(UnsupportedOperationException::new("removed " + removed.get_element() + " vs " + original_element));
			}
		} else if removed.is_white_space() || removed.get_element() instanceof CsmIndent || removed.get_element() instanceof CsmUnindent {
			self.diff_index += 1;
		} else if original_element.is_white_space() {
			self.original_index += 1;
		} else if removed.is_child() {
			// see issue #3721 this case is linked for example to a change of type of variable declarator
			self.node_text.remove_element(self.original_index);
			self.diff_index += 1;
		} else if original_element.is_child() && removed.is_token() {
			// see issue #4747 this case is linked for example to a change of the annotation name
			// This happens because changing a name results in the creation of a token when the syntax of the modified
			// node is evaluated, whereas parsing an annotation results in a representation containing a child node
			// (representing the annotation name). When the annotation name is modified, the element to be deleted (the
			// child node) is compared with the token containing the character string representing the new annotation
			// name.
			self.node_text.remove_element(self.original_index);
			self.diff_index += 1;
		} else {
			return Err(UnsupportedOperationException::new("removed " + removed.get_element() + " vs " + original_element));
		}
		self.clean_the_line_of_left_over_space(removed_group, removed);
	}

	fn clean_the_line_of_left_over_space(&mut self, removed_group: &com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup, removed: &com::github::javaparser::printer::lexicalpreservation::removed::Removed) {
		if self.original_index >= self.original_elements.size() {
			// if all elements were already processed there is nothing to do
			return;
		}
		// because in this case we are trying to remove the indentation of the next child element
		if !removed_group.is_processed() && removed_group.is_last_element(removed) && removed_group.isa_complete_line() && !removed.is_new_line() {
			let last_element_index: Integer = removed_group.get_last_element_index();
			let indentation: Optional<Integer> = removed_group.get_indentation();
			if indentation.isPresent() && !self.is_replaced(last_element_index) {
				 {
					let i: i32 = 0;
					while i < indentation.get() {
						{
							if self.original_elements.get(self.original_index).is_space_or_tab() {
								// If the current element is a space, remove it
								self.node_text.remove_element(self.original_index);
							} else if self.original_index >= 1 && self.original_elements.get(self.original_index - 1).is_space_or_tab() {
								// If the current element is not a space itself we remove the space in front of (before) it
								self.node_text.remove_element(self.original_index - 1);
								self.original_index -= 1;
							}
							// Remove remaining newline character if needed
							if self.node_text.get_text_element(self.original_index).is_newline() {
								self.node_text.remove_element(self.original_index);
								self.original_index =  if self.original_index > 0 { self.original_index -= 1 !!!check!!! post decrement } else { 0 };
							}
						}
						i += 1;
					 }
				 }
	
			}
			// Mark RemovedGroup as processed
			removed_group.processed();
		}
	}

	fn apply_kept_diff_element(&mut self, kept: &com::github::javaparser::printer::lexicalpreservation::kept::Kept, original_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement, original_element_is_child: bool, original_element_is_token: bool) /* thrown(java.lang.UnsupportedOperationException | java.lang.IllegalStateException) */ {
		if original_element.is_comment() {
			self.original_index += 1;
		} else if kept.is_child() && (kept.get_element() as CsmChild).get_child() instanceof Comment {
			self.diff_index += 1;
		} else if kept.is_child() && original_element_is_child {
			self.diff_index += 1;
			self.original_index += 1;
		} else if kept.is_child() && original_element_is_token {
			if original_element.is_white_space_or_comment() {
				self.original_index += 1;
			} else if original_element.is_identifier() && self.is_node_with_type_arguments(kept) {
				self.diff_index += 1;
				// skip all token related to node with type argument declaration
				// for example:
				// List i : in this case originalElement is "List" and the next token is space. There is nothing to
				// skip. in the originalElements list.
				// List<String> i : in this case originalElement is "List" and the next token is
				// "<" so we have to skip all the tokens which are used in the typed argument declaration
				// [<][String][>](3 tokens) in the originalElements list.
				// List<List<String>> i : in this case originalElement is "List" and the next
				// token is "<" so we have to skip all the tokens which are used in the typed arguments declaration
				// [<][List][<][String][>][>](6 tokens) in the originalElements list.
				let step: i32 = self.get_index_to_next_token_element(original_element as TokenTextElement, 0);
				self.original_index += step;
				self.original_index += 1;
			} else if original_element.is_identifier() && self.is_type_with_fully_qualified_name(kept) {
				self.diff_index += 1;
				// skip all token related to node with the fully qualified name
				// for example:
				// java.lang.Object is represented in originalElement as a list of tokens "java", ".", "lang", ".",
				// "Object".
				// So we have to skip 5 tokens.
				let step: i32 = self.get_index_to_next_token_element(original_element as TokenTextElement, kept);
				self.original_index += step;
				// positioning on the next token
				self.original_index += 1;
			} else if (original_element.is_identifier() || original_element.is_keyword()) && self.is_array_type(kept) {
				let token_to_skip: i32 = self.get_index_to_next_token_element_in_array_type(original_element as TokenTextElement, &self.get_array_level(kept));
				self.diff_index += 1;
				self.original_index += token_to_skip;
				self.original_index += 1;
			} else if original_element.is_identifier() {
				self.original_index += 1;
				self.diff_index += 1;
			} else if kept.is_primitive_type() {
				self.original_index += 1;
				self.diff_index += 1;
			} else {
				// original is a token so we keep it (for example an unexpected semicolon)
				self.original_index += 1;
			}
		} else if kept.is_token() && original_element_is_token {
			let original_text_token: TokenTextElement = original_element as TokenTextElement;
			if kept.get_token_type()? == original_text_token.get_token_kind() {
				self.original_index += 1;
				self.diff_index += 1;
			} else if kept.is_new_line() && original_text_token.is_newline() {
				self.original_index += 1;
				self.diff_index += 1;
			} else if kept.is_new_line() && original_text_token.is_space_or_tab() {
				self.original_index += 1;
			} else if kept.is_white_space_or_comment() {
				self.diff_index += 1;
			} else if original_text_token.is_white_space_or_comment() {
				self.original_index += 1;
			} else if !kept.is_new_line() && original_text_token.is_separator() {
				// case where originalTextToken is a separator like ";" and
				// kept is not a new line or whitespace for example "}"
				// see issue 2351
				self.original_index += 1;
			} else {
				return Err(UnsupportedOperationException::new("Csm token " + kept.get_element() + " NodeText TOKEN " + original_text_token));
			}
		} else if kept.is_token() && original_element_is_child {
			self.diff_index += 1;
		} else if kept.is_white_space() {
			self.diff_index += 1;
		} else if kept.is_indent() {
			self.diff_index += 1;
		} else if kept.is_unindent() {
			// Nothing to do
			self.diff_index += 1;
		} else {
			return Err(UnsupportedOperationException::new("kept " + kept.get_element() + " vs " + original_element));
		}
	}

	fn get_array_level(&self, element: &com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement) -> i32 {
		let csm_elem: CsmElement = element.get_element();
		if self.is_array_type(element) {
			let child: Node = (csm_elem as LexicalDifferenceCalculator.CsmChild).get_child();
			return (child as ArrayType).get_array_level();
		}
		return 0;
	}

	fn is_array_type(&self, element: &com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement) -> bool {
		let csm_elem: CsmElement = element.get_element();
		return csm_elem instanceof LexicalDifferenceCalculator.CsmChild && (csm_elem as LexicalDifferenceCalculator.CsmChild).get_child() instanceof ArrayType;
	}

	fn is_type_with_fully_qualified_name(&self, element: &com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement) -> bool {
		if !element.is_child() {
			return false;
		}
	
		let child: CsmChild = element.get_element() as CsmChild;
		if !ClassOrInterfaceType.class.isAssignableFrom(&child.get_child().getClass()) {
			return false;
		}
	
		return (child.get_child() as ClassOrInterfaceType).get_scope().isPresent();
	}

	fn is_node_with_type_arguments(&self, element: &com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement) -> bool {
		if !element.is_child() {
			return false;
		}
	
		let child: CsmChild = element.get_element() as CsmChild;
		if !NodeWithTypeArguments.class.isAssignableFrom(&child.get_child().getClass()) {
			return false;
		}
	
		let type_args: Optional<NodeList<Type>> = (child.get_child() as NodeWithTypeArguments).get_type_arguments();
		return type_args.isPresent() && type_args.get().size() > 0;
	}

	fn get_index_to_next_token_element(&self, element: &com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement, kept: &com::github::javaparser::printer::lexicalpreservation::difference_element::DifferenceElement) -> i32 {
		// number of token to skip
		let step: i32 = 0;
		// verify if the DifferenceElement is a ClassOrInterfaceType with a fully qualified name
		if !self.is_type_with_fully_qualified_name(kept) {
			return 0;
		}
	
		let child: CsmChild = kept.get_element() as CsmChild;
		// split the type fully qualified node name to an array of tokens
		let parts: Vec<String> = (child.get_child() as ClassOrInterfaceType).get_name_with_scope().split("\\.");
		let token: JavaToken = element.get_token();
		for part in parts {
			if part.equals(&token.as_string()) {
				// get 'dot' token
				token = token.get_next_token().get();
				if !".".equals(&token.as_string()) {
					break;
				}
	
				// get the next part
				token = token.get_next_token().get();
				step += 2;
				continue;
			}
			// there is no match so we don't have token to skip
			step = 0;
			break;
		}
		return step;
	}

	fn get_index_to_next_token_element(&self, element: &com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement, nested_diamond_operator: i32) /* thrown(java.lang.AssertionError) */ -> i32 {
		// number of token to skip
		let step: i32 = 0;
		let next: Optional<JavaToken> = element.get_token().get_next_token();
		if !next.isPresent() {
			return step;
		}
	
		// because there is a token, first we need to increment the number of token to skip
		step += 1;
		// manage nested diamond operators by incrementing the level on LT token and decrementing on GT
		let next_token: JavaToken = next.get();
		let kind: Kind = Kind::value_of(&next_token.get_kind())?;
		if self.is_diamond_operator(kind) {
			if Kind::GT.equals(kind) {
				nested_diamond_operator -= 1;
			}
			else {nested_diamond_operator += 1;
			}
	
		}
		// for example in this declaration List <String> a;
		if nested_diamond_operator == 0 && !next_token.get_category()?.is_whitespace() {
			return step;
		}
	
		// recursively analyze token to skip
		return step += self.get_index_to_next_token_element(TokenTextElement::new(next_token), nested_diamond_operator)?;
	}

	fn get_index_to_next_token_element_in_array_type(&self, element: &com::github::javaparser::printer::lexicalpreservation::token_text_element::TokenTextElement, array_level: i32) /* thrown(java.lang.AssertionError) */ -> i32 {
		// number of token to skip
		let step: i32 = 0;
		let next: Optional<JavaToken> = element.get_token().get_next_token();
		if !next.isPresent() {
			return step;
		}
	
		// because there is a token, first we need to increment the number of token to skip
		step += 1;
		// manage array Level by decrementing the level on right bracket token
		let next_token: JavaToken = next.get();
		let kind: Kind = Kind::value_of(&next_token.get_kind())?;
		if self.is_bracket(kind) {
			if Kind::RBRACKET.equals(kind) {
				array_level -= 1;
			}
	
		}
		// for example in this declaration int [] a;
		if array_level == 0 && !next_token.get_category()?.is_whitespace() {
			return step;
		}
	
		// recursively analyze token to skip
		return step += self.get_index_to_next_token_element_in_array_type(TokenTextElement::new(next_token), array_level)?;
	}

	fn is_diamond_operator(&self, kind: &com::github::javaparser::java_token::Kind) -> bool {
		return Kind::GT.equals(kind) || Kind::LT.equals(kind);
	}

	fn is_bracket(&self, kind: &com::github::javaparser::java_token::Kind) -> bool {
		return Kind::LBRACKET.equals(kind) || Kind::RBRACKET.equals(kind);
	}

	fn next_is_right_brace(&self, index: i32) -> bool {
		let elements: List<TextElement> = self.original_elements.subList(index, &self.original_elements.size());
		for element in elements {
			if !element.is_space_or_tab() {
				return element.is_token();
			}
		}
		return false;
	}

	fn add_indent(&self) {
		 {
			let i: i32 = 0;
			while i < self.STANDARD_INDENTATION_SIZE {
				{
					self.indentation.add(TokenTextElement::new(GeneratedJavaParserConstants.SPACE));
				}
				i += 1;
			 }
		 }
	
	}

	fn remove_indent(&self) {
		 {
			let i: i32 = 0;
			while i < self.STANDARD_INDENTATION_SIZE && !self.indentation.isEmpty() {
				{
					self.indentation.remove(self.indentation.size() - 1);
				}
				i += 1;
			 }
		 }
	
	}

	fn apply_added_diff_element(&mut self, added: &com::github::javaparser::printer::lexicalpreservation::added::Added) /* thrown(java.lang.IllegalStateException) */ {
		if added.is_indent() {
			self.add_indent();
			self.added_indentation = true;
			self.diff_index += 1;
			return;
		}
		if added.is_unindent() {
			self.remove_indent();
			self.added_indentation = false;
			self.diff_index += 1;
			return;
		}
		let added_text_element: TextElement = added.to_text_element()?;
		let used: bool = false;
		let is_previous_element_newline: bool = (self.original_index > 0) && self.original_elements.get(self.original_index - 1).is_newline();
		if is_previous_element_newline {
			let elements: List<TextElement> = self.process_indentation(self.indentation, &self.original_elements.subList(0, self.original_index - 1));
			let next_is_right_brace: bool = self.next_is_right_brace(self.original_index);
			for e in elements {
				if !next_is_right_brace && e instanceof TokenTextElement && self.original_elements.get(self.original_index).is_token(&(e as TokenTextElement).get_token_kind()) {
					self.original_index += 1;
				} else {
					self.node_text.add_element(self.original_index += 1 !!!check!!! post increment, e);
				}
			}
		} else if self.is_afterl_brace(self.node_text, self.original_index) && !self.isa_replacement(self.diff_index) {
			if added_text_element.is_newline() {
				used = true;
			}
			self.node_text.add_element(self.original_index += 1 !!!check!!! post increment, TokenTextElement::new(&TokenTypes::eol_token_kind()));
			// This remove the space in "{ }" when adding a new line
			while self.original_index >= 2 && self.original_elements.get(self.original_index - 2).is_space_or_tab() {
				self.original_elements.remove(self.original_index - 2);
				self.original_index -= 1;
			}
			for e in self.process_indentation(self.indentation, &self.original_elements.subList(0, self.original_index - 1)) {
				self.node_text.add_element(self.original_index += 1 !!!check!!! post increment, e);
			}
			// inserted by us in this transformation we do not want to insert it again
			if !self.added_indentation {
				for e in self.indentation_block() {
					self.node_text.add_element(self.original_index += 1 !!!check!!! post increment, e);
				}
			}
		}
		if !used {
			// Handling trailing comments
			let sufficient_tokens_remain_to_skip: bool = self.node_text.number_of_elements() > self.original_index + 2;
			let current_is_a_comment: bool = self.node_text.get_text_element(self.original_index).is_comment();
			let previous_is_a_comment: bool = self.original_index > 0 && self.node_text.get_text_element(self.original_index - 1).is_comment();
			let current_is_newline: bool = self.node_text.get_text_element(self.original_index).is_newline();
			let is_first_element: bool = self.original_index == 0;
			let previous_is_white_space: bool = self.original_index > 0 && self.node_text.get_text_element(self.original_index - 1).is_white_space();
			let comment_is_before_added_element: bool = current_is_a_comment && added_text_element.get_range().isPresent() && self.node_text.get_text_element(self.original_index).get_range().map(|range|range.is_before(&added_text_element.get_range().get())).orElse(false);
			if sufficient_tokens_remain_to_skip && current_is_a_comment && comment_is_before_added_element {
				// Need to get behind the comment:
				// FIXME: Why 2? This comment and the next newline?
				self.original_index += 2;
				// Defer originalIndex increment
				self.node_text.add_element(self.original_index, added_text_element);
				// We want to adjust the indentation while considering the new element that we added
				self.original_index = self.adjust_indentation(self.indentation, self.node_text, self.original_index, false)?;
				// Now we can increment
				self.original_index += 1;
			} else if current_is_newline && previous_is_a_comment {
				/* 
	                 * Manage the case where we want to add an element, after an expression which is followed by a comment on the same line.
	                 * This is not the same case as the one who handles the trailing comments, because in this case the node text element is a new line (not a comment)
	                 * For example : {@code private String a; // this is a }
	                 */ 
				// Insert after the new line which follows this comment.
				self.original_index += 1;
				// We want to adjust the indentation while considering the new element that we added
				self.original_index = self.adjust_indentation(self.indentation, self.node_text, self.original_index, false)?;
				// Defer originalIndex increment
				self.node_text.add_element(self.original_index, added_text_element);
				// Now we can increment.
				self.original_index += 1;
			} else if current_is_newline && added_text_element.is_child() {
				// not <code>  \nvalue();</code> --> this case appears on member replacement for example
				if !is_previous_element_newline && !is_first_element && !previous_is_white_space {
					// Insert after the new line
					self.original_index += 1;
					// We want to adjust the indentation while considering the new element that we
					// added
					self.original_index = self.adjust_indentation(self.indentation, self.node_text, self.original_index, false)?;
				}
				self.node_text.add_element(self.original_index, added_text_element);
				self.original_index += 1;
			} else {
				self.node_text.add_element(self.original_index, added_text_element);
				self.original_index += 1;
			}
		}
		if added_text_element.is_newline() {
			let followed_by_unindent: bool = self.is_followed_by_unindent(self.diff_elements, self.diff_index);
			let next_is_right_brace: bool = self.next_is_right_brace(self.original_index);
			let next_is_new_line: bool = self.original_elements.get(self.original_index).is_newline();
			if (!next_is_new_line && !next_is_right_brace) || followed_by_unindent {
				self.original_index = self.adjust_indentation(self.indentation, self.node_text, self.original_index, followed_by_unindent)?;
			}
		}
		self.diff_index += 1;
	}

	fn is_followed_by_unindent(&self, diff_elements: &/* Java */ java::util::List /**/, diff_index: i32) -> bool {
		let next_index_value: i32 = diff_index + 1;
		return (next_index_value) < diff_elements.size() && diff_elements.get(next_index_value).is_added() && diff_elements.get(next_index_value).get_element() instanceof CsmUnindent;
	}

	fn adjust_indentation(&self, indentation: &/* Java */ java::util::List /**/, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, node_text_index: i32, followed_by_unindent: bool) /* thrown(java.lang.IllegalStateException) */ -> i32 {
		let indentation_adj: List<TextElement> = self.process_indentation(indentation, &node_text.get_elements().subList(0, node_text_index - 1));
		if node_text_index < node_text.number_of_elements() && node_text.get_text_element(node_text_index).is_token() {
			indentation_adj = indentation_adj.subList(0, indentation_adj.size() - Math::min(self.STANDARD_INDENTATION_SIZE, &indentation_adj.size()));
		} else if followed_by_unindent {
			indentation_adj = indentation_adj.subList(0, &Math::max(0, indentation_adj.size() - self.STANDARD_INDENTATION_SIZE));
		}
		for e in indentation_adj {
			if (node_text_index < node_text.number_of_elements()) && node_text.get_text_element(node_text_index).is_space_or_tab() {
				node_text_index += 1;
			} else {
				node_text.get_elements().add(node_text_index += 1 !!!check!!! post increment, e);
			}
		}
		if node_text_index < 0 {
			return Err(IllegalStateException::new());
		}
		return node_text_index;
	}

	fn isa_replacement(&self, diff_index: i32) -> bool {
		return (diff_index > 0) && self.diff_elements.get(diff_index).is_added() && self.diff_elements.get(diff_index - 1).is_removed();
	}

	fn is_replaced(&self, diff_index: i32) -> bool {
		return (diff_index < self.diff_elements.size() - 1) && self.diff_elements.get(diff_index + 1).is_added() && self.diff_elements.get(diff_index).is_removed();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return "Difference{" + self.diff_elements + '}';
	}
}

struct EnforcingIndentationContext {
	start: i32,
	extra_characters: i32,
}

impl EnforcingIndentationContext {
	pub fn new(start: i32) -> com::github::javaparser::printer::lexicalpreservation::difference::EnforcingIndentationContext {
		this(start, 0);
	}

	pub fn new(start: i32, extra_characters: i32) -> com::github::javaparser::printer::lexicalpreservation::difference::EnforcingIndentationContext {
		self.start = start;
		self.extraCharacters = extra_characters;
	}
}

pub struct ArrayIterator<T> {
	iterator: /* Java */ java::util::ListIterator /**/,
}

impl<T> ArrayIterator {
	pub fn new(elements: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::difference::ArrayIterator {
		this(elements, 0);
	}

	pub fn new(elements: &/* Java */ java::util::List /**/, index: i32) -> com::github::javaparser::printer::lexicalpreservation::difference::ArrayIterator {
		self.iterator = elements.listIterator(index);
	}

	pub fn has_next(&self) -> bool {
		return self.iterator.hasNext();
	}

	pub fn next(&self) -> T {
		return self.iterator.next();
	}

	pub fn has_previous(&self) -> bool {
		return self.iterator.hasPrevious();
	}

	pub fn previous(&self) -> T {
		return self.iterator.previous();
	}

	pub fn next_index(&self) -> i32 {
		return self.iterator.nextIndex();
	}

	pub fn previous_index(&self) -> i32 {
		return self.iterator.previousIndex();
	}

	pub fn index(&self) -> i32 {
		return self.iterator.nextIndex() - 1;
	}

	pub fn remove(&self) {
		self.iterator.remove();
		;
	}

	pub fn set(&self, e: &T) {
		self.iterator.set(e);
	}

	pub fn add(&self, e: &T) {
		self.iterator.add(e);
		;
	}
}

impl<T> /* Java */ java::util::ListIterator /**/ for ArrayIterator<T> {}

impl<T> /* Java */ java::util::Iterator /**/ for ArrayIterator<T> {}