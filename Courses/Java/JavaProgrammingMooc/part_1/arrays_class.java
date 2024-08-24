import java.util.Arrays;

public class arrays_class {
    public static void main(String[] args) {
        int[] numbers = {1,5,6,4,8,2,8};
        Arrays.sort(numbers);
        System.out.println(Arrays.toString(numbers));

        char[] characters = {'a','Z','x','y','d','t','G'};
        Arrays.sort(characters);
        System.out.println(Arrays.toString(characters));
    }
}
