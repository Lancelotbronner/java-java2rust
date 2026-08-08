use java::util::Objects;
use java::util::function::BiConsumer;

pub trait FailableBiConsumer<T, U, E: /* Java */ java::lang::Throwable /**/>;