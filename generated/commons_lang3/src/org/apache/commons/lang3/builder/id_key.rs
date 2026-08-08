struct IDKey {
	value: /* Java */ java::lang::Object /**/,
	id: i32,
}

impl IDKey {
	fn new(value: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::id_key::IDKey {
		// This is the Object hash code
		self.id = System::identityHashCode(value);
		// There have been some cases (LANG-459) that return the
		// same identity hash code for different objects. So
		// the value is also added to disambiguate these cases.
		self.value = value;
	}

	pub fn equals(&self, other: &/* Java */ java::lang::Object /**/) -> bool {
		if !(other instanceof IDKey) {
			return false;
		}
		/* final */ let id_key: IDKey = other as IDKey;
		if self.id != id_key.id {
			return false;
		}
		// Note that identity equals is used.
		return self.value == id_key.value;
	}

	pub fn hash_code(&self) -> i32 {
		return self.id;
	}
}