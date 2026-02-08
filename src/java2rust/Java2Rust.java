package java2rust;

import com.github.javaparser.ParseException;
import com.github.javaparser.ParseProblemException;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;


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

}