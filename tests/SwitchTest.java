import org.junit.jupiter.api.Test;

/**
 * Tests for switch statement conversion.
 */
public class SwitchTest {
	@Test
	public void canConvertSwitchToMatch() {
		String java = """
			class A {
			    String dayName(int day) {
			        switch (day) {
			            case 1: return "Monday";
			            case 2: return "Tuesday";
			            default: return "Unknown";
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn day_name(&self, day: i32) -> String { match day { 1 => "Monday", 2 => "Tuesday", _ => "Unknown" } }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}

	@Test
	public void canConvertSwitchWithBlocks() {
		String java = """
			class A {
			    void process(int code) {
			        switch (code) {
			            case 1:
			                System.out.println("one");
			                break;
			            case 2:
			                System.out.println("two");
			                break;
			            default:
			                System.out.println("other");
			        }
			    }
			}
			""";
		String expected = """
			struct A {
			}
			
			impl A {
			    fn process(&self, code: i32) { match code { 1 => { System::out.println("one"); }, 2 => { System::out.println("two"); }, _ => { System::out.println("other"); } } }
			
			}
			""";
		Java2Rust.assertConversion(java, expected);
	}
}
