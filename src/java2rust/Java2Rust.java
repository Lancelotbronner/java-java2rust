package java2rust;

import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.StaticJavaParser;
import java2rust.rust.RustJar;
import java2rust.rust.RustUnit;
import org.apache.commons.lang3.ArrayUtils;

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

	/*
	public static String convert(CompilationUnit unit, RustModule mod) {
		IdTrackerVisitor idTrackerVisitor = new IdTrackerVisitor();
		IdTracker idTracker = new IdTracker();
		idTrackerVisitor.visit(unit, idTracker);
		TypeTrackerVisitor typeTrackerVisitor = new TypeTrackerVisitor(idTracker);
		typeTrackerVisitor.visit(unit, null);

		RustVisitor dumper = new RustVisitor();
		dumper.visit(unit, null);
		return dumper.getSource();
	}
	 */

	public static String identifier(String java) {
		if (Character.isLowerCase(java.charAt(0))) {
			StringBuilder sb = new StringBuilder();
			for (Character c : java.toCharArray()) {
				if (Character.isUpperCase(c)) {
					sb.append("_").append(Character.toLowerCase(c));
				} else {
					sb.append(c);
				}
			}
			return sb.toString();
		}
		return java;
	}

	public static String pascalCaseToSnakeCase(String java) {
		StringBuilder sb = new StringBuilder();
		Character[] chars = ArrayUtils.toObject(java.toCharArray());
		int i = 0;
		for (Character c : chars) {
			if (Character.isUpperCase(c)) {
				if (!sb.isEmpty() && !Character.isUpperCase(ArrayUtils.get(chars, i + 1, '\0')))
					sb.append('_');
				sb.append(Character.toLowerCase(c));
			} else {
				sb.append(c);
			}
			i++;
		}
		return sb.toString();
	}

	public static String camelCaseToSnakeCase(String java) {
		StringBuilder sb = new StringBuilder();
		Character[] chars = ArrayUtils.toObject(java.toCharArray());
		int i = 0;
		for (Character c : chars) {
			if (Character.isUpperCase(c)) {
				if (!sb.isEmpty() && !Character.isUpperCase(ArrayUtils.get(chars, i + 1, 'A')))
					sb.append('_');
				sb.append(Character.toLowerCase(c));
			} else {
				sb.append(c);
			}
			i++;
		}
		return sb.toString();
	}
}