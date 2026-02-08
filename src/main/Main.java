import com.github.javaparser.ParseProblemException;
import com.github.javaparser.ParserConfiguration;
import com.github.javaparser.StaticJavaParser;
import com.github.javaparser.ast.CompilationUnit;
import java2rust.Java2Rust;
import org.apache.commons.io.FilenameUtils;
import org.jspecify.annotations.NonNull;

void convert(@NonNull File item) throws IOException {
	File[] children = item.listFiles();
	if (children == null) {
		if (item.getPath().endsWith(".java")) {
			CompilationUnit unit;
			try {
				unit = StaticJavaParser.parse(item);
			} catch (ParseProblemException e) {
				System.out.printf("[ERROR] %s\n%s", item.getPath(), e);
				return;
			}
			String code = Java2Rust.convert(unit);
			Files.writeString(Path.of(FilenameUtils.removeExtension(item.getPath()) + ".rs"), code);
		}
		return;
	}

	for (File file : children)
		convert(file);
}

void main(String[] args) throws IOException {
	ParserConfiguration config = new ParserConfiguration();
	config.setLanguageLevel(ParserConfiguration.LanguageLevel.JAVA_25);
	StaticJavaParser.setConfiguration(config);

	for (String arg : args) {
		File item = new File(arg);
		convert(item);
	}
}
