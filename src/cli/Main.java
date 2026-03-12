package cli;

import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.StaticJavaParser;
import java2rust.JavaTranspiler;
import java2rust.rust.RustJar;
import java2rust.rust.RustPackage;
import picocli.CommandLine;
import picocli.CommandLine.Option;
import picocli.CommandLine.Parameters;

import java.io.File;
import java.io.IOException;
import java.nio.file.Path;

@CommandLine.Command(name = "java2rust", version = "java2rust 1.0", mixinStandardHelpOptions = true)
public class Main implements Runnable {
	@Parameters(paramLabel = "<crate>", description = "the root of the crate to generate", index = "0")
	private File crate;

	@Parameters(paramLabel = "<java>", description = "Java files or directories to convert to Rust.", index = "1..")
	private File[] input;

	@Option(names = "--maven")
	private String[] maven;

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

		JavaTranspiler transpiler = new JavaTranspiler(crate);
		config.setSymbolResolver(transpiler.solver);

		boolean hasError = false;
		for (String dep : maven) {
			try {
				System.out.println("=> " + dep);
				transpiler.addMavenDependency(dep);
			} catch (Exception e) {
				System.err.println(e);
				hasError = true;
			}
		}
		if (hasError)
			return;

		int files = 0;
		for (File input : this.input) {
			System.out.printf("=> %s\n", input.toString());
			transpiler.addPackage(input);
			System.out.printf("\t%s files added\n", transpiler.tasks.size() - files);
			files = transpiler.tasks.size();
		}

		System.out.printf("==> Compiling %s Java files...\n", transpiler.tasks.size());
		transpiler.compile(t -> System.out.printf("\t%s\n", t.relativePath));

		long total = transpiler.numberOfTasksToAnalyze();
		System.out.printf("==> Analyzing %s Java files...\n", total);
		transpiler.analyze();

		/*
		System.out.print("=> Printing analysis results\n");
		print(transpiler.lib);
		 */

		Path outputPath = crate.toPath();

		System.out.printf("==> Writing Rust files to %s...\n", crate);
		transpiler.lib.generate(outputPath.resolve("current/src"));

		for (RustJar jar : transpiler.crates) {
			System.out.printf("==> Generating '%s' crate...\n", jar.name);
			try {
				jar.generate(outputPath);
			} catch (IOException e) {
				System.err.printf("Failed to write: %s\n", e.getLocalizedMessage());
			}
		}
	}

	void print(RustPackage mod) {
		System.out.printf("==> %s\n%s", mod.path, mod);
		for (RustPackage submodule : mod.submodules())
			print(submodule);
	}
}
