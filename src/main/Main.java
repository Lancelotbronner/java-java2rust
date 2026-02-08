package main;

import com.github.javaparser.ParseProblemException;
import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.Problem;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import com.github.javaparser.symbolsolver.resolution.typesolvers.CombinedTypeSolver;
import com.github.javaparser.symbolsolver.resolution.typesolvers.ReflectionTypeSolver;
import java2rust.Java2Rust;
import java2rust.rust.RustModule;
import org.apache.commons.io.FilenameUtils;
import org.apache.commons.lang3.StringUtils;
import org.jspecify.annotations.NonNull;
import picocli.CommandLine;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

@CommandLine.Command(name = "java2rust", version = "java2rust 1.0", mixinStandardHelpOptions = true)
public class Main implements Runnable {
	@Option(names = { "-o", "--output" }, description = "the output file or directory")
	private File output;

	@Parameters(paramLabel = "<java>", description = "Java files or directories to convert to Rust.")
	private File[] input;

	@Option(names = "--language")
	private ParserConfiguration.LanguageLevel languageLevel = ParserConfiguration.LanguageLevel.JAVA_25;

	static void main(String[] args) {
		int exitCode = new CommandLine(new Main()).execute(args);
		System.exit(exitCode);
	}

	@Override
	public void run() {
		ParserConfiguration config = new ParserConfiguration();
		config.setLanguageLevel(languageLevel);
		StaticJavaParser.setConfiguration(config);

		RustModule lib = RustModule.lib(Java2Rust.toSnakeCase(FilenameUtils.removeExtension(output.getName())));
		CombinedTypeSolver solver = new CombinedTypeSolver();
		solver.add(new ReflectionTypeSolver());

		for (File input : this.input) {
			System.out.printf("==> %s\n", input.toString());
			consider(input, input, lib);
		}

		System.out.println("\n[ Rust Modules ]\n");
		print(lib);
	}

	void print(RustModule mod) {
		System.out.printf("==> %s\n%s", mod.path, mod);
		for (RustModule submodule : mod.submodules())
			print(submodule);
	}

	void consider(@NonNull File input, @NonNull File root, RustModule mod) {
		File[] children = input.listFiles();
		if (children == null) {
			if (input
				.getPath()
				.endsWith(".java")) {
				String name = Java2Rust.toSnakeCase(FilenameUtils.removeExtension(input.getName()));
				convert(input, root, mod.submodule(name, true));
			}
			return;
		}

		for (File file : children)
			consider(file, root, mod);
	}

	void convert(@NonNull File input, @NonNull File root, RustModule mod) {
		String prefix = StringUtils.getCommonPrefix(input.getPath(), root.getPath());
		String relativePath = input
			.getPath()
			.substring(prefix.length());
		System.out.println(relativePath);

		String filename = FilenameUtils.removeExtension(input.getName());
		filename = Java2Rust.toSnakeCase(filename) + ".rs";
		Path outputPath = Path.of(
			this.output.toString(),
			Paths
				.get(relativePath)
				.getParent()
				.toString(),
			"src",
			filename);
		System.out.print('\t');
		System.out.println(filename);

		String code;
		try {
			CompilationUnit unit = StaticJavaParser.parse(input);
			code = Java2Rust.convert(unit, mod);
		} catch (ParseProblemException e) {
			StringBuilder sb = new StringBuilder("/*\n");

			for (Problem problem : e.getProblems()) {
				sb.append("FIXME: ");
				sb.append(problem.getVerboseMessage());
				sb.append('\n');
				System.out.print('\t');
				System.out.println(problem.getVerboseMessage());
			}

			sb.append("*/\n");
			code = sb.toString();
		} catch (Throwable e) {
			code = "/*\nFIXME: %s\n*/\n".formatted(e.toString());
			System.err.println(e.getMessage());
		}
		try {
			Files.createDirectories(outputPath.getParent());
			Files.writeString(outputPath, code);
		} catch (IOException e) {
			System.out.printf("[ERROR] %s\n%s", relativePath, e);
		}
	}
}
