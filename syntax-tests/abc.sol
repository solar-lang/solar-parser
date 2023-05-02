

fun main() =
    let name = readline "Hello, what's your name?: ",
        greeting = greet name
    in
        println greeting

fun greeting(name: String) -> String = "hello " ++ &name ++ "\n"

# error: "name" of type String is not callable

