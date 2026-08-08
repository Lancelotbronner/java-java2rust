use crate::org::apache::commons::lang3::function::FailableSupplier;

pub struct AbstractSupplier<T, B: org::apache::commons::lang3::builder::abstract_supplier::AbstractSupplier, E: /* Java */ java::lang::Throwable /**/>;

impl<T, B: org::apache::commons::lang3::builder::abstract_supplier::AbstractSupplier, E: /* Java */ java::lang::Throwable /**/> AbstractSupplier {
	pub fn new() -> org::apache::commons::lang3::builder::abstract_supplier::AbstractSupplier {
	// empty
	}

	fn as_this(&self) -> B {
		return self as B;
	}
}

impl<T, B: org::apache::commons::lang3::builder::abstract_supplier::AbstractSupplier, E: /* Java */ java::lang::Throwable /**/> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for AbstractSupplier<T, B, E> {}