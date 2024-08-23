import java.util.*;

public class leapYear {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int year = scanner.nextInt();

        if (year%100 == 0) {
            if (year%400 == 0) {
                System.out.println("This year is a leap year.");
            } else {
                System.out.println("This year is not a leap year");
            }
        } else if (year%4 == 0) {
            System.out.println("This year is a leap year.");
        } else {
            System.out.println("This year is not a leap year.");
        }

        scanner.close();
    }
}
