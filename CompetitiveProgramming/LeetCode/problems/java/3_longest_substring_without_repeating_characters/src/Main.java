public class Main {
    public static void main(String[] args) {
        String s = "abcabcbb";
        System.out.println(lengthOfLongestSubstring(s));
    }

    public static int lengthOfLongestSubstring(String s) {
        int maxLength = 0;
        int[] ascii_array = new int[128];
        int start = 0;
        for (int end = 0; end < s.length(); end++) {
            start = Math.max(start, ascii_array[s.charAt(end)]);
            maxLength = Math.max(maxLength, end - start + 1);
            ascii_array[s.charAt(end)] = end + 1;
        }
        return maxLength;
    }
}