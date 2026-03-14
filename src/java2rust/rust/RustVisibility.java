package java2rust.rust;

import org.jspecify.annotations.NonNull;

public record RustVisibility(@NonNull String in) {
	public static RustVisibility INFERRED = new RustVisibility("");
	public static RustVisibility PUB = new RustVisibility("crate");

	public static RustVisibility pub(boolean isPublic) {
		return isPublic ? PUB : INFERRED;
	}

	@Override
	public @NonNull String toString() {
		if (this == INFERRED)
			return "";
		if (this == PUB)
			return "pub ";
		return "pub(in %s) ".formatted(in);
	}
}
