use java::util::ArrayList;
use java::util::Collection;
use java::util::Collections;
use java::util::Enumeration;
use java::util::Iterator;
use java::util::List;
use java::util::Objects;
use java::util::Set;
use java::util::Spliterator;
use java::util::Spliterators;
use java::util::Spliterators::AbstractSpliterator;
use java::util::function::BiConsumer;
use java::util::function::BinaryOperator;
use java::util::function::Consumer;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::function::Supplier;
use java::util::stream::Collector;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use java::util::stream::StreamSupport;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::function::Failable;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableFunction;
use crate::org::apache::commons::lang3::function::FailablePredicate;

pub struct Streams;

impl Streams {
	pub fn failable_stream<T>(&self, stream: &/* Java */ java::util::Collection /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return org::apache::commons::lang3::stream::streams::Streams::failable_stream(&org::apache::commons::lang3::stream::streams::Streams::of(stream));
	}

	pub fn failable_stream<T>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return FailableStream<>::new(stream);
	}

	pub fn failable_stream<T>(&self, value: &T) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return org::apache::commons::lang3::stream::streams::Streams::failable_stream(&org::apache::commons::lang3::stream::streams::Streams::stream_of(value));
	}

	pub fn failable_stream<T>(&self, values: &T) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return org::apache::commons::lang3::stream::streams::Streams::failable_stream(&org::apache::commons::lang3::stream::streams::Streams::of(values));
	}

	pub fn instances_of<E>(&self, clazz: &/* Java */ java::lang::Class /**/, collection: &/* Java */ java::util::Collection /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::instances_of(clazz, &org::apache::commons::lang3::stream::streams::Streams::of(collection));
	}

	fn instances_of<E>(&self, clazz: &/* Java */ java::lang::Class /**/, stream: &/* Java */ java::util::stream::Stream /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::of(stream).filter(clazz::isInstance) as Stream<E>;
	}

	pub fn non_null<E>(&self, collection: &/* Java */ java::util::Collection /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::of(collection).filter(Objects::nonNull);
	}

	pub fn non_null<E>(&self, array: &E) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::non_null(&org::apache::commons::lang3::stream::streams::Streams::stream_of(array));
	}

	pub fn non_null<E>(&self, array: &E) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::non_null(&org::apache::commons::lang3::stream::streams::Streams::of(array));
	}

	pub fn non_null<E>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return org::apache::commons::lang3::stream::streams::Streams::of(stream).filter(Objects::nonNull);
	}

	pub fn of<E>(&self, collection: &/* Java */ java::util::Collection /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return  if collection == null { Stream::empty() } else { collection.stream() };
	}

	pub fn of<E>(&self, enumeration: &/* Java */ java::util::Enumeration /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return StreamSupport::stream(EnumerationSpliterator<>::new(Long::MAX_VALUE, Spliterator::ORDERED, enumeration), false);
	}

	pub fn of<E>(&self, iterable: &/* Java */ java::lang::Iterable /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return  if iterable == null { Stream::empty() } else { StreamSupport::stream(&iterable.spliterator(), false) };
	}

	pub fn of<E>(&self, iterator: &/* Java */ java::util::Iterator /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return  if iterator == null { Stream::empty() } else { StreamSupport::stream(&Spliterators::spliteratorUnknownSize(iterator, Spliterator::ORDERED), false) };
	}

	fn of<E>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> /* Java */ java::util::stream::Stream /**/ {
		return  if stream == null { Stream::empty() } else { stream };
	}

	pub fn of<T>(&self, values: &T) -> /* Java */ java::util::stream::Stream /**/ {
		return  if values == null { Stream::empty() } else { Stream::of(values) };
	}

	pub fn stream<E>(&self, collection: &/* Java */ java::util::Collection /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return org::apache::commons::lang3::stream::streams::Streams::failable_stream(collection);
	}

	pub fn stream<T>(&self, stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		return org::apache::commons::lang3::stream::streams::Streams::failable_stream(stream);
	}

	fn stream_of<T>(&self, value: &T) -> /* Java */ java::util::stream::Stream /**/ {
		return  if value == null { Stream::empty() } else { Stream::of(value) };
	}

	pub fn to_array<T>(&self, element_type: &/* Java */ java::lang::Class /**/) -> /* Java */ java::util::stream::Collector /**/ {
		return ArrayCollector<>::new(element_type);
	}

	pub fn new() -> org::apache::commons::lang3::stream::streams::Streams {
	// empty
	}
}

pub struct ArrayCollector<E> {
	element_type: /* Java */ java::lang::Class /**/,
}

impl<E> ArrayCollector {
	static characteristics: /* Java */ java::util::Set /**/ = Collections::emptySet();

	pub fn new(element_type: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::stream::streams::ArrayCollector {
		self.elementType = Objects::requireNonNull(element_type, "elementType");
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

impl<E> /* Java */ java::util::stream::Collector /**/ for ArrayCollector<E> {}

struct EnumerationSpliterator<T> {
	enumeration: /* Java */ java::util::Enumeration /**/,
}

impl<T> EnumerationSpliterator {
	fn new(estimated_size: i64, additional_characteristics: i32, enumeration: &/* Java */ java::util::Enumeration /**/) -> org::apache::commons::lang3::stream::streams::EnumerationSpliterator {
		super(estimated_size, additional_characteristics);
		self.enumeration = Objects::requireNonNull(enumeration, "enumeration");
	}

	pub fn for_each_remaining(&self, action: &/* Java */ java::util::function::Consumer /**/) {
		while self.enumeration.hasMoreElements() {
			self.next(action);
		}
	}

	fn next(&self, action: &/* Java */ java::util::function::Consumer /**/) -> bool {
		action.accept(&self.enumeration.nextElement());
		return true;
	}

	pub fn try_advance(&self, action: &/* Java */ java::util::function::Consumer /**/) -> bool {
		return self.enumeration.hasMoreElements() && self.next(action);
	}
}

impl<T> /* Java */ java::util::Spliterator /**/ for EnumerationSpliterator<T> {}

pub struct FailableStream<T> {
	stream: /* Java */ java::util::stream::Stream /**/,
	terminated: bool,
}

impl<T> FailableStream {
	pub fn new(stream: &/* Java */ java::util::stream::Stream /**/) -> org::apache::commons::lang3::stream::streams::FailableStream {
		self.stream = stream;
	}

	pub fn all_match(&self, predicate: &org::apache::commons::lang3::function::failable_predicate::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> bool {
		self.assert_not_terminated()?;
		return self.stream().allMatch(&Failable::as_predicate(predicate));
	}

	pub fn any_match(&self, predicate: &org::apache::commons::lang3::function::failable_predicate::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> bool {
		self.assert_not_terminated()?;
		return self.stream().anyMatch(&Failable::as_predicate(predicate));
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

	pub fn filter(&mut self, predicate: &org::apache::commons::lang3::function::failable_predicate::FailablePredicate) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::stream::streams::FailableStream {
		self.assert_not_terminated()?;
		self.stream = self.stream.filter(&Failable::as_predicate(predicate));
		return self;
	}

	pub fn for_each(&self, action: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) {
		self.make_terminated();
		self.stream().forEach(&Failable::as_consumer(action));
	}

	fn make_terminated(&mut self) /* thrown(java.lang.IllegalStateException) */ {
		self.assert_not_terminated()?;
		self.terminated = true;
	}

	pub fn map<R>(&self, mapper: &org::apache::commons::lang3::function::failable_function::FailableFunction) /* thrown(java.lang.IllegalStateException) */ -> org::apache::commons::lang3::stream::streams::FailableStream {
		self.assert_not_terminated()?;
		return FailableStream<>::new(&self.stream.map(&Failable::as_function(mapper)));
	}

	pub fn reduce(&self, identity: &T, accumulator: &/* Java */ java::util::function::BinaryOperator /**/) /* thrown(java.lang.IllegalStateException) */ -> T {
		self.make_terminated()?;
		return self.stream().reduce(identity, accumulator);
	}

	pub fn stream(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return self.stream;
	}
}