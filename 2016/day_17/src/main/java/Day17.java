import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

static String INPUT = "yjjvjgan";
static int MAX_WIDTH = 4;
static int MAX_HEIGHT = 4;

class CurrentLocation {
    public int x, y;
    public String str, hash;

    public CurrentLocation(int x, int y, String str){
        this.x = x;
        this.y = y;
        this.str = str;
        this.hash = hashString(str);
    }

    public boolean canMoveUp() {
        if(y >= MAX_HEIGHT - 1) {
            return false;
        }
        return isDoorOpen(0);
    }

    public boolean canMoveDown() {
        if(y <= 0) {
            return false;
        }
        return isDoorOpen(1);
    }

    public boolean canMoveLeft() {
        if(x <= 0) {
            return false;
        }
        return isDoorOpen(2);
    }

    public boolean canMoveRight() {
        if( x >= MAX_WIDTH - 1) {
            return false;
        }
        return isDoorOpen(3);
    }

    private boolean isDoorOpen(int index) {
        return switch (hash.charAt(index)) {
            case 'b', 'c', 'd', 'e', 'f' -> true;
            default -> false;
        };
    }

    private static String hashString(String s) {
        try {
            byte[] b = s.getBytes();
            MessageDigest md = MessageDigest.getInstance("MD5");
            byte[] digest = md.digest(b);
            StringBuilder sb = new StringBuilder(digest.length * 2);
            for (byte value : digest) {
                sb.append(String.format("%02x", value));
            }
            return sb.toString();
        } catch (NoSuchAlgorithmException e) {
            return "";
        }
    }
}

public class Day17 {
    public int getStepsToEnd(String input) {
        CurrentLocation cur = new CurrentLocation(0, 3, input);
        Set<CurrentLocation> currentLocations = new HashSet<>();
        currentLocations.add(cur);

        int longestPath = 0;

        int index = 0;
        while(true) {
            if(currentLocations.isEmpty()){
                return longestPath;
            }

            Set<CurrentLocation> nextLocations = new HashSet<>();

            for(CurrentLocation c : currentLocations){
                if(c.x == 3 && c.y == 0){
                    longestPath = index;
                    continue;
                }

                if(c.canMoveUp()){
                    nextLocations.add(new CurrentLocation(c.x, c.y + 1, c.str + "U"));
                }
                if(c.canMoveDown()){
                    nextLocations.add(new CurrentLocation(c.x, c.y - 1, c.str + "D"));
                }
                if(c.canMoveLeft()){
                    nextLocations.add(new CurrentLocation(c.x - 1, c.y, c.str + "L"));
                }
                if(c.canMoveRight()){
                    nextLocations.add(new CurrentLocation(c.x + 1, c.y, c.str + "R"));
                }
            }

            currentLocations = nextLocations;
            index++;
        }
    }
}

void main() {
    Day17 day17 = new Day17();
    int part1 = day17.getStepsToEnd(INPUT);
//    int part1 = day17.getStepsToEnd("ihgpwlah");
    System.out.printf("Part 1: %d", part1);
}