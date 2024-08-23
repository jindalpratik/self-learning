
import java.util.Scanner;

public class reading_booleans {

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.println("Write a boolean ");
        boolean value = Boolean.valueOf(scanner.nextLine());
        System.out.println("You wrote " + value);
        scanner.close();
    }
}
