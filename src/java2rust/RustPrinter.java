package java2rust;

import java.util.ArrayList;
import java.util.List;

public class RustPrinter {

	private final String indentation;
	private final StringBuilder buf = new StringBuilder();
	private final List<Integer> marks = new ArrayList<>();
	private int level = 0;
	private boolean indented = false;
	private boolean isWithinComment = false;

	public RustPrinter(final String indentation) {
		this.indentation = indentation;
	}

	public void indent() {
		level++;
	}

	public void unindent() {
		level--;
	}

	public void println(final String arg) {
		print(arg);
		println();
	}

	public void print(final String arg) {
		if (!indented) {
			makeIndent();
			indented = true;
		}
		buf.append(arg);
	}

	public void println() {
		buf.append(System.lineSeparator());
		indented = false;
	}

	public void startComment() {
		if (isWithinComment) return;
		isWithinComment = true;
		print("/* ");
	}

	public void comment(final String arg) {
		startComment();
		print(arg);
	}

	public void endComment() {
		if (!isWithinComment) return;
		isWithinComment = false;
		print("*/ ");
	}

	private void makeIndent() {
		buf.append(String
			.valueOf(indentation)
			.repeat(Math.max(0, level)));
	}

	public int push() {
		marks.add(buf.length());
		return marks.size();
	}

	public String getMark(int mark) {
		return buf.substring(marks.get(mark - 1));
	}

	public void pop() {
		buf.delete(marks.getLast(), buf.length());
		marks.removeLast();
	}

	public void drop() {
		marks.removeLast();
	}

	@Override
	public String toString() {
		return getSource();
	}

	public String getSource() {
		return buf.toString();
	}
}
