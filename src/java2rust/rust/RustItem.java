package java2rust.rust;

public abstract class RustItem {
	public final String javaName;
	public final String rustName;
	public final RustModule module;
	public final boolean isPublic;

	protected RustItem(String javaName, String rustName, RustModule module, boolean isPublic) {
		this.javaName = javaName;
		this.rustName = rustName;
		this.module = module;
		this.isPublic = isPublic;
	}

	public String name() {
		return javaName;
	}
}
