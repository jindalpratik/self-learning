/* *****************************************************************************
 *  Name:              Ada Lovelace
 *  Coursera User ID:  123456
 *  Last modified:     October 16, 1842
 **************************************************************************** */

import edu.princeton.cs.algs4.StdIn;
import edu.princeton.cs.algs4.StdOut;

public class HelloGoodbye {
    public static void main(String[] args) {
        String name1 = StdIn.readString();
        String name2 = StdIn.readString();
        StdOut.println("Hello " + name1 + " and " + name2 + ".");
        StdOut.println("Goodbye " + name2 + " and " + name1 + ".");
    }
}
