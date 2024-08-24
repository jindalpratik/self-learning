import java.util.*;

public class number_and_sum_of_numbers {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int sum = 0;

        while (true) { 
            int num = sc.nextInt();
            if (num == 0) {
                break;
            }
            sum += num;
        }

        System.out.println(sum);

        sc.close();
    }
}
