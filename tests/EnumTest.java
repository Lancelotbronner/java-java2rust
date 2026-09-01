import org.junit.jupiter.api.Test;

/**
 * Tests for enum conversion.
 */
public class EnumTest {
	@Test
	public void canConvertSimpleEnumToRust() {
		String java = """
			enum Color { RED, GREEN, BLUE }
			""";
		String expected = """
			enum Color {
			    RED, GREEN, BLUE
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertEnumWithValuesToRust() {
		String java = """
			enum Status {
			    ACTIVE(1),
			    INACTIVE(0);
			    int code;
			    Status(int c) { code = c; }
			}
			""";
		String expected = """
			enum Status {
			    ACTIVE(1), INACTIVE(0);
			    let code: i32;
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
