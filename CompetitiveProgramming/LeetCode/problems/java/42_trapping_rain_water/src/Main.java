public class Main {
    public static void main(String[] args) {
        int[] height = { 0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1 };
        System.out.println(trap(height));
    }

    public static int trap(int[] height) {
        if (height.length < 3) {
            return 0;
        }

        int[] prefixMax = new int[height.length];
        int[] suffixmax = new int[height.length];
        int max = 0;

        for (int i = 0; i < height.length; i++) {
            prefixMax[i] = max;
            max = Math.max(max, height[i]);
        }

        max = 0;

        for (int i = height.length - 1; i >= 0; i--) {
            suffixmax[i] = max;
            max = Math.max(max, height[i]);
        }

        int water = 0;
        for (int i = 0; i < height.length - 1; i++) {
            int minimum = Math.min(prefixMax[i], suffixmax[i]);
            if (minimum <= height[i]) {
                continue;
            } else {
                water += minimum - height[i];
            }
        }

        return water;
    }
}