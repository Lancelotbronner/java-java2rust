use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use java::util;
use java::util::stream::Collectors;

pub struct VisitorList<N: com::github::javaparser::ast::node::Node> {
	inner_list: /* Java */ java::util::List /**/,
	hashcode_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
	equals_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
	itr: /* Java */ java::util::Iterator /**/ = inner_list.iterator(),
	itr: /* Java */ java::util::ListIterator /**/ = inner_list.listIterator(index),
}

impl<N: com::github::javaparser::ast::node::Node> VisitorList {
	pub fn new(hashcode_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, equals_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor) -> com::github::javaparser::utils::visitor_list::VisitorList {
		self.hashcodeVisitor = hashcode_visitor;
		self.equalsVisitor = equals_visitor;
		self.inner_list = ArrayList<>::new();
	}

	pub fn add(&self, elem: &N) -> bool {
		return self.inner_list.add(EqualsHashcodeOverridingFacade::new(elem));
	}

	pub fn add(&self, index: i32, elem: &N) {
		self.inner_list.add(index, EqualsHashcodeOverridingFacade::new(elem));
	}

	pub fn add_all(&self, col: &/* Java */ java::util::Collection /**/) -> bool {
		let modified: bool = false;
		for elem in col {
			if self.add(elem) {
				modified = true;
			}
	
		}
		return modified;
	}

	pub fn add_all(&self, index: i32, col: &/* Java */ java::util::Collection /**/) -> bool {
		if col.isEmpty() {
			return false;
		}
	
		for elem in col {
			if index == self.size() {
				self.add(elem);
			}
			else {self.add(index, elem);
			}
	
			index += 1;
		}
		return true;
	}

	pub fn clear(&self) {
		self.inner_list.clear();
	}

	pub fn contains(&self, elem: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_list.contains(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn contains_all(&self, col: &/* Java */ java::util::Collection /**/) -> bool {
		for elem in col {
			if !self.contains(elem) {
				return false;
			}
	
		}
		return true;
	}

	pub fn get(&self, index: i32) -> N {
		return self.inner_list.get(index).overridden;
	}

	pub fn index_of(&self, elem: &/* Java */ java::lang::Object /**/) -> i32 {
		return self.inner_list.indexOf(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn is_empty(&self) -> bool {
		return self.inner_list.isEmpty();
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return Iterator<N>::new() {
			/* final */ let itr: Iterator<EqualsHashcodeOverridingFacade> = self.inner_list.iterator(),
			pub fn has_next(&self) -> bool {
				return self.itr.hasNext();
			}
	
			pub fn next(&self) -> N {
				return self.itr.next().overridden;
			}
	
			pub fn remove(&self) {
				self.itr.remove();
			}
	
		};
	}

	pub fn has_next(&self) -> bool {
		return self.itr.hasNext();
	}

	pub fn next(&self) -> N {
		return self.itr.next().overridden;
	}

	pub fn remove(&self) {
		self.itr.remove();
	}

	pub fn last_index_of(&self, elem: &/* Java */ java::lang::Object /**/) -> i32 {
		return self.inner_list.lastIndexOf(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn list_iterator(&self) -> /* Java */ java::util::ListIterator /**/ {
		return self.list_iterator(0);
	}

	pub fn list_iterator(&self, index: i32) -> /* Java */ java::util::ListIterator /**/ {
		return ListIterator<N>::new() {
			/* final */ let itr: ListIterator<EqualsHashcodeOverridingFacade> = self.inner_list.listIterator(index),
			pub fn has_next(&self) -> bool {
				return self.itr.hasNext();
			}
	
			pub fn next(&self) -> N {
				return self.itr.next().overridden;
			}
	
			pub fn remove(&self) {
				self.itr.remove();
			}
	
			pub fn add(&self, elem: &N) {
				self.itr.add(EqualsHashcodeOverridingFacade::new(elem as N));
			}
	
			pub fn has_previous(&self) -> bool {
				return self.itr.hasPrevious();
			}
	
			pub fn next_index(&self) -> i32 {
				return self.itr.nextIndex();
			}
	
			pub fn previous(&self) -> N {
				return self.itr.previous().overridden;
			}
	
			pub fn previous_index(&self) -> i32 {
				return self.itr.previousIndex();
			}
	
			pub fn set(&self, elem: &N) {
				self.itr.set(EqualsHashcodeOverridingFacade::new(elem as N));
			}
	
		};
	}

	pub fn has_next(&self) -> bool {
		return self.itr.hasNext();
	}

	pub fn next(&self) -> N {
		return self.itr.next().overridden;
	}

	pub fn remove(&self) {
		self.itr.remove();
	}

	pub fn add(&self, elem: &N) {
		self.itr.add(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn has_previous(&self) -> bool {
		return self.itr.hasPrevious();
	}

	pub fn next_index(&self) -> i32 {
		return self.itr.nextIndex();
	}

	pub fn previous(&self) -> N {
		return self.itr.previous().overridden;
	}

	pub fn previous_index(&self) -> i32 {
		return self.itr.previousIndex();
	}

	pub fn set(&self, elem: &N) {
		self.itr.set(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn remove(&self, elem: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_list.remove(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn remove(&self, index: i32) -> N {
		return self.inner_list.remove(index).overridden;
	}

	pub fn remove_all(&self, col: &/* Java */ java::util::Collection /**/) -> bool {
		let modified: bool = false;
		for elem in col {
			if self.remove(elem) {
				modified = true;
			}
	
		}
		return modified;
	}

	pub fn retain_all(&self, col: &/* Java */ java::util::Collection /**/) -> bool {
		let old_size: i32 = self.size();
		self.clear();
		self.add_all(col as Collection<? extends N>);
		return self.size() != old_size;
	}

	pub fn set(&self, index: i32, elem: &N) -> N {
		return self.inner_list.set(index, EqualsHashcodeOverridingFacade::new(elem as N)).overridden;
	}

	pub fn size(&self) -> i32 {
		return self.inner_list.size();
	}

	pub fn sub_list(&self, from_index: i32, to_index: i32) -> /* Java */ java::util::List /**/ {
		return VisitorList<N>::new(self.hashcode_visitor, self.equals_visitor) {
			{
				self.innerList = VisitorList.innerList.subList(from_index, to_index);
			}};
	}

	pub fn to_array(&self) -> &[/* Java */ java::lang::Object /**/] {
		return self.inner_list.stream().map(|facade|facade.overridden).collect(&Collectors::toList()).toArray();
	}

	pub fn to_array<T>(&self, arr: &&[T]) -> &[T] {
		return self.inner_list.stream().map(|facade|facade.overridden).collect(&Collectors::toList()).toArray(arr);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		let sb: StringBuilder = StringBuilder::new("[");
		if self.size() == 0 {
			return sb.append("]").toString();
		}
	
		for facade in self.inner_list {
			sb.append(facade.overridden.to_string() + ", ");
		}
		return sb.replace(sb.length() - 2, &sb.length(), "]").toString();
	}
}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::List /**/ for VisitorList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::SequencedCollection /**/ for VisitorList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::Collection /**/ for VisitorList<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::lang::Iterable /**/ for VisitorList<N> {}

struct EqualsHashcodeOverridingFacade {
	overridden: N,
}

impl EqualsHashcodeOverridingFacade {
	fn new(overridden: &N) -> com::github::javaparser::utils::visitor_list::EqualsHashcodeOverridingFacade {
		self.overridden = overridden;
	}

	pub fn accept<R, A>(&self, v: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, arg: &A) /* thrown(java.lang.AssertionError) */ -> R {
		return Err(AssertionError::new());
	}

	pub fn accept<A>(&self, v: &com::github::javaparser::ast::visitor::void_visitor::VoidVisitor, arg: &A) /* thrown(java.lang.AssertionError) */ {
		return Err(AssertionError::new());
	}

	pub fn hash_code(&self) -> i32 {
		return self.overridden.accept(, null);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if obj == null || !(obj instanceof VisitorList.EqualsHashcodeOverridingFacade) {
			return false;
		}
		return self.overridden.accept(, (obj as EqualsHashcodeOverridingFacade).overridden);
	}
}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EqualsHashcodeOverridingFacade {}