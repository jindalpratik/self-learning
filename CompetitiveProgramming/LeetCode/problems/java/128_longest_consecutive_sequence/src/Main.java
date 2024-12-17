import java.util.HashSet;

public class Main {
    public static void main(String[] args) {
        int[] nums = { 100, 4, 200, 1, 3, 2 };
        System.out.println(longestConsecutive(nums));
    }

    public static int longestConsecutive(int[] nums) {
        HashSet<Integer> set = new HashSet<>();
        for (int i : nums) {
            set.add(i);
        }

        int maxConsecutive = 0;

        for (int i : set) {
            if (set.contains(i - 1)) {
                continue;
            }

            int current = i;
            int consecutive = 1;

            while (set.contains(current + 1)) {
                consecutive++;
                current++;
            }

            maxConsecutive = Math.max(maxConsecutive, consecutive);
        }
        return maxConsecutive;
    }
}