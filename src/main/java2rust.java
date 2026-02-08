package main;

import com.github.javaparser.ParseProblemException;
import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.Problem;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import java2rust.Java2Rust;
import org.apache.commons.io.FilenameUtils;
import org.apache.commons.lang3.StringUtils;
import org.jspecify.annotations.NonNull;
import picocli.CommandLine;
import picocli.CommandLine.Command;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

@Command(name = "java2rust", version = "java2rust 1.0", mixinStandardHelpOptions = true)
public class java2rust implements Runnable {
	@Option(names = { "-o", "--output" }, description = "the output file or directory")
	private File output;

	@Parameters(paramLabel = "<java>", description = "Java files or directories to convert to Rust.")
	private File[] input;

	@Option(names = "--language")
	private ParserConfiguration.LanguageLevel languageLevel = ParserConfiguration.LanguageLevel.JAVA_25;

	static void main(String[] args) {
		int exitCode = new CommandLine(new java2rust()).execute(args);
		System.exit(exitCode);
	}

	@Override
	public void run() {
		ParserConfiguration config = new ParserConfiguration();
		config.setLanguageLevel(languageLevel);
		StaticJavaParser.setConfiguration(config);

		for (File input : this.input) {
			System.out.printf("==> %s\n", input.toString());
			consider(input, input);
		}
	}

	void consider(@NonNull File input, @NonNull File root) {
		File[] children = input.listFiles();
		if (children == null) {
			if (input
				.getPath()
				.endsWith(".java"))
				convert(input, root);
			return;
		}

		for (File file : children)
			consider(file, root);
	}

	void convert(@NonNull File input, @NonNull File root) {
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
			filename);
		System.out.print('\t');
		System.out.println(filename);

		String code;
		try {
			CompilationUnit unit = StaticJavaParser.parse(input);
			code = Java2Rust.convert(unit);
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
			code = "/*\nFIXME: %s\n".formatted(e.getLocalizedMessage());
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
