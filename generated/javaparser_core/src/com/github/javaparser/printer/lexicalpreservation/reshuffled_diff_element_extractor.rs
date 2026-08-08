use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmElement;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmMix;
use crate::com::github::javaparser::printer::concretesyntaxmodel::CsmToken;
use crate::com::github::javaparser::printer::lexicalpreservation::Difference::ArrayIterator;
use java::util;

pub struct ReshuffledDiffElementExtractor {
	node_text: com::github::javaparser::printer::lexicalpreservation::node_text::NodeText,
}

impl ReshuffledDiffElementExtractor {
	fn of(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> com::github::javaparser::printer::lexicalpreservation::reshuffled_diff_element_extractor::ReshuffledDiffElementExtractor {
		return ReshuffledDiffElementExtractor::new(node_text);
	}

	fn new(node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> com::github::javaparser::printer::lexicalpreservation::reshuffled_diff_element_extractor::ReshuffledDiffElementExtractor {
		self.nodeText = node_text;
	}

	pub fn extract(&self, diff_elements: &/* Java */ java::util::List /**/) {
		let iterator: ArrayIterator<DifferenceElement> = ArrayIterator<>::new(diff_elements);
		while iterator.has_next() {
			let diff_element: DifferenceElement = iterator.next();
			if diff_element instanceof Reshuffled {
				let reshuffled: Reshuffled = diff_element as Reshuffled;
				// First, let's see how many tokens we need to attribute to the previous version of the of the CsmMix
				let elements_from_previous_order: CsmMix = reshuffled.get_previous_order();
				let elements_from_next_order: CsmMix = reshuffled.get_next_order();
				// This contains indexes from elementsFromNextOrder to indexes from elementsFromPreviousOrder
				let correspondance_between_next_order_and_previous_order: Map<Integer, Integer> = self.get_correspondance_between_next_order_and_previous_order(elements_from_previous_order, elements_from_next_order);
				// We now find out which Node Text elements corresponds to the elements in the original CSM
				let node_text_index_of_previous_elements: List<Integer> = self.find_index_of_corresponding_node_text_element(&elements_from_previous_order.get_elements(), self.node_text);
				let node_text_index_of_previous_elements_iterator: PeekingIterator<Integer> = PeekingIterator<>::new(node_text_index_of_previous_elements);
				let node_text_index_to_previous_c_s_m_index: Map<Integer, Integer> = HashMap<>::new();
				while node_text_index_of_previous_elements_iterator.has_next() {
					let value: i32 = node_text_index_of_previous_elements_iterator.next()?;
					if value != -1 {
						node_text_index_to_previous_c_s_m_index.put(value, &node_text_index_of_previous_elements_iterator.current_index());
					}
				}
				let last_node_text_index: i32 = node_text_index_of_previous_elements.stream().max(Integer::compareTo).orElse(-1);
				// Elements to be added at the end
				let elements_to_be_added_at_the_end: List<CsmElement> = LinkedList<>::new();
				let next_order_elements: List<CsmElement> = elements_from_next_order.get_elements();
				let elements_to_add_before_given_original_c_s_m_element: Map<Integer, List<CsmElement>> = HashMap<>::new();
				 {
					let ni: i32 = 0;
					while ni < next_order_elements.size() {
						{
							// If it has a mapping, then it is kept
							if !correspondance_between_next_order_and_previous_order.containsKey(ni) {
								// Ok, it is something new. Where to put it? Let's see what is the first following
								// element that has a mapping
								let original_csm_index: i32 = -1;
								 {
									let nj: i32 = ni + 1;
									while nj < next_order_elements.size() && original_csm_index == -1 {
										{
											if correspondance_between_next_order_and_previous_order.containsKey(nj) {
												original_csm_index = correspondance_between_next_order_and_previous_order.get(nj);
												if !elements_to_add_before_given_original_c_s_m_element.containsKey(original_csm_index) {
													elements_to_add_before_given_original_c_s_m_element.put(original_csm_index, LinkedList<>::new());
												}
												elements_to_add_before_given_original_c_s_m_element.get(original_csm_index).add(&next_order_elements.get(ni));
											}
										}
										nj += 1;
									 }
								 }
	
								// it does not preceed anything, so it goes at the end
								if original_csm_index == -1 {
									elements_to_be_added_at_the_end.add(&next_order_elements.get(ni));
								}
							}
						}
						ni += 1;
					 }
				 }
	
				// We go over the original node text elements, in the order they appear in the NodeText.
				// Considering an original node text element (ONE)
				// * we verify if it corresponds to a CSM element. If it does not we just move on, otherwise
				// we find the correspond OCE (Original CSM Element)
				// * we first add new elements that are marked to be added before OCE
				// * if OCE is marked to be present also in the "after" CSM we add a kept element,
				// otherwise we add a removed element
				// Remove the whole Reshuffled element
				iterator.remove();
				if last_node_text_index != -1 {
					 {
						let nt_index: i32 = 0;
						while nt_index <= last_node_text_index {
							{
								if node_text_index_to_previous_c_s_m_index.containsKey(nt_index) {
									let index_of_original_c_s_m_element: i32 = node_text_index_to_previous_c_s_m_index.get(nt_index);
									if elements_to_add_before_given_original_c_s_m_element.containsKey(index_of_original_c_s_m_element) {
										for element_to_add in elements_to_add_before_given_original_c_s_m_element.get(index_of_original_c_s_m_element) {
											iterator.add(Added::new(element_to_add));
										}
									}
									let original_c_s_m_element: CsmElement = elements_from_previous_order.get_elements().get(index_of_original_c_s_m_element);
									let to_be_kept: bool = correspondance_between_next_order_and_previous_order.containsValue(index_of_original_c_s_m_element);
									if to_be_kept {
										iterator.add(Kept::new(original_c_s_m_element));
									} else {
										iterator.add(Removed::new(original_c_s_m_element));
									}
								}
							// else we have a simple node text element, without associated csm element, just keep ignore it
							}
							nt_index += 1;
						 }
					 }
	
				}
				// add all of them
				for element_to_add in elements_to_be_added_at_the_end {
					iterator.add(Added::new(element_to_add));
				}
			}
		}
	}

	fn get_correspondance_between_next_order_and_previous_order(&self, elements_from_previous_order: &com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix, elements_from_next_order: &com::github::javaparser::printer::concretesyntaxmodel::csm_mix::CsmMix) /* thrown(java.lang.UnsupportedOperationException) */ -> /* Java */ java::util::Map /**/ {
		let correspondance_between_next_order_and_previous_order: Map<Integer, Integer> = HashMap<>::new();
		let previous_order_elements_iterator: ArrayIterator<CsmElement> = ArrayIterator<>::new(&elements_from_previous_order.get_elements());
		let sync_next_index: i32 = 0;
		while previous_order_elements_iterator.has_next() {
			let pe: CsmElement = previous_order_elements_iterator.next();
			let next_order_elements_iterator: ArrayIterator<CsmElement> = ArrayIterator<>::new(&elements_from_next_order.get_elements(), sync_next_index);
			while next_order_elements_iterator.has_next() {
				let ne: CsmElement = next_order_elements_iterator.next();
				if !correspondance_between_next_order_and_previous_order.values().contains(&previous_order_elements_iterator.index()) && DifferenceElementCalculator::matching(ne, pe)? {
					correspondance_between_next_order_and_previous_order.put(&next_order_elements_iterator.index(), &previous_order_elements_iterator.index());
					// set the position to start on the next {@code nextOrderElementsIterator} iteration
					sync_next_index = next_order_elements_iterator.next_index();
					break;
				}
			}
		}
		return correspondance_between_next_order_and_previous_order;
	}

	fn find_index_of_corresponding_node_text_element(&self, elements: &/* Java */ java::util::List /**/, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> /* Java */ java::util::List /**/ {
		let corresponding_indices: List<Integer> = ArrayList<>::new();
		let csm_element_list_iterator: PeekingIterator<CsmElement> = PeekingIterator<>::new(elements);
		while csm_element_list_iterator.has_next() {
			let is_first_iteration_on_csm_elements: bool = !csm_element_list_iterator.has_previous();
			let previous_csm_element_index: i32 = csm_element_list_iterator.previous_index();
			let csm_element: CsmElement = csm_element_list_iterator.next()?;
			let potential_matches: Map<MatchClassification, Integer> = EnumMap<>::new(MatchClassification.class);
			let node_text_list_iterator: PeekingIterator<TextElement> = PeekingIterator<>::new(&node_text.get_elements());
			while node_text_list_iterator.has_next() {
				let is_first_iteration_on_node_text_elements: bool = !node_text_list_iterator.has_previous();
				let text_element: TextElement = node_text_list_iterator.next()?;
				let current_text_element_index: i32 = node_text_list_iterator.current_index();
				if !corresponding_indices.contains(current_text_element_index) {
					let is_corresponding: bool = csm_element.is_corresponding_element(text_element);
					if is_corresponding {
						let has_same_previous_element: bool = false;
						if !is_first_iteration_on_node_text_elements && !is_first_iteration_on_csm_elements {
							let previous_text_element: TextElement = node_text.get_text_element(current_text_element_index - 1);
							has_same_previous_element = elements.get(previous_csm_element_index).is_corresponding_element(previous_text_element);
						}
						let has_same_next_element: bool = false;
						if csm_element_list_iterator.has_next() {
							let next_text_element: TextElement = node_text_list_iterator.peek();
							has_same_next_element = elements.get(&csm_element_list_iterator.next_index()).is_corresponding_element(next_text_element);
						}
						if has_same_previous_element && has_same_next_element {
							potential_matches.putIfAbsent(MatchClassification::ALL, current_text_element_index);
						} else if has_same_previous_element {
							potential_matches.putIfAbsent(MatchClassification::PREVIOUS_AND_SAME, current_text_element_index);
						} else if has_same_next_element {
							potential_matches.putIfAbsent(MatchClassification::NEXT_AND_SAME, current_text_element_index);
						} else {
							potential_matches.putIfAbsent(MatchClassification::SAME_ONLY, current_text_element_index);
						}
					} else if self.is_almost_corresponding_element(text_element, csm_element) {
						potential_matches.putIfAbsent(MatchClassification::ALMOST, current_text_element_index);
					}
				}
			}
			// Prioritize the matches from best to worst
			let best_match_key: Optional<MatchClassification> = potential_matches.keySet().stream().min(&Comparator::comparing(MatchClassification::getPriority));
			if best_match_key.isPresent() {
				corresponding_indices.add(&potential_matches.get(&best_match_key.get()));
			} else {
				corresponding_indices.add(-1);
			}
		}
		return corresponding_indices;
	}

	fn is_almost_corresponding_element(&self, text_element: &com::github::javaparser::printer::lexicalpreservation::text_element::TextElement, csm_element: &com::github::javaparser::printer::concretesyntaxmodel::csm_element::CsmElement) -> bool {
		if csm_element.is_corresponding_element(text_element) {
			return false;
		}
		return text_element.is_white_space() && csm_element instanceof CsmToken && (csm_element as CsmToken).is_white_space();
	}
}

enum MatchClassification {
	priority: i32,
}