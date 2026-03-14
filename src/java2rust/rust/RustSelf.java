package java2rust.rust;

import org.jspecify.annotations.NonNull;

import java.util.function.Supplier;

public record RustSelf(@NonNull String template, Supplier<RustSelf> mut) {
	@Override
	public @NonNull String toString() {
		return template;
	}

	public static RustSelf REF = new RustSelf("&self", () -> RustSelf.MUT);
	public static RustSelf MUT = new RustSelf("&mut self", () -> RustSelf.MUT);
}
