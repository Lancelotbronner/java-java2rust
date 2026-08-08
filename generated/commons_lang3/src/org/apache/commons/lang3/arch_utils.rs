use java::util::HashMap;
use java::util::Map;
use crate::org::apache::commons::lang3::arch::Processor;
use crate::org::apache::commons::lang3::stream::Streams;

pub struct ArchUtils;

impl ArchUtils {
	static ARCH_TO_PROCESSOR: /* Java */ java::util::Map /**/;

	init {
	    ARCH_TO_PROCESSOR = new HashMap<>();
	    init();
	}

	fn add_processor(&self, key: &/* Java */ java::lang::String /**/, processor: &org::apache::commons::lang3::arch::processor::Processor) /* thrown(java.lang.IllegalStateException) */ {
		if self.ARCH_TO_PROCESSOR.containsKey(key) {
			return Err(IllegalStateException::new("Key " + key + " already exists in processor map"));
		}
		self.ARCH_TO_PROCESSOR.put(key, processor);
	}

	fn add_processors(&self, processor: &org::apache::commons::lang3::arch::processor::Processor, keys: &/* Java */ java::lang::String /**/) {
		Streams::of(keys).forEach(|e|org::apache::commons::lang3::arch_utils::ArchUtils::add_processor(e, processor)?);
	}

	pub fn get_processor(&self) -> org::apache::commons::lang3::arch::processor::Processor {
		return org::apache::commons::lang3::arch_utils::ArchUtils::get_processor(&SystemProperties::get_os_arch());
	}

	pub fn get_processor(&self, value: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::arch::processor::Processor {
		return self.ARCH_TO_PROCESSOR.get(value);
	}

	fn init(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::init__x86_32_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init__x86_64_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_i_a64_32_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_i_a64_64_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_pp_c_32_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_pp_c_64_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init__aarch_64_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_risc_v_32_bit();
		org::apache::commons::lang3::arch_utils::ArchUtils::init_risc_v_64_bit();
	}

	fn init__aarch_64_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_64, Processor::org::apache::commons::lang3::arch::processor::Type::AARCH_64), "aarch64");
	}

	fn init_i_a64_32_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_32, Processor::org::apache::commons::lang3::arch::processor::Type::IA_64), "ia64_32", "ia64n");
	}

	fn init_i_a64_64_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_64, Processor::org::apache::commons::lang3::arch::processor::Type::IA_64), "ia64", "ia64w");
	}

	fn init_pp_c_32_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_32, Processor::org::apache::commons::lang3::arch::processor::Type::PPC), "ppc", "power", "powerpc", "power_pc", "power_rs");
	}

	fn init_pp_c_64_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_64, Processor::org::apache::commons::lang3::arch::processor::Type::PPC), "ppc64", "power64", "powerpc64", "power_pc64", "power_rs64");
	}

	fn init_risc_v_32_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_32, Processor::org::apache::commons::lang3::arch::processor::Type::RISC_V), "riscv32");
	}

	fn init_risc_v_64_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_64, Processor::org::apache::commons::lang3::arch::processor::Type::RISC_V), "riscv64");
	}

	fn init__x86_32_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_32, Processor::org::apache::commons::lang3::arch::processor::Type::X86), "x86", "i386", "i486", "i586", "i686", "pentium");
	}

	fn init__x86_64_bit(&self) {
		org::apache::commons::lang3::arch_utils::ArchUtils::add_processors(Processor::new(Processor::org::apache::commons::lang3::arch::processor::Arch::BIT_64, Processor::org::apache::commons::lang3::arch::processor::Type::X86), "x86_64", "amd64", "em64t", "universal");
	}

	pub fn new() -> org::apache::commons::lang3::arch_utils::ArchUtils {
	// empty
	}
}