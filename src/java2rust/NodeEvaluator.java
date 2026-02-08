package java2rust;


import com.github.javaparser.ast.Node;
import com.github.javaparser.ast.body.FieldDeclaration;
import com.github.javaparser.ast.body.MethodDeclaration;
import com.github.javaparser.ast.body.VariableDeclarator;

/**
 * Created by aschoerk on 17.05.16.
 */
public class NodeEvaluator {

	static boolean isNonStaticFieldDeclaration(Node n) {
		if (n instanceof VariableDeclarator) {
			if (n
				.getParentNode()
				.get()
				.getParentNode()
				.get() instanceof FieldDeclaration fd) {
				return !fd.isStatic();
			}
		}
		return false;
	}

	static boolean isNonStaticMethodDeclaration(Node n) {
		if (n instanceof MethodDeclaration) {
			return !((MethodDeclaration) n).isStatic();
		}
		return false;
	}

}
