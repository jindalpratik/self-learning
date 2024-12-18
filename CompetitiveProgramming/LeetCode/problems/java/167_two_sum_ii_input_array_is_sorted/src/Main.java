import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] num = { 2, 7, 11, 15 };
        int target = 9;
        System.out.println(Arrays.toString(twoSum(num, target)));
    }

    public static int[] twoSum(int[] numbers, int target) {
        int i = 0;
        int j = numbers.length - 1;
        while (true) {
            if (numbers[i] + numbers[j] == target) {
                return new int[] { i + 1, j + 1 };
            } else if (numbers[i] + numbers[j] > target) {
                j--;
            } else {
                i++;
            }
        }
    }
}