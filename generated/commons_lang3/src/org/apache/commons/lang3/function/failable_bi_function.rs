use java::util::Objects;
use java::util::function::BiFunction;
use java::util::function::Function;

pub trait FailableBiFunction<T, U, R, E: /* Java */ java::lang::Throwable /**/>;