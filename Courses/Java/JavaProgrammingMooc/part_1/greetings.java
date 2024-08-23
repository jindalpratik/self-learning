
import java.util.Scanner;

public class greetings {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.println("What is your name: ");

        String name = scanner.nextLine();

        System.out.println("Hi " + name);

        scanner.close();
    }
}
