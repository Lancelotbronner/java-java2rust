use java::io::Serializable;
use java::util::ArrayList;
use java::util::Arrays;
use java::util::List;
use java::util::Objects;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::ArrayUtils;
use crate::org::apache::commons::lang3::ObjectUtils;

pub struct DiffBuilder<T> {
	diffs: /* Java */ java::util::List /**/,
	equals: bool,
	left: T,
	right: T,
	style: org::apache::commons::lang3::builder::to_string_style::ToStringStyle,
	to_string_format: /* Java */ java::lang::String /**/,
}

impl<T> DiffBuilder {
	static TO_STRING_FORMAT: /* Java */ java::lang::String /**/ = "%s differs from %s";

	pub fn builder<T>(&self) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		return Builder<>::new();
	}

	pub fn new(left: &T, right: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		this(left, right, style, true);
	}

	pub fn new(left: &T, right: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, test_objects_equals: bool) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		this(left, right, style, test_objects_equals, self.TO_STRING_FORMAT);
	}

	fn new(left: &T, right: &T, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle, test_objects_equals: bool, to_string_format: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		self.left = Objects::requireNonNull(left, "left");
		self.right = Objects::requireNonNull(right, "right");
		self.diffs = ArrayList<>::new();
		self.toStringFormat = to_string_format;
		self.style =  if style != null { style } else { ToStringStyle::org::apache::commons::lang3::builder::to_string_style::ToStringStyle::DEFAULT_STYLE };
		// Don't compare any fields if objects equal
		self.equals = test_objects_equals && Objects::equals(left, right);
	}

	fn add<F>(&self, field_name: &/* Java */ java::lang::String /**/, left: &org::apache::commons::lang3::builder::diff_builder::SerializableSupplier, right: &org::apache::commons::lang3::builder::diff_builder::SerializableSupplier, type: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		self.diffs.add(SDiff<>::new(field_name, left, right, type));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: bool, rhs: bool) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Boolean::valueOf(lhs), |()|Boolean::valueOf(rhs), Boolean.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[bool], rhs: &&[bool]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils::to_object(rhs), Vec<Boolean>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: i8, rhs: i8) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Byte::valueOf(lhs), |()|Byte::valueOf(rhs), Byte.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[i8], rhs: &&[i8]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils.toObject(lhs), |()|ArrayUtils.toObject(rhs), Vec<Byte>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: u16, rhs: u16) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Character::valueOf(lhs), |()|Character::valueOf(rhs), Character.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[u16], rhs: &&[u16]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils.toObject(rhs), Vec<Character>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, diff_result: &org::apache::commons::lang3::builder::diff_result::DiffResult) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		Objects::requireNonNull(diff_result, "diffResult");
		if self.equals {
			return self;
		}
		diff_result.get_diffs().forEach(|diff|self.append(field_name + "." + diff.get_field_name(), &diff.get_left(), &diff.get_right()));
		return self;
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: f64, rhs: f64) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Double::doubleToLongBits(lhs) == Double::doubleToLongBits(rhs) { self } else { self.add(field_name, |()|Double::valueOf(lhs), |()|Double::valueOf(rhs), Double.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[f64], rhs: &&[f64]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils::to_object(rhs), Vec<Double>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: f32, rhs: f32) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Float::floatToIntBits(lhs) == Float::floatToIntBits(rhs) { self } else { self.add(field_name, |()|Float::valueOf(lhs), |()|Float::valueOf(rhs), Float.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[f32], rhs: &&[f32]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays.equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils::to_object(rhs), Vec<Float>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: i32, rhs: i32) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Integer::valueOf(lhs), |()|Integer::valueOf(rhs), Integer.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[i32], rhs: &&[i32]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays.equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils.toObject(rhs), Vec<Integer>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: i64, rhs: i64) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Long::valueOf(lhs), |()|Long::valueOf(rhs), Long.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[i64], rhs: &&[i64]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils::to_object(lhs), |()|ArrayUtils.toObject(rhs), Vec<Long>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &/* Java */ java::lang::Object /**/, rhs: &/* Java */ java::lang::Object /**/) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		if self.equals || lhs == rhs {
			return self;
		}
		// rhs cannot be null, as lhs != rhs
		/* final */ let test: Object =  if lhs != null { lhs } else { rhs };
		if ObjectUtils::is_array(test) {
			if test instanceof Vec<bool> {
				return self.append(field_name, lhs as Vec<bool>, rhs as Vec<bool>);
			}
			if test instanceof Vec<i8> {
				return .append(field_name, lhs as Vec<i8>, rhs as Vec<i8>);
			}
			if test instanceof Vec<char> {
				return self.append(field_name, lhs as Vec<char>, rhs as Vec<char>);
			}
			if test instanceof Vec<f64> {
				return self.append(field_name, lhs as Vec<f64>, rhs as Vec<f64>);
			}
			if test instanceof Vec<f32> {
				return self.append(field_name, lhs as Vec<f32>, rhs as Vec<f32>);
			}
			if test instanceof Vec<i32> {
				return self.append(field_name, lhs as Vec<i32>, rhs as Vec<i32>);
			}
			if test instanceof Vec<i64> {
				return self.append(field_name, lhs as Vec<i64>, rhs as Vec<i64>);
			}
			if test instanceof Vec<i16> {
				return .append(field_name, lhs as Vec<i16>, rhs as Vec<i16>);
			}
			return self.append(field_name, lhs as Vec<Object>, rhs as Vec<Object>);
		}
		// Not array type
		return  if Objects::equals(lhs, rhs) { self } else { self.add(field_name, |()|lhs, |()|rhs, Object.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[/* Java */ java::lang::Object /**/], rhs: &&[/* Java */ java::lang::Object /**/]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|lhs, |()|rhs, Vec<Object>.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: i16, rhs: i16) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || lhs == rhs { self } else { self.add(field_name, |()|Short::valueOf(lhs), |()|Short::valueOf(rhs), Short.class) };
	}

	pub fn append(&self, field_name: &/* Java */ java::lang::String /**/, lhs: &&[i16], rhs: &&[i16]) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return  if self.equals || Arrays::equals(lhs, rhs) { self } else { self.add(field_name, |()|ArrayUtils.toObject(lhs), |()|ArrayUtils::to_object(rhs), Vec<Short>.class) };
	}

	pub fn build(&self) -> org::apache::commons::lang3::builder::diff_result::DiffResult {
		return DiffResult<>::new(self.left, self.right, self.diffs, self.style, self.to_string_format);
	}

	fn get_left(&self) -> T {
		return self.left;
	}

	fn get_right(&self) -> T {
		return self.right;
	}
}

impl<T> org::apache::commons::lang3::builder::builder::Builder for DiffBuilder<T> {}

pub struct Builder<T> {
	left: T,
	right: T,
	style: org::apache::commons::lang3::builder::to_string_style::ToStringStyle,
	test_objects_equals: bool = true,
	to_string_format: /* Java */ java::lang::String /**/ = TO_STRING_FORMAT,
}

impl<T> Builder {
	pub fn new() -> org::apache::commons::lang3::builder::diff_builder::Builder {
	// empty
	}

	pub fn build(&self) -> org::apache::commons::lang3::builder::diff_builder::DiffBuilder {
		return DiffBuilder<>::new(self.left, self.right, self.style, self.test_objects_equals, self.to_string_format);
	}

	pub fn set_left(&mut self, left: &T) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		self.left = left;
		return self;
	}

	pub fn set_right(&mut self, right: &T) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		self.right = right;
		return self;
	}

	pub fn set_style(&mut self, style: &org::apache::commons::lang3::builder::to_string_style::ToStringStyle) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		self.style =  if style != null { style } else { ToStringStyle::org::apache::commons::lang3::builder::to_string_style::ToStringStyle::DEFAULT_STYLE };
		return self;
	}

	pub fn set_test_objects_equals(&mut self, test_objects_equals: bool) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		self.testObjectsEquals = test_objects_equals;
		return self;
	}

	pub fn set_to_string_format(&mut self, to_string_format: &/* Java */ java::lang::String /**/) -> org::apache::commons::lang3::builder::diff_builder::Builder {
		self.toStringFormat =  if to_string_format != null { to_string_format } else {  };
		return self;
	}
}

struct SDiff<T> {
	left_supplier: org::apache::commons::lang3::builder::diff_builder::SerializableSupplier,
	right_supplier: org::apache::commons::lang3::builder::diff_builder::SerializableSupplier,
}

impl<T> SDiff {
	static serialVersionUID: i64 = 1;

	fn new(field_name: &/* Java */ java::lang::String /**/, left_supplier: &org::apache::commons::lang3::builder::diff_builder::SerializableSupplier, right_supplier: &org::apache::commons::lang3::builder::diff_builder::SerializableSupplier, type: &/* Java */ java::lang::Class /**/) -> org::apache::commons::lang3::builder::diff_builder::SDiff {
		super(field_name, type);
		self.leftSupplier = Objects::requireNonNull(left_supplier);
		self.rightSupplier = Objects::requireNonNull(right_supplier);
	}

	pub fn get_left(&self) -> T {
		return self.left_supplier.get();
	}

	pub fn get_right(&self) -> T {
		return self.right_supplier.get();
	}
}

impl<T> /* Java */ java::util::Map::Entry /**/ for SDiff<T> {}

impl<T> /* Java */ java::lang::Comparable /**/ for SDiff<T> {}

impl<T> /* Java */ java::io::Serializable /**/ for SDiff<T> {}

trait SerializableSupplier<T>;