import java.util.Scanner;

public class programme_execution_waits_for_input {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.println("Write the first string:");
        String first = scanner.nextLine();
        System.out.println("Write the second string:");
        String second = scanner.nextLine();
        System.out.println("Write the third string:");
        String third = scanner.nextLine();

        System.out.println("You wrote:");
        System.out.println(first);
        System.out.println(second);
        System.out.println(third);

        scanner.close();
    }
}
