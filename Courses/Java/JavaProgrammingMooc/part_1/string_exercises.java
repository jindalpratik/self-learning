public class string_exercises {
    public static void main(String[] args) {
        String str = "Hello world";
        StringBuilder reversed = new StringBuilder(str);
        String newstr = reversed.reverse().toString();
        System.out.println(newstr);
    }
}
