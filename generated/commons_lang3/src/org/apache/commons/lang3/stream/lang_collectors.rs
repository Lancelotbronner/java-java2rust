use java::util::Arrays;
use java::util::Collections;
use java::util::Objects;
use java::util::Set;
use java::util::StringJoiner;
use java::util::function::BiConsumer;
use java::util::function::BinaryOperator;
use java::util::function::Function;
use java::util::function::Supplier;
use java::util::stream::Collector;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::StringUtils;

pub struct LangCollectors;

impl LangCollectors {
	static CH_NOID: /* Java */ java::util::Set /**/ = Collections::emptySet();

	pub fn collect<T, R, A>(&self, collector: &/* Java */ java::util::stream::Collector /**/, array: &T) -> R {
		return Streams::of(array).collect(collector);
	}

	pub fn joining(&self) -> /* Java */ java::util::stream::Collector /**/ {
		return SimpleCollector<>::new(StringBuilder::new, StringBuilder::append, StringBuilder::append, StringBuilder::toString, self.CH_NOID);
	}

	pub fn joining(&self, delimiter: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::util::stream::Collector /**/ {
		return org::apache::commons::lang3::stream::lang_collectors::LangCollectors::joining(delimiter, StringUtils::EMPTY, StringUtils::EMPTY);
	}

	pub fn joining(&self, delimiter: &/* Java */ java::lang::CharSequence /**/, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::util::stream::Collector /**/ {
		return org::apache::commons::lang3::stream::lang_collectors::LangCollectors::joining(delimiter, prefix, suffix, Objects::toString);
	}

	pub fn joining(&self, delimiter: &/* Java */ java::lang::CharSequence /**/, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, to_string: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::stream::Collector /**/ {
		return SimpleCollector<>::new(|()|StringJoiner::new(delimiter, prefix, suffix), |(a, t)|a.add(&to_string.apply(t)), StringJoiner::merge, StringJoiner::toString, self.CH_NOID);
	}

	fn new() -> org::apache::commons::lang3::stream::lang_collectors::LangCollectors {
	// No instance
	}
}

struct SimpleCollector<T, A, R> {
	accumulator: /* Java */ java::util::function::BiConsumer /**/,
	characteristics: /* Java */ java::util::Set /**/,
	combiner: /* Java */ java::util::function::BinaryOperator /**/,
	finisher: /* Java */ java::util::function::Function /**/,
	supplier: /* Java */ java::util::function::Supplier /**/,
}

impl<T, A, R> SimpleCollector {
	fn new(supplier: &/* Java */ java::util::function::Supplier /**/, accumulator: &/* Java */ java::util::function::BiConsumer /**/, combiner: &/* Java */ java::util::function::BinaryOperator /**/, finisher: &/* Java */ java::util::function::Function /**/, characteristics: &/* Java */ java::util::Set /**/) -> org::apache::commons::lang3::stream::lang_collectors::SimpleCollector {
		self.supplier = supplier;
		self.accumulator = accumulator;
		self.combiner = combiner;
		self.finisher = finisher;
		self.characteristics = characteristics;
	}

	pub fn accumulator(&self) -> /* Java */ java::util::function::BiConsumer /**/ {
		return self.accumulator;
	}

	pub fn characteristics(&self) -> /* Java */ java::util::Set /**/ {
		return self.characteristics;
	}

	pub fn combiner(&self) -> /* Java */ java::util::function::BinaryOperator /**/ {
		return self.combiner;
	}

	pub fn finisher(&self) -> /* Java */ java::util::function::Function /**/ {
		return self.finisher;
	}

	pub fn supplier(&self) -> /* Java */ java::util::function::Supplier /**/ {
		return self.supplier;
	}
}

impl<T, A, R> /* Java */ java::util::stream::Collector /**/ for SimpleCollector<T, A, R> {}