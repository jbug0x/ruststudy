# Dia 1 — Toolchain, variáveis, tipos, funções, controle de fluxo

## Por quê

Rust é compilado e estaticamente tipado, mas com inferência de tipos — então não parece C em termos de verbosidade, mas também não deixa passar erro de tipo em runtime como Python/JS deixariam. `cargo` resolve build, dependências e testes em uma ferramenta só, diferente de C/C++ onde isso é espalhado em vários sistemas.

Mutabilidade é opt-in (`let` é imutável por padrão, `let mut` é explícito) — isso já é uma pista do tema central da linguagem (segurança de dados compartilhados), que vai aparecer com força no dia 2.

## Mão na massa

Em `exercises/day01-basics/src/main.rs`:

1. Rode `cargo run -p day01-basics` e veja o "Hello, world!" rodar.
2. Implemente um conversor de temperatura (Celsius <-> Fahrenheit) que lê um valor fixo no código (sem input de usuário ainda) e imprime o resultado.
3. Depois, implemente uma função `is_even(n: i32) -> bool` e use `if/else` para imprimir se uma lista de números fixa (`[1, 2, 3, 4, 5]`) é par ou ímpar, usando um loop `for`.

## Frame a frame

Depois de rodar, responda em `notes/day-01.md` (neste mesmo arquivo, abaixo):

- O que o compilador reclamou antes de compilar, e por quê?
- Por que `let` sem `mut` não deixa reatribuir? O que isso evita?
- Qual a diferença entre `i32`, `f64` e `usize` — por que `usize` aparece em índices de array/vetor?

## Desafio

Sem copiar o padrão anterior: escreva uma função que recebe um `i32` e devolve se é primo, usando só `for`, `if` e `%`. Teste com números de 1 a 20.

## Minhas anotações

_(preencher depois de fazer o exercício)_
