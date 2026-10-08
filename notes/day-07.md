# Dia 7 — Rust para criptografia: bytes, hex, hashing

## Por quê

Esse é o dia que conecta a semana com o motivo original de escolher Rust: segurança de memória sem GC é exatamente o motivo pelo qual o ecossistema [RustCrypto](https://github.com/RustCrypto) (usado por projetos como o Tor, entre outros) confia na linguagem para implementar primitivas criptográficas. Hoje você não vai implementar criptografia "de verdade" (isso é trabalho de especialista e não se deve reinventar em produção), mas vai entender a mecânica básica: como Rust representa bytes (`u8`, `&[u8]`), como converter para hexadecimal, e como usar uma crate de hash real (`sha2`) — além de montar uma cifra de brinquedo (XOR) só para entender o mecanismo, nunca para uso real.

## Mão na massa

Em `exercises/day07-crypto-intro/src/main.rs` (já com `sha2` e `hex` configurados no `Cargo.toml`):

1. Leia um arquivo pequeno (crie um `.txt` de teste) como `Vec<u8>` com `std::fs::read`.
2. Calcule o SHA-256 desse conteúdo usando a crate `sha2`, e imprima o resultado em hexadecimal usando a crate `hex`.
3. Implemente `fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8>` que aplica XOR byte a byte entre os dados e uma chave repetida ciclicamente.
4. Prove pra você mesmo que XOR é reversível: aplique `xor_cipher` duas vezes com a mesma chave e confirme que volta ao original.

## Frame a frame

- Por que hash (SHA-256) é uma via de mão única (não dá pra "decifrar" um hash), enquanto XOR é reversível? Qual a diferença fundamental de propósito entre as duas coisas?
- Por que a cifra XOR de brinquedo de hoje é insegura para uso real (pense: o que acontece se a chave for menor que os dados, ou se um atacante souber parte do texto original)?
- O que `&[u8]` representa, e por que quase tudo em criptografia trabalha em termos de bytes em vez de `String`?

## Desafio

Pesquise rapidamente (sem precisar entender a matemática completa ainda) o que é uma MAC (Message Authentication Code) e por que hash simples (SHA-256 sozinho) não é suficiente pra autenticar uma mensagem — isso é a ponte natural pro próximo passo depois dessa semana.

## Minhas anotações

_(preencher depois de fazer o exercício)_

## Reflexão da semana

- O que fez mais sentido?
- O que ainda confunde (ownership? lifetimes? traits?)?
- Qual seria um bom próximo projeto de uma semana, agora puxando pra criptografia de verdade?
