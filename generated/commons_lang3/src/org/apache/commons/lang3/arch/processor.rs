pub struct Processor {
	arch: org::apache::commons::lang3::arch::processor::Arch,
	type: org::apache::commons::lang3::arch::processor::Type,
}

impl Processor {
	pub fn new(arch: &org::apache::commons::lang3::arch::processor::Arch, type: &org::apache::commons::lang3::arch::processor::Type) -> org::apache::commons::lang3::arch::processor::Processor {
		self.arch = arch;
		self.type = type;
	}

	pub fn get_arch(&self) -> org::apache::commons::lang3::arch::processor::Arch {
		return self.arch;
	}

	pub fn get_type(&self) -> org::apache::commons::lang3::arch::processor::Type {
		return self.type;
	}

	pub fn is32_bit(&self) -> bool {
		return Arch::BIT_32 == self.arch;
	}

	pub fn is64_bit(&self) -> bool {
		return Arch::BIT_64 == self.arch;
	}

	pub fn is_aarch64(&self) -> bool {
		return Type::AARCH_64 == self.type;
	}

	pub fn isi_a64(&self) -> bool {
		return Type::IA_64 == self.type;
	}

	pub fn isppc(&self) -> bool {
		return Type::PPC == self.type;
	}

	pub fn isriscv(&self) -> bool {
		return Type::RISC_V == self.type;
	}

	pub fn is_x86(&self) -> bool {
		return Type::X86 == self.type;
	}

	pub fn to_string(&self) -> /* Java */ java::lang::String /**/ {
		/* final */ let builder: StringBuilder = StringBuilder::new();
		builder.append(&self.type.get_label()).append(' ').append(&self.arch.get_label());
		return builder.toString();
	}
}

pub enum Arch {
	label: /* Java */ java::lang::String /**/,
}

pub enum Type {
	label: /* Java */ java::lang::String /**/,
}