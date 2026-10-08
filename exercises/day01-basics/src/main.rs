fn main() {
    println!("Dia 1: variaveis, tipos, funcoes, controle de fluxo");

    // TODO 1: implemente um conversor de temperatura Celsius -> Fahrenheit.
    // Formula: F = C * 9.0 / 5.0 + 32.0
    let celsius: f64 = 25.0;
    println!("{celsius}C = {}F", celsius_to_fahrenheit(celsius));

    // TODO 2: implemente is_even(n: i32) -> bool e use um for para
    // imprimir se cada numero de [1, 2, 3, 4, 5] e par ou impar.
    let n: [i32; 5] = [1, 2, 3, 4, 5];
    for x in n {
        if is_even(x) {
            println!("{x} é par");
        } else {
            println!("{x} é impar");
        }
    }

    // Desafio: implemente is_prime(n: i32) -> bool e teste de 1 a 20.
    for n in 1..=20 {
        println!("{} é primo: {}", n, is_prime(n));
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
