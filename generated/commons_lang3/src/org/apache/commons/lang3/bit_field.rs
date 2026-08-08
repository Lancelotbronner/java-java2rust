pub struct BitField {
	mask: i32,
	shift_count: i32,
}

impl BitField {
	pub fn new(mask: i32) -> org::apache::commons::lang3::bit_field::BitField {
		self.mask = mask;
		self.shiftCount =  if mask == 0 { 0 } else { Integer::numberOfTrailingZeros(mask) };
	}

	pub fn clear(&self, holder: i32) -> i32 {
		return holder & ~self.mask;
	}

	pub fn clear_byte(&self, holder: i8) -> i8 {
		return self.clear(holder) as i8;
	}

	pub fn clear_short(&self, holder: i16) -> i16 {
		return self.clear(holder) as i16;
	}

	pub fn get_raw_value(&self, holder: i32) -> i32 {
		return holder & self.mask;
	}

	pub fn get_short_raw_value(&self, holder: i16) -> i16 {
		return self.get_raw_value(holder) as i16;
	}

	pub fn get_short_value(&self, holder: i16) -> i16 {
		return self.get_value(holder) as i16;
	}

	pub fn get_value(&self, holder: i32) -> i32 {
		return self.get_raw_value(holder) /* signed */ >> self.shift_count;
	}

	pub fn is_all_set(&self, holder: i32) -> bool {
		return (holder & self.mask) == self.mask;
	}

	pub fn is_set(&self, holder: i32) -> bool {
		return (holder & self.mask) != 0;
	}

	pub fn set(&self, holder: i32) -> i32 {
		return holder | self.mask;
	}

	pub fn set_boolean(&self, holder: i32, flag: bool) -> i32 {
		return  if flag { self.set(holder) } else { self.clear(holder) };
	}

	pub fn set_byte(&self, holder: i8) -> i8 {
		return self.set(holder) as i8;
	}

	pub fn set_byte_boolean(&self, holder: i8, flag: bool) -> i8 {
		return  if flag { self.set_byte(holder) } else { self.clear_byte(holder) };
	}

	pub fn set_short(&self, holder: i16) -> i16 {
		return self.set(holder) as i16;
	}

	pub fn set_short_boolean(&self, holder: i16, flag: bool) -> i16 {
		return  if flag { self.set_short(holder) } else { self.clear_short(holder) };
	}

	pub fn set_short_value(&self, holder: i16, value: i16) -> i16 {
		return self.set_value(holder, value) as i16;
	}

	pub fn set_value(&self, holder: i32, value: i32) -> i32 {
		return holder & ~self.mask | value << self.shift_count & self.mask;
	}
}