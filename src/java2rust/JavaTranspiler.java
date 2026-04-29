package java2rust;

import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.stmt.Statement;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.quality.NotNull;
import com.github.javaparser.resolution.declarations.ResolvedMethodDeclaration;
import com.github.javaparser.resolution.declarations.ResolvedReferenceTypeDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import com.github.javaparser.symbolsolver.JavaSymbolSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.CombinedTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.JarTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.JavaParserTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.ReflectionTypeSolver;
import com.github.javaparser.utils.SourceZip;
import java2rust.rust.*;
import javaparser.SourceZipTypeSolver;
import org.apache.commons.io.FilenameUtils;
import org.jspecify.annotations.NonNull;
import org.jspecify.annotations.Nullable;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;
import java.util.function.UnaryOperator;

public final class JavaTranspiler {
	public final List<RustJar> crates = new ArrayList<>();
	public final JavaSymbolSolver solver;
	private final CombinedTypeSolver solvers = new CombinedTypeSolver();
	private final Set<File> directories = new HashSet<>();
	private final HashMap<String, String> names = new HashMap<>();
	private final Set<String> errors = new HashSet<>();
	private final Map<String, RustMethod> methods = new HashMap<>();

	public JavaTranspiler() {
		this.solver = new JavaSymbolSolver(solvers);
		solvers.setExceptionHandler(CombinedTypeSolver.ExceptionHandlers.IGNORE_ALL);
		solvers.add(new ReflectionTypeSolver());
	}

	public void addMavenDependency(String maven) throws Exception {
		String[] components = maven.split(":", 4);
		if (components.length == 4)
			throw new Exception("Invalid maven dependency string '" + maven + "'");
		String[] path = components[0].split("\\.");
		Path resolved = Paths
			.get(System.getProperty("user.home"), ".m2")
			.resolve("repository", path)
			.resolve(components[1], components[2]);
		Path sources = resolved
			.resolve(components[1] + "-" + components[2] + "-sources.jar")
			.toRealPath();

		if (Files.exists(sources)) {
			addSourceZip(maven, components[1], sources);
			return;
		}
		System.out.println("\tCould not locate sources jar");

		Path jar = resolved.resolve(components[1] + "-" + components[2] + ".jar").toRealPath();
		if (!Files.exists(jar))
			throw new Exception("Could not locate '%s' at '%s'".formatted(maven, jar));
		System.out.println("\tRegistered jar");
		solvers.add(new JarTypeSolver(jar));
	}

	public void addSourceZip(String id, String name, Path path) throws IOException {
		SourceZip zip = new SourceZip(path, StaticJavaParser.getParserConfiguration());
		SourceZipTypeSolver solver = new SourceZipTypeSolver(zip);
		solvers.add(solver);

		System.out.println("\tParsing sources jar...");
		solver.parseIfNecessary();
		for (Path file : solver.paths)
			System.out.printf("\t%s\n", file);
		System.out.printf("\tParsed %s source files%n", solver.paths.size());
		System.out.printf("\tRegistered %s types%n", solver.types.size());

		RustJar jar = new RustJar(id, name, zip);
		addJar(jar);
	}

	public void addJar(RustJar jar) {
		//TODO: ensure preliminary visits are made?
		crates.add(jar);
	}

	public void addSources(File input) {
		File src = input.toPath().resolve("src").toFile();
		RustPackage lib = RustPackage.lib(FilenameUtils.removeExtension(input.getName()));
		RustJar jar = new RustJar(
			input.getAbsolutePath(),
			input.getName(),
			src.toPath(),
			lib,
			null);
		addJar(jar);
		if (directories.contains(src))
			return;
		if (!addItem(src, src, jar, lib))
			return;
		solvers.add(new JavaParserTypeSolver(src));
		System.out.printf("\tParsed %s source files%n", jar.units.size());
	}

	public void register(RustMethod method) {
		methods.put(method.resolved.getQualifiedSignature(), method);
	}

	public @Nullable RustMethod method(@NotNull String qualifiedSignature) {
		RustMethod m = methods.get(qualifiedSignature);
		if (m == null && errors.add(qualifiedSignature))
			System.err.printf("Unknown method (not registered) '%s'\n".formatted(qualifiedSignature));
		return m;
	}

	public @Nullable RustMethod method(@NotNull ResolvedMethodDeclaration resolved) {
		return methods.get(resolved.getQualifiedSignature());
	}

	public @NonNull RustMethod method(@NotNull MethodDeclaration decl) {
		ResolvedMethodDeclaration resolved = decl.resolve();
		String signature = resolved.getQualifiedSignature();
		RustMethod method = methods.get(signature);
		if (method == null) {
			method = new RustMethod(null, decl, resolved);
			methods.put(signature, method);
		}
		return method;
	}

	public long numberOfTasksToAnalyze() {
		return this.crates.size();
	}

	public void preanalyze() {
		for (RustJar jar : crates)
			jar.preanalyze(this);
	}

	public void analyze() {
		for (RustJar jar : crates)
			jar.analyze(this);
	}

	public void generate(Path output) throws IOException {
		for (RustJar jar : crates)
			jar.generate(output);
	}

	public void register(RustItem item) {
		names.put(item.id(), item.path());
	}

	public void registerName(String id, String name) {
		names.put(id, name);
	}

	public String nameOf(String id) {
		return names.get(id);
	}

	public String describe(Type type) {
		if (type.isVoidType())
			return "()";
		try {
			ResolvedType ty = type.resolve();
			if (ty != null)
				return describe(ty);
		} catch (Throwable e) {
			System.err.printf("Couldn't describe type: %s\n", e.getLocalizedMessage());
			return "/* %s */ %s".formatted(e.getMessage(), type);
		}
		return "/* Java */ %s".formatted(type);
	}

	public String describe(@NotNull ResolvedType ty) {
		if (ty.isPrimitive())
			return switch (ty.asPrimitive()) {
				case BYTE -> "i8";
				case SHORT -> "i16";
				case CHAR -> "u16";
				case INT -> "i32";
				case LONG -> "i64";
				case BOOLEAN -> "bool";
				case FLOAT -> "f32";
				case DOUBLE -> "f64";
			};
		if (ty.isArray())
			return "&[%s]".formatted(describe(ty.asArrayType().getComponentType()));
		if (ty.isReferenceType())
			return this.describeViaId(
				ty.asReferenceType().getId(),
				ty.asReferenceType().describe());
		if (ty.isTypeVariable())
			return ty.asTypeVariable().describe();
		throw new UnsupportedOperationException("Unknown ResolvedType " + ty);
	}

	public String describe(@NotNull ResolvedReferenceTypeDeclaration ty) {
		return describeViaId(ty.getId(), ty.getName());
	}

	private String describeViaId(@NotNull String id, String insert) {
		return describeViaId(id, () -> insert);
	}

	private String describeViaId(@NotNull String id, Supplier<String> insert) {
		if (names.get(id) instanceof String name)
			return name;
		if (errors.add(id))
			System.err.printf("Unknown identifier (not related to type solving) '%s'%n", id);
		//		String name = insert.get().replace(".", "::");
		//		return names.put(id, name);
		return "/* Java */ %s /**/".formatted(id.replace(".", "::"));
	}

	private String describeViaId(@NotNull String id, String insert, UnaryOperator<String> convert) {
		return describeViaId(id, () -> convert.apply(insert));
	}

	public String nameOf(String id, String defaultValue) {
		String name = names.get(id);
		return name == null ? defaultValue : name;
	}

	public String describe(Expression expr) {
		if (expr == null)
			return "";
		RustVisitor visitor = new RustVisitor(this);
		expr.accept(visitor, null);
		return visitor.toString();
	}

	public String describe(Statement stmt, @Nullable IRustFunction method) {
		if (stmt == null)
			return "";
		RustVisitor visitor = new RustVisitor(this);
		visitor.method = method;
		if (method != null)
			visitor.item = method.item();
		stmt.accept(visitor, null);
		return visitor.toString();
	}

	private boolean addSubItem(File input, File root, RustJar jar, RustPackage parentModule) {
		String moduleName = FilenameUtils.removeExtension(input.getName());
		RustPackage module = parentModule.submodule(moduleName, RustVisibility.PUB);
		boolean added = addItem(input, root, jar, module);
		if (!added)
			module.delete();
		return added;
	}

	private boolean addItem(File input, File root, RustJar jar, RustPackage module) {
		File[] children = input.listFiles();
		if (children == null) {
			if (input.getPath().endsWith(".java")) {
				try {
					jar.add(input.toPath(), module);
				} catch (IOException e) {
					System.err.printf("Couldn't add unit to jar: %s\n", e);
				}
				return true;
			}
			return false;
		}

		boolean containsSource = false;
		for (File file : children)
			if (addSubItem(file, root, jar, module))
				containsSource = true;
		if (containsSource)
			directories.add(input);
		return true;
	}
}
