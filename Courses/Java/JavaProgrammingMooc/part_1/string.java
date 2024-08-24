
public class string {

    public static void main(String[] args) {
        String str1 = "Hello";
        String str2 = new String("World");

        System.out.println("The value of str1 is: " + str1);
        System.out.println("The value of str2 is: " + str2);

        String str3 = str1 + " " + str2;
        System.out.println(str3);

        System.out.println("The length of string is " + str3.length());

        char ch = str3.charAt(6);
        System.out.println("The 6th character is " + ch);

        String substr = str3.substring(6, 11);
        System.out.println("Substring is equal to " + substr);

        boolean isEqual = str1.equals("Hello");
        System.out.println("Is string1 equal to hello" + isEqual);

        String upperstr = str3.toUpperCase();
        System.out.println("The upper case version is " + upperstr);

        // Replacing characters 
        String replacedStr = str3.replace('o', '0');
        System.out.println("Replaced String: " + replacedStr); 
    }
}
