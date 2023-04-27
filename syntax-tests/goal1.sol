
fun main() =
    let n = readline,
        g = greet n
        in
            println g

fun greet(name: String) -> String = "Hello" ++ name