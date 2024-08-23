
public class precedence_and_parenthesis {

    public static void main(String[] args) {
        int calculationWithParentheses = (1 + 1) + 3 * (2 + 5);
        System.out.println(calculationWithParentheses); // prints 23

        int calculationWithoutParentheses = 1 + 1 + 3 * 2 + 5;
        System.out.println(calculationWithoutParentheses); // prints 13
    }
}
