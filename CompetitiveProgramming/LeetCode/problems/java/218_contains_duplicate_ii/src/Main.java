import java.util.HashSet;

public class Main {
    public static void main(String[] args) {
        int[] nums = {1,2,3,1};
        System.out.println(containsNearbyDuplicate(nums, 2));
    }

    public static boolean containsNearbyDuplicate(int[] nums, int k) {
        HashSet<Integer> hset = new HashSet<>();

        int count = Math.min(k, nums.length);
        for(int i = 0; i < count; i++) {
            if (hset.contains(nums[i])) {
                return true;
            }
            hset.add(nums[i]);
        }
        for(int i = k; i < nums.length; i++) {
            if(hset.contains(nums[i])) {
                return true;
            }
            hset.remove(nums[i-k]);
            hset.add(nums[i]);
        }
        return false;
    }
}