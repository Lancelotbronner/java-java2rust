use java::util::Objects;
use java::util::function::BiPredicate;

pub trait FailableBiPredicate<T, U, E: /* Java */ java::lang::Throwable /**/>;