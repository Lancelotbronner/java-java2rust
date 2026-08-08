use java::util::Collection;
use java::util::concurrent::ExecutionException;
use java::util::concurrent::Future;
use java::util::concurrent::TimeUnit;
use java::util::concurrent::TimeoutException;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::exception::UncheckedInterruptedException;

pub trait UncheckedFuture<V>;