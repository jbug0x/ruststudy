fn main() {
    println!("Dia 4: collections, iterators, closures");

    let lines = vec!["Ana,28", "Beto,17", "Carla,35", "Davi,19"];

    // TODO 1: defina struct Person { name: String, age: u32 }
    // e parseie `lines` em Vec<Person> usando .map() e .collect()

    // TODO 2: use HashMap<u32, u32> para contar quantas pessoas tem
    // cada idade (ok se todas forem distintas aqui, o ponto e o padrao)

    // TODO 3: use .filter() + .sum() (ou .fold()) para somar as idades
    // de quem tem mais de 18 anos

    // Desafio: fn oldest(people: &[Person]) -> Option<&Person>
    // usando .max_by_key()

    let _ = &lines; // remova quando usar `lines` de verdade
}
