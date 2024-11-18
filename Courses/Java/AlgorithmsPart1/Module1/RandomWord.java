/* *****************************************************************************
 *  Name:              Ada Lovelace
 *  Coursera User ID:  123456
 *  Last modified:     October 16, 1842
 **************************************************************************** */

import edu.princeton.cs.algs4.StdIn;
import edu.princeton.cs.algs4.StdOut;
import edu.princeton.cs.algs4.StdRandom;

public class RandomWord {
    public static void main(String[] args) {
        String word = "";
        double count = 1.0;
        while (!StdIn.isEmpty()) {
            String newWord = StdIn.readString();
            if (StdRandom.bernoulli(1 / count)) {
                word = newWord;
            }
            count++;
        }
        StdOut.println(word);
    }
}
