package java2rust;

import com.github.javaparser.ParseProblemException;
import com.github.javaparser.Problem;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.symbolsolver.JavaSymbolSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.CombinedTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.JavaParserTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.ReflectionTypeSolver;
import java2rust.rust.RustModule;
import org.apache.commons.io.FilenameUtils;
import org.apache.commons.lang3.StringUtils;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.function.Consumer;

/**
 * Created by aschoerk on 30.04.16.
 */
public final class JavaTranspiler {
	public final RustModule lib;
	public final File output;
	private final CombinedTypeSolver solvers = new CombinedTypeSolver();
	private final JavaSymbolSolver solver;
	private final Set<File> directories = new HashSet<>();
	private final List<Task> tasks = new ArrayList<>();

	public JavaTranspiler(File output) {
		this.output = output;
		this.lib = RustModule.lib(Java2Rust.toSnakeCase(FilenameUtils.removeExtension(output.getName())));
		this.solver = new JavaSymbolSolver(solvers);
		solvers.add(new ReflectionTypeSolver());
	}

	public void add(File input) {
		if (directories.contains(input))
			return;
		add(input, input, lib);
	}

	public void transpile(
		Consumer<Task> willProcess,
		Consumer<Problem> onProblem,
		Consumer<Throwable> onError
	) {
		for (Task task : tasks) {
			willProcess.accept(task);
			String code;
			try {
				CompilationUnit unit = StaticJavaParser.parse(task.input);
				code = Java2Rust.convert(unit, task.module);
			} catch (ParseProblemException e) {
				StringBuilder sb = new StringBuilder("/*\n");

				for (Problem problem : e.getProblems()) {
					sb.append("FIXME: ");
					sb.append(problem.getVerboseMessage());
					sb.append('\n');
					onProblem.accept(problem);
				}

				sb.append("*/\n");
				code = sb.toString();
			} catch (Throwable e) {
				code = "/*\nFIXME: %s\n*/\n".formatted(e.toString());
				onError.accept(e);
			}
			try {
				Files.createDirectories(task.output
					.getParentFile()
					.toPath());
				Files.writeString(task.output.toPath(), code);
			} catch (IOException e) {
				onError.accept(e);
			}
		}
	}

	private boolean add(File input, File root, RustModule mod) {
		File[] children = input.listFiles();
		if (children == null) {
			if (input
				.getPath()
				.endsWith(".java")) {
				tasks.add(new Task(input, root, output, mod));
				return true;
			}
			return false;
		}

		boolean containsSource = false;
		for (File file : children)
			if (add(file, root, mod))
				containsSource = true;
		if (containsSource) {
			directories.add(input);
			solvers.add(new JavaParserTypeSolver(input));
		}
		return true;
	}

	public static final class Task {
		public final File input;
		public final File output;
		public final RustModule module;
		public final String relativePath;

		Task(File input, File root, File output, RustModule parentModule) {
			this.input = input;

			String prefix = StringUtils.getCommonPrefix(input.getPath(), root.getPath());
			relativePath = input
				.getPath()
				.substring(prefix.length() + 1);

			String moduleName = FilenameUtils.removeExtension(input.getName());
			module = parentModule.submodule(moduleName, true);

			Path outputPath = Path.of(
				output.toString(),
				Paths
					.get(relativePath)
					.getParent()
					.toString(),
				"src",
				module.rustName + ".rs");
			this.output = new File(outputPath.toString());
		}
	}
}
