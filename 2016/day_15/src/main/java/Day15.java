static String input = "Disc #1 has 13 positions; at time=0, it is at position 11.\n" +
        "Disc #2 has 5 positions; at time=0, it is at position 0.\n" +
        "Disc #3 has 17 positions; at time=0, it is at position 11.\n" +
        "Disc #4 has 3 positions; at time=0, it is at position 0.\n" +
        "Disc #5 has 7 positions; at time=0, it is at position 2.\n" +
        "Disc #6 has 19 positions; at time=0, it is at position 17.";

public class Day15 {
    private Disk[] disks;
    private Pattern diskParsePatter = Pattern.compile("(\\d+) positions; at time=0, it is at position (\\d+)");

    public Day15(String input) {
        this.disks = input.lines().map(line -> {
            Matcher m = diskParsePatter.matcher(line);
            if(m.find()){
                int positions = Integer.parseInt(m.group(1));
                int positionAtT0 = Integer.parseInt(m.group(2));
                return new Disk(positions, positionAtT0);
            }
            return null;
        }).toArray(Disk[]::new);
    }

    public int getFirstTWhereAllDisksPass() {
        int curTime = 0;
        outer:
        while(true) {
            for(int d = 0; d < disks.length; d++){
                if(!disks[d].CanPassAtTime(curTime+d +1)) {
                    curTime++;
                    continue outer;
                }
            }
            break;
        }
        return curTime;
    }
}



void main() {
    Day15 stack = new Day15(input);
    int part1 = stack.getFirstTWhereAllDisksPass();
    System.out.printf("Part 1: %d", part1);
}
