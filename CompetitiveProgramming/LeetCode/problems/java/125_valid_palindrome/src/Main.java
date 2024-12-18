public class Main {
    public static void main(String[] args) {
        String s = new String("A man, a plan, a canal: Panama");
        System.out.println(isPalindrome(s));
    }

    public static boolean isPalindrome(String s) {
        int i = 0;
        int j = s.length() - 1;

        if (s.isEmpty()) {
            return true;
        }
        while (i <= j) {
            char curr = s.charAt(i);
            char curr_back = s.charAt(j);
            if (!Character.isLetterOrDigit(curr)) {
                i++;
            } else if (!Character.isLetterOrDigit(curr_back)) {
                j--;
            } else {
                if (Character.toLowerCase(curr) != Character.toLowerCase(curr_back)) {
                    return false;
                }
                i++;
                j--;
            }
        }

        return true;
    }
}