use java::util::Objects;
use java::util::Random;

struct CachedRandomBits {
	random: /* Java */ java::util::Random /**/,
	cache: &[i8],
	bit_index: i32,
}

impl CachedRandomBits {
	static MAX_CACHE_SIZE: i32 = Integer::MAX_VALUE /* signed */ >> 3;

	static MAX_BITS: i32 = 32;

	static BIT_INDEX_MASK: i32 = 0x7;

	static BITS_PER_BYTE: i32 = 8;

	fn new(cache_size: i32, random: &/* Java */ java::util::Random /**/) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::cached_random_bits::CachedRandomBits {
		if cache_size <= 0 {
			return Err(IllegalArgumentException::new("cacheSize must be positive"));
		}
		self.cache =  if cache_size <= self.MAX_CACHE_SIZE { : [i8; cache_size] = [0; cache_size] } else { : [i8; self.MAX_CACHE_SIZE] = [0; self.MAX_CACHE_SIZE] };
		self.random = Objects::requireNonNull(random, "random");
		self.random.nextBytes(self.cache);
		self.bitIndex = 0;
	}

	pub fn next_bits(&mut self, bits: i32) /* thrown(java.lang.IllegalArgumentException) */ -> i32 {
		if bits > self.MAX_BITS || bits <= 0 {
			return Err(IllegalArgumentException::new("number of bits must be between 1 and " + self.MAX_BITS));
		}
		let result: i32 = 0;
		// number of generated bits up to now
		let generated_bits: i32 = 0;
		while generated_bits < bits {
			// Convert bitIndex to byte index by dividing by 8 (right shift by 3)
			if self.bit_index /* signed */ >> 3 >= self.cache.length {
				// This should only happen if the bitIndex is exactly matching the cache length
				assert!( self.bit_index == self.cache.length * self.BITS_PER_BYTE);
				self.random.nextBytes(self.cache);
				self.bit_index = 0;
			}
			// Calculate how many bits we can extract from the current byte
			// 1. Get current position within byte (0-7) using bitIndex & 0x7
			// 2. Calculate remaining bits in byte: 8 - (position within byte)
			// 3. Take minimum of remaining bits in byte and bits still needed
			/* final */ let generated_bits_in_iteration: i32 = Math::min(self.BITS_PER_BYTE - (self.bit_index & self.BIT_INDEX_MASK), bits - generated_bits);
			// Shift existing result left to make room for new bits
			result = result << generated_bits_in_iteration;
			// Extract and append new bits:
			// 1. Get byte from cache (bitIndex >> 3 converts bit index to byte index)
			// 2. Shift right by bit position within byte (bitIndex & 0x7)
			// 3. Mask to keep only the bits we want ((1 << generatedBitsInIteration) - 1)
			result |= self.cache[self.bit_index /* signed */ >> 3] /* signed */ >> (self.bit_index & self.BIT_INDEX_MASK) & ((1 << generated_bits_in_iteration) - 1);
			// Update counters
			generated_bits += generated_bits_in_iteration;
			self.bit_index += generated_bits_in_iteration;
		}
		return result;
	}
}