use java::util::function::Supplier;

pub trait FailableSupplier<T, E: /* Java */ java::lang::Throwable /**/>;