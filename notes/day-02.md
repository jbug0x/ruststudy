# Dia 2 — Ownership, borrowing, referências, slices

## Por quê

Esse é o conceito que diferencia Rust de tudo. Sem garbage collector e sem o programador gerenciar `malloc`/`free` manualmente, Rust usa um sistema de regras verificadas em tempo de compilação (o "borrow checker") para garantir que não existam use-after-free, double-free ou data races — sem custo em runtime. É também o motivo pelo qual a comunidade de criptografia confia em Rust para código sensível: menos classe de bugs de memória = menos vulnerabilidade.

A regra central: cada valor tem um único "dono" (owner). Quando o dono some de escopo, o valor é liberado. Referências (`&`) permitem "emprestar" o valor sem transferir a posse — mas só pode haver **ou** várias referências imutáveis **ou** uma mutável, nunca as duas ao mesmo tempo.

## Mão na massa

Em `exercises/day02-ownership/src/main.rs`:

1. Escreva uma função `word_count(text: &str) -> usize` que conta quantas palavras tem num texto, usando `.split_whitespace()`.
2. Escreva uma função `longest_word(text: &str) -> &str` que devolve a maior palavra (slice `&str`, sem clonar string).
3. Tente escrever uma versão que *move* a `String` em vez de passar referência, e veja o erro de compilação ao tentar usar a string de novo depois.

## Frame a frame

- Reproduza de propósito um erro "borrow of moved value" e cole o erro nas suas anotações. O que a mensagem do compilador está te dizendo exatamente?
- Por que `longest_word` pode devolver um `&str` que "aponta" para dentro do argumento, sem copiar nada? O que isso tem a ver com lifetimes (mesmo sem anotar lifetime explicitamente ainda)?
- Qual a diferença prática entre `String` e `&str`?

## Desafio

Escreva `fn reverse_words(text: &str) -> String` que devolve as palavras na ordem reversa (sem inverter os caracteres de cada palavra). Pense em por que essa função precisa devolver `String` (dono novo) e não `&str`.

## Minhas anotações

_(preencher depois de fazer o exercício)_
