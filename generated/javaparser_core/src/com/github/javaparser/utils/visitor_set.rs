use crate::com::github::javaparser::ast::Node;
use crate::com::github::javaparser::ast::visitor::GenericVisitor;
use crate::com::github::javaparser::ast::visitor::Visitable;
use crate::com::github::javaparser::ast::visitor::VoidVisitor;
use java::util::Collection;
use java::util::HashSet;
use java::util::Iterator;
use java::util::Set;
use java::util::stream::Collectors;

pub struct VisitorSet<N: com::github::javaparser::ast::node::Node> {
	inner_set: /* Java */ java::util::Set /**/ = HashSet<>::new(),
	hashcode_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
	equals_visitor: com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor,
	itr: /* Java */ java::util::Iterator /**/ = inner_set.iterator(),
}

impl<N: com::github::javaparser::ast::node::Node> VisitorSet {
	pub fn new(hashcode_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor, equals_visitor: &com::github::javaparser::ast::visitor::generic_visitor::GenericVisitor) -> com::github::javaparser::utils::visitor_set::VisitorSet {
		self.hashcodeVisitor = hashcode_visitor;
		self.equalsVisitor = equals_visitor;
	}

	pub fn add(&self, elem: &N) -> bool {
		return self.inner_set.add(EqualsHashcodeOverridingFacade::new(elem));
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

	pub fn clear(&self) {
		self.inner_set.clear();
	}

	pub fn contains(&self, elem: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_set.contains(EqualsHashcodeOverridingFacade::new(elem as N));
	}

	pub fn contains_all(&self, col: &/* Java */ java::util::Collection /**/) -> bool {
		for elem in col {
			if !self.contains(elem) {
				return false;
			}
	
		}
		return true;
	}

	pub fn is_empty(&self) -> bool {
		return self.inner_set.isEmpty();
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return Iterator<N>::new() {
			/* final */ let itr: Iterator<EqualsHashcodeOverridingFacade> = self.inner_set.iterator(),
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

	pub fn remove(&self, elem: &/* Java */ java::lang::Object /**/) -> bool {
		return self.inner_set.remove(EqualsHashcodeOverridingFacade::new(elem as N));
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

	pub fn size(&self) -> i32 {
		return self.inner_set.size();
	}

	pub fn to_array(&self) -> &[/* Java */ java::lang::Object /**/] {
		return self.inner_set.stream().map(|facade|facade.overridden).collect(&Collectors::toList()).toArray();
	}

	pub fn to_array<T>(&self, arr: &&[T]) -> &[T] {
		return self.inner_set.stream().map(|facade|facade.overridden).collect(&Collectors::toList()).toArray(arr);
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.inner_set.stream().map(|facade|facade.overridden.to_string()).collect(&Collectors::joining(",", "[", "]"));
	}
}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::Set /**/ for VisitorSet<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::util::Collection /**/ for VisitorSet<N> {}

impl<N: com::github::javaparser::ast::node::Node> /* Java */ java::lang::Iterable /**/ for VisitorSet<N> {}

struct EqualsHashcodeOverridingFacade {
	overridden: N,
}

impl EqualsHashcodeOverridingFacade {
	fn new(overridden: &N) -> com::github::javaparser::utils::visitor_set::EqualsHashcodeOverridingFacade {
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
		if obj == null || !(obj instanceof VisitorSet.EqualsHashcodeOverridingFacade) {
			return false;
		}
		return self.overridden.accept(, (obj as EqualsHashcodeOverridingFacade).overridden);
	}
}

impl com::github::javaparser::ast::visitor::visitable::Visitable for EqualsHashcodeOverridingFacade {}