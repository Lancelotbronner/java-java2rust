pub struct DataKey<T>;

impl<T> DataKey {
	pub fn hash_code(&self) -> i32 {
		return self.getClass().hashCode();
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		return obj != null && self.getClass().equals(&obj.getClass());
	}
}