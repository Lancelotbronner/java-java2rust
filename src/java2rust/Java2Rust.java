package java2rust;

import com.github.javaparser.ParseProblemException;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import org.apache.commons.lang3.ArrayUtils;


public class Java2Rust {
	public static String convert(String javaString) {
		try {
			CompilationUnit compilationUnit = StaticJavaParser.parse(javaString);
			return convert(compilationUnit);
		} catch (ParseProblemException e) {
			return e.toString();
		}
	}

	public static String convert(CompilationUnit unit) {
		IdTrackerVisitor idTrackerVisitor = new IdTrackerVisitor();
		IdTracker idTracker = new IdTracker();
		idTrackerVisitor.visit(unit, idTracker);
		TypeTrackerVisitor typeTrackerVisitor = new TypeTrackerVisitor(idTracker);
		typeTrackerVisitor.visit(unit, null);

		RustDumpVisitor dumper = new RustDumpVisitor(true, idTracker, typeTrackerVisitor);
		dumper.visit(unit, null);
		return dumper.getSource();
	}

	public static String identifier(String java) {
		if (Character.isLowerCase(java.charAt(0))) {
			StringBuilder sb = new StringBuilder();
			for (Character c : java.toCharArray()) {
				if (Character.isUpperCase(c)) {
					sb
						.append("_")
						.append(Character.toLowerCase(c));
				} else {
					sb.append(c);
				}
			}
			return sb.toString();
		}
		return java;
	}

	public static String toSnakeCase(String java) {
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
}