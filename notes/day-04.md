# Dia 4 — Collections, iterators, closures

## Por quê

`Vec<T>` e `HashMap<K, V>` são as coleções do dia a dia em Rust. O que torna Rust produtivo apesar de toda a rigidez do ownership é o sistema de iterators: em vez de loops manuais com índice, você encadeia operações (`.map()`, `.filter()`, `.fold()`, `.collect()`) de forma declarativa — parecido com programação funcional — mas sem custo de performance, porque o compilador otimiza tudo isso em tempo de compilação ("zero-cost abstractions").

Closures (`|x| x + 1`) são funções anônimas que podem capturar variáveis do ambiente — essenciais para usar com iterators.

## Mão na massa

Em `exercises/day04-collections-iterators/src/main.rs`:

1. Dado um `Vec<&str>` com linhas de um CSV simples (`"nome,idade"`), parseie cada linha em uma struct `Person { name: String, age: u32 }` usando `.map()` e `.collect()`.
2. Use um `HashMap<String, u32>` para contar quantas vezes cada idade aparece.
3. Use `.filter()` e `.sum()` (ou `.fold()`) para calcular a soma das idades de pessoas com mais de 18 anos.

## Frame a frame

- Por que `.collect()` precisa que você diga o tipo de destino (`Vec<Person>`, `HashMap<...>`), explicitamente ou por inferência do contexto?
- Qual a diferença entre um iterator "lazy" (não faz nada até você chamar `.collect()`/`.sum()`/etc.) e um loop `for` tradicional?
- O que uma closure captura por padrão: por referência ou por valor? Quando isso importa?

## Desafio

Escreva uma função que recebe o `Vec<Person>` e devolve a pessoa com a maior idade usando `.max_by_key()`, tratando o caso de lista vazia com `Option`.

## Minhas anotações

_(preencher depois de fazer o exercício)_
