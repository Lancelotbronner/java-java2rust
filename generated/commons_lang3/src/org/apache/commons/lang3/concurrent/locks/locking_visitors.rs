use java::util::Objects;
use java::util::concurrent::locks::Lock;
use java::util::concurrent::locks::ReadWriteLock;
use java::util::concurrent::locks::ReentrantLock;
use java::util::concurrent::locks::ReentrantReadWriteLock;
use java::util::concurrent::locks::StampedLock;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::builder::AbstractSupplier;
use crate::org::apache::commons::lang3::function::Failable;
use crate::org::apache::commons::lang3::function::FailableConsumer;
use crate::org::apache::commons::lang3::function::FailableFunction;
use crate::org::apache::commons::lang3::function::Suppliers;

pub struct LockingVisitors;

impl LockingVisitors {
	pub fn create<O>(&self, object: &O, read_write_lock: &/* Java */ java::util::concurrent::locks::ReadWriteLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReadWriteLockVisitor {
		return LockingVisitors.ReadWriteLockVisitor<>::new(object, read_write_lock);
	}

	pub fn create<O>(&self, object: &O, reentrant_lock: &/* Java */ java::util::concurrent::locks::ReentrantLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReentrantLockVisitor {
		return LockingVisitors.ReentrantLockVisitor<>::new(object, reentrant_lock);
	}

	pub fn reentrant_lock_visitor<O>(&self, object: &O) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReentrantLockVisitor {
		return org::apache::commons::lang3::concurrent::locks::locking_visitors::LockingVisitors::create(object, ReentrantLock::new());
	}

	pub fn reentrant_read_write_lock_visitor<O>(&self, object: &O) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReadWriteLockVisitor {
		return org::apache::commons::lang3::concurrent::locks::locking_visitors::LockingVisitors::create(object, ReentrantReadWriteLock::new());
	}

	pub fn stamped_lock_visitor<O>(&self, object: &O) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::StampedLockVisitor {
		return LockingVisitors.StampedLockVisitor<>::new(object, StampedLock::new());
	}

	pub fn new() -> org::apache::commons::lang3::concurrent::locks::locking_visitors::LockingVisitors {
	// empty
	}
}

pub struct LockVisitor<O, L> {
	lock: L,
	object: O,
	read_lock_supplier: /* Java */ java::util::function::Supplier /**/,
	write_lock_supplier: /* Java */ java::util::function::Supplier /**/,
}

impl<O, L> LockVisitor {
	fn new(builder: &org::apache::commons::lang3::concurrent::locks::locking_visitors::LVBuilder) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::LockVisitor {
		self.object = Objects::requireNonNull(builder.object, "object");
		self.lock = Objects::requireNonNull(builder.lock, "lock");
		self.readLockSupplier = Objects::requireNonNull(builder.readLockSupplier, "readLockSupplier");
		self.writeLockSupplier = Objects::requireNonNull(builder.writeLockSupplier, "writeLockSupplier");
	}

	fn new(object: &O, lock: &L, read_lock_supplier: &/* Java */ java::util::function::Supplier /**/, write_lock_supplier: &/* Java */ java::util::function::Supplier /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::LockVisitor {
		self.object = Objects::requireNonNull(object, "object");
		self.lock = Objects::requireNonNull(lock, "lock");
		self.readLockSupplier = Objects::requireNonNull(read_lock_supplier, "readLockSupplier");
		self.writeLockSupplier = Objects::requireNonNull(write_lock_supplier, "writeLockSupplier");
	}

	pub fn accept_read_locked(&self, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) {
		self.lock_accept_unlock(self.read_lock_supplier, consumer);
	}

	pub fn accept_write_locked(&self, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) {
		self.lock_accept_unlock(self.write_lock_supplier, consumer);
	}

	pub fn apply_read_locked<T>(&self, function: &org::apache::commons::lang3::function::failable_function::FailableFunction) -> T {
		return self.lock_apply_unlock(self.read_lock_supplier, function);
	}

	pub fn apply_write_locked<T>(&self, function: &org::apache::commons::lang3::function::failable_function::FailableFunction) -> T {
		return self.lock_apply_unlock(self.write_lock_supplier, function);
	}

	pub fn get_lock(&self) -> L {
		return self.lock;
	}

	pub fn get_object(&self) -> O {
		return self.object;
	}

	fn lock_accept_unlock(&self, lock_supplier: &/* Java */ java::util::function::Supplier /**/, consumer: &org::apache::commons::lang3::function::failable_consumer::FailableConsumer) {
		/* final */ let lock: Lock = Objects::requireNonNull(&Suppliers::get(lock_supplier), "lock");
		lock.lock();
		let r0 = 'try0: {
			Failable::accept(consumer, self.object);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		lock.unlock();
	
	}

	fn lock_apply_unlock<T>(&self, lock_supplier: &/* Java */ java::util::function::Supplier /**/, function: &org::apache::commons::lang3::function::failable_function::FailableFunction) -> T {
		/* final */ let lock: Lock = Objects::requireNonNull(&Suppliers::get(lock_supplier), "lock");
		lock.lock();
		let r0 = 'try0: {
			return Failable::apply(function, self.object);
			break 'try0 Ok(());
		};
		match r0 {
			Err(e) => Err(e)?,
			Ok => (),
		}
		lock.unlock();
	
	}
}

pub struct LVBuilder<O, L, B: org::apache::commons::lang3::concurrent::locks::locking_visitors::LVBuilder> {
	lock: L,
	object: O,
	read_lock_supplier: /* Java */ java::util::function::Supplier /**/,
	write_lock_supplier: /* Java */ java::util::function::Supplier /**/,
}

impl<O, L, B: org::apache::commons::lang3::concurrent::locks::locking_visitors::LVBuilder> LVBuilder {
	pub fn new() -> org::apache::commons::lang3::concurrent::locks::locking_visitors::LVBuilder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::LockVisitor {
		return LockVisitor<>::new(self);
	}

	fn get_read_lock_supplier(&self) -> /* Java */ java::util::function::Supplier /**/ {
		return self.read_lock_supplier;
	}

	fn get_write_lock_supplier(&self) -> /* Java */ java::util::function::Supplier /**/ {
		return self.write_lock_supplier;
	}

	pub fn set_lock(&mut self, lock: &L) -> B {
		self.lock = lock;
		return self.as_this();
	}

	pub fn set_object(&mut self, object: &O) -> B {
		self.object = object;
		return self.as_this();
	}

	pub fn set_read_lock_supplier(&mut self, read_lock_supplier: &/* Java */ java::util::function::Supplier /**/) -> B {
		self.readLockSupplier = read_lock_supplier;
		return self.as_this();
	}

	pub fn set_write_lock_supplier(&mut self, write_lock_supplier: &/* Java */ java::util::function::Supplier /**/) -> B {
		self.writeLockSupplier = write_lock_supplier;
		return self.as_this();
	}
}

impl<O, L, B: org::apache::commons::lang3::concurrent::locks::locking_visitors::LVBuilder> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for LVBuilder<O, L, B> {}

pub struct ReadWriteLockVisitor<O>;

impl<O> ReadWriteLockVisitor {
	pub fn builder<O>(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		return Builder<>::new();
	}

	fn new(builder: &org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReadWriteLockVisitor {
		super(builder);
	}

	fn new(object: &O, read_write_lock: &/* Java */ java::util::concurrent::locks::ReadWriteLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReadWriteLockVisitor {
		super(object, read_write_lock, readWriteLock::readLock, readWriteLock::writeLock);
	}
}

pub struct Builder<O>;

impl<O> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReadWriteLockVisitor {
		return ReadWriteLockVisitor<>::new(self);
	}

	pub fn set_lock(&self, read_write_lock: &/* Java */ java::util::concurrent::locks::ReadWriteLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		self.set_read_lock_supplier(readWriteLock::readLock);
		self.set_write_lock_supplier(readWriteLock::writeLock);
		return super.set_lock(read_write_lock);
	}
}

impl<O> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<O> {}

pub struct ReentrantLockVisitor<O>;

impl<O> ReentrantLockVisitor {
	pub fn builder<O>(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		return Builder<>::new();
	}

	fn new(builder: &org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReentrantLockVisitor {
		super(builder);
	}

	fn new(object: &O, reentrant_lock: &/* Java */ java::util::concurrent::locks::ReentrantLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReentrantLockVisitor {
		super(object, reentrant_lock, |()|reentrant_lock, |()|reentrant_lock);
	}
}

pub struct Builder<O>;

impl<O> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::ReentrantLockVisitor {
		return ReentrantLockVisitor<>::new(self);
	}

	pub fn set_lock(&self, reentrant_lock: &/* Java */ java::util::concurrent::locks::ReentrantLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		self.set_read_lock_supplier(|()|reentrant_lock);
		self.set_write_lock_supplier(|()|reentrant_lock);
		return super.set_lock(reentrant_lock);
	}
}

impl<O> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<O> {}

pub struct StampedLockVisitor<O>;

impl<O> StampedLockVisitor {
	pub fn builder<O>(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		return Builder<>::new();
	}

	fn new(builder: &org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::StampedLockVisitor {
		super(builder);
	}

	fn new(object: &O, stamped_lock: &/* Java */ java::util::concurrent::locks::StampedLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::StampedLockVisitor {
		super(object, stamped_lock, stampedLock::asReadLock, stampedLock::asWriteLock);
	}
}

pub struct Builder<O>;

impl<O> Builder {
	pub fn new() -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
	// empty
	}

	pub fn get(&self) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::StampedLockVisitor {
		return StampedLockVisitor<>::new(self);
	}

	pub fn set_lock(&self, stamped_lock: &/* Java */ java::util::concurrent::locks::StampedLock /**/) -> org::apache::commons::lang3::concurrent::locks::locking_visitors::Builder {
		self.set_read_lock_supplier(stampedLock::asReadLock);
		self.set_write_lock_supplier(stampedLock::asWriteLock);
		return super.set_lock(stamped_lock);
	}
}

impl<O> org::apache::commons::lang3::function::failable_supplier::FailableSupplier for Builder<O> {}