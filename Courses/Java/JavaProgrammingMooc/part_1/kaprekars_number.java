
import java.util.*;

public class kaprekars_number {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        int num = sc.nextInt();

        int sq = num * num;

        int digs = 0;

        while (sq != 0) {
            sq /= 10;
            digs++;
        }

        sq = num * num;

        for (int i = 1; i < digs; i++) {
            int mult = (int) Math.pow(10, i);
            if (mult == sq) {
                continue;
            }
            int sum = sq / mult + sq % mult;
            if (sum == num) {
                System.out.println(sq / mult + " " + sq % mult);
            }
        }
    }
}
