fun main(){
    val instagramBio : String? = "growth"
    if (instagramBio != null){
        println(instagramBio.uppercase())
    }
    println(instagramBio?.uppercase())
}