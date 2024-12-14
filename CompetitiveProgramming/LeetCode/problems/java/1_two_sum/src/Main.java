import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }

    public static int[] twoSum(int[] nums, int target) {
        HashMap<Integer,Integer> map = new HashMap<>();

        for(int i = 0; i < nums.length ; i++) {
            if (map.get(nums[i]) != null) {
                return new int[]{map.get(nums[i]), i};
            }
            map.putIfAbsent(target - nums[i], i);
        }

        return new int[]{};
    }
}