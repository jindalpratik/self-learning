
import java.util.Scanner;

public class palindrome {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        String str = sc.nextLine();

        String reversedStr = new StringBuilder(str).reverse().toString();

        if(str.equals(reversedStr)) {
            System.out.println("The string is palindrome.");
        } else {
            System.out.println("The string is not palindrome");
        }
    }
}
