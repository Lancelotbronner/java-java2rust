import org.junit.jupiter.api.Test;

/**
 * Tests for try-catch conversion.
 */
public class TryCatchTest {
	@Test
	public void canConvertTryCatchToRust() {
		String java = """
			class A {
			    void risky() {
			        try {
			            System.out.println("trying");
			        } catch (Exception e) {
			            System.out.println("error");
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn risky(&self) {
			        let r0 = 'try0: {
			            System::out.println("trying");
			            break 'try0 Ok(());
			        };
			        match r0 { Err(e @ Exception) => { System::out.println("error"); }, Err(e) => Err(e)?, Ok => (), }
			    }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
