fun main() {
    var number = 22

    val maxIntegerValue = Int.MAX_VALUE
    val minIntegerValue = Int.MIN_VALUE

    println("Int maximum value is: $maxIntegerValue")
    println("Int minimum value is: $minIntegerValue")

    number = 2147483647

    val myMaxByteValue: Byte = Byte.MAX_VALUE
    val myMinByteValue: Byte = Byte.MIN_VALUE

    println("Byte maximum value is: $myMaxByteValue")
    println("Byte maximum value is: $myMinByteValue")

    val myMaxShortValue: Short = Short.MAX_VALUE
    val myMinShortValue: Short = Short.MIN_VALUE

    println("Short maximum value is: $myMaxShortValue")
    println("Short maximum value is: $myMinShortValue")

}