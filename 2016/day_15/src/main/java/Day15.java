static String input = "Disc #1 has 13 positions; at time=0, it is at position 11.\n" +
        "Disc #2 has 5 positions; at time=0, it is at position 0.\n" +
        "Disc #3 has 17 positions; at time=0, it is at position 11.\n" +
        "Disc #4 has 3 positions; at time=0, it is at position 0.\n" +
        "Disc #5 has 7 positions; at time=0, it is at position 2.\n" +
        "Disc #6 has 19 positions; at time=0, it is at position 17.";

public class Day15 {
    private Disk[] disks;
    private final Pattern diskParsePatter = Pattern.compile("(\\d+) positions; at time=0, it is at position (\\d+)");

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

    public void addPart2Disk() {
        Disk part2Disk = new Disk(11, 0);
        Disk[] newDiskArray = new Disk[this.disks.length +1];
        newDiskArray[newDiskArray.length - 1] = part2Disk;
        for(int i = 0; i < disks.length; i++){
            newDiskArray[i] = disks[i];
        }
        this.disks = newDiskArray;
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
    System.out.printf("Part 1: %d\n", part1);
    stack.addPart2Disk();
    int part2 = stack.getFirstTWhereAllDisksPass();
    System.out.printf("Part 2: %d\n", part2);
}
