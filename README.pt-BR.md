# AzzyAI HG para Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

Um cérebro mais esperto para o seu **homúnculo ou mercenário**. Ele decide
quem atacar, qual habilidade usar e quando, e você ajusta tudo numa janela ou
no navegador, sem editar código.

HG é uma versão comunitária baseada no [AzzyAI original](https://github.com/SpenceKonde/AzzyAI),
não um produto oficial da Gravity. Esta branch está em testes para RO LATAM
e servidores privados; o comportamento e as habilidades disponíveis dependem
do servidor. Respeite as regras do seu servidor sobre IA personalizada.

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

## Vale a pena trocar?

**Combina com você se** joga com Eira, Bayeri, Sera, Dieter ou Eleanor, quer
ajustar o comportamento sem ler código, ou caça grupos de monstros e quer
habilidades em área bem usadas.

**Talvez não precise se** a sua IA atual já faz o que você quer, ou se o seu
servidor tem regras de habilidades diferentes e você prefere não testar.

**Experimentar é seguro:** faça backup da pasta `AI` antes. Voltar é só
restaurar o backup. Suas táticas e lista de amigos podem ser mantidas (veja o
[checklist de atualização](docs/MANUAL.pt-BR.md#atualizações-e-backups)).

## O que muda por padrão

Isto vem ligado desde o início; cada item pode ser desligado na ferramenta de
configuração.

- **Banco de monstros** (`UseMobInfo`): elemento, plantas e chefes são levados em conta.
- **MVPs** (`BossAggro`): o homúnculo não começa lutas com MVPs.
- **Lutas ordenadas** (`CommandedFightStay`): um monstro que você mandou atacar é combatido até morrer.

Extras opcionais, desligados por padrão: escolha de alvo mais esperta
(`TargetPriority`), manter distância pelo alcance real do monstro
(`KiteByRange`) e fugir quando o HP cai rápido (`UseDamageDanger`). Os testes
offline cobrem esses extras, mas eles ainda não foram verificados no jogo.

## Situação real

A HG é testada offline e num servidor rAthena local. Isso não garante o
funcionamento em todo cliente ou servidor oficial. Habilidades novas só
funcionam se o servidor as oferecer e o homúnculo as tiver aprendido. Ativar
uma habilidade não significa usá-la em todo ataque: alcance, SP, cooldown e
elemento também influenciam. Cooldowns do servidor, custos de SP e
interrupções de conjuração continuam valendo.

## Instalação rápida

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

## Escolha como configurar

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

## Para começar

- Em `ServerType`, escolha **Official** (oficial) ou **Private** (privado).
- Ative somente as habilidades desejadas, nos níveis já aprendidos.
- Confira ataque e uso de área antes de mexer nas opções avançadas.
- Altere uma coisa por vez e recarregue a IA entre os testes.

## Guias e ajuda

| Idioma | Manual do usuário |
|---|---|
| Português (BR) | [Instalação, configurações, homúnculos e problemas comuns](docs/MANUAL.pt-BR.md) |
| English | [Installation, settings, homunculi and troubleshooting](docs/MANUAL.md) |
| Español | [Instalación, ajustes, homúnculos y problemas frecuentes](docs/MANUAL.es.md) |

Ao relatar um problema, informe homúnculo, servidor/cliente, habilidades
ativadas e o que aconteceu. **Nunca compartilhe senhas, credenciais ou arquivos
privados.** Use [Issues](https://github.com/jknpj/AzzyAI/issues).

Para desenvolvedores e usuários avançados:
[notas de versão e referência técnica, em inglês](docs/RELEASE_NOTES.md).
A documentação antiga incluída no pacote é uma referência histórica, não o
manual atual da HG.

## Créditos

Baseado no AzzyAI de Dr. Azzy / [SpenceKonde](https://github.com/SpenceKonde/AzzyAI),
com contribuições de [samufacanha2](https://github.com/samufacanha2/AzzyAI),
[Kisaro](https://github.com/Kisaro/AzzyAI) e
[vorpal1337](https://github.com/vorpal1337). Site original: [drazzy.com](http://drazzy.com/ro/ai).
