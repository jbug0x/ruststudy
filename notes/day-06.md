# Dia 6 — Módulos, testes, cargo ecosystem, erro idiomático

## Por quê

Até aqui tudo morou em um `main.rs` só, de propósito, pra não distrair do conceito do dia. Na prática, código Rust real é organizado em módulos (`mod`), separado em lib (`lib.rs`) + bin (`main.rs`), e testado com `#[test]` + `cargo test` — sem framework externo, já vem na linguagem. Isso facilita reaproveitar código entre dias/exercícios e começar a pensar em "biblioteca" em vez de "script".

Tratamento de erro idiomático em Rust real normalmente usa o operador `?` para propagar erros (em vez de `match` em todo lugar) e, em projetos maiores, crates como `thiserror` (para definir erros customizados) e `anyhow` (para erros genéricos em binários). Hoje você vai sentir a diferença entre tratar erro manualmente (dia 3) e propagar com `?`.

## Mão na massa

Em `exercises/day06-modules-testing/`:

1. Separe o projeto em `src/lib.rs` (com a lógica) e `src/main.rs` (só chamando a lib).
2. Organize a lógica em pelo menos um módulo (`mod contacts;` ou similar), reaproveitando algo do dia 3 ou dia 4.
3. Escreva pelo menos 3 testes com `#[test]` cobrindo: caso de sucesso, caso de erro, e um caso de borda (lista vazia, string vazia, etc).
4. Reescreva uma função que antes usava `match` para tratar `Result` e troque por propagação com `?`.

## Frame a frame

- O que `cargo test` faz exatamente — como ele descobre quais funções são testes?
- Por que separar lib e bin facilita testar? (dica: pense em o que dá pra importar com `use`)
- O que o operador `?` faz por baixo dos panos — ele é equivalente a que `match`?

## Desafio

Adicione um teste que verifica que uma função retorna `Err` da variante certa (não só "deu erro"), usando `assert!(matches!(resultado, Err(MeuErro::Variante)))`.

## Minhas anotações

_(preencher depois de fazer o exercício)_
