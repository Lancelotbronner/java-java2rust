import org.junit.jupiter.api.Test;

/**
 * Tests for block statement conversion.
 */
public class BlockTest {
	@Test
	public void canConvertBlockStatementsToRust() {
		String java = """
			class A {
			    void block() {
			        {
			            int x = 1;
			            int y = 2;
			        }
			        System.out.println("done");
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn block(&self) {
			        {
			            let x: i32 = 1;
			            let y: i32 = 2;
			        }
			        System::out.println("done");
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
