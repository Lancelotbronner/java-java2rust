package test;

import org.junit.jupiter.api.Test;

/**
 * Created by aschoerk on 15.05.16.
 */
public class ElseTest extends Base {
    @Test
    public void canConvertNewToStaticCallNew() {
        new Integer(10);
        assertThat(call("new Class()"), containsString("Class::new()"));
        assertThat(call("new Class(i)"), containsString("Class::new(i)"));
    }

}
