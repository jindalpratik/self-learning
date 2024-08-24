
import java.util.Scanner;

public class prime_no {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int num = sc.nextInt();

        sc.close();
        boolean flag = true;

        if (num == 0 || num == 1) {
            flag = false;
        } else if (num > 2) {
            for (int i = 2; i < (num / 2) + 1; i++) {
                if (num % i == 0) {
                    flag = false;
                    break;
                }
            }
        }

        if (flag) {
            System.out.println("The number is prime.");
        } else {
            System.out.println("The number is not prime.");
        }
    }
}
