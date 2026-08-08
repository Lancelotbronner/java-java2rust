use java::util::Objects;
use java::util::function::Consumer;
use java::util::function::Function;

pub trait FailableConsumer<T, E: /* Java */ java::lang::Throwable /**/>;