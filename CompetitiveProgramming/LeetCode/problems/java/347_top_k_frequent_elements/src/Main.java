import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;

public class Main {
    public static void main(String[] args) {
        System.out.println(Arrays.toString(topKFrequent(new int[]{1, 1, 1, 2, 2, 3}, 2)));
    }

    public static int[] topKFrequent(int[] nums, int k) {
        HashMap<Integer, Integer> map = new HashMap<>();
        for(int num: nums) {
            map.put(num, map.getOrDefault(num, 0) + 1);
        }

        PriorityQueue<Map.Entry<Integer, Integer>> pq = new PriorityQueue<>(
                (e1, e2) -> e2.getValue() - e1.getValue()
        );

        pq.addAll(map.entrySet());

/*
      The above is the same thing as the 3 lines below

        for(Map.Entry<Integer, Integer> entry : map.entrySet() ) {
            pq.add(entry);
        }
*/
        int[] topK = new int[k];

        for(int i = 0; i< k ; i++) {
            topK[i] = pq.poll().getKey();
        }

        return topK;
    }
}