import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] nums = { 1, 2, 3, 4 };
        System.out.println(Arrays.toString(productExceptSelf(nums)));
    }

    public static int[] productExceptSelf(int[] nums) {
        int[] ans = new int[nums.length];
        int prodR = 1;
        int prodL = 1;
        for (int i = 0; i < nums.length; i++) {
            ans[i] = prodR;
            prodR *= nums[i];
        }
        for (int i = (nums.length - 1); i >= 0; i--) {
            ans[i] *= prodL;
            prodL *= nums[i];
        }
        return ans;
    }
}