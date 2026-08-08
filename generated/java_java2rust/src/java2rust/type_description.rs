pub struct TypeDescription {
	clazz: /* Java */ java::lang::Class /**/,
	array_count: i32,
}

impl TypeDescription {
	pub fn new(array_count: i32, clazz: &/* Java */ java::lang::Class /**/) -> java2rust::type_description::TypeDescription {
		self.arrayCount = array_count;
		self.clazz = clazz;
	}

	pub fn get_array_count(&self) -> i32 {
		return self.array_count;
	}

	pub fn get_clazz(&self) -> /* Java */ java::lang::Class /**/ {
		return self.clazz;
	}
}