use java::util::function::Supplier;

pub struct Suppliers;

impl Suppliers {
	static NUL: /* Java */ java::util::function::Supplier /**/ = |()|null;

	pub fn get<T>(&self, supplier: &/* Java */ java::util::function::Supplier /**/) -> T {
		return  if supplier == null { null } else { supplier.get() };
	}

	pub fn nul<T>(&self) -> /* Java */ java::util::function::Supplier /**/ {
		return self.NUL;
	}

	pub fn new() -> org::apache::commons::lang3::function::suppliers::Suppliers {
	// empty
	}
}