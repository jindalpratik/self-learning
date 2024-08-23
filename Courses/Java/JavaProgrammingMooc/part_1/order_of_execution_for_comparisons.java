
public class order_of_execution_for_comparisons {

    public static void main(String[] args) {
        int number = 5;

        if (number == 0) {
            System.out.println("Number is equal to 0");
        } else if (number < 0) {
            System.out.println("The number is less than 0");
        } else if (number > 0) {
            System.out.println("The number is greater than 0");
        }
    }
}
