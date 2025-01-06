import java.util.*;

public class Main {
    public static void main(String[] args) {
        System.out.println(generateParenthesis(1));
    }

    private static ArrayList<String> arstr;
    private static StringBuilder str;
    private static int start;
    private static int end;

    public static List<String> generateParenthesis(int n) {
        arstr = new ArrayList<>();
        str = new StringBuilder();
        start = n;
        end = n;
        recursive();
        return arstr;
    }

    public static void recursive() {
        if (start == 0 && end == 0) {
            arstr.add(str.toString());
            return;
        }

        if (start > 0) {
            str.append('(');
            start--;
            recursive();
            start++;
            str.deleteCharAt(str.length() - 1);
        }

        if (end > start) {
            str.append(')');
            end--;
            recursive();
            end++;
            str.deleteCharAt(str.length() - 1);
        }
    }
}