import java.util.Scanner;

public class seconds_in_a_day {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        System.out.println("Give number of days to convert to seconds: ");
        int days = sc.nextInt();
        System.out.println(days * 24 * 60 * 60);
        sc.close();
    }
}
