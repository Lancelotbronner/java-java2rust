use java::util::Objects;
use java::util::function::Predicate;

pub trait FailablePredicate<T, E: /* Java */ java::lang::Throwable /**/>;