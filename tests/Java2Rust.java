import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.StaticJavaParser;
import java2rust.JavaTranspiler;
import java2rust.RustJar;
import java2rust.RustUnit;

import static org.junit.jupiter.api.Assertions.assertEquals;


public class Java2Rust {
	public static void assertConversion(String java, String rust) {
		assertEquals(rust.trim(), test(java).trim());
	}

	public static String test(String java) {
		JavaTranspiler transpiler = new JavaTranspiler();

		ParserConfiguration config = new ParserConfiguration();
		config.setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
		config.setSymbolResolver(transpiler.solver);
		StaticJavaParser.setConfiguration(config);

		transpiler.addSourceCode("test.java", java);
		transpiler.preanalyze();
		transpiler.analyze();

		StringBuilder sb = new StringBuilder();
		RustJar crate = transpiler.crates.getFirst();

		for (RustUnit unit : crate.units) {
			if (crate.units.size() > 1) {
				sb.append("// ");
				sb.append(unit.path);
			}
			sb.append("\n");
			sb.append(unit);
			sb.append("\n");
		}

		return sb.toString().trim();
	}
}