import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;


public class DiskTests {
    @Test
    public void simpleDiskTest() {
        Disk d = new Disk(5, 0);
        for(int i = 0; i < 16; i++){
            if(i % 5 == 0){
                assertTrue(d.CanPassAtTime(i));
            } else {
                assertFalse(d.CanPassAtTime(i));
            }
        }
    }
}
