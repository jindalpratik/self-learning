fun main(args: Array<String>) {
    var x = 5
    val y = 3

    //here we are doing operations
    /*
    Block comments
     */
    println("x + y = ${x + y}")
    println("x - y = ${x - y}")
    println("x * y = ${x * y}")
    println("x / y = ${x / y}")
    println("x % y = ${x % y}")

    var result = x + y

    result += 2
    println("result = $result")

    result -= 2
    println("result = $result")

    result *= 2
    println("result = $result")

    result /= 2
    println("result = $result")

    result %= 2
    println("result = $result")

    println("3 + 3 * 4 = ${(3 + 3) * 4}")

    x = 0
    println("x = ${x++}")
    println("x = ${++x}")

    println("x = ${x--}")
    println("x = ${--x}")

    val myNumber = 100
    if (myNumber != 150) {
        println("They are not equal")
    }else if (myNumber <= 150) {
        println("Less than or equal to 150")
    }else {
        println("All the conditions failed.")
    }

    val isActive = false
    if (isActive) {
        println("The user is active.")
    }else {
        println("The user is not active.")
    }

    val isPlaying = true
    val score = 100

    if (isPlaying && score == 100){
        println("Next level opened")
    }else {
        println("Still at the same level")
    }

    val num1 = 5
    val num2 = -3
    val text = if (num1 > 0 || num2 > 0)
        "This is text 1"
    else
        "This is text 2"
    println("$text")
}