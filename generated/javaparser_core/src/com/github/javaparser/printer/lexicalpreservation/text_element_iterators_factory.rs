use java::util::Iterator;
use java::util::LinkedList;
use java::util::List;

struct TextElementIteratorsFactory;

impl TextElementIteratorsFactory {
	fn reverse_iterator(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, index: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		let text_element: TextElement = node_text.get_text_element(index);
		if text_element instanceof TokenTextElement {
			return SingleElementIterator<TokenTextElement>::new(text_element as TokenTextElement) {
				pub fn remove(&self) {
					node_text.remove_element(index);
				}
	
			};
		}
		if text_element instanceof ChildTextElement {
			let child_text_element: ChildTextElement = text_element as ChildTextElement;
			let text_for_child: NodeText = child_text_element.get_node_text_for_wrapped_node();
			return com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::TextElementIteratorsFactory::reverse_iterator(text_for_child);
		}
		return Err(IllegalArgumentException::new());
	}

	pub fn remove(&self) {
		node_text.remove_element(index);
	}

	pub fn reverse_iterator(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText) -> /* Java */ java::util::Iterator /**/ {
		return com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::TextElementIteratorsFactory::partial_reverse_iterator(node_text, node_text.number_of_elements() - 1);
	}

	pub fn partial_reverse_iterator(&self, node_text: &com::github::javaparser::printer::lexicalpreservation::node_text::NodeText, from_index: i32) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::util::Iterator /**/ {
		let elements: List<Iterator<TokenTextElement>> = LinkedList<>::new();
		 {
			let i: i32 = from_index;
			while i >= 0 {
				{
					elements.add(&com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::TextElementIteratorsFactory::reverse_iterator(node_text, i)?);
				}
				i -= 1;
			 }
		 }
	
		return ComposedIterator<>::new(elements);
	}
}

struct CascadingIterator<E> {
	next_provider: com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::Provider,
	current: /* Java */ java::util::Iterator /**/,
	next: /* Java */ java::util::Iterator /**/,
	last_returned_from_current: bool = false,
	last_returned_from_next: bool = false,
}

impl<E> CascadingIterator {
	pub fn new(current: &/* Java */ java::util::Iterator /**/, next_provider: &com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::Provider) -> com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::CascadingIterator {
		self.nextProvider = next_provider;
		self.current = current;
	}

	pub fn has_next(&mut self) -> bool {
		if self.current.hasNext() {
			return true;
		}
		if self.next == null {
			self.next = self.next_provider.provide();
		}
		return self.next.hasNext();
	}

	pub fn next(&mut self) -> E {
		if self.current.hasNext() {
			self.last_returned_from_current = true;
			self.last_returned_from_next = false;
			return self.current.next();
		}
		if self.next == null {
			self.next = self.next_provider.provide();
		}
		self.last_returned_from_current = false;
		self.last_returned_from_next = true;
		return self.next.next();
	}

	pub fn remove(&self) /* thrown(java.lang.IllegalArgumentException) */ {
		if self.last_returned_from_current {
			self.current.remove();
			return;
		}
		if self.last_returned_from_next {
			self.next.remove();
			return;
		}
		return Err(IllegalArgumentException::new());
	}
}

impl<E> /* Java */ java::util::Iterator /**/ for CascadingIterator<E> {}

trait Provider<E>;

struct EmptyIterator<E>;

impl<E> EmptyIterator {
	pub fn has_next(&self) -> bool {
		return false;
	}

	pub fn next(&self) /* thrown(java.lang.IllegalArgumentException) */ -> E {
		return Err(IllegalArgumentException::new());
	}
}

impl<E> /* Java */ java::util::Iterator /**/ for EmptyIterator<E> {}

struct SingleElementIterator<E> {
	element: E,
	returned: bool,
}

impl<E> SingleElementIterator {
	fn new(element: &E) -> com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::SingleElementIterator {
		self.element = element;
	}

	pub fn has_next(&self) -> bool {
		return !self.returned;
	}

	pub fn next(&mut self) -> E {
		self.returned = true;
		return self.element;
	}

	pub fn remove(&self) {
	}
}

impl<E> /* Java */ java::util::Iterator /**/ for SingleElementIterator<E> {}

struct ComposedIterator<E> {
	elements: /* Java */ java::util::List /**/,
	curr_index: i32,
}

impl<E> ComposedIterator {
	fn new(elements: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::text_element_iterators_factory::ComposedIterator {
		self.elements = elements;
		self.curr_index = 0;
	}

	pub fn has_next(&self) -> bool {
		if self.curr_index >= self.elements.size() {
			return false;
		}
		if self.elements.get(self.curr_index).hasNext() {
			return true;
		}
		self.curr_index += 1;
		return self.has_next();
	}

	pub fn next(&self) /* thrown(java.lang.IllegalArgumentException) */ -> E {
		if !self.has_next() {
			return Err(IllegalArgumentException::new());
		}
		return self.elements.get(self.curr_index).next();
	}

	pub fn remove(&self) {
		self.elements.get(self.curr_index).remove();
	}
}

impl<E> /* Java */ java::util::Iterator /**/ for ComposedIterator<E> {}