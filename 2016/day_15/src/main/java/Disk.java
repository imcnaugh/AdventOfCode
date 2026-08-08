public record Disk(int positions, int positionAtT0){
    public boolean CanPassAtTime(int time){
        return ((positionAtT0 + time) % positions) == 0;
    }
}