
import java.util.Scanner;

public class square_root_of_sum {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int num1 = sc.nextInt();
        int num2 = sc.nextInt();

        double sqrtNum = Math.sqrt(num1 + num2);

        System.out.println(sqrtNum);

        sc.close();

    }
}
