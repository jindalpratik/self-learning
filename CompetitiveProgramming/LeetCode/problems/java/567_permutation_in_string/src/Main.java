import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        String s1 = "ab";
        String s2 = "eidbaooo";
        System.out.println(checkInclusion(s1, s2));
    }

    public static boolean checkInclusion(String s1, String s2) {
        if (s1.length() > s2.length()) {
            return false;
        }

        int[] freqMap = new int[26];
        for (char ch : s1.toCharArray()) {
            freqMap[ch - 97] += 1;
        }

        int len = s1.length();
        int l = 0;
        int r = len - 1;

        int[] stringFreq = new int[26];
        char[] s2Char = s2.toCharArray();
        for (int i = 0; i <= r; i++) {
            stringFreq[s2Char[i] - 97] += 1;
        }

        while (r < s2.length() - 1) {
            if (Arrays.equals(freqMap, stringFreq)) {
                return true;
            }
            stringFreq[s2Char[l] - 97] -= 1;
            l++;
            r++;
            stringFreq[s2Char[r] - 97] += 1;
        }
        return Arrays.equals(freqMap, stringFreq);
    }
}