# AzzyAI HG para Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

[![AzzyAI HG: IA de homúnculo e mercenário para Ragnarok Online](docs/hg-banner.svg)](#início-rápido)

* * *

Um cérebro mais esperto para o seu **homúnculo ou mercenário**. Ele decide
quem atacar, qual habilidade usar e quando, e você ajusta tudo numa janela ou
no navegador, sem editar código.

HG é uma versão comunitária baseada no [AzzyAI original](https://github.com/SpenceKonde/AzzyAI),
não um produto oficial da Gravity. Esta branch está em testes para RO LATAM
e servidores privados; o comportamento e as habilidades disponíveis dependem
do servidor.

> **[Abra a configuração no navegador](https://jknpj.github.io/AzzyAI-webgui/)** para configurar a IA, ou leia o **[manual do usuário](docs/MANUAL.pt-BR.md)** para o guia completo.

## Início rápido

Três passos: instalar, configurar e testar. Mude uma coisa por vez.

### 1. Instalar

1. **Faça backup de toda a pasta `AI` do jogo**, principalmente se já usa outra IA.
2. Baixe um pacote em [Releases](https://github.com/jknpj/AzzyAI/releases),
   quando disponível, ou o código da branch **LATAM-HG**.
3. Copie o **conteúdo da [pasta USER_AI deste projeto](USER_AI)** para
   `AI\USER_AI` dentro do jogo. Não crie `AI\USER_AI\USER_AI`.
4. No jogo, use **`/hoai`** até a mensagem confirmar a IA personalizada do
   homúnculo. Para mercenários, use **`/merai`**.
5. Invoque o homúnculo novamente ou reconecte para carregar os arquivos.
6. Configure o servidor e as habilidades; teste em um monstro fácil.

**Vai atualizar uma instalação existente?** Preserve configurações, táticas e
lista de amigos. Veja o [checklist de atualização](docs/MANUAL.pt-BR.md#atualizações-e-backups).

### 2. Configurar

| Ferramenta | Como abrir |
|---|---|
| GUI Windows | Execute [AzzyAIConfig.exe](USER_AI/AzzyAIConfig.exe) **na pasta `AI\USER_AI` do seu jogo**. |
| Web GUI | Abra [AzzyAI Web Configuration](https://jknpj.github.io/AzzyAI-webgui/), selecione **AzzyAI HG** e abra a pasta `USER_AI` do jogo. |

Use **Apply** para salvar. Se o navegador não gravar diretamente na pasta,
exporte/baixe a configuração e copie para o jogo. Depois recarregue a IA.
A web GUI oferece inglês, português brasileiro e espanhol; os nomes das opções
continuam em inglês. A GUI Windows usa rótulos em inglês; o manual traduzido
explica as principais opções.

**Confira a pasta que está editando:** o jogo não lê uma cópia solta na pasta
de downloads ou no código-fonte.

Primeiras configurações para conferir:

- Em `ServerType`, escolha **Official** (oficial) ou **Private** (privado).
- Ative somente as habilidades desejadas, nos níveis já aprendidos.
- Confira ataque e uso de área antes de mexer nas opções avançadas.
- Altere uma coisa por vez e recarregue a IA entre os testes.

### 3. Peça a uma IA para ajudar a ajustar

Não sabe por que uma habilidade nunca é usada, ou o que mudar? Peça a qualquer
chat de IA (ChatGPT, Copilot, Gemini, Claude...) que leia os seus arquivos e
explique. Não precisa instalar nada: abra um chat, anexe os arquivos e cole o
pedido.

<details>
<summary><b>Ver os arquivos para anexar e o pedido para colar</b></summary>

**Anexe** (da pasta `AI\USER_AI` do jogo, ou da pasta do jogo para os logs):

- `H_Config.lua`, as configurações do seu homúnculo. Se usar um preset por tipo, também `Types\<Nome>.lua`.
- `AAIStartH.txt`, o relatório de inicialização. Mostra se os dados de monstros foram carregados.
- `AAI_ERROR.log`, se existir.

**Não anexe** o `A_Friends.lua`, a pasta `data` nem nada com senhas ou dados da
conta. Se tiver dúvida, abra os arquivos num editor de texto antes.

**Cole isto** e preencha as três últimas linhas:

```text
Você está me ajudando a configurar a AzzyAI HG, uma IA de homúnculo para Ragnarok Online.
Leia primeiro os arquivos anexados e este manual:
https://github.com/jknpj/AzzyAI/blob/LATAM-HG/docs/MANUAL.pt-BR.md
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

</details>

A IA não vê o seu jogo nem executa a IA, então trate a resposta como uma
sugestão: confira a evidência que ela citar, mude uma coisa, teste num monstro
fácil e volte ao backup se piorar.

### Atualizar a HG

O aplicativo Windows tem um menu **Update**: **Update AI from git...** puxa o seu
clone local do git, ou baixa a branch `LATAM-HG` do GitHub (não precisa de git), e
copia sobre a sua instalação. Ele mantém suas configurações, táticas, lista de
amigos e opções extras, e guarda cada arquivo que substitui em
`backup\update-<hora>\`. Depois, recarregue a IA no jogo. Atualizando à mão?
Siga o [checklist de atualização](docs/MANUAL.pt-BR.md#atualizações-e-backups).

## Como a HG funciona

[![Como a AzzyAI HG decide o que fazer: um monstro aparece, a HG o consulta, decide e conjura dentro das regras do servidor](docs/hg-how-pt-BR.svg)](docs/hg-how-pt-BR.svg)

O homúnculo lê qual monstro está vendo e o consulta numa lista de cerca de 2.700
monstros. Suas configurações e táticas decidem então quem atacar e qual
habilidade usar, e os dados do servidor mantêm cada conjuração dentro do que o
seu servidor aceita. [Abrir a figura em tamanho grande](docs/hg-how-pt-BR.svg).

## O que você pode configurar

| Homúnculo | Habilidades que a HG pode usar | Bom saber |
|---|---|---|
| **Eira** | Xeno Slasher, Eraser Cutter, Silent Breeze, Twister Cutter, Absolute Zephyr, Overed Boost, Light of Regenerate | Light of Regenerate é opcional e vem desligada: exige intimidade Leal e reduz a intimidade a cada conjuração. |
| **Bayeri** | Stahl Horn, Heilige Stange, Stein Wand, Goldene Ferse, Angriffs Modus, Glanzen Spies, Heilige Pferd, Goldene Tone | O preset do Bayeri escolhe entre Stahl Horn, Spies e as habilidades em área pela DEF do alvo. |
| **Sera** | Needle of Paralyze, Poison Mist, Summon Legion, Pain Killer, Toxin of Mandara, Needle Stinger | Toxin of Mandara e Needle Stinger têm conjurações que podem ser interrompidas. |
| **Dieter** | Magma Flow, Granitic Armor, Lava Slide, Pyroclastic, Volcanic Ash, Blast Forge, Tempering | Os ataques dele são em área, então o preset dele ataca até um único monstro. Habilidades de fogo são puladas contra monstros de Fogo. |
| **Eleanor** | Style Change, Sonic Claw, Silvervein Rush, Midnight Frenzy, Tinder Breaker, CBC, EQC, Blazing And Furious, The One Fighter Rises | Sonic Claw exige o estilo luta; Tinder Breaker, CBC e EQC exigem o estilo agarrar. A IA troca de estilo por você. |
| **Lif, Amistr, Filir, Vanilmirth** | As habilidades deles, e as herdadas num Homúnculo S | A HG detecta sozinha o tipo base de um Homúnculo S. |
| **Mercenários** | Táticas, habilidades e kiting, como no AzzyAI original | Não usam o banco de monstros. |

As habilidades só funcionam se o seu servidor as oferecer e o homúnculo as tiver
aprendido. O [manual do usuário](docs/MANUAL.pt-BR.md#6-dicas-por-homúnculo-s) tem notas sobre cada um.

## Por que usar esta?

O AzzyAI original (a branch `master`) continua sendo uma boa IA. A HG mantém o
que ele faz e acrescenta o que está abaixo. É o que você notaria no jogo:

| | AzzyAI original | HG |
|---|---|---|
| **Que monstro é este?** | Sem dados de monstros. Você escreve uma linha de tática para cada um. | Conhece elemento, raça e se é chefe/MVP/planta de cada monstro (cerca de 2.700), então funciona bem em monstros que você nunca configurou. |
| **Habilidades x elemento** | Usa a habilidade mesmo num monstro que resiste a ela. | Pula a habilidade que daria menos da metade do dano (ajustável) e usa outro ataque. O Dieter não joga Lava Slide num monstro de Fogo. |
| **Habilidades em área** | Conta monstros, seja qual for o elemento. | Conta só os monstros que realmente sofreriam dano e mira neles, mesmo quando o da frente é imune. |
| **Plantas e cogumelos** | Adivinha as plantas pelo comportamento (`AutoDetectPlant`). | Sabe exatamente quais são plantas (recebem 1 de dano de tudo) e guarda o SP para os outros. |
| **MVPs** | Só o que você colocar nas táticas. | Por padrão não *começa* luta com MVP, mas ainda defende você. Opcional: atacar sempre os MVPs. |
| **Conjuração que vive sendo interrompida** | Limite fixo de tentativas (`SkillRetryLimit`), nada mais esperto. | Pausa a habilidade por alguns segundos depois de algumas interrupções e usa outra coisa. |
| **Níveis das habilidades** | A maioria das habilidades de Homúnculo S para no nível 5 na tabela dele. | Níveis máximos atuais (10 na maioria) e só conjura níveis que o homúnculo já aprendeu. |
| **Habilidades de Homúnculo S** | O básico de Eira, Bayeri, Sera, Dieter e Eleanor. | Também Twister Cutter, Absolute Zephyr, Toxin of Mandara, Needle Stinger, Glanzen Spies, Heilige Pferd, Goldene Tone, Blazing And Furious, The One Fighter Rises, Blast Forge e Tempering. |
| **Eleanor** | Troca de estilo e combos. | Também esferas pelas regras do servidor (quanto cada habilidade custa), tempo dos combos e as duas habilidades novas. |
| **Os números do seu servidor** | Um único conjunto de valores. | Dados oficiais (RO LATAM, Episódio 19) ou do rAthena, escolhidos com `ServerType`, para que cooldowns, alcances e custos batam com o seu servidor. |
| **Configuração** | Aplicativo Windows. | O aplicativo Windows e uma **página no navegador** em English, Português e Español, com descrições das habilidades, presets por homúnculo e sugestão de táticas de monstros por mapa. |
| **Atualizações** | Manuais. | O aplicativo Windows atualiza a IA e os dados de monstros para você, sem mexer nas suas configurações. |
| **Testes** | Sem suíte de testes offline. | Cada mudança roda contra um servidor simulado (dezenas de cenários) antes de ser publicada. |

As funções que reconhecem monstros são para **homúnculos**. Mercenários recebem
as ferramentas e as correções de bugs, mas não o banco de monstros.

### Vale a pena trocar?

**Combina com você se** joga com Eira, Bayeri, Sera, Dieter ou Eleanor, quer
ajustar o comportamento sem ler código, ou caça grupos de monstros e quer
habilidades em área bem usadas.

**Talvez não precise se** a sua IA atual já faz o que você quer, ou se o seu
servidor tem regras de habilidades diferentes e você prefere não testar.

**Experimentar é seguro:** faça backup da pasta `AI` antes. Voltar é só
restaurar o backup. Suas táticas e lista de amigos podem ser mantidas (veja o
[checklist de atualização](docs/MANUAL.pt-BR.md#atualizações-e-backups)).

### O que muda por padrão

Isto vem ligado desde o início; cada item pode ser desligado na ferramenta de
configuração.

- **Banco de monstros** (`UseMobInfo`): elemento, plantas e chefes são levados em conta.
- **MVPs** (`BossAggro`): o homúnculo não começa lutas com MVPs.
- **Lutas ordenadas** (`CommandedFightStay`): um monstro que você mandou atacar é combatido até morrer.

Extras opcionais, desligados por padrão: escolha de alvo mais esperta
(`TargetPriority`), manter distância pelo alcance real do monstro
(`KiteByRange`) e fugir quando o HP cai rápido (`UseDamageDanger`). Os testes
offline cobrem esses extras, mas eles ainda não foram verificados no jogo.

### Situação real

A HG é testada offline e num servidor rAthena local. Isso não garante o
funcionamento em todo cliente ou servidor oficial. Habilidades novas só
funcionam se o servidor as oferecer e o homúnculo as tiver aprendido. Ativar
uma habilidade não significa usá-la em todo ataque: alcance, SP, cooldown e
elemento também influenciam. Cooldowns do servidor, custos de SP e
interrupções de conjuração continuam valendo.

## Guias e ajuda

| Idioma | Manual do usuário |
|---|---|
| Português (BR) | [Instalação, configurações, homúnculos e problemas comuns](docs/MANUAL.pt-BR.md) |
| English | [Installation, settings, homunculi and troubleshooting](docs/MANUAL.md) |
| Español | [Instalación, ajustes, homúnculos y problemas frecuentes](docs/MANUAL.es.md) |

Para relatar um problema, use o [formulário de relato de bug](https://github.com/jknpj/AzzyAI/issues/new/choose)
e informe homúnculo, servidor/cliente, habilidades ativadas e o que aconteceu.
**Nunca compartilhe senhas, credenciais ou arquivos privados.**
Achou um problema de segurança? Não o publique: veja a [política de segurança](SECURITY.md) (em inglês).
Quer ajudar? Leia [como contribuir](CONTRIBUTING.md) (em inglês).

Para desenvolvedores e usuários avançados:
[notas de versão e referência técnica, em inglês](docs/RELEASE_NOTES.md).
A documentação antiga incluída no pacote é uma referência histórica, não o
manual atual da HG.

**Baixe somente das fontes oficiais** (este repositório e suas Releases, e a
página web indicada acima): um repositório com nome parecido ou um README
copiado não torna um download oficial.

## Créditos

Baseado no AzzyAI de Dr. Azzy / [SpenceKonde](https://github.com/SpenceKonde/AzzyAI),
com contribuições de [samufacanha2](https://github.com/samufacanha2/AzzyAI),
[Kisaro](https://github.com/Kisaro/AzzyAI) e
[vorpal1337](https://github.com/vorpal1337). Site original: [drazzy.com](http://drazzy.com/ro/ai).

## Licença

O AzzyAI original não tem licença do autor, então o código que a HG herdou dele
também não tem. O que é novo na HG (a harness de testes, as ferramentas, os
manuais, os dados de monstros e os presets por tipo) está sob a **GNU General
Public License v3.0**. O [NOTICE.md](NOTICE.md) (em inglês) lista exatamente
quais arquivos são quais, e o [LICENSES/GPL-3.0.txt](LICENSES/GPL-3.0.txt) tem o
texto da licença. A ferramenta de configuração no navegador tem a sua própria
licença GPL-3.0. Ragnarok Online pertence à Gravity Co., Ltd.; este projeto não
é afiliado a ela.
