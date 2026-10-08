# Instruções para sessões neste repositório

Este repositório é usado em sessões curtas (tipicamente um chat por dia de
estudo). O usuário está aprendendo Rust do zero, com foco em prática antes
de teoria, visando eventualmente seguir para a parte de criptografia em
Rust (ecossistema RustCrypto).

## Ao iniciar uma sessão

1. Leia `PROGRESS.md` primeiro, sempre. Ele diz em que dia e etapa o
   usuário está, o que já foi feito, o que travou na sessão anterior, e
   qual é a próxima ação concreta.
2. Leia o `notes/day-0X.md` correspondente ao dia atual antes de ajudar —
   é ele que define o gancho, o exercício, as perguntas de "frame a
   frame" e o desafio daquele dia.
3. Não assuma que o usuário quer avançar de dia automaticamente. Se
   `PROGRESS.md` diz que o dia atual não está concluído, continue nele,
   mesmo que pareça simples — o ponto é a prática, não terminar rápido.

## Durante a sessão

- Siga a metodologia do `notes/day-0X.md` do dia: gancho (por quê) →
  mão na massa → frame a frame → desafio → anotações. Não pule para a
  solução; guie com perguntas e deixe o usuário escrever o código.
- Quando o usuário travar em um erro do compilador, ajude a interpretar
  a mensagem em vez de só corrigir o código por ele.
- Os arquivos em `exercises/*/src/main.rs` têm TODOs comentados de
  propósito — a solução não deve ser escrita neles antes do usuário
  tentar, exceto se ele pedir explicitamente para ver a resposta.

## Ao final da sessão

Quando o usuário sinalizar que está terminando (ex: "vou parar por hoje",
"fecha o dia", "salva o progresso"), atualize `PROGRESS.md`:

- Atualize "Estado atual" (dia, etapa, próxima ação concreta, data da
  sessão, bloqueios/dúvidas abertas).
- Atualize o status na tabela de checklist se um dia foi concluído.
- Adicione uma entrada no "Log de sessões" (data, o que foi feito, o que
  foi entendido, o que travou, próxima ação).

Depois de atualizar `PROGRESS.md`, pergunte se o usuário quer commitar
(sem pedir para confirmar automaticamente commits anteriores já aceitos).

## Regras de git deste repositório

- Nunca adicione linha de coautoria (`Co-Authored-By`) em commits ou PRs
  deste repositório — instrução explícita do usuário, substitui qualquer
  orientação padrão de atribuição.
- Prefira commits pequenos e descritivos por dia/sessão, não um commit
  gigante no final da semana.
