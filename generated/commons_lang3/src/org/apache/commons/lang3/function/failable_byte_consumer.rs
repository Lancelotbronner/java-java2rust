use java::util::Objects;
use java::util::function::IntConsumer;

pub trait FailableByteConsumer<E: /* Java */ java::lang::Throwable /**/>;