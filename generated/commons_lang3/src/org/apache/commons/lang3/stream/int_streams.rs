use java::util::stream::IntStream;

pub struct IntStreams;

impl IntStreams {
	pub fn of(&self, values: i32) -> /* Java */ java::util::stream::IntStream /**/ {
		return  if values == null { IntStream::empty() } else { IntStream::of(values) };
	}

	pub fn range(&self, end_exclusive: i32) -> /* Java */ java::util::stream::IntStream /**/ {
		return IntStream::range(0, end_exclusive);
	}

	pub fn range_closed(&self, end_inclusive: i32) -> /* Java */ java::util::stream::IntStream /**/ {
		return IntStream::rangeClosed(0, end_inclusive);
	}

	pub fn new() -> org::apache::commons::lang3::stream::int_streams::IntStreams {
	// empty
	}
}