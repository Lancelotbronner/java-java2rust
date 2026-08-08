use java::util::ArrayList;
use java::util::Iterator;
use java::util::List;
use java::util::Objects;
use java::util::StringTokenizer;
use java::util::stream::Stream;
use java::util::stream::StreamSupport;
use crate::org::apache::commons::lang3::ArrayUtils;

pub struct IterableStringTokenizer;

impl IterableStringTokenizer {
	pub fn new(str: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::util::iterable_string_tokenizer::IterableStringTokenizer {
		super(str);
	}

	pub fn new(str: &/* Java */ java::lang::String /**/, delim: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::util::iterable_string_tokenizer::IterableStringTokenizer {
		super(str, delim);
	}

	pub fn new(str: &/* Java */ java::lang::String /**/, delim: &/* Java */ java::lang::String /**/, return_delims: bool) -> org::apache::commons::lang3::util::iterable_string_tokenizer::IterableStringTokenizer {
		super(str, delim, return_delims);
	}

	pub fn iterator(&self) -> /* Java */ java::util::Iterator /**/ {
		return Iterator<String>::new() {
			pub fn has_next(&self) -> bool {
				return self.hasMoreElements();
			}
	
			pub fn next(&self) -> String {
				return Objects::toString(&self.nextElement(), null);
			}
	
		};
	}

	pub fn has_next(&self) -> bool {
		return self.hasMoreElements();
	}

	pub fn next(&self) -> /* Java */ java::lang::String /**/ {
		return Objects::toString(&self.nextElement(), null);
	}

	pub fn to_array(&self) -> &[/* Java */ java::lang::String /**/] {
		return self.to_list().toArray(ArrayUtils::EMPTY_STRING_ARRAY);
	}

	pub fn to_list(&self) -> /* Java */ java::util::List /**/ {
		/* final */ let list: List<String> = ArrayList<>::new();
		self.forEach(list::add);
		return list;
	}

	pub fn to_stream(&self) -> /* Java */ java::util::stream::Stream /**/ {
		return StreamSupport::stream(&self.spliterator(), false);
	}
}

impl /* Java */ java::lang::Iterable /**/ for IterableStringTokenizer {}

impl /* Java */ java::util::Enumeration /**/ for IterableStringTokenizer {}