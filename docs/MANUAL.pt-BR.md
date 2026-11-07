# Manual do usuário: AzzyAI

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Voltar ao README](../README.pt-BR.md) | [Notas técnicas em inglês](RELEASE_NOTES.md) | [Referência de comandos, opções e táticas](REFERENCE.pt-BR.md)

## 1. Antes de começar

Use a IA somente onde seu servidor permitir. O AzzyAI configura o comportamento:
não desbloqueia habilidades, remove cooldowns ou impede interrupções por dano.
Clientes e servidores podem se comportar de formas diferentes. Comece em
monstros fáceis.

Homúnculo e mercenário têm configurações separadas. Os nomes das opções ficam
em inglês para corresponder às ferramentas.

## 2. Instalar e ativar

1. Feche as janelas de configuração e faça backup de toda a pasta `AI` do jogo.
2. Baixe um pacote de release, quando disponível, ou a branch **LATAM-HG**.
3. Coloque o conteúdo de `USER_AI` do projeto em `AI\USER_AI` do jogo.
   O resultado deve conter `AI\USER_AI\AI.lua`, sem outra pasta aninhada.
4. Digite `/hoai` até selecionar a IA personalizada do homúnculo.
   Para mercenários, use `/merai`.
5. Invoque novamente ou reconecte. Repita a recarga após salvar configurações.

Instalar o AzzyAI substitui a IA daquela pasta. Para manter outra IA em um dos tipos
de unidade, preserve o arquivo de entrada correspondente: `AI.lua` para
homúnculos ou `AI_M.lua` para mercenários. Não misture versões dos arquivos
de suporte do AzzyAI.

### Atualizações e backups

Antes de atualizar, guarde uma cópia de:

| Arquivos | Conteúdo |
|---|---|
| `H_Config.lua`, `M_Config.lua` | Configurações do homúnculo e mercenário |
| `H_Tactics.lua`, `M_Tactics.lua` | Comportamento por monstro |
| `H_PVP_Tact.lua`, `M_PVP_Tact.lua` | Comportamento em PVP |
| `H_Extra.lua`, `M_Extra.lua` | Opções extras e código personalizado |
| `A_Friends.lua` e `data` | Lista de amigos e informações salvas pela IA |

Guarde o backup completo fora da pasta de IA do jogo.
Não sobrescreva arquivos personalizados sem conferir nem substitua o código
novo pela pasta antiga inteira. Compare/importe as configurações na instalação
atualizada, salve, recarregue e teste.

O GUI do Windows faz isso por você (menu **Update**, seção 3). Ele nunca
sobrescreve os arquivos da tabela acima (nem `Types\*.lua`, salvo se você marcar
a opção) e faz backup de tudo que substitui.

## 3. Configurar pelas GUIs

### Aplicativo Windows

Execute `AzzyAIConfig.exe` na pasta `AI\USER_AI` instalada no jogo. Ele edita
os arquivos ao lado do executável, mesmo aberto por atalho. Confira o caminho
da configuração antes de salvar.

- **Homunculus / Mercenary:** comportamento geral, habilidades e buffs.
- **Homunculus Tactics / Mercenary Tactics:** comportamento por monstro.
- **PVP Tactics / Merc PVP Tactics:** comportamento para jogadores.
- **Extra Options:** opções avançadas e Lua personalizado; não edite código se não souber.

A busca encontra nomes e descrições. Os filtros por tipo facilitam encontrar
opções; **esconder uma configuração não a desativa**. Use **Clear filters** para
ver tudo. **Apply** salva; **Revert** descarta alterações pendentes.
Os rótulos do aplicativo Windows estão em inglês.

**Menu Update (somente no aplicativo Windows):**

- **Update AI from git...** faz `git pull --ff-only` no seu clone local ou baixa
  o branch `LATAM-HG` do GitHub (sem precisar de git) e copia o `USER_AI` dele
  sobre a pasta onde o GUI roda. Suas configurações, tactics, extra e listas de
  avoid são mantidas; com *Also overwrite the homunculus presets* marcado, os
  `Types\*.lua` também são substituídos. O que for substituído vai para
  `backup\update-<hora>\`. Um `AzzyAIConfig.exe` mais novo é salvo como
  `.exe.new` e trocado quando você aceita reiniciar. Recarregue a IA no jogo depois.
- **Update monster/skill data from rAthena...** roda `tools\harness\rathena_sync.py`
  no seu clone (precisa de Python 3 e internet) e copia o `MobInfo.lua`
  regenerado para cá. O clone também recebe os fixtures do harness e o
  `webgui\mobdata.js`, que você commita se mantém o repositório.

A pasta do clone fica salva em `AzzyAIConfig.update.txt`.

### Navegador

Abra a [web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Escolha **AzzyAI** (não o Classic), o idioma e **File > Open USER_AI folder** para abrir a
pasta instalada no jogo.

- Chrome/Edge podem gravar diretamente quando o acesso à pasta é suportado e autorizado.
- Sem gravação direta, use o ZIP baixado/exportado e extraia os arquivos de
  configuração para `AI\USER_AI` do jogo.
- Sem abrir a pasta, a página não está lendo as configurações instaladas.
- Uma configuração lembrada pelo navegador pode não ser igual aos arquivos atuais.

Clique em **Apply**, confirme a gravação/exportação e recarregue a IA no jogo.
Alguns recursos do navegador precisam de conexão com a internet.

### Configurações por tipo de homúnculo

As duas GUIs têm o seletor **Configurações de** na aba Homúnculo. *Global* edita o `H_Config.lua`; escolher um tipo (Lif, Amistr, Filir, Vanilmirth, Eira, Bayeri, Sera, Dieter, Eleanor) edita o `Types/<Nome>.lua`, que guarda só os valores diferentes dos globais e é aplicado depois do `H_Config.lua` quando aquele homúnculo inicia. **Usar valores globais** remove os ajustes do tipo. O seletor também substitui os antigos filtros Homúnculo S / Homúnculo base: um Homúnculo S lista as habilidades próprias, um base lista só as suas, e *Global* lista tudo. Apagar o arquivo do tipo desativa o preset. O Dieter já vem com `AutoMobCount = 1`, pois todos os ataques dele são em área e o valor global 2 o deixava quase parado contra um único monstro.

## 4. Comportamento básico

| Opção | Significado prático |
|---|---|
| `ServerType` | **Official** para oficiais; **Private** para rAthena/Hercules. |
| `UseAttackSkill` | Permite habilidades ofensivas automáticas. |
| `UseHomunSSkillAttack` / `UseHomunSSkillChase` | Permite habilidades S ao atacar / perseguir. |
| `AggroHP` / `AggroSP` | Porcentagem de HP/SP necessária para procurar alvos. Aumentar pode reduzir a agressividade. |
| `SuperPassive` | Impede o combate normal; confira se ninguém está atacando. |
| `DoNotChase` | Impede perseguir alvos. Um alvo distante pode nunca entrar no alcance. |
| `KiteMonsters` | Permite manter distância quando as táticas também permitem. |
| `UseDamageDanger` | Desligado por padrão. Se o homúnculo perder `DamageDangerDrop`% (30) do HP máximo em 3 s com HP abaixo de `DamageDangerHP`% (60), ele kita mesmo com `KiteMonsters` desligado, e o Bayeri usa Stein Wand. Ele só enxerga o próprio HP, não os buffs dos inimigos. |
| `UseDanceAttack` | Ativa movimento durante ataques. Pode mudar o posicionamento; não remove cooldowns. |
| `CommandedFightStay` | Ligado por padrão. Um monstro que você mandou o homúnculo atacar é combatido até morrer ou até você dar outra ordem, mesmo que você se afaste. Use 0 para ele desistir e voltar quando o alvo sair dos limites de movimento ou o tempo limite de ataque passar. |

A detecção automática ajuda em alguns clientes privados, mas escolha o tipo
de servidor explicitamente ao investigar problemas. A detecção do tipo original
e do nível das habilidades ajuda com as herdadas, mas não substitui aprendê-las.

## 5. Habilidades, área e SP

Ative a habilidade desejada e use um nível não superior ao aprendido.
Uma habilidade pode não ser escolhida por:

- Falta de SP ou reserva configurada.
- Cooldown, alcance ou elemento do alvo.
- Requisitos de área ou táticas do monstro.
- Outra habilidade disponível com pontuação maior (veja abaixo).
- Cast, combo ou pausa temporária ainda em andamento.

Eira, Bayeri e Sera comparam suas habilidades de ataque em vez de seguir uma
lista fixa: cada ataque disponível recebe uma pontuação pelo multiplicador de
dano, o elemento do alvo (de `MobInfo.lua`), quantos monstros atinge, a DEF do
alvo nas habilidades que não a ignoram e o tempo de conjuração. Abaixo de 40% de
SP a pontuação usa dano por SP, então as habilidades mais baratas ganham. Por
exemplo, a Eira usa Absolute Zephyr ou Twister Cutter contra monstros fracos a
Vento e Eraser Cutter contra monstros de Vento. As demais habilidades mantêm a
ordem habitual.

`AutoMobMode` controla habilidades automáticas em área. `AutoMobCount` define
a quantidade necessária; **1 pode fazer habilidades de área competirem mesmo
contra um único alvo**. A eficácia do elemento também pode influenciar a
contagem. `AoEMaximizeTargets` ajuda a mirar a área; habilidades centradas no
homúnculo continuam exigindo monstros próximos dele.

`AttackSkillReserveSP` preserva SP em vez de gastar tudo no ataque.
`AutoSkillDelay` é uma espera extra da IA. **0 remove só essa espera**, não o
cooldown, cast ou interrupções do servidor. Menor nem sempre é melhor.
O preset incluído e os valores internos de reserva da IA podem diferir; vale o
valor do arquivo instalado.

## 6. Dicas por Homúnculo S

| Homúnculo | O que conferir |
|---|---|
| Eira | Ative os ataques desejados, como Twister Cutter ou Absolute Zephyr. Light of Regenerate é opcional e desligada por padrão: exige lealdade, reduz intimidade ao conjurar e sacrifica Eira para ressuscitar o dono. |
| Bayeri | **Glanzen Spies** usa um alvo; **Heilige Pferd** é centrada no Bayeri. O preset do Bayeri liga Pferd e Spies; Spies substitui Stahl Horn só quando pontua mais contra o alvo (principalmente com DEF alta); com exatamente dois monstros, Stange/Pferd só a partir de DEF 110. |
| Sera | Toxin of Mandara e Needle Stinger têm casts que podem ser interrompidos. Mantenha o alvo no alcance e confira as pausas de falha abaixo. Pain Killer e Summon Legion têm opções próprias. |
| Dieter | Ative ataques e buffs separadamente. Resistência ou imunidade ao fogo pode impedir a escolha de habilidades de fogo. |
| Eleanor | Sonic Claw e a sequência de luta exigem estilo de luta; Tinder Breaker/CBC/EQC exigem agarrar. Esferas, SP e tempo do combo importam. The One Fighter Rises e Blazing And Furious são habilidades em área: seguem `AutoMobMode`/`AutoMobCount` em vez de serem usadas contra um único alvo, exceto The One Fighter Rises contra um alvo isolado com DEF 220 ou mais. Com 3+ monstros ela pula as cadeias de Sonic Claw, e Blazing And Furious só é usada com 6+ esferas. |
| Vanilmirth | **Caprice** (herdada também pelos Homunculus S baseados em Vanilmirth) é controlada por `UseVanilCaprice` (ligada por padrão). Chaotic Blessing segue `UseAutoHeal`. Self Destruct nunca é usada automaticamente. |
São dicas de configuração, não um ranking ou uma build ideal. As regras do
servidor podem variar, inclusive alcance e forma de mirar habilidades.

O preset incluído usa `AutoMobCount` 2. O preset da Eira liga **Twister Cutter**
e **Absolute Zephyr**; a IA as usa quando pontuam mais que
Eraser Cutter ou Xeno Slasher. O preset da Sera liga **Needle Stinger** (mesma regra contra
Needle of Paralyze) e `AutoMobCount` 1; os presets por tipo também ligam
Spies do Bayeri e Blazing And Furious da Eleanor, e **Blast Forge** (Dieter) está
ligada. As comparações offline são simulações, não medições em um servidor oficial.
Todas as habilidades de ataque do Dieter são de área, então com
`AutoMobCount` 2 ele não ataca um único monstro com elas: coloque `AutoMobCount`
em 1 se você enfrenta principalmente um monstro por vez com o Dieter (Bayeri e
Sera perdem dano contra monstros isolados). Os detalhes estão nas
[notas técnicas](RELEASE_NOTES.md#default-preset-tuning) e nas
[tabelas de comparação](preset-tuning/README.md).

### Eleanor: combos ou somente Sonic Claw

Para combos, escolha **Always** ou **Tactics** em `AutoComboMode` e confira
`AutoComboSpheres`: é o limite de esferas para **iniciar** a sequência automática,
não uma exigência repetida em cada etapa. Um limite alto faz esperar mais.
As novas habilidades ofensivas podem competir com o combo.

Ritmo da sequência: `EleanorComboGap` (padrão 500 ms) é a espera antes de cada etapa seguinte, e `EleanorComboDance` (padrão 1) mantém o ataque com dança entre as habilidades do combo para cancelar parte da animação. Se habilidades forem recusadas, use 0 no segundo e cerca de 800 no primeiro. Ambos vão no `H_Extra.lua`.

Para um experimento ofensivo só com Sonic:

1. Ative `UseEleanorSonicClaw`.
2. Coloque `AutoComboMode` em **Never** e desligue `Berserk_ComboAlways`.
3. Desligue Tinder Breaker, CBC, EQC, Blazing And Furious e The One Fighter Rises.
4. Coloque Eleanor em **estilo de luta** e recarregue a IA após salvar.

Ataques normais e buffs habilitados continuam ativos. A IA não consegue ler o
estilo da Eleanor: após uma troca manual de estilo ela percebe quando um
iniciador de combo (Sonic Claw ou Tinder Breaker) é recusado e passa a seguir o
novo estilo; outra recusa de iniciador em menos de 5 segundos é tratada como
falta de esferas. No estilo de agarrar ela só ganha esferas ao ser atingida, então,
sem esferas para Tinder Breaker e sem levar golpes por 5 segundos, ela volta ao
estilo de luta; use `EleanorDoNotSwitchMode = 1` para manter o estilo escolhido.
Se jogo e IA ainda discordarem, recarregue e teste
novamente; os ícones de esferas não são necessariamente uma contagem exata
em tempo real.

`SphereTrackInterval` é uma opção avançada de estimativa (padrão **800 ms**),
não um ajuste de velocidade ou cooldown. Reduza somente após confirmar
ataques normais mais rápidos; um valor baixo demais pode causar recusas.
Uma opção em `H_Extra.lua` prevalece sobre o valor salvo pela GUI.

## 7. Interrupções repetidas de cast

Encontre estas opções em **AutoSkill Options**:

| Opção | Padrão | O que faz |
|---|---|---|
| `CastInterruptLimit` | 3 | Ignora temporariamente uma habilidade após esse número de falhas contadas. **0 desativa**. |
| `CastInterruptPause` | 5000 | Duração em milissegundos: **5000 = 5 segundos**. |
| `CastInterruptUnconfirmed` | Desligada | Conta também falhas sem cast/conclusão visível, com alvo vivo no alcance e SP suficiente. |

Normalmente só contam casts ofensivos observados e interrompidos com indício
de dano. Um cast concluído zera o contador daquela habilidade. Na pausa, outra
habilidade disponível ou ataques normais podem ser escolhidos.
Buffs, combos e pedidos manuais de skill não acionam essa pausa.

Use `CastInterruptUnconfirmed` somente se o cliente esconder casts e as falhas
repetidas não forem contadas. **Ela pode contar recusas do servidor, não só
interrupções.** Não use para mascarar SP, alcance ou habilidades mal configuradas.
Contadores e pausas reiniciam quando o jogo recarrega a IA.
Essas opções não tornam casts ininterruptíveis nem alteram regras das skills.

## 8. Táticas de monstros

Nas abas de táticas, escolha quais monstros ignorar, priorizar ou atacar e se
deve usar habilidades. Confira a linha padrão e uma eventual linha específica
para seu alvo.

A web GUI pode **Suggest tactics** por mapa ou monstro. Revise antes de aplicar:
a base de monstros pode não corresponder exatamente ao seu servidor.

### O que o banco de monstros muda (só homúnculo)

Esta versão traz uma lista com cerca de 2.700 monstros, com elemento, raça e se cada um
é chefe, MVP ou planta. O homúnculo lê qual monstro está vendo e usa essa lista,
então age de forma sensata até em monstros para os quais você nunca escreveu uma
linha de tática. Mercenários não usam isso.

| Opção | Padrão | Em palavras simples |
|---|---|---|
| `UseMobInfo` | Ligado | Chave geral. Desligada = comportamento antigo, e nenhuma das opções abaixo faz efeito. |
| `SkillElementMinRate` | 50 | A habilidade é pulada quando o elemento do monstro reduziria o dano dela abaixo dessa porcentagem, e a IA ataca de outro jeito. Monstros imunes são sempre pulados. Diminua o número para usar habilidades com mais liberdade; 0 pula só os imunes. |
| `PlantSkills` | Desligado | Plantas e cogumelos recebem 1 de dano de tudo, então nenhuma habilidade é gasta neles. Ligue para usar habilidades neles mesmo assim. |
| `BossAggro` | Reagir a MVPs | O homúnculo nunca *começa* uma luta com um MVP, mas ainda defende você e a si mesmo. `React to bosses` inclui os mini-chefes; `Tactics` deixa as suas táticas decidirem. |
| `AttackMVPs` | Desligado | Ataca sempre os MVPs. Vale mais que o `BossAggro`. |
| `AttackOnlyMVPs` | Desligado | `1` = os outros monstros só recebem reação, `2` = os outros monstros são ignorados. Útil para limpar uma instância. |
| `TargetPriority` | Desligado | Escolhe o próximo alvo por mais do que a distância. Entre monstros com a mesma prioridade de tática: primeiro os que atacam você ou um amigo, plantas por último, depois os monstros que suas habilidades conseguem ferir, depois o centro do maior grupo quando você tem habilidade em área e, por fim, o mais próximo. |
| `KiteByRange` | Desligado | Com kiting ligado, mantém distância pelo alcance real do monstro: um arqueiro faz o homúnculo recuar mais cedo e mais longe do que um monstro corpo a corpo. Não foge de um monstro cujo alcance chega ao `KiteBounds`, porque não haveria para onde ir. Só importa quando o kiting já funciona para você: `KiteMonsters` ligado, a tática de kite do monstro definida e `ForceKite` para homúnculos sem ataque à distância. |

A lista de monstros vem do rAthena (Renewal), então alguns monstros podem ser
diferentes no seu servidor. `TargetPriority` e `KiteByRange` são novas e ainda
não foram verificadas no jogo; teste primeiro em monstros fáceis.

Para um teste controlado em boneco, configure aquele monstro como **Ignore**
na escolha automática e mande o homúnculo atacar com **Alt + botão direito**.
Isso afeta todos os monstros com o mesmo ID de tática, não só um boneco.

## 9. Problemas comuns

| Sintoma | Confira primeiro |
|---|---|
| Não ataca nada | `/hoai`, pasta instalada, recarga, modo passivo, servidor, táticas e limites de HP/SP. |
| Ataque manual funciona, automático não | Agressividade, táticas do alvo, alcance e restrições de kill-steal. |
| Habilidade nova nunca aparece | Nível aprendido, ativação, SP, alcance, limite de área e habilidades concorrentes. |
| Cast começa sempre e falha | Interrupções por dano, alcance, SP e opções de pausa; nem toda falha é interrupção. |
| GUI não muda o jogo | Apply/exportação, pasta correta, recarga e opções em `H_Extra.lua`. |
| Eleanor usa a sequência errada | Estilo, combos e esferas; recarregue após mudanças. |

`AAIStartH.txt`, na pasta do cliente, registra o carregamento da IA do homúnculo.
`AAI_ERROR.log` pode mostrar erros e pausas temporárias de skills ofensivas.
Se o arquivo inicial não atualizar, confira permissões e ativação da IA.
Passos detalhados de diagnóstico estão nas
[notas técnicas](RELEASE_NOTES.md#troubleshooting-automatic-attacks).

Ao relatar, inclua servidor/cliente, homúnculo/tipo original, skills ativadas,
passos e trechos relevantes dos logs. Remova informações de conta, senhas e
dados privados. Nunca envie arquivos de credenciais.
Use [Issues](https://github.com/jknpj/AzzyAI/issues).
