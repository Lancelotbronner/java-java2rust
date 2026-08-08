use java::util::Objects;

pub struct ConstantInitializer<T> {
	object: T,
}

impl<T> ConstantInitializer {
	static FMT_TO_STRING: /* Java */ java::lang::String /**/ = "ConstantInitializer@%d [ object = %s ]";

	pub fn new(obj: &T) -> org::apache::commons::lang3::concurrent::constant_initializer::ConstantInitializer {
		self.object = obj;
	}

	pub fn equals(&self, obj: &/* Java */ java::lang::Object /**/) -> bool {
		if self == obj {
			return true;
		}
		if !(obj instanceof ConstantInitializer<?>) {
			return false;
		}
		/* final */ let c: ConstantInitializer<?> = obj as ConstantInitializer<?>;
		return Objects::equals(&self.get_object(), &c.get_object());
	}

	pub fn get(&self) /* thrown(org.apache.commons.lang3.concurrent.ConcurrentException) */ -> T {
		return self.get_object();
	}

	pub fn get_object(&self) -> T {
		return self.object;
	}

	pub fn hash_code(&self) -> i32 {
		return Objects::hashCode(self.object);
	}

	pub fn is_initialized(&self) -> bool {
		return true;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		return String::format(self.FMT_TO_STRING, &Integer::valueOf(&System::identityHashCode(self)), &self.get_object());
	}
}

impl<T> org::apache::commons::lang3::concurrent::concurrent_initializer::ConcurrentInitializer for ConstantInitializer<T> {}

impl<T> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for ConstantInitializer<T> {}