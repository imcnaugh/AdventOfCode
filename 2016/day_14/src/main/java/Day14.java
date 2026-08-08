import java.security.*;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

record TriAndIndex(char c, int index){}

public class Day14 {

    private String salt;
    private Pattern tripPattern = Pattern.compile("(.)\\1{2,}");

    private LinkedList<TriAndIndex> tripsAndIndexes = new LinkedList<>();

    public Day14(String salt) throws Exception {
        this.salt = salt;
    }

    public int getIndexOf64thHash() throws Exception {
        AtomicInteger countOfHashes = new AtomicInteger();
        int index = 0;
        while(true){
            String hashOfIndex = getBase64EncodedHash(index);
            Matcher m = tripPattern.matcher(hashOfIndex);
            if(m.find()){
                String i = m.group(1);
                char c = i.charAt(0);

                while(true){
                    if (tripsAndIndexes.peek() == null) break;
                    if (tripsAndIndexes.peek().index() + 1000 > index) break;
                    tripsAndIndexes.removeFirst();
                }

                Pattern fivePattern = generateFivePattern(c);
                Matcher fmp = fivePattern.matcher(hashOfIndex);
                if(fmp.find()){
                    for (Iterator<TriAndIndex> it = tripsAndIndexes.iterator(); it.hasNext();) {
                        TriAndIndex idk = it.next();

                        if (idk.c() == c) {
                            int cur = countOfHashes.incrementAndGet();

                            if (cur == 64) {
                                return idk.index();
                            }

                            it.remove();
                        }
                    }
                }

                TriAndIndex t = new TriAndIndex(c, index);
                tripsAndIndexes.add(t);
            }
            index++;
        }
    }

    private String getBase64EncodedHash(int count) throws Exception
    {
        String s = salt + count;
        for(int i = 0; i < 2017; i++){
            s = hashString(s);
        }
        return s;
    }

    private static String hashString(String s) throws NoSuchAlgorithmException {
        byte[] b = s.getBytes();
        MessageDigest md = MessageDigest.getInstance("MD5");
        byte[] digest = md.digest(b);
        StringBuilder sb = new StringBuilder(digest.length * 2);
        for (byte value : digest) {
            sb.append(String.format("%02x", value));
        }
        return sb.toString();
    }

    private Pattern generateFivePattern(char c) {
        return Pattern.compile(Pattern.quote(String.valueOf(c)) + "{5}");
    }
}
