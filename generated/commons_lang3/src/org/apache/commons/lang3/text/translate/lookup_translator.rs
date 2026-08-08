use java::io::IOException;
use java::io::Writer;
use java::util::HashMap;
use java::util::HashSet;

pub struct LookupTranslator {
	lookup_map: /* Java */ java::util::HashMap /**/,
	prefix_set: /* Java */ java::util::HashSet /**/,
	shortest: i32,
	longest: i32,
}

impl LookupTranslator {
	pub fn new(lookup: &&[/* Java */ java::lang::CharSequence /**/]) -> org::apache::commons::lang3::text::translate::lookup_translator::LookupTranslator {
		self.lookup_map = HashMap<>::new();
		self.prefix_set = HashSet<>::new();
		let tmp_shortest: i32 = Integer::MAX_VALUE;
		let tmp_longest: i32 = 0;
		if lookup != null {
			for /* final */ seq in lookup {
				self.lookupMap.put(&seq[0].toString(), &seq[1].toString());
				self.prefixSet.add(&seq[0].charAt(0));
				/* final */ let sz: i32 = seq[0].length();
				if sz < tmp_shortest {
					tmp_shortest = sz;
				}
				if sz > tmp_longest {
					tmp_longest = sz;
				}
			}
		}
		self.shortest = tmp_shortest;
		self.longest = tmp_longest;
	}

	pub fn translate(&self, input: &/* Java */ java::lang::CharSequence /**/, index: i32, out: &/* Java */ java::io::Writer /**/) /* thrown(java.io.IOException) */ -> i32 {
		// check if translation exists for the input at position index
		if self.prefix_set.contains(&input.charAt(index)) {
			let max: i32 = self.longest;
			if index + self.longest > input.length() {
				max = input.length() - index;
			}
			// implement greedy algorithm by trying maximum match first
			 {
				let i: i32 = max;
				while i >= self.shortest {
					{
						/* final */ let sub_seq: CharSequence = input.subSequence(index, index + i);
						/* final */ let result: String = self.lookup_map.get(&sub_seq.toString());
						if result != null {
							out.write(result);
							return i;
						}
					}
					i -= 1;
				 }
			 }
	
		}
		return 0;
	}
}