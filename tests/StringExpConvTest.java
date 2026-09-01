import org.junit.jupiter.api.Test;

/**
 * Tests for string expression conversion.
 */
public class StringExpConvTest {
	@Test
	public void canConvertStringConcatenationToRust() {
		String java = """
			class A {
			    void concat(int i) {
			        String s = "5 choose " + i + "gdgahdgs";
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn concat(&self, i: i32) {
			        let s: String = format!("5 choose {}gdgahdgs", i);
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
