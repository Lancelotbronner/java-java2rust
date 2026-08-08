use java::util::Objects;
use java::util::function::Function;

pub trait FailableFunction<T, R, E: /* Java */ java::lang::Throwable /**/>;