public class Main {
    public static void main(String[] args) {
        String s = "ABAB";
        System.out.println(characterReplacement(s, 2));
    }

    public static int characterReplacement(String s, int k) {
        int[] charArray = new int[26];
        int maxFreq = 0;
        int start = 0;
        int maxLen = 1;
        for (int end = 0; end < s.length(); end++) {
            char curr = s.charAt(end);
            charArray[curr - 65]++;
            maxFreq = Math.max(charArray[curr - 65], maxFreq);
            while (end - start - maxFreq >= k) {
                charArray[s.charAt(start) - 65]--;
                start++;
            }
            maxLen = Math.max(maxLen, end - start + 1);
        }
        return maxLen;
    }
}