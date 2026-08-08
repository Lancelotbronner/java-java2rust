use java::util::function::Consumer;
use java::util::function::Function;

pub struct Consumers;

impl Consumers {
	static NOP: /* Java */ java::util::function::Consumer /**/ = Function::identity()::apply;

	pub fn accept<T>(&self, consumer: &/* Java */ java::util::function::Consumer /**/, object: &T) {
		if consumer != null {
			consumer.accept(object);
		}
	}

	pub fn nop<T>(&self) -> /* Java */ java::util::function::Consumer /**/ {
		return self.NOP;
	}

	fn new() -> org::apache::commons::lang3::function::consumers::Consumers {
	// No instances.
	}
}