
import java.util.*;

public class first_n_prime {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        System.out.print("Enter the number of primes number to be printed: ");
        int num = sc.nextInt();
        int c = 2;
        int i = 0;

        while(i < num) {
            if (isPrime(c)) {
                System.out.println(c);
                i++;
            }
            c++;
        }

        sc.close();
    }

    public static boolean isPrime(int num) {
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

        return flag;
    }
}
