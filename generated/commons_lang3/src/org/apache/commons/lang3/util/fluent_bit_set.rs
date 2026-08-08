use java::io::Serializable;
use java::util::BitSet;
use java::util::Objects;
use java::util::stream::IntStream;

pub struct FluentBitSet {
	bit_set: /* Java */ java::util::BitSet /**/,
}

impl FluentBitSet {
	static serialVersionUID: i64 = 1;

	pub fn new() -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		this(BitSet::new());
	}

	pub fn new(set: &/* Java */ java::util::BitSet /**/) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bitSet = Objects::requireNonNull(set, "set");
	}

	pub fn new(nbits: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		this(BitSet::new(nbits));
	}

	pub fn and(&self, set: &/* Java */ java::util::BitSet /**/) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.and(set);
		return self;
	}

	pub fn and(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.and(set.bitSet);
		return self;
	}

	pub fn and_not(&self, set: &/* Java */ java::util::BitSet /**/) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.andNot(set);
		return self;
	}

	pub fn and_not(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bitSet.andNot(set.bitSet);
		return self;
	}

	pub fn bit_set(&self) -> /* Java */ java::util::BitSet /**/ {
		return self.bit_set;
	}

	pub fn cardinality(&self) -> i32 {
		return self.bit_set.cardinality();
	}

	pub fn clear(&self) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.clear();
		return self;
	}

	pub fn clear(&self, bit_index_array: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		for /* final */ e in bit_index_array {
			self.bitSet.clear(e);
		}
		return self;
	}

	pub fn clear(&self, bit_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.clear(bit_index);
		return self;
	}

	pub fn clear(&self, from_index: i32, to_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.clear(from_index, to_index);
		return self;
	}

	pub fn clone(&self) -> /* Java */ java::lang::Object /**/ {
		return FluentBitSet::new(self.bit_set.clone() as BitSet);
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if !(obj instanceof FluentBitSet) {
			return false;
		}
		/* final */ let other: FluentBitSet = obj as FluentBitSet;
		return Objects::equals(self.bit_set, other.bitSet);
	}

	pub fn flip(&self, bit_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.flip(bit_index);
		return self;
	}

	pub fn flip(&self, from_index: i32, to_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.flip(from_index, to_index);
		return self;
	}

	pub fn get(&self, bit_index: i32) -> bool {
		return self.bit_set.get(bit_index);
	}

	pub fn get(&self, from_index: i32, to_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		return FluentBitSet::new(&self.bit_set.get(from_index, to_index));
	}

	pub fn hash_code(&self) -> i32 {
		return self.bit_set.hashCode();
	}

	pub fn intersects(&self, set: &/* Java */ java::util::BitSet /**/) -> bool {
		return self.bit_set.intersects(set);
	}

	pub fn intersects(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> bool {
		return self.bit_set.intersects(set.bitSet);
	}

	pub fn is_empty(&self) -> bool {
		return self.bit_set.isEmpty();
	}

	pub fn length(&self) -> i32 {
		return self.bit_set.length();
	}

	pub fn next_clear_bit(&self, from_index: i32) -> i32 {
		return self.bit_set.nextClearBit(from_index);
	}

	pub fn next_set_bit(&self, from_index: i32) -> i32 {
		return self.bit_set.nextSetBit(from_index);
	}

	pub fn or(&self, set: &/* Java */ java::util::BitSet /**/) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.or(set);
		return self;
	}

	pub fn or(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		for /* final */ e in set {
			self.bitSet.or(e.bitSet);
		}
		return self;
	}

	pub fn or(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bitSet.or(set.bitSet);
		return self;
	}

	pub fn previous_clear_bit(&self, from_index: i32) -> i32 {
		return self.bit_set.previousClearBit(from_index);
	}

	pub fn previous_set_bit(&self, from_index: i32) -> i32 {
		return self.bit_set.previousSetBit(from_index);
	}

	pub fn set(&self, bit_index_array: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		for /* final */ e in bit_index_array {
			self.bit_set.set(e);
		}
		return self;
	}

	pub fn set(&self, bit_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.set(bit_index);
		return self;
	}

	pub fn set(&self, bit_index: i32, value: bool) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.set(bit_index, value);
		return self;
	}

	pub fn set(&self, from_index: i32, to_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.set(from_index, to_index);
		return self;
	}

	pub fn set(&self, from_index: i32, to_index: i32, value: bool) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.set(from_index, to_index, value);
		return self;
	}

	pub fn set_inclusive(&self, from_index: i32, to_index: i32) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.set(from_index, to_index + 1);
		return self;
	}

	pub fn size(&self) -> i32 {
		return self.bit_set.size();
	}

	pub fn stream(&self) -> /* Java */ java::util::stream::IntStream /**/ {
		return self.bit_set.stream();
	}

	pub fn to_byte_array(&self) -> &[i8] {
		return self.bit_set.toByteArray();
	}

	pub fn to_long_array(&self) -> &[i64] {
		return self.bit_set.toLongArray();
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return self.bit_set.toString();
	}

	pub fn xor(&self, set: &/* Java */ java::util::BitSet /**/) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.xor(set);
		return self;
	}

	pub fn xor(&self, set: &org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet) -> org::apache::commons::lang3::util::fluent_bit_set::FluentBitSet {
		self.bit_set.xor(set.bitSet);
		return self;
	}
}

impl /* Java */ java::lang::Cloneable /**/ for FluentBitSet {}

impl /* Java */ java::io::Serializable /**/ for FluentBitSet {}