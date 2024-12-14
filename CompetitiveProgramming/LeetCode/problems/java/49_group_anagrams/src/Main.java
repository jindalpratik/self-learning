import java.util.*;

public class Main {
    public static void main(String[] args) {
        String[] strs = {"eat","tea","tan","ate","nat","bat"};
        System.out.println(groupAnagrams(strs));
    }

    public static List<List<String>> groupAnagrams(String[] strs) {
        HashMap<String, ArrayList<String>> map = new HashMap<>();
        for(String i: strs) {
            char[] tempStr = i.toCharArray();
            Arrays.sort(tempStr);
            String sort = new String(tempStr);
            map.putIfAbsent(sort, new ArrayList<>());
            map.get(sort).add(i);
        }
        return new ArrayList<>(map.values());
    }
}