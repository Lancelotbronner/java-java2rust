package java2rust.rust;

import org.jspecify.annotations.NonNull;

public record RustVisibility(@NonNull String in) {
	public static RustVisibility PRIVATE = new RustVisibility("");
	public static RustVisibility PUB = new RustVisibility("crate");

	public static RustVisibility pub(boolean isPublic) {
		return isPublic ? PUB : PRIVATE;
	}

	@Override
	public String toString() {
		if (this == PRIVATE)
			return "";
		if (this == PUB)
			return "pub ";
		return "pub(in %s) ".formatted(in);
	}
}
