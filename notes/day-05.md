# Dia 5 — Traits, generics, lifetimes básicos

## Por quê

Traits são o equivalente de Rust a "interfaces" — definem comportamento que um tipo precisa implementar (ex: `Display`, `PartialEq`, ou um trait seu `Summable`). Generics (`fn largest<T: PartialOrd>(list: &[T]) -> &T`) permitem escrever código que funciona para qualquer tipo que satisfaça certos traits, sem duplicar lógica e sem o custo de runtime que linguagens com generics "apagados" (erasure) ou dynamic dispatch teriam por padrão.

Lifetimes (`'a`) são anotações que descrevem por quanto tempo uma referência é válida — o compilador já infere isso na maioria dos casos (como você viu no dia 2), mas quando há ambiguidade (ex: uma função que recebe duas referências e devolve uma delas), você precisa anotar explicitamente.

## Mão na massa

Em `exercises/day05-traits-generics/src/main.rs`:

1. Defina um trait `Summary { fn summarize(&self) -> String; }`.
2. Implemente `Summary` para duas structs diferentes (ex: `Article` e `Tweet`).
3. Escreva uma `struct Stack<T> { items: Vec<T> }` genérica com `push`, `pop` (devolvendo `Option<T>`), e `peek`.
4. Escreva uma função `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` que devolve a string mais longa entre duas.

## Frame a frame

- Por que `fn longest` precisa de anotação de lifetime explícita, mas `longest_word` do dia 2 não precisou?
- Qual a diferença entre `impl Trait` como parâmetro de função e generics com `<T: Trait>` — eles resolvem o mesmo problema?
- Por que um `Stack<T>` genérico é melhor que ter `StackI32`, `StackString`, etc. duplicados?

## Desafio

Adicione ao trait `Summary` um método com implementação default (`fn summarize(&self) -> String { String::from("(sem resumo)") }`) e crie uma terceira struct que não sobrescreve esse método — veja o comportamento.

## Minhas anotações

_(preencher depois de fazer o exercício)_
