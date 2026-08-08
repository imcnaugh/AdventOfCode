import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class Day14Tests {

    @Test
    public void Test() throws Exception {
        Day14 d14 = new Day14("abc");
        int part1 = d14.getIndexOf64thHash();
        assertEquals(22859, part1);
    }

    @Test
    public void Part1() throws Exception {
        Day14 d14 = new Day14("jlmsuwbz");
        int part1 = d14.getIndexOf64thHash();
        assertEquals(35186, part1);
    }


}
