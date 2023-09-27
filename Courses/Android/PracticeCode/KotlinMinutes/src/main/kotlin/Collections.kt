fun main() {
    val names = listOf("Ali", "Maya", "Chen")
    println(names[2])
    val names2 = mutableListOf("Ali", "Maya", "Chen")
    names2.add("Dmytro")
    for (name in names2){
        println(name)
    }
    for (i in 1..5) {
        println(i)
    }
}