# ruststudy

Estudo de Rust do zero, estruturado em 7 dias, com foco em prática + entendimento do "porquê" por trás de cada conceito. Objetivo final não é virar expert em uma semana — é sair capaz de escrever código Rust simples por conta própria e com uma base real para depois seguir para a parte de criptografia (ecossistema RustCrypto e afins).

## Metodologia

Cada dia segue a mesma estrutura, pensada para quem aprende melhor fazendo e depois entendendo o que aconteceu:

1. **Gancho (o "porquê")** — por que esse conceito existe, que problema ele resolve em relação a outras linguagens.
2. **Mão na massa primeiro** — escrever e rodar código antes de ler teoria extensa. O objetivo é ver o compilador reclamar, ver o programa funcionar, sentir o problema na pele.
3. **Frame a frame** — depois que funcionar (ou quebrar), parar e entender linha a linha: por que o borrow checker reclamou, por que esse erro apareceu, o que o compilador está realmente dizendo.
4. **Desafio** — uma extensão do exercício para fazer sem ajuda, fixando o conceito.
5. **Notas** — registrar em `notes/dayXX.md` o que foi entendido, o que confundiu, e perguntas abertas.

## Estrutura do repositório

```
notes/              anotações e reflexões de cada dia
exercises/           workspace cargo com um crate por dia
  day01-basics/
  day02-ownership/
  day03-structs-enums/
  day04-collections-iterators/
  day05-traits-generics/
  day06-modules-testing/
  day07-crypto-intro/
```

Cada crate em `exercises/` tem um `main.rs` com TODOs e comentários guiando o exercício do dia — a ideia é editar esses arquivos diretamente, não criar do zero.

## Plano da semana

| Dia | Tema | Projeto prático |
|---|---|---|
| [1](notes/day-01.md) | Toolchain, variáveis, tipos, funções, controle de fluxo | Conversor de temperatura / calculadora de linha de comando |
| [2](notes/day-02.md) | Ownership, borrowing, referências, slices | Ferramenta de contagem de palavras |
| [3](notes/day-03.md) | Structs, enums, pattern matching, `Option`/`Result` | Gerenciador simples de contatos/inventário |
| [4](notes/day-04.md) | Collections (`Vec`, `HashMap`), iterators, closures | Analisador de CSV com estatísticas |
| [5](notes/day-05.md) | Traits, generics, lifetimes básicos | Estrutura de dados genérica (pilha/fila) com comportamento via trait |
| [6](notes/day-06.md) | Módulos, testes, cargo ecosystem, tratamento de erro idiomático | Refatorar os dias anteriores em lib + bin com testes |
| [7](notes/day-07.md) | Introdução a Rust para criptografia: bytes, hex, hashing | CLI que calcula hash de arquivo + cifra XOR de brinquedo |

## Como rodar um exercício

```bash
cd exercises
cargo run -p day01-basics
cargo test -p day06-modules-testing
```

## Próximos passos (depois da semana 1)

- Explorar o ecossistema [RustCrypto](https://github.com/RustCrypto) (hashes, ciphers, MACs, curvas elípticas).
- Ler sobre `unsafe`, FFI, e como Rust garante segurança de memória sem GC — base para entender por que a comunidade de criptografia confia na linguagem.
- Considerar um projeto maior: implementar um algoritmo clássico (ex: Caesar, RC4 educacional) do zero, sem usar crates, só para entender o mecanismo antes de usar bibliotecas prontas.
