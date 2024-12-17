import java.util.HashSet;

public class Main {
    public static void main(String[] args) {
        int[] arr = { 1, 2, 3, 1 };
        System.out.println(containsDuplicate(arr));
    }

    public static boolean containsDuplicate(int[] nums) {
        HashSet<Integer> st = new HashSet<>();
        for (int num : nums) {
            st.add(num);
        }
        return nums.length != st.size();
    }

    public static boolean containsDuplicateAlternative(int[] nums) {
        HashSet<Integer> st = new HashSet<>();
        for (int i = 0; i < nums.length; i++) {
            if (st.contains(nums[i])) {
                return true;
            }
            st.add(nums[i]);
        }
        return false;
    }
}