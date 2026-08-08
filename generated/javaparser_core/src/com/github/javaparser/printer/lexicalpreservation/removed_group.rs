use crate::com::github::javaparser::JavaToken;
use crate::com::github::javaparser::TokenRange;
use crate::com::github::javaparser::TokenTypes;
use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmToken;
use java::util::Iterator;
use java::util::List;
use java::util::Optional;
use java::util::function::Function;
use java::util::stream::Collectors;
use java::util::stream::IntStream;

struct RemovedGroup {
	first_element_index: /* Java */ java::lang::Integer /**/,
	removed_list: /* Java */ java::util::List /**/,
	is_processed: bool = false,
	has_only_whitespace_java_token_in_front_function: /* Java */ java::util::function::Function /**/ = |begin|self.has_only_white_space_for_token_function(begin, |token|token.get_previous_token()),
	has_only_whitespace_java_token_behind_function: /* Java */ java::util::function::Function /**/ = |end|self.has_only_white_space_for_token_function(end, |token|token.get_next_token()),
	has_only_whitespace_in_front_function: /* Java */ java::util::function::Function /**/ = |token_range|has_only_whitespace_java_token_in_front_function.apply(&token_range.get_begin()),
	has_only_whitespace_behind_function: /* Java */ java::util::function::Function /**/ = |token_range|has_only_whitespace_java_token_behind_function.apply(&token_range.get_end()),
	current_index: i32 = 0,
}

impl RemovedGroup {
	fn new(first_element_index: &/* Java */ java::lang::Integer /**/, removed_list: &/* Java */ java::util::List /**/) /* thrown(java.lang.IllegalArgumentException) */ -> com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup {
		if first_element_index == null {
			return Err(IllegalArgumentException::new("firstElementIndex should not be null"));
		}
		if removed_list == null || removed_list.isEmpty() {
			return Err(IllegalArgumentException::new("removedList should not be null or empty"));
		}
		self.firstElementIndex = first_element_index;
		self.removedList = removed_list;
	}

	pub fn of(&self, first_element_index: &/* Java */ java::lang::Integer /**/, removed_list: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::removed_group::RemovedGroup {
		return RemovedGroup::new(first_element_index, removed_list);
	}

	fn processed(&mut self) {
		self.is_processed = true;
	}

	fn is_processed(&self) -> bool {
		return self.is_processed;
	}

	fn get_indices_being_removed(&self) -> /* Java */ java::util::List /**/ {
		return IntStream::range(self.first_element_index, self.first_element_index + self.removed_list.size()).boxed().collect(&Collectors::toList());
	}

	fn get_last_element_index(&self) -> /* Java */ java::lang::Integer /**/ {
		let indices_being_removed: List<Integer> = self.get_indices_being_removed();
		return indices_being_removed.get(indices_being_removed.size() - 1);
	}

	fn get_first_element(&self) -> com::github::javaparser::printer::lexicalpreservation::removed::Removed {
		return self.removed_list.get(0);
	}

	fn get_last_element(&self) -> com::github::javaparser::printer::lexicalpreservation::removed::Removed {
		return self.removed_list.get(self.removed_list.size() - 1);
	}

	fn is_last_element(&self, element: &com::github::javaparser::printer::lexicalpreservation::removed::Removed) -> bool {
		return self.get_last_element().equals(element);
	}

	fn isa_complete_line(&self) -> bool {
		return self.has_only_whitespace(&self.get_first_element(), self.has_only_whitespace_in_front_function) && self.has_only_whitespace(&self.get_last_element(), self.has_only_whitespace_behind_function);
	}

	fn has_only_whitespace(&self, start_element: &com::github::javaparser::printer::lexicalpreservation::removed::Removed, has_only_whitespace_function: &/* Java */ java::util::function::Function /**/) -> bool {
		let has_only_whitespace: bool = false;
		if start_element.is_child() {
			let csm_child: LexicalDifferenceCalculator.CsmChild = start_element.get_element() as LexicalDifferenceCalculator.CsmChild;
			let child: Node = csm_child.get_child();
			let token_range: Optional<TokenRange> = child.get_token_range();
			if token_range.isPresent() {
				has_only_whitespace = has_only_whitespace_function.apply(&token_range.get());
			}
		} else if start_element.is_token() {
			let token: CsmToken = start_element.get_element() as CsmToken;
			if token.is_new_line() {
				has_only_whitespace = true;
			}
		}
		return has_only_whitespace;
	}

	fn has_only_white_space_for_token_function(&self, token: &com::github::javaparser::java_token::JavaToken, token_function: &/* Java */ java::util::function::Function /**/) -> bool {
		let token_result: Optional<JavaToken> = token_function.apply(token);
		if token_result.isPresent() {
			if TokenTypes::is_whitespace_but_not_end_of_line(&token_result.get().get_kind()) {
				return self.has_only_white_space_for_token_function(&token_result.get(), token_function);
			}
			if TokenTypes::is_end_of_line_token(&token_result.get().get_kind()) {
				return true;
			}
			return false;
		}
		return true;
	}

	fn get_indentation(&self) -> /* Java */ java::util::Optional /**/ {
		let first_element: Removed = null;
		let indentation: i32 = 0;
		// search for the first element which is not a new line
		let it: Iterator = self.iterator();
		while it.hasNext() {
			first_element = it.next() as Removed;
			if first_element.is_new_line() {
				continue;
			}
	
			break;
		}
		if first_element.is_child() {
			let csm_child: LexicalDifferenceCalculator.CsmChild = first_element.get_element() as LexicalDifferenceCalculator.CsmChild;
			let child: Node = csm_child.get_child();
			let token_range: Optional<TokenRange> = child.get_token_range();
			if token_range.isPresent() {
				let begin: JavaToken = token_range.get().get_begin();
				if self.has_only_whitespace_java_token_in_front_function.apply(begin) {
					let previous_token: Optional<JavaToken> = begin.get_previous_token();
					while previous_token.isPresent() && (TokenTypes::is_whitespace_but_not_end_of_line(&previous_token.get().get_kind())) {
						indentation += 1;
						previous_token = previous_token.get().get_previous_token();
					}
					if previous_token.isPresent() {
						if TokenTypes::is_end_of_line_token(&previous_token.get().get_kind()) {
							return Optional::of(&Integer::valueOf(indentation));
						}
						return Optional::empty();
					}
					return Optional::of(&Integer::valueOf(indentation));
				}
			}
		}
		return Optional::empty();
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return Iterator<Removed>::new() {
			let current_index: i32 = 0,
			pub fn has_next(&self) -> bool {
				return self.current_index < self.removed_list.size() && self.removed_list.get(self.current_index) != null;
			}
	
			pub fn next(&self) -> Removed {
				return self.removed_list.get(self.current_index += 1 !!!check!!! post increment);
			}
	
		};
	}

	pub fn has_next(&self) -> bool {
		return self.current_index < self.removed_list.size() && self.removed_list.get(self.current_index) != null;
	}

	pub fn next(&self) -> com::github::javaparser::printer::lexicalpreservation::removed::Removed {
		return self.removed_list.get(self.current_index += 1 !!!check!!! post increment);
	}
}

impl /* Java */ java::lang::Iterable /**/ for RemovedGroup {}