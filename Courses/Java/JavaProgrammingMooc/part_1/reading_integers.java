import java.util.Scanner;

public class reading_integers {
    public static void main(String[] args) {
        String valusOfString = "42";
        int value = Integer.valueOf(valusOfString);

        System.out.println(value);

        Scanner scanner = new Scanner(System.in);

        System.out.println("Write a value");
        value = Integer.valueOf(scanner.nextLine());

        System.out.println("You wrote " + value);

        scanner.close();
    }
}
