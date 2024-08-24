
import java.util.*;

public class array_3 {

    public static void main(String[] args) {

        int[] arr = {1, 2, 3, 4, 5, 6, -7, -8, -9};

        int[] temp = new int[arr.length];

        int j = 0;

        int k = arr.length - 1;

        for (int value : arr) {

            if (value % 2 != 0) {
                temp[j++] = value;
            } else {
                temp[k--] = value;
            }

        }

        System.out.println(j + " " + k);

        System.arraycopy(temp, 0, arr, 0, arr.length);

        System.out.println(Arrays.toString(arr));

        Arrays.sort(arr, 0, k + 1);

        Arrays.sort(arr, j + 1, arr.length);

        System.out.println(Arrays.toString(arr));

    }

}

