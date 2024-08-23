import java.util.Scanner;

public class input_string_as_a_part_of_output {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.println("Write soemthing: ");

        String message = scanner.nextLine();

        System.out.println("You wrote " + message);

        scanner.close();
    }
}
