
import java.util.*;

public class comparing_strings {

    public static void main(String[] args) {
        Scanner reader = new Scanner(System.in);

        System.out.println("Enter a string: ");
        String input = reader.nextLine();

        if (input.equals("a string")) {
            System.out.println("Great you read the instructions correctly. ");
        } else {
            System.out.println("You didn't read the instructions. ");
        }

        System.out.println("Input two strings");
        String first = reader.nextLine();
        String second = reader.nextLine();

        if (first.equals(second)) {
            System.out.println("The strings were the same!");
        } else {
            System.out.println("The strings were different!");
        }

        if (first.equals("two strings")) {
            System.out.println("Clever!");
        }

        if (second.equals("two strings")) {
            System.out.println("Sneaky!");
        }

        reader.close();
    }
}
