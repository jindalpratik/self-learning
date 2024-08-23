
import java.util.Scanner;

public class conversation {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.println("Greetings! How are you doing?");

        String firString = scanner.nextLine();

        System.out.println(firString);

        System.out.println("Oh, how interesting. Tell me more!");

        String secondString = scanner.nextLine();

        System.out.println(secondString);

        System.out.println("Thanks for sharing!");

        scanner.close();
    }
}
