use java::io::IOException;
use java::util::Iterator;
use java::util::StringJoiner;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::exception::UncheckedException;
use crate::org::apache::commons::lang3::function::FailableBiConsumer;

pub struct AppendableJoiner<T> {
	prefix: /* Java */ java::lang::CharSequence /**/,
	suffix: /* Java */ java::lang::CharSequence /**/,
	delimiter: /* Java */ java::lang::CharSequence /**/,
	appender: org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer,
}

impl<T> AppendableJoiner {
	pub fn builder<T>(&self) -> org::apache::commons::lang3::appendable_joiner::Builder {
		return Builder<>::new();
	}

	fn joina<A: /* Java */ java::lang::Appendable /**/, T>(&self, appendable: &A, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, elements: &T) /* thrown(java.io.IOException) */ -> A {
		return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::join_array(appendable, prefix, suffix, delimiter, appender, elements);
	}

	fn join_array<A: /* Java */ java::lang::Appendable /**/, T>(&self, appendable: &A, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, elements: &&[T]) /* thrown(java.io.IOException) */ -> A {
		appendable.append(prefix);
		if elements != null {
			if elements.length > 0 {
				appender.accept(appendable, elements[0]);
			}
			 {
				let i: i32 = 1;
				while i < elements.length {
					{
						appendable.append(delimiter);
						appender.accept(appendable, elements[i]);
					}
					i += 1;
				 }
			 }
	
		}
		appendable.append(suffix);
		return appendable;
	}

	fn joini<T>(&self, string_builder: &/* Java */ java::lang::StringBuilder /**/, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, elements: &/* Java */ java::lang::Iterable /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::StringBuilder /**/ {
		let r0 = 'try0: {
			return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::join_iterable(string_builder, prefix, suffix, delimiter, appender, elements);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				// Cannot happen with a StringBuilder.
				break 'try0 Err(UncheckedException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn join_iterable<A: /* Java */ java::lang::Appendable /**/, T>(&self, appendable: &A, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, elements: &/* Java */ java::lang::Iterable /**/) /* thrown(java.io.IOException) */ -> A {
		appendable.append(prefix);
		if elements != null {
			/* final */ let iterator: Iterator<T> = elements.iterator();
			if iterator.hasNext() {
				appender.accept(appendable, &iterator.next());
			}
			while iterator.hasNext() {
				appendable.append(delimiter);
				appender.accept(appendable, &iterator.next());
			}
		}
		appendable.append(suffix);
		return appendable;
	}

	fn joinsb<T>(&self, string_builder: &/* Java */ java::lang::StringBuilder /**/, prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, elements: &T) /* thrown(org.apache.commons.lang3.exception.UncheckedException | java.io.IOException) */ -> /* Java */ java::lang::StringBuilder /**/ {
		let r0 = 'try0: {
			return match org::apache::commons::lang3::appendable_joiner::AppendableJoiner::join_array(string_builder, prefix, suffix, delimiter, appender, elements) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IOException) => {
				// Cannot happen with a StringBuilder.
				break 'try0 Err(UncheckedException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn non_null(&self, value: &/* Java */ java::lang::CharSequence /**/) -> /* Java */ java::lang::CharSequence /**/ {
		return  if value != null { value } else { StringUtils::EMPTY };
	}

	fn new(prefix: &/* Java */ java::lang::CharSequence /**/, suffix: &/* Java */ java::lang::CharSequence /**/, delimiter: &/* Java */ java::lang::CharSequence /**/, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer) -> org::apache::commons::lang3::appendable_joiner::AppendableJoiner {
		self.prefix = org::apache::commons::lang3::appendable_joiner::AppendableJoiner::non_null(prefix);
		self.suffix = org::apache::commons::lang3::appendable_joiner::AppendableJoiner::non_null(suffix);
		self.delimiter = org::apache::commons::lang3::appendable_joiner::AppendableJoiner::non_null(delimiter);
		self.appender =  if appender != null { appender } else { |(a, e)|a.append(&String.valueOf(e)) };
	}

	pub fn join(&self, string_builder: &/* Java */ java::lang::StringBuilder /**/, elements: &/* Java */ java::lang::Iterable /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedException) */ -> /* Java */ java::lang::StringBuilder /**/ {
		return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::joini(string_builder, self.prefix, self.suffix, self.delimiter, self.appender, elements)?;
	}

	pub fn join(&self, string_builder: &/* Java */ java::lang::StringBuilder /**/, elements: &T) /* thrown(org.apache.commons.lang3.exception.UncheckedException | java.io.IOException) */ -> /* Java */ java::lang::StringBuilder /**/ {
		return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::joinsb(string_builder, self.prefix, self.suffix, self.delimiter, self.appender, elements)?;
	}

	pub fn joina<A: /* Java */ java::lang::Appendable /**/>(&self, appendable: &A, elements: &/* Java */ java::lang::Iterable /**/) /* thrown(java.io.IOException) */ -> A {
		return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::join_iterable(appendable, self.prefix, self.suffix, self.delimiter, self.appender, elements)?;
	}

	pub fn joina<A: /* Java */ java::lang::Appendable /**/>(&self, appendable: &A, elements: &T) /* thrown(java.io.IOException) */ -> A {
		return org::apache::commons::lang3::appendable_joiner::AppendableJoiner::joina(appendable, self.prefix, self.suffix, self.delimiter, self.appender, elements)?;
	}
}

pub struct Builder<T> {
	prefix: /* Java */ java::lang::CharSequence /**/,
	suffix: /* Java */ java::lang::CharSequence /**/,
	delimiter: /* Java */ java::lang::CharSequence /**/,
	appender: org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer,
}

impl<T> Builder {
	fn new() -> org::apache::commons::lang3::appendable_joiner::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::appendable_joiner::AppendableJoiner {
		return AppendableJoiner<>::new(self.prefix, self.suffix, self.delimiter, self.appender);
	}

	pub fn set_delimiter(&mut self, delimiter: &/* Java */ java::lang::CharSequence /**/) -> org::apache::commons::lang3::appendable_joiner::Builder {
		self.delimiter = delimiter;
		return self;
	}

	pub fn set_element_appender(&mut self, appender: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer) -> org::apache::commons::lang3::appendable_joiner::Builder {
		self.appender = appender;
		return self;
	}

	pub fn set_prefix(&mut self, prefix: &/* Java */ java::lang::CharSequence /**/) -> org::apache::commons::lang3::appendable_joiner::Builder {
		self.prefix = prefix;
		return self;
	}

	pub fn set_suffix(&mut self, suffix: &/* Java */ java::lang::CharSequence /**/) -> org::apache::commons::lang3::appendable_joiner::Builder {
		self.suffix = suffix;
		return self;
	}
}

impl<T> /* Java */ java::util::function::Supplier /**/ for Builder<T> {}