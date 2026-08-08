use java::time::Duration;
use java::util::Collection;
use java::util::Collections;
use java::util::Objects;
use java::util::function::Function;
use java::util::function::Predicate;
use java::util::stream::Collectors;
use java::util::stream::Stream;
use crate::org::apache::commons::lang3::function::Predicates;
use crate::org::apache::commons::lang3::time::DurationUtils;

pub struct ThreadUtils;

impl ThreadUtils {
	pub static ALWAYS_TRUE_PREDICATE: org::apache::commons::lang3::thread_utils::AlwaysTruePredicate = AlwaysTruePredicate::new();

	pub fn find_thread_by_id(&self, thread_id: i64) /* thrown(java.lang.IllegalArgumentException) */ -> /* Java */ java::lang::Thread /**/ {
		if thread_id <= 0 {
			return Err(IllegalArgumentException::new("The thread id must be greater than zero"));
		}
		/* final */ let result: Collection<Thread> = org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(|t|t != null && t.getId() == thread_id as Predicate<Thread>);
		return  if result.isEmpty() { null } else { result.iterator().next() };
	}

	pub fn find_thread_by_id(&self, thread_id: i64, thread_group_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::lang::Thread /**/ {
		Objects::requireNonNull(thread_group_name, "threadGroupName");
		/* final */ let thread: Thread = org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_by_id(thread_id)?;
		if thread != null && thread.getThreadGroup() != null && thread.getThreadGroup().getName().equals(thread_group_name) {
			return thread;
		}
		return null;
	}

	pub fn find_thread_by_id(&self, thread_id: i64, thread_group: &/* Java */ java::lang::ThreadGroup /**/) -> /* Java */ java::lang::Thread /**/ {
		Objects::requireNonNull(thread_group, "threadGroup");
		/* final */ let thread: Thread = org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_by_id(thread_id)?;
		if thread != null && thread_group.equals(&thread.getThreadGroup()) {
			return thread;
		}
		return null;
	}

	pub fn find_thread_groups(&self, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(&org::apache::commons::lang3::thread_utils::ThreadUtils::get_system_thread_group(), true, predicate);
	}

	pub fn find_thread_groups(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/, recurse: bool, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::Collection /**/ {
		Objects::requireNonNull(thread_group, "threadGroup");
		Objects::requireNonNull(predicate, "predicate");
		let count: i32 = thread_group.activeGroupCount();
		let thread_groups: Vec<ThreadGroup>;
		loop { {
			//slightly grow the array size
			thread_groups = : [Option<ThreadGroup>; count + count / 2 + 1] = [None; count + count / 2 + 1];
			count = thread_group.enumerate(thread_groups, recurse);
		//return value of enumerate() must be strictly less than the array size according to Javadoc
		}if !(count >= thread_groups.length) break;}
		return Collections::unmodifiableCollection(&Stream::of(thread_groups).limit(count).filter(predicate).collect(&Collectors::toList()));
	}

	pub fn find_thread_groups(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/, recurse: bool, predicate: &org::apache::commons::lang3::thread_utils::ThreadGroupPredicate) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(thread_group, recurse, predicate::test as Predicate<ThreadGroup>);
	}

	pub fn find_thread_groups(&self, predicate: &org::apache::commons::lang3::thread_utils::ThreadGroupPredicate) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(&org::apache::commons::lang3::thread_utils::ThreadUtils::get_system_thread_group(), true, predicate);
	}

	pub fn find_thread_groups_by_name(&self, thread_group_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(&org::apache::commons::lang3::thread_utils::ThreadUtils::predicate_thread_group(thread_group_name));
	}

	pub fn find_threads(&self, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(&org::apache::commons::lang3::thread_utils::ThreadUtils::get_system_thread_group(), true, predicate);
	}

	pub fn find_threads(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/, recurse: bool, predicate: &/* Java */ java::util::function::Predicate /**/) -> /* Java */ java::util::Collection /**/ {
		Objects::requireNonNull(thread_group, "The group must not be null");
		Objects::requireNonNull(predicate, "The predicate must not be null");
		let count: i32 = thread_group.activeCount();
		let threads: Vec<Thread>;
		loop { {
			//slightly grow the array size
			threads = : [Option<Thread>; count + count / 2 + 1] = [None; count + count / 2 + 1];
			count = thread_group.enumerate(threads, recurse);
		//return value of enumerate() must be strictly less than the array size according to javadoc
		}if !(count >= threads.length) break;}
		return Collections::unmodifiableCollection(&Stream::of(threads).limit(count).filter(predicate).collect(&Collectors::toList()));
	}

	pub fn find_threads(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/, recurse: bool, predicate: &org::apache::commons::lang3::thread_utils::ThreadPredicate) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(thread_group, recurse, predicate::test as Predicate<Thread>);
	}

	pub fn find_threads(&self, predicate: &org::apache::commons::lang3::thread_utils::ThreadPredicate) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(&org::apache::commons::lang3::thread_utils::ThreadUtils::get_system_thread_group(), true, predicate);
	}

	pub fn find_threads_by_name(&self, thread_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(&org::apache::commons::lang3::thread_utils::ThreadUtils::predicate_thread(thread_name));
	}

	pub fn find_threads_by_name(&self, thread_name: &/* Java */ java::lang::String /**/, thread_group_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::Collection /**/ {
		Objects::requireNonNull(thread_name, "threadName");
		Objects::requireNonNull(thread_group_name, "threadGroupName");
		return Collections::unmodifiableCollection(&org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(&org::apache::commons::lang3::thread_utils::ThreadUtils::predicate_thread_group(thread_group_name)).stream().flatMap(|group|org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(group, false, &org::apache::commons::lang3::thread_utils::ThreadUtils::predicate_thread(thread_name)).stream()).collect(&Collectors::toList()));
	}

	pub fn find_threads_by_name(&self, thread_name: &/* Java */ java::lang::String /**/, thread_group: &/* Java */ java::lang::ThreadGroup /**/) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(thread_group, false, &org::apache::commons::lang3::thread_utils::ThreadUtils::predicate_thread(thread_name));
	}

	pub fn get_all_thread_groups(&self) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_thread_groups(&Predicates::true_predicate());
	}

	pub fn get_all_threads(&self) -> /* Java */ java::util::Collection /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::find_threads(&Predicates::true_predicate());
	}

	pub fn get_system_thread_group(&self) -> /* Java */ java::lang::ThreadGroup /**/ {
		let thread_group: ThreadGroup = Thread::currentThread().getThreadGroup();
		while thread_group != null && thread_group.getParent() != null {
			thread_group = thread_group.getParent();
		}
		return thread_group;
	}

	pub fn join(&self, thread: &/* Java */ java::lang::Thread /**/, duration: &/* Java */ java::time::Duration /**/) /* thrown(java.lang.InterruptedException) */ {
		DurationUtils::accept(thread::join, duration);
	}

	fn name_predicate<T>(&self, name: &/* Java */ java::lang::String /**/, name_getter: &/* Java */ java::util::function::Function /**/) -> /* Java */ java::util::function::Predicate /**/ {
		return |t|t != null && Objects::equals(&name_getter.apply(t), &Objects::requireNonNull(name)) as Predicate<T>;
	}

	fn predicate_thread(&self, thread_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::function::Predicate /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::name_predicate(thread_name, Thread::getName);
	}

	fn predicate_thread_group(&self, thread_group_name: &/* Java */ java::lang::String /**/) -> /* Java */ java::util::function::Predicate /**/ {
		return org::apache::commons::lang3::thread_utils::ThreadUtils::name_predicate(thread_group_name, ThreadGroup::getName);
	}

	pub fn sleep(&self, duration: &/* Java */ java::time::Duration /**/) /* thrown(java.lang.InterruptedException) */ {
		DurationUtils::accept(Thread::sleep, duration);
	}

	pub fn sleep_quietly(&self, duration: &/* Java */ java::time::Duration /**/) /* thrown(java.lang.InterruptedException) */ {
		let r0 = 'try0: {
			if let Err(e) = org::apache::commons::lang3::thread_utils::ThreadUtils::sleep(duration) {
				return Err(e);
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ InterruptedException) => {
			// Ignore & be quiet.
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	pub fn new() -> org::apache::commons::lang3::thread_utils::ThreadUtils {
	// empty
	}
}

struct AlwaysTruePredicate;

impl AlwaysTruePredicate {
	fn new() -> org::apache::commons::lang3::thread_utils::AlwaysTruePredicate {
	}

	pub fn test(&self, thread: &/* Java */ java::lang::Thread /**/) -> bool {
		return true;
	}

	pub fn test(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/) -> bool {
		return true;
	}
}

impl org::apache::commons::lang3::thread_utils::ThreadPredicate for AlwaysTruePredicate {}

impl org::apache::commons::lang3::thread_utils::ThreadGroupPredicate for AlwaysTruePredicate {}

pub struct NamePredicate {
	name: /* Java */ java::lang::String /**/,
}

impl NamePredicate {
	pub fn new(name: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::thread_utils::NamePredicate {
		Objects::requireNonNull(name, "name");
		self.name = name;
	}

	pub fn test(&self, thread: &/* Java */ java::lang::Thread /**/) -> bool {
		return thread != null && thread.getName().equals(self.name);
	}

	pub fn test(&self, thread_group: &/* Java */ java::lang::ThreadGroup /**/) -> bool {
		return thread_group != null && thread_group.getName().equals(self.name);
	}
}

impl org::apache::commons::lang3::thread_utils::ThreadPredicate for NamePredicate {}

impl org::apache::commons::lang3::thread_utils::ThreadGroupPredicate for NamePredicate {}

pub trait ThreadGroupPredicate;

pub struct ThreadIdPredicate {
	thread_id: i64,
}

impl ThreadIdPredicate {
	pub fn new(thread_id: i64) /* thrown(java.lang.IllegalArgumentException) */ -> org::apache::commons::lang3::thread_utils::ThreadIdPredicate {
		if thread_id <= 0 {
			return Err(IllegalArgumentException::new("The thread id must be greater than zero"));
		}
		self.threadId = thread_id;
	}

	pub fn test(&self, thread: &/* Java */ java::lang::Thread /**/) -> bool {
		return thread != null && thread.getId() == self.thread_id;
	}
}

impl org::apache::commons::lang3::thread_utils::ThreadPredicate for ThreadIdPredicate {}

pub trait ThreadPredicate;