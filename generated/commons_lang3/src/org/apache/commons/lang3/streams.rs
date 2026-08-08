use java::util::ArrayList;
use java::util::Collection;
use java::util::Collections;
use java::util::List;
use java::util::Set;
use java::util::function::BiConsumer;
use java::util::function::BinaryOperator;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::function::Supplier;
use java::util::stream::Collector;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::Functions::FailableConsumer;
use crate::org::apache::commons::lang3::Functions::FailableFunction;
use crate::org::apache::commons::lang3::Functions::FailablePredicate;

pub struct Streams;

impl Streams {
	pub fn stream<O>(&self, stream: &/* Java */ java::util::Collection /**/) -> org::apache::commons::lang3::streams::FailableStream {
		return org::apache::commons::lang3::streams::Streams::stream(&stream.stream());
	}

	pub fn stream<O>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::streams::FailableStream {
		return FailableStream<>::new(stream);
	}

	pub fn to_array<O>(&self, element_type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::stream::Collector /**/ {
		return ArrayCollector<>::new(element_type);
	}

	pub fn new() -> org::apache::commons::lang3::streams::Streams {
	// empty
	}
}

pub struct ArrayCollector<O> {
	element_type: /* Java */ java::lang::Class /**/,
}

impl<O> ArrayCollector {
	static characteristics: /* Java */ java::util::Set /**/ = Collections::emptySet();

	pub fn new(element_type: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::streams::ArrayCollector {
		self.elementType = element_type;
	}

	pub fn accumulator(&self) -> /* Java */ java::util::function::BiConsumer /**/ {
		return List::add;
	}

	pub fn characteristics(&self) -> /* Java */ java::util::Set /**/ {
		return self.characteristics;
	}

	pub fn combiner(&self) -> /* Java */ java::util::function::BinaryOperator /**/ {
		return |(left, right)|{
			left.addAll(right);
			return left;
		};
	}

	pub fn finisher(&self) -> /* Java */ java::util::function::Function /**/ {
		return |list|list.toArray(&ArrayUtils::new_instance(self.element_type, &list.size()));
	}

	pub fn supplier(&self) -> /* Java */ java::util::function::Supplier /**/ {
		return ArrayList::new;
	}
}

impl<O> /* Java */ java::util::stream::Collector /**/ for ArrayCollector<O> {}

pub struct FailableStream<O> {
	stream: /* Java */ java::util::stream::Stream /**/,
	terminated: bool,
}

impl<O> FailableStream {
	pub fn new(stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::streams::FailableStream {
		self.stream = stream;
	}

	pub fn all_match(&self, predicate: &org::apache::commons::lang3::functions::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> bool {
		self.assert_not_terminated()?;
		return self.stream().allMatch(&Functions::as_predicate(predicate));
	}

	pub fn any_match(&self, predicate: &org::apache::commons::lang3::functions::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> bool {
		self.assert_not_terminated()?;
		return self.stream().anyMatch(&Functions::as_predicate(predicate));
	}

	fn assert_not_terminated(&self) /* thrown(java.lang.IllegalStateException) */ {
		if self.terminated {
			return Err(IllegalStateException::new("This stream is already terminated."));
		}
	}

	pub fn collect<A, R>(&self, collector: &/* Java */ java::util::stream::Collector /**/) -> R {
		self.make_terminated();
		return self.stream().collect(collector);
	}

	pub fn collect<A, R>(&self, supplier: &/* Java */ java::util::function::Supplier /**/, accumulator: &/* Java */ java::util::function::BiConsumer /**/, combiner: &/* Java */ java::util::function::BiConsumer /**/) -> R {
		self.make_terminated();
		return self.stream().collect(supplier, accumulator, combiner);
	}

	pub fn filter(&mut self, predicate: &org::apache::commons::lang3::functions::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::streams::FailableStream {
		self.assert_not_terminated()?;
		self.stream = self.stream.filter(&Functions::as_predicate(predicate));
		return self;
	}

	pub fn for_each(&self, action: &org::apache::commons::lang3::functions::FailableConsumer) {
		self.make_terminated();
		self.stream().forEach(&Functions::as_consumer(action));
	}

	fn make_terminated(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		self.assert_not_terminated()?;
		self.terminated = true;
	}

	pub fn map<R>(&self, mapper: &org::apache::commons::lang3::functions::FailableFunction) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::streams::FailableStream {
		self.assert_not_terminated()?;
		return FailableStream<>::new(&self.stream.map(&Functions::as_function(mapper)));
	}

	pub fn reduce(&self, identity: &O, accumulator: &/* Java */ java::util::function::BinaryOperator /**/) /* thrown(java.lang.IllegalStateException) */ -> O {
		self.make_terminated()?;
		return self.stream().reduce(identity, accumulator);
	}

	pub fn stream(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return self.stream;
	}
}