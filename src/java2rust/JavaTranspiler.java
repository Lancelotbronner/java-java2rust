package java2rust;

import com.github.javaparser.ParseProblemException;
import com.github.javaparser.Problem;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.ast.expr.Expression;
import com.github.javaparser.ast.stmt.Statement;
import com.github.javaparser.ast.type.Type;
import com.github.javaparser.quality.NotNull;
import com.github.javaparser.resolution.declarations.ResolvedReferenceTypeDeclaration;
import com.github.javaparser.resolution.types.ResolvedType;
import com.github.javaparser.symbolsolver.JavaSymbolSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.CombinedTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.JavaParserTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.ReflectionTypeSolver;
import java2rust.rust.RustItem;
import java2rust.rust.RustModule;
import java2rust.rust.RustVisibility;
import org.apache.commons.io.FilenameUtils;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;

/**
 * Created by aschoerk on 30.04.16.
 */
public final class JavaTranspiler {
	public final RustModule lib;
	public final File output;
	public final JavaSymbolSolver solver;
	public final HashMap<String, Task> tasks = new HashMap<>();
	private final CombinedTypeSolver solvers = new CombinedTypeSolver();
	private final Set<File> directories = new HashSet<>();
	private final HashMap<String, String> names = new HashMap<>();

	public JavaTranspiler(File crate) {
		this.output = new File(crate, "src");
		this.lib = RustModule.lib(Java2Rust.camelCaseToSnakeCase(FilenameUtils.removeExtension(crate.getName())));
		this.solver = new JavaSymbolSolver(solvers);
		solvers.setExceptionHandler(CombinedTypeSolver.ExceptionHandlers.IGNORE_ALL);
		solvers.add(new ReflectionTypeSolver());
	}

	public JavaTranspiler(String lib) {
		this.output = null;
		this.lib = RustModule.lib(lib);
		this.solver = new JavaSymbolSolver(solvers);
		solvers.add(new ReflectionTypeSolver());
	}

	public void addPackage(File input) {
		if (directories.contains(input))
			return;
		if (!addItem(input, input, lib))
			return;
		solvers.add(new JavaParserTypeSolver(input));
	}

	public void addCode(String filename, String java) {
		tasks.put(filename, new Task(filename, java, lib));
	}

	public void compile(Consumer<Task> onCompile) {
		for (Task task : tasks.values()) {
			onCompile.accept(task);
			task.compile();
			if (task.unit != null)
				new DeclVisitor(this, task).visit(task.unit, null);
		}
	}

	public long numberOfTasksToAnalyze() {
		return tasks.values().stream().filter(t -> t.unit != null).count();
	}

	public void analyze() {
		lib.analyze(this);
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
		try {
			ResolvedType ty = type.resolve();
			if (ty != null)
				return describe(ty);
		} catch (Throwable e) {
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
		if (ty.isReferenceType())
			return this.describeViaId(ty.asReferenceType().getId());
		if (ty.isTypeVariable())
			return ty.asTypeVariable().describe();
		return "/* Java */ %s".formatted(ty.describe());
	}

	public String describe(@NotNull ResolvedReferenceTypeDeclaration ty) {
		return describeViaId(ty.getId());
	}

	private String describeViaId(@NotNull String id) {
		return this.nameOf(id, "/* Java */ " + id + " /**/");
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

	public String describe(Statement stmt) {
		if (stmt == null)
			return "";
		RustVisitor visitor = new RustVisitor(this);
		stmt.accept(visitor, null);
		return visitor.toString();
	}

	public void generate(
		Consumer<Task> onProcess,
		Consumer<Problem> onProblem,
		Consumer<Throwable> onError
	) {
		for (Task task : tasks.values()) {
			onProcess.accept(task);
			String code = generate(task, onProblem, onError);

			try {
				Files.createDirectories(task.output.getParentFile().toPath());
				Files.writeString(task.output.toPath(), code);
			} catch (IOException e) {
				onError.accept(e);
			}
		}
	}

	private String generate(Task task, Consumer<Problem> onProblem, Consumer<Throwable> onError) {
		String code;

		if (task.problem != null) {
			StringBuilder sb = new StringBuilder("/*\n");

			for (Problem problem : task.problem.getProblems()) {
				sb.append("FIXME: ");
				sb.append(problem.getVerboseMessage());
				sb.append('\n');
				onProblem.accept(problem);
			}

			sb.append("*/\n");
			code = sb.toString();
		} else {
			try {
				if (task.exception != null)
					throw task.exception;
				//				code = Java2Rust.convert(task.unit, task.module);
				code = "TODO";
			} catch (Throwable e) {
				code = "/*\nFIXME: %s\n*/\n".formatted(e.toString());
				onError.accept(e);
			}
		}

		return code;
	}

	public String generate(String path) {
		Task task = tasks.get(path);
		if (task == null)
			return null;
		return generate(task, _ -> {}, _ -> {});
	}

	private boolean addSubItem(File input, File root, RustModule parentModule) {
		String moduleName = FilenameUtils.removeExtension(input.getName());
		RustModule module = parentModule.submodule(moduleName, RustVisibility.PUB);
		boolean added = addItem(input, root, module);
		if (!added)
			module.delete();
		return added;
	}

	private boolean addItem(File input, File root, RustModule module) {
		File[] children = input.listFiles();
		if (children == null) {
			if (input.getPath().endsWith(".java")) {
				tasks.put(input.getAbsolutePath(), new Task(input, root, output, module));
				return true;
			}
			return false;
		}

		boolean containsSource = false;
		for (File file : children)
			if (addSubItem(file, root, module))
				containsSource = true;
		if (containsSource)
			directories.add(input);
		return true;
	}

	public static final class Task {
		public final File input;
		public final String code;
		public final File output;
		public final RustModule module;
		public final Path relativePath;
		public CompilationUnit unit;
		public Throwable exception;
		public ParseProblemException problem;

		Task(File input, File root, File output, RustModule mod) {
			this.input = input;
			this.code = null;

			module = mod;
			relativePath = root.toPath().relativize(input.toPath());

			Path outputPath = Path.of(
				output.toString(),
				relativePath.getParent().toString(),
				"src",
				module.name + ".rs");
			this.output = new File(outputPath.toString());
		}

		Task(String filename, String java, RustModule module) {
			this.input = null;
			this.output = null;
			this.relativePath = Path.of(filename);
			this.code = java;
			this.module = module;
		}

		void compile() {
			try {
				if (input != null)
					unit = StaticJavaParser.parse(input);
				else if (code != null)
					unit = StaticJavaParser.parse(code);
			} catch (ParseProblemException e) {
				problem = e;
			} catch (Throwable e) {
				exception = e;
			}
		}
	}
}
