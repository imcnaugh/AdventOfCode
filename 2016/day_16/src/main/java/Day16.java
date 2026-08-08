static String input = "10111100110001111";

public class Day16 {
    public Day16() {
    }

    public String expandToLen(String initial, int len){
        String s = initial;
        while(s.length() <= len){
            s = doubleIt(s);
        }
        return s.substring(0, len);
    }

    public String doubleIt(String s){
        StringBuilder sb = new StringBuilder();
        for(int i = s.length() - 1; i >= 0; i--){
            switch (s.charAt(i)){
                case '0':
                    sb.append('1');
                    break;
                case '1':
                    sb.append('0');
                    break;
                default:
                    break;

            }
        }
        return s + "0" + sb;
    }

    public String checkSum(String s){
        if(s.length() % 2 == 1){
            return s;
        }
        StringBuilder sb = new StringBuilder();
        for(int i = 0; i < s.length(); i = i + 2){
            char a = s.charAt(i);
            char b = s.charAt(i+1);
            if(a == b) {
                sb.append('1');
            }else {
                sb.append('0');
            }
        }
        return checkSum(sb.toString());
    }
}

void main() {
    Day16 day16 = new Day16();
    String expanded = day16.expandToLen(input, 272);
    String checksum = day16.checkSum(expanded);
    System.out.printf("Part 1: %s\n", checksum);
}