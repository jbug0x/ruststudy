# Dia 3 — Structs, enums, pattern matching, Option/Result

## Por quê

Rust não tem `null`. Em vez disso, qualquer valor que "pode não existir" é modelado explicitamente com `Option<T>` (`Some(valor)` ou `None`), e qualquer operação que "pode falhar" devolve `Result<T, E>` (`Ok(valor)` ou `Err(erro)`). O compilador obriga você a tratar os dois casos — não existe esquecer um `if (x != null)` como em outras linguagens. Isso elimina uma classe inteira de bugs em runtime (null pointer exception não existe em Rust seguro).

`enum` em Rust não é só uma lista de constantes como em C/Java — cada variante pode carregar dados diferentes. Combinado com `match` (pattern matching exaustivo), isso vira a ferramenta principal para modelar estado.

## Mão na massa

Em `exercises/day03-structs-enums/src/main.rs`:

1. Defina `struct Contact { name: String, phone: Option<String> }`.
2. Defina `enum ContactError { NotFound, InvalidName }`.
3. Implemente `fn find_contact(contacts: &[Contact], name: &str) -> Result<&Contact, ContactError>`.
4. No `main`, use `match` para tratar os dois casos de erro de forma diferente, e `if let Some(phone) = &contact.phone` para imprimir o telefone só se existir.

## Frame a frame

- Por que o compilador te obriga a tratar todas as variantes de um `match`? O que acontece se você tentar deixar uma de fora?
- Qual a diferença entre usar `match`, `if let` e `.unwrap()` para lidar com `Option`/`Result`? Quando cada um é apropriado (e por que `.unwrap()` é perigoso em código real)?
- `derive(Debug)` — o que esse atributo faz, e por que ele não vem "de graça" por padrão?

## Desafio

Adicione `enum ContactError { NotFound, InvalidName }` com uma variante extra `DuplicateName(String)` carregando o nome duplicado, e implemente `fn add_contact` que devolve esse erro se o nome já existir na lista.

## Minhas anotações

_(preencher depois de fazer o exercício)_
