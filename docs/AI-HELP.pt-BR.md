# Peça a uma IA para ajudar a ajustar o AzzyAI

[English](AI-HELP.md) | [Português (BR)](AI-HELP.pt-BR.md) | [Español](AI-HELP.es.md)

[Voltar ao README](../README.pt-BR.md) | [Manual do usuário](MANUAL.pt-BR.md)

Não sabe por que uma habilidade nunca é usada, ou o que mudar? Peça a qualquer
chat de IA (ChatGPT, Copilot, Gemini, Claude...) que leia os seus arquivos e
explique. Não precisa instalar nada: abra um chat, anexe os arquivos e cole o
pedido abaixo.

## 1. Anexe estes arquivos

Da pasta `AI\USER_AI` do jogo, ou da pasta do jogo para os logs:

- `H_Config.lua`, as configurações do seu homúnculo. Se usar um preset por tipo, também `Types\<Nome>.lua`.
- `AAIStartH.txt`, o relatório de inicialização. Mostra o número da versão e se os dados de monstros foram carregados.
- `AAI_ERROR.log`, se existir.

**Não anexe** o `A_Friends.lua`, a pasta `data` nem nada com senhas ou dados da
conta. Se tiver dúvida, abra os arquivos num editor de texto antes.

## 2. Cole este pedido

Preencha as três últimas linhas:

```text
Você está me ajudando a configurar o AzzyAI, uma IA de homúnculo para Ragnarok Online
(uma continuação comunitária do AzzyAI original).
Leia primeiro os arquivos anexados e este manual:
https://github.com/jknpj/AzzyAI/blob/main/docs/MANUAL.pt-BR.md
(se não conseguir abrir o link, avise e eu colo o trecho relevante).

Regras para a sua resposta:
1. Use palavras simples. Não sou programador.
2. Para cada sugestão, informe: o nome exato da opção no meu H_Config.lua, o valor
   atual, o valor sugerido e o motivo, citando a linha ou a seção do manual que
   sustenta a sugestão.
3. Sugira só opções que existam nos meus arquivos ou no manual. Nunca invente uma.
4. Diga claramente o que você LEU nos meus arquivos e o que está SUPONDO.
5. Se não tiver como saber, diga isso e me faça UMA pergunta em vez de supor.
6. Sugira uma mudança por vez e me lembre de fazer backup da pasta AI e recarregar
   a IA no jogo antes de testar.

Meu homúnculo: <Eira / Bayeri / Sera / Dieter / Eleanor / outro>
Meu servidor: <oficial RO LATAM / privado>
O que está errado ou o que eu quero: <por exemplo: "o Dieter nunca usa Lava Slide em grupos">
```

## 3. Confira a resposta

A IA não vê o seu jogo nem executa a IA, e o pedido acima não foi testado com
todos os chats. Trate a resposta como uma sugestão: confira a evidência que ela
citar, mude uma coisa, teste num monstro fácil e volte ao backup se piorar.
