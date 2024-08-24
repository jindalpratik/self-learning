import java.util.Arrays;
import java.util.Scanner;

public class kaprekars_constant {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        while(n != 6174) {
            n = kap(n);
            System.out.println(n);
        }
    }

    private static int kap(int n) {
        String numStr = String.format("%04d", n);
        char[] chararr = numStr.toCharArray();
        Arrays.sort(chararr);
        String ascendString = new String(chararr);
        String decendString = new StringBuilder(ascendString).reverse().toString();

        int ascendInt = Integer.parseInt(ascendString);
        int descndInt = Integer.parseInt(decendString);

        return descndInt - ascendInt;
    }
}
