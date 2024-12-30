import java.util.Stack;

public class Main {
    public static void main(String[] args) {
        String[] tokens = { "2", "1", "+", "3", "*" };
        System.out.println(evalRPN(tokens));
    }

    public static int evalRPN(String[] tokens) {
        Stack<Integer> st = new Stack<>();

        for (String token : tokens) {
            switch (token) {
                case "+": {
                    int val_2 = st.pop();
                    int val_1 = st.pop();
                    st.push(val_1 + val_2);
                    break;
                }
                case "-": {
                    int val_2 = st.pop();
                    int val_1 = st.pop();
                    st.push(val_1 - val_2);
                    break;
                }
                case "*": {
                    int val_2 = st.pop();
                    int val_1 = st.pop();
                    st.push(val_1 * val_2);
                    break;
                }
                case "/": {
                    int val_2 = st.pop();
                    int val_1 = st.pop();
                    st.push(val_1 / val_2);
                    break;
                }
                default: {
                    st.push(Integer.valueOf(token));
                }
            }
        }

        if (!st.empty()) {
            return st.pop();
        } else {
            return 0;
        }
    }
}