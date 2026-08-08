use java::util::List;
use java::util::ListIterator;
use java::util::NoSuchElementException;
use java::util::Objects;

pub struct PeekingIterator<E> {
	iterator: /* Java */ java::util::ListIterator /**/,
	exhausted: bool,
	slot_filled: bool,
	slot: E,
}

impl<E> PeekingIterator {
	pub fn peeking_iterator<E>(&self, iterator: &/* Java */ java::util::ListIterator /**/) -> com::github::javaparser::printer::lexicalpreservation::peeking_iterator::PeekingIterator {
		Objects::requireNonNull(iterator, "iterator");
		if iterator instanceof PeekingIterator<?> {
			/* final */ let it: PeekingIterator<E> = iterator as PeekingIterator<E>;
			return it;
		}
		return PeekingIterator<>::new(iterator);
	}

	pub fn new(iterator: &/* Java */ java::util::ListIterator /**/) -> com::github::javaparser::printer::lexicalpreservation::peeking_iterator::PeekingIterator {
		self.iterator = iterator;
	}

	pub fn new(list: &/* Java */ java::util::List /**/) -> com::github::javaparser::printer::lexicalpreservation::peeking_iterator::PeekingIterator {
		self.iterator = list.listIterator();
	}

	fn fill(&mut self) {
		if self.exhausted || self.slot_filled {
			return;
		}
		if self.iterator.hasNext() {
			self.slot = self.iterator.next();
			self.slot_filled = true;
		} else {
			self.exhausted = true;
			self.slot = null;
			self.slot_filled = false;
		}
	}

	pub fn has_next(&self) -> bool {
		if self.exhausted {
			return false;
		}
		return self.slot_filled || self.iterator.hasNext();
	}

	pub fn peek(&self) -> E {
		self.fill();
		return  if self.exhausted { null } else { self.slot };
	}

	pub fn element(&self) /* thrown(java.util.NoSuchElementException) */ -> E {
		self.fill();
		if self.exhausted {
			return Err(NoSuchElementException::new());
		}
		return self.slot;
	}

	pub fn next(&mut self) /* thrown(java.util.NoSuchElementException) */ -> E {
		if !self.has_next() {
			return Err(NoSuchElementException::new());
		}
		/* final */ let x: E =  if self.slot_filled { self.slot } else { self.iterator.next() };
		// reset the lookahead slot
		self.slot = null;
		self.slot_filled = false;
		return x;
	}

	pub fn remove(&self) /* thrown(java.lang.IllegalStateException) */ {
		if self.slot_filled {
			return Err(IllegalStateException::new("peek() or element() called before remove()"));
		}
		self.iterator.remove();
	}

	pub fn has_previous(&self) -> bool {
		return self.iterator.hasPrevious();
	}

	pub fn previous(&self) -> E {
		return self.iterator.previous();
	}

	pub fn next_index(&self) -> i32 {
		return self.iterator.nextIndex();
	}

	pub fn current_index(&self) -> i32 {
		if !self.has_previous() {
			return self.previous_index();
		}
	
		return self.next_index() - 1;
	}

	pub fn previous_index(&self) -> i32 {
		return self.iterator.previousIndex();
	}

	pub fn set(&self, e: &E) /* thrown(java.lang.IllegalStateException) */ {
		if self.slot_filled {
			return Err(IllegalStateException::new("peek() or element() called before set()"));
		}
		self.iterator.set(e);
	}

	pub fn add(&self, e: &E) /* thrown(java.lang.IllegalStateException) */ {
		if self.slot_filled {
			return Err(IllegalStateException::new("peek() or element() called before add()"));
		}
		self.iterator.add(e);
	}
}

impl<E> /* Java */ java::util::ListIterator /**/ for PeekingIterator<E> {}

impl<E> /* Java */ java::util::Iterator /**/ for PeekingIterator<E> {}

impl<E> com::github::javaparser::printer::lexicalpreservation::lookahead_iterator::LookaheadIterator for PeekingIterator<E> {}