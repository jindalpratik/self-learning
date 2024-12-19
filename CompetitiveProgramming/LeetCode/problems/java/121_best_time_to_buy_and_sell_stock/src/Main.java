public class Main {
    public static void main(String[] args) {
        int[] prices = { 7, 1, 5, 3, 6, 4 };
        System.out.println(maxProfit(prices));
    }

    public static int maxProfit(int[] prices) {
        int buy = prices[0];
        int profit = 0;

        for (int i : prices) {
            if (i < buy) {
                buy = i;
            }

            profit = Math.max(profit, i - buy);
        }

        return profit;
    }
}