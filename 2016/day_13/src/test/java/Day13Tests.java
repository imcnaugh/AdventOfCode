import org.junit.jupiter.api.Test;

public class Day13Tests {

    @Test
    public void IdkTest() {
        Day13 test = new Day13(10, 10, 7, 7, 4);

        int idk = test.bfs();
        System.out.print(idk);
    }

    @Test void part1() {

        Day13 test = new Day13(1358, 100, 100, 31, 39);

        System.out.print(test.bfs());

    }
}
