package java2rust;

import org.jspecify.annotations.NonNull;

import java.util.function.Supplier;

public record RustSelf(@NonNull String template, boolean isMut, Supplier<RustSelf> mut) {
	@Override
	public @NonNull String toString() {
		return template;
	}

	public static RustSelf OWNED = new RustSelf("self", true, () -> RustSelf.OWNED);
	public static RustSelf REF = new RustSelf("&self", false, () -> RustSelf.MUT);
	public static RustSelf MUT = new RustSelf("&mut self", true, () -> RustSelf.MUT);
}
