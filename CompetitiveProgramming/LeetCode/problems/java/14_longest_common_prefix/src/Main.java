public class Main {
    public static void main(String[] args) {
        String[] strs = { "flower", "flow", "flight" };
        System.out.println(longestCommonPrefix(strs));
    }

    public static String longestCommonPrefix(String[] strs) {
        StringBuilder returnStr = new StringBuilder();
        int index = 0;
        while (true) {
            if (index >= strs[0].length()) {
                return returnStr.toString();
            }

            char curr = strs[0].charAt(index);

            for (int i = 1; i < strs.length; i++) {
                if (index >= strs[i].length() || curr != strs[i].charAt(index)) {
                    return returnStr.toString();
                }
            }

            returnStr.append(curr);
            index++;

        }
    }
}