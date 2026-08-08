use java::lang::invoke::MethodHandle;
use java::lang::invoke::MethodHandleProxies;
use java::lang::invoke::MethodHandles;
use java::lang::reflect::Method;
use java::util::Objects;
use java::util::function::BiConsumer;
use java::util::function::BiFunction;
use java::util::function::Function;
use java::util::function::Supplier;
use crate::org::apache::commons::lang3::exception::UncheckedIllegalAccessException;

pub struct MethodInvokers;

impl MethodInvokers {
	pub fn as_bi_consumer<T, U>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::util::function::BiConsumer /**/ {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(BiConsumer.class, method);
	}

	pub fn as_bi_function<T, U, R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::util::function::BiFunction /**/ {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(BiFunction.class, method);
	}

	pub fn as_failable_bi_consumer<T, U>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::function::failable_bi_consumer::FailableBiConsumer {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(FailableBiConsumer.class, method);
	}

	pub fn as_failable_bi_function<T, U, R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::function::failable_bi_function::FailableBiFunction {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(FailableBiFunction.class, method);
	}

	pub fn as_failable_function<T, R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::function::failable_function::FailableFunction {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(FailableFunction.class, method);
	}

	pub fn as_failable_supplier<R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> org::apache::commons::lang3::function::failable_supplier::FailableSupplier {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(FailableSupplier.class, method);
	}

	pub fn as_function<T, R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::util::function::Function /**/ {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(Function.class, method);
	}

	pub fn as_interface_instance<T>(&self, interface_class: &/* Java */ java::lang::Class /**/, method: &/* Java */ java::lang::reflect::Method /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedIllegalAccessException) */ -> T {
		return MethodHandleProxies::asInterfaceInstance(&Objects::requireNonNull(interface_class, "interfaceClass"), &org::apache::commons::lang3::function::method_invokers::MethodInvokers::unreflect_unchecked(method)?);
	}

	pub fn as_supplier<R>(&self, method: &/* Java */ java::lang::reflect::Method /**/) /* thrown(org.apache.commons.lang3.exception.UncheckedIllegalAccessException) */ -> /* Java */ java::util::function::Supplier /**/ {
		return org::apache::commons::lang3::function::method_invokers::MethodInvokers::as_interface_instance(Supplier.class, method)?;
	}

	fn require_method(&self, method: &/* Java */ java::lang::reflect::Method /**/) -> /* Java */ java::lang::reflect::Method /**/ {
		return Objects::requireNonNull(method, "method");
	}

	fn unreflect(&self, method: &/* Java */ java::lang::reflect::Method /**/) /* thrown(java.lang.IllegalAccessException) */ -> /* Java */ java::lang::invoke::MethodHandle /**/ {
		return MethodHandles::lookup().unreflect(&org::apache::commons::lang3::function::method_invokers::MethodInvokers::require_method(method));
	}

	fn unreflect_unchecked(&self, method: &/* Java */ java::lang::reflect::Method /**/) /* thrown(java.lang.IllegalAccessException | org.apache.commons.lang3.exception.UncheckedIllegalAccessException) */ -> /* Java */ java::lang::invoke::MethodHandle /**/ {
		let r0 = 'try0: {
			return match org::apache::commons::lang3::function::method_invokers::MethodInvokers::unreflect(method) {
				Err(e) => break 'try0 Err(e),
				Ok(s) => s,
			};
			break 'try0 Ok(());
		};
		match r0 {
			Err(e @ IllegalAccessException) => {
				break 'try0 Err(UncheckedIllegalAccessException::new(e));
			},
			Err(e) => Err(e)?,
			Ok => (),
		}
	}

	fn new() -> org::apache::commons::lang3::function::method_invokers::MethodInvokers {
	// noop
	}
}