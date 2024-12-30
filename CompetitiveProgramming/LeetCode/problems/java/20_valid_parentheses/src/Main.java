import java.util.Stack;

public class Main {
    public static void main(String[] args) {
        String s = "((";
        System.out.println(isValid(s));
    }

    public static boolean isValid(String s) {
        Stack<Character> st = new Stack<>();

        for (char ch : s.toCharArray()) {
            if (ch == '(' || ch == '{' || ch == '[') {
                st.push(ch);
            } else if (ch == ')') {
                if (!st.empty() && st.peek() == '(') {
                    st.pop();
                } else {
                    return false;
                }
            } else if (ch == '}') {
                if (!st.empty() && st.peek() == '{') {
                    st.pop();
                } else {
                    return false;
                }
            } else if (ch == ']') {
                if (!st.empty() && st.peek() == '[') {
                    st.pop();
                } else {
                    return false;
                }
            }
        }
        return st.empty();
    }
}