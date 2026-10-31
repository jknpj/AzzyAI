# AzzyAI HG para Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

Configure como seu **homúnculo ou mercenário** segue você, ataca monstros e
usa habilidades, sem precisar editar código.

HG é uma versão comunitária baseada no [AzzyAI original](https://github.com/SpenceKonde/AzzyAI),
não um produto oficial da Gravity. Esta branch está em testes para RO LATAM
e servidores privados; o comportamento e as habilidades disponíveis dependem
do servidor. Respeite as regras do seu servidor sobre IA personalizada.

## O que ela faz?

- Configura habilidades e buffs de Eira, Bayeri, Sera, Dieter e Eleanor.
- Define comportamentos diferentes para cada monstro.
- Permite escolher quando atacar, seguir, manter distância ou usar área.
- Tenta outras opções por um tempo quando uma conjuração ofensiva é interrompida repetidamente.
- Oferece configuração por **aplicativo Windows** ou **navegador**.

Habilidades novas só funcionam se o servidor as oferecer e o homúnculo as
tiver aprendido. Ativar uma habilidade não significa usá-la em todo ataque:
alcance, SP, cooldown, elemento e outras configurações também influenciam.

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

Cooldowns do servidor, custos de SP, interrupções e requisitos das habilidades
continuam valendo. HG não remove essas regras.

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
