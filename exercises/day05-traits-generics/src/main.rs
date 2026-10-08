fn main() {
    println!("Dia 5: traits, generics, lifetimes basicos");

    // TODO 1: defina trait Summary { fn summarize(&self) -> String; }

    // TODO 2: implemente Summary para duas structs diferentes,
    // por exemplo Article { title: String, body: String }
    // e Tweet { user: String, content: String }

    // TODO 3: implemente uma Stack<T> generica:
    // struct Stack<T> { items: Vec<T> }
    // impl<T> Stack<T> {
    //     fn new() -> Self { ... }
    //     fn push(&mut self, item: T) { ... }
    //     fn pop(&mut self) -> Option<T> { ... }
    //     fn peek(&self) -> Option<&T> { ... }
    // }

    // TODO 4: implemente
    // fn longest<'a>(a: &'a str, b: &'a str) -> &'a str

    // Desafio: adicione um metodo default a Summary e crie uma terceira
    // struct que nao sobrescreve esse metodo.
}
