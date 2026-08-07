import java.util.HashSet;
import java.util.Set;

enum Area{
    Wall,
    Space
}

class Point {
    public int x;
    public int y;
    public Point(int x, int y){
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (!(obj instanceof Point other)) return false;

        return x == other.x && y == other.y;
    }

    @Override
    public int hashCode() {
        return 31 * x + y;
    }
}

public class Day13 {

    Area[][] building;
    int targetX, targetY;

    public Day13(int input, int width, int height, int targetX, int targetY){
        this.building = new Area[height][width];
        for(int y = 0; y < height; y++){
            for(int x = 0; x < width; x++){
                this.building[y][x] = getArea(x, y, input);
            }
        }
        this.targetX = targetX;
        this.targetY = targetY;
    }

    public String toString() {
        StringBuilder retVal = new StringBuilder();
        for (int y = 0; y < building.length; y++) {
            for (int x = 0; x < building[0].length; x++) {
                if(y == targetY && x == targetX){
                    retVal.append("X");
                }else {
                    retVal.append(building[y][x] == Area.Space ? "." : "#");
                }
            }
            retVal.append("\n");
        }
        return retVal.toString();
    }

    public int bfs() {
        int curX = 1;
        int curY = 1;

        Set<Point> current = new HashSet<>();
        Set<Point> seen = new HashSet<>();
        current.add(new Point(curX, curY));

        return recurBfs(current, seen, 0);
    }

    private int recurBfs(Set<Point> current, Set<Point> seen, int stepCount) {
        if(current.isEmpty()) {
            return -1;
        }

        Set<Point> nextCurrent = new HashSet<>();

        for (Point p: current){
            if(p.x == this.targetX && p.y == this.targetY){
                return stepCount;
            }
            seen.add(new Point(p.x, p.y));

            if(p.x + 1 < building[0].length && building[p.y][p.x+1] == Area.Space){
                Point pn = new Point(p.x + 1, p.y);
                if(!seen.contains(pn)){
                    nextCurrent.add(pn);
                }
            }

            if(p.x - 1 >= 0 && building[p.y][p.x - 1] == Area.Space){
                Point ps = new Point(p.x -1, p.y);
                if(!seen.contains(ps)){
                    nextCurrent.add(ps);
                }
            }

            if(p.y + 1 < building.length && building[p.y + 1][p.x] == Area.Space){
                Point pe = new Point(p.x, p.y + 1);
                if(!seen.contains(pe)){
                    nextCurrent.add(pe);
                }
            }

            if(p.y - 1 >= 0 && building[p.y - 1][p.x] == Area.Space){
                Point pw = new Point(p.x, p.y - 1);
                if(!seen.contains(pw)){
                    nextCurrent.add(pw);
                }
            }
        }

        return recurBfs(nextCurrent, seen, stepCount + 1);
    }

    private Area getArea(int x, int y, int favNum) {
        int num = (x*x) + (3*x) + (2*x*y) + y + (y*y) + favNum;
        return Integer.bitCount(num) % 2 == 0 ? Area.Space : Area.Wall;
    }
}
