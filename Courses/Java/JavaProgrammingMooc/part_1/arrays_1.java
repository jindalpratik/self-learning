
import java.util.Arrays;

public class arrays_1 {
    public static void main(String[] args) {
        int[] arr = getNumber();
        System.out.println(Arrays.toString(arr));
        
    }

    private static int[] getNumber() {
        int[] numbers = {1,2,3,4,5};
        return numbers;
    }
}