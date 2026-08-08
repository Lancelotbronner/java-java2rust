use crate::java2rust::Java2Rust;
use commons_lang3::org::junit::jupiter::api::Test;

pub struct ForTester;

impl ForTester {
	pub fn can_convert_complete_for_to_rust(&self) {
		let java: String = r#"
		void main() {
		    for (int i = 10; i < 100; i++)
		        System.out.println("i: " + i);
		    for (int i = 10; i < 100; i++) {
		        System.out.println("i: " + i);
		    }
		}
		"#;
		let expected: String = r#"
		for i in 10..100 {
		   System::out.println("i: " + i);
		}
		"#;
		Java2Rust::assert_conversion(java, expected);
	}
}