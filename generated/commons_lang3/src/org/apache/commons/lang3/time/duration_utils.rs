use java::time::Duration;
use java::time::Instant;
use java::time::temporal::ChronoUnit;
use java::time::temporal::Temporal;
use java::time::temporal::TemporalUnit;
use java::util::Objects;
use java::util::concurrent::TimeUnit;
use crate::org::apache::commons::lang3::LongRange;
use crate::org::apache::commons::lang3::ObjectUtils;
use crate::org::apache::commons::lang3::StringUtils;
use crate::org::apache::commons::lang3::function::FailableBiConsumer;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableRunnable;
use crate::org::apache::commons::lang3::math::NumberUtils;

pub struct DurationUtils;

impl DurationUtils {
	static LONG_TO_INT_RANGE: org::apache::commons::lang3::long_range::LongRange = LongRange::of(NumberUtils::LONG_INT_MIN_VALUE, NumberUtils::LONG_INT_MAX_VALUE);

	pub fn accept<T: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer, duration: &/* Java */ java::time::Duration /**/) /* thrown(T | E) */ {
		if consumer != null && duration != null {
			consumer.accept(&duration.toMillis(), &org::apache::commons::lang3::time::duration_utils::DurationUtils::get_nanos_of_milli(duration))?;
		}
	}

	pub fn get(&self, key: &/* Java */ java::lang::String /**/, unit: &/* Java */ java::time::temporal::TemporalUnit /**/, def: i64) -> /* Java */ java::time::Duration /**/ {
		return Duration::of(&org::apache::commons::lang3::time::duration_utils::DurationUtils::get_long(key, def), unit);
	}

	fn get_long(&self, key: &/* Java */ java::lang::String /**/, def: i64) -> i64 {
		return  if StringUtils::is_empty(key) { def } else { Long::getLong(key, def) };
	}

	pub fn get_millis(&self, key: &/* Java */ java::lang::String /**/, def: i64) -> /* Java */ java::time::Duration /**/ {
		return Duration::ofMillis(&org::apache::commons::lang3::time::duration_utils::DurationUtils::get_long(key, def));
	}

	pub fn get_nanos_of_miili(&self, duration: &/* Java */ java::time::Duration /**/) -> i32 {
		return org::apache::commons::lang3::time::duration_utils::DurationUtils::get_nanos_of_milli(duration);
	}

	pub fn get_nanos_of_milli(&self, duration: &/* Java */ java::time::Duration /**/) -> i32 {
		return org::apache::commons::lang3::time::duration_utils::DurationUtils::zero_if_null(duration).getNano() % 1_000_000;
	}

	pub fn get_seconds(&self, key: &/* Java */ java::lang::String /**/, def: i64) -> /* Java */ java::time::Duration /**/ {
		return Duration::ofSeconds(&org::apache::commons::lang3::time::duration_utils::DurationUtils::get_long(key, def));
	}

	pub fn is_positive(&self, duration: &/* Java */ java::time::Duration /**/) -> bool {
		return !duration.isNegative() && !duration.isZero();
	}

	fn now<E: /* Java */ java::lang::Throwable /**/>(&self, now_consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) /* thrown(E | E) */ -> /* Java */ java::time::Instant /**/ {
		/* final */ let start: Instant = Instant::now();
		now_consumer.accept(start)?;
		return start;
	}

	pub fn of<E: /* Java */ java::lang::Throwable /**/>(&self, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) /* thrown(E | E) */ -> /* Java */ java::time::Duration /**/ {
		return org::apache::commons::lang3::time::duration_utils::DurationUtils::since(&org::apache::commons::lang3::time::duration_utils::DurationUtils::now(consumer::accept)?);
	}

	pub fn of<E: /* Java */ java::lang::Throwable /**/>(&self, runnable: &org::apache::commons::lang3::function::failable_runnable::FailableRunnable) /* thrown(E | E) */ -> /* Java */ java::time::Duration /**/ {
		return org::apache::commons::lang3::time::duration_utils::DurationUtils::of(|start|runnable.run()?)?;
	}

	pub fn since(&self, start_inclusive: &/* Java */ java::time::temporal::Temporal /**/) -> /* Java */ java::time::Duration /**/ {
		return Duration::between(start_inclusive, &Instant::now());
	}

	fn to_chrono_unit(&self, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::time::temporal::ChronoUnit /**/ {
		// TODO when using Java >= 9: Use TimeUnit.toChronoUnit().
		match Objects::requireNonNull(time_unit) {
			NANOSECONDS =>  {
				return ChronoUnit::NANOS;
			}
			MICROSECONDS =>  {
				return ChronoUnit::MICROS;
			}
			MILLISECONDS =>  {
				return ChronoUnit::MILLIS;
			}
			SECONDS =>  {
				return ChronoUnit::SECONDS;
			}
			MINUTES =>  {
				return ChronoUnit::MINUTES;
			}
			HOURS =>  {
				return ChronoUnit::HOURS;
			}
			DAYS =>  {
				return ChronoUnit::DAYS;
			}
			_ =>  {
				return Err(IllegalArgumentException::new(&time_unit.toString()));
			}
		}
	}

	pub fn to_duration(&self, amount: i64, time_unit: &/* Java */ java::util::concurrent::TimeUnit /**/) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::time::Duration /**/ {
		return Duration::of(amount, &org::apache::commons::lang3::time::duration_utils::DurationUtils::to_chrono_unit(time_unit)?);
	}

	pub fn to_millis_int(&self, duration: &/* Java */ java::time::Duration /**/) -> i32 {
		Objects::requireNonNull(duration, "duration");
		// intValue() does not do a narrowing conversion here
		return self.LONG_TO_INT_RANGE.fit(&Long::valueOf(&duration.toMillis())).intValue();
	}

	pub fn zero_if_null(&self, duration: &/* Java */ java::time::Duration /**/) -> /* Java */ java::time::Duration /**/ {
		return ObjectUtils::get_if_null(duration, Duration::ZERO);
	}

	pub fn new() -> org::apache::commons::lang3::time::duration_utils::DurationUtils {
	// empty
	}
}