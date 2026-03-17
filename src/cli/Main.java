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
	@Parameters(paramLabel = "<output>", description = "The directory in which to generate Rust crates.", index = "0")
	private File output;

	@Option(names = "--sources", description = "Java files or directories to convert to Rust crates.")
	private File[] sources;

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

		JavaTranspiler transpiler = new JavaTranspiler();
		config.setSymbolResolver(transpiler.solver);

		for (int i = 0; i < sources.length; i++) {
			try {
				sources[i] = sources[i].getCanonicalFile();
			} catch (IOException e) {
				System.err.printf("Failed to resolve sources: %s\n ", e);
			}
		}

		boolean hasError = false;
		for (String dep : maven) {
			System.out.println("=> " + dep);
			try {
				transpiler.addMavenDependency(dep);
			} catch (Exception e) {
				System.err.println(e);
				hasError = true;
			}
		}
		if (hasError)
			return;

		for (File sources : this.sources) {
			System.out.printf("=> %s\n", sources);
			transpiler.addSources(sources);
		}

		System.out.printf("==> Processing %s Java files...\n", transpiler.crates.size());
		transpiler.preanalyze();

		System.out.printf("==> Analyzing %s Java files...\n", transpiler.numberOfTasksToAnalyze());
		transpiler.analyze();

		System.out.printf("==> Generating '%s' crates...\n", transpiler.crates.size());
		for (RustJar jar : transpiler.crates) {
			System.out.printf("\t%s\n", jar.name);
			try {
				jar.generate(output.toPath());
			} catch (IOException e) {
				System.err.printf("\tFailed to write: %s\n", e.getLocalizedMessage());
			}
		}

		System.out.println("==> Done!");
	}
}
