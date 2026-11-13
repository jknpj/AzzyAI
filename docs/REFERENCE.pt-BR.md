# Referência do AzzyAI: comandos, opções e táticas

[English](REFERENCE.md) | **Português (Brasil)** | [Español](REFERENCE.es.md)

[Voltar ao manual](MANUAL.pt-BR.md) | [README](../README.pt-BR.md) | [Referência técnica em inglês](TECHNICAL.md)

> **Começando agora?** Use o [manual do usuário](MANUAL.pt-BR.md) para instalação, configuração
> e solução de problemas. Esta página é uma consulta detalhada de comandos, opções e táticas.

Esta referência atualiza o conteúdo do manual original do AzzyAI 1.50 para a versão 2.0.1:

- Os nomes, padrões e valores das opções foram conferidos em `USER_AI/Defaults.lua`,
  `USER_AI/H_Config.lua` e `USER_AI/Const_.lua`. Quando o manual original estava incorreto ou
  desatualizado, esta página segue o código (as diferenças estão na [seção 7](#7-what-changed-from-the-original-manual)).
- **Padrão** é o valor do `H_Config.lua` distribuído (homúnculo). `Defaults.lua` contém um
  segundo valor, interno, usado apenas quando a opção não existe no arquivo de configuração.
  Mercenários têm seu próprio `M_Config.lua`.
- **Novo** identifica uma opção que não existe no AzzyAI original.
- Seu `H_Extra.lua` / `M_Extra.lua` é carregado por último e sobrescreve o que a GUI salvou.
- Nada foi testado novamente dentro do jogo para esta página; o comportamento foi consultado
  no código e no manual original.

## Conteúdo

1. [Comandos no jogo](#1-commanding-in-game)
2. [Amigos](#2-friends)
3. [Opções](#3-options)
   - [Combate](#options-engaging)
   - [Movimento e comportamento ocioso](#options-movement)
   - [Habilidades ofensivas](#options-attack-skills)
   - [Buffs](#options-buffs)
   - [Outras habilidades](#options-other-skills)
   - [Modo Berserk](#options-berserk)
   - [Kiting](#options-kiting)
   - [Temporização e outras opções](#options-timing)
4. [Táticas](#4-tactics)
   - [Formato do arquivo](#tactics-format)
   - [Tática básica](#tactics-basic)
   - [Outros campos da tática](#tactics-fields)
5. [Configurações avançadas](#5-advanced-settings)
6. [Notas de instalação](#6-installation-notes-from-the-original)
7. [Diferenças em relação ao original](#7-what-changed-from-the-original-manual)
8. [Introdução à criação de scripts](#8-homunculus-scripting-primer)

<a id="1-commanding-in-game"></a>
## 1. Comandos no jogo

O cliente permite enviar alguns comandos à IA. A IA recebe comandos, nunca texto do chat,
portanto não é possível digitar um comando para ela.

| O que você faz | O que a IA faz | Opções relacionadas |
|---|---|---|
| Tecla de **espera**: **Alt+T** (homúnculo), **Ctrl+T** (mercenário) | Alterna o modo de espera: a unidade deixa de ser agressiva. Pressione novamente para sair. | `DefendStandby` (continua defendendo você em espera), `StickyStandby` (volta à espera após uma luta defensiva) |
| **Alt + clique direito** em uma célula | Move-se até lá. Também serve para [adicionar amigos](#2-friends). Um movimento para mais de 14 células de você é ignorado e registrado como erro. | `MoveSticky`, `MoveStickyFight` |
| **Alt + clique direito duplo** em um monstro | Interrompe o que estava fazendo e ataca esse monstro. | `CommandedFightStay` (**novo**), `UseBerserkAttack` |
| Usa uma habilidade **em um monstro** | Ataca o alvo, começando com essa habilidade. | `UseBerserkSkill` |
| Usa uma habilidade **em um jogador**, fora do PVP | Aproxima-se até ficar ao alcance e usa a habilidade nele. | |
| Usa uma habilidade sem alvo ou de chão | O cliente a executa sem avisar a IA, que pode perder o controle do intervalo e da duração dessa habilidade. | |
| **Senta** | Deixa de ser agressiva e fica ao seu lado. | `DoNotUseRest`, `RestXOff`, `RestYOff` |

O manual original cita Alt+R para espera em um trecho; a lista acima segue o `A_Friends.lua`
distribuído e a descrição de `DefendStandby`. Os atalhos podem ser alterados no cliente,
então confira os seus.

Com `CommandedFightStay` ativado (padrão), um monstro que você mandou atacar é perseguido e
combatido até morrer ou você dar outra ordem, mesmo que você se afaste. A desistência por
limites de movimento e `AttackTimeLimit` não se aplicam a esse alvo, e o limite de 14 células de
distância de você é ignorado para ele, então você pode mandá-lo lutar enquanto fica recuado.
Use 0 para restaurar o comportamento antigo.

`AllowSBR44` (padrão 0) faz a IA bloquear o uso manual de SBR44, para proteger contra cliques
acidentais. Ela não consegue fazer o mesmo com Self Destruct.

<a id="2-friends"></a>
## 2. Amigos

Um jogador adicionado como amigo é defendido como se fosse o dono, até ser removido. Adicionar
o mercenário ou homúnculo de outro jogador só funciona até ele relogar, teleportar ou invocá-lo
novamente, pois o ID muda.

Um amigo a mais de ~20 células (fora da tela) não é defendido: o cliente não informa atores tão
longe, então a IA não vê o que está atacando ele. O mesmo limite vale para os monstros que ela
consegue perseguir.

**Método de caminhada** (`MirAIFriending` = 1, o padrão):

1. Mande o homúnculo caminhar até a célula **imediatamente ao norte ou ao sul** do jogador.
2. Ele anda em **círculo** ao redor do alvo se o amigo foi adicionado, ou **vai e volta em
   linha reta** se o amigo foi removido.

**Método de espera** (`StandbyFriending` = 1, método antigo do AzzyAI): coloque a unidade em
espera; então **você** se senta uma célula **a oeste** da pessoa para adicioná-la, ou uma célula
**a leste** para removê-la.

**Manualmente:** `A_Friends.lua` guarda a lista, uma linha por jogador, usando o ID da conta
(obtido em um site como ROPD):

```lua
MyFriends[2322797]=1       -- um amigo
MyFriends[2322797]=KOS     -- listas PVP: NEUTRAL, KOS (atacar ao avistar), ENEMY (hostil, não atacado), ALLY (aliado)
```

Preserve `A_Friends.lua` ao atualizar; ele não é reconstruído.

| Opção | Padrão | O que faz |
|---|---|---|
| `AssumeHomun` | 1 | O homúnculo e seu mercenário tornam-se amigos automaticamente. Use 0 apenas se não quiser que um roube as mortes do outro. |
| `NewAutoFriend` | 1 | Adiciona automaticamente seu próprio mercenário e homúnculo como amigos. Em `H_Extra.lua`, use 0 se a outra unidade usa outra IA; depois invoque ambos e relogue ou teleporte para ativar isso. |
| `FriendAttack[...]` | veja [5](#5-advanced-settings) | Quais movimentos de um amigo (atacar, conjurar...) fazem a IA atacar o monstro que ele tem como alvo. |

<a id="3-options"></a>
## 3. Opções

Os nomes são os usados nos arquivos de configuração; as GUIs exibem os mesmos nomes e possuem
uma caixa de pesquisa.

<a id="options-engaging"></a>
### 3.1 Iniciar e manter combates

A unidade normalmente procura monstros permitidos pelas [táticas](#4-tactics), quando estão
perto de você.

| Opção | Padrão | O que faz |
|---|---|---|
| `AggroHP`, `AggroSP` | 0, 0 | A unidade só inicia combate enquanto a porcentagem de HP / SP está **acima** desses valores. 100 significa que nunca inicia uma luta e apenas defende. (O valor interno de reserva de `AggroHP` é 60.) |
| `StationaryAggroDist`, `MobileAggroDist` | 12, 12 | Distância de você, em células, dentro da qual procura monstros enquanto você está parado / em movimento. |
| `SuperPassive` | 0 | 1 = nunca luta sem uma ordem direta. |
| `DoNotChase` | 0 | 1 = nunca caminha até um monstro. Ainda ataca à distância com habilidades de alcance, e táticas `CHASE_ALWAYS` ignoram essa opção. |
| `DoNotAttackMoving` | 0 | 1 = ignora monstros em movimento (ajuda a evitar roubo de mortes e monstros rápidos; ruim para subir de nível AFK). |
| `OpportunisticTargeting` | 0 | 1 = durante a perseguição, reavalia os alvos e troca para um melhor, se aparecer. |
| `AttackTimeLimit` | 10000 | Milissegundos atacando antes de considerar o alvo inalcançável e abandoná-lo. 0 = nunca. Não se aplica a um alvo comandado (`CommandedFightStay`). |
| `RescueOwnerLowHP` | 0 | Altera o resgate de acordo com seu HP (veja [táticas de resgate](#rescue)). |
| `FleeHP` | 0 | Só faz kiting abaixo desta porcentagem de HP. 0 ignora a condição. |
| `UseDanceAttack` | 1 | Só homúnculo: faz uma “dança” durante ataques normais para atacar um pouco mais rápido. A dança impede a regeneração de SP durante a luta; com uma habilidade ofensiva, normalmente é melhor desativá-la. |
| `DanceMinSP` | 0 | SP mínimo para dançar. Negativo = porcentagem do SP máximo. |
| `TankMonsterLimit` | 4 | Máximo de monstros que tenta segurar para outro atacante com a tática `TACT_TANK`. |
| `AutoDetectPlant` | 1 | **Só mercenário.** Trata monstros que nunca viu mover ou atacar como plantas e usa `MyTact[11]` para eles. Use 0 contra monstros estacionários. (O homúnculo usa o banco de monstros.) |
| `AttackLastFullSP` | 0 | 1 = monstros `TACT_ATTACK_LAST` só são atacados com SP cheio. |
| `UseAvoid` | 0 | **Só homúnculo.** 1 = o cliente fecha quando aparece um monstro listado em `H_Avoid.lua` (por padrão, MVPs invocáveis e alguns monstros perigosos). Teste antes: em algumas versões do Windows, aparece uma janela de erro em vez de fechar. |
| `UseDamageDanger`, `DamageDangerDrop`, `DamageDangerHP` | 0, 30, 60 | **Novo.** Afasta-se após uma queda súbita de HP, mesmo com `KiteMonsters` desativado. Veja o [manual](MANUAL.pt-BR.md#4-comportamento-básico). |
| `UseMobInfo`, `SkillElementMinRate`, `PlantSkills`, `BossAggro`, `AttackMVPs`, `AttackOnlyMVPs`, `TargetPriority` | 1, 50, 0, 1, 0, 0, 0 | **Novo.** Opções do banco de monstros; explicadas no [manual](MANUAL.pt-BR.md#o-que-o-banco-de-monstros-muda-só-homúnculo). |

<a id="options-movement"></a>
### 3.2 Movimento e comportamento ocioso

| Opção | Padrão | O que faz |
|---|---|---|
| `FollowStayBack` | 2 | Quantas células fica atrás de você ao seguir. |
| `StationaryMoveBounds`, `MobileMoveBounds` | 14, 14 | Até que distância de você pode ir para reagir (com você parado / em movimento). Além disso, abandona tudo e volta. Ambos vêm em 14; diminua `MobileMoveBounds` se a unidade ficar para trás quando você corre. |
| `MoveSticky` | 0 | 1 = após uma ordem de movimento, fica em torno do destino até receber outra ordem de movimento, inclusive voltando a ele depois de uma luta. |
| `MoveStickyFight` | 0 | 1 = mesmo fixada dessa forma, continua lutando normalmente. |
| `StayAwayDistance` | 0 | **Novo.** Mantém aproximadamente esta distância de você enquanto ociosa (limitada pelos limites de movimento). 0 = desativado. Útil em instâncias nas quais o homúnculo segura os monstros. |
| `StayAwayHold` | 0 | **Novo.** 1 = fica em um ponto e volta a ele após as lutas, em vez de seguir você. |
| `UseIdleWalk` | 4 | Caminha enquanto ociosa, quando HP está acima de `AggroHP`, SP acima de `AggroSP` e acima de `IdleWalkSP` por cento. 0 desativado; 1 círculo ao seu redor; 2 cruz; 3 retângulo; 4 aleatório; 5 percorre `MyRoute` de ida e volta; 6 percorre `MyRoute` em ciclo. |
| `IdleWalkDistance` | 4 | Distância de você para caminhadas de 1 a 4. |
| `IdleWalkSP` | 80 | Porcentagem de SP que precisa superar para caminhar quando ociosa. |
| `RelativeRoute` | 1 | 1 = células de `MyRoute` são deslocamentos relativos a você; 0 = células absolutas do mapa. Veja [percurso de rota](#5-advanced-settings). |
| `DoNotUseRest` | 0 | 1 = não deixa de ser agressiva quando você senta. |
| `RestXOff`, `RestYOff` | 2, 0 | Deslocamento em relação a você no qual espera quando você senta (pode ser negativo). |
| `ChaseSPPause` | 0 | 1 = adia perseguir e seguir até logo após um pulso de regeneração de SP. A tática `CHASE_ALWAYS` ignora isso; `CHASE_CLEVER` força esse comportamento. |
| `ChaseSPPauseSP` | -60 | Só pausa abaixo deste SP. Negativo = porcentagem. |
| `ChaseSPPauseTime` | 3000 | Espera máxima, em ms, pelo próximo pulso de SP. |

<a id="options-attack-skills"></a>
### 3.3 Habilidades ofensivas

| Opção | Padrão | O que faz |
|---|---|---|
| `UseAttackSkill` | 1 | Habilidades ofensivas automáticas. |
| `UseSkillOnly` | -1 | **-1 (perseguir, recomendado)** usa habilidades ao perseguir e atacar. **0** só ao atacar. **1** ataca apenas com habilidades e não usa uma habilidade de longo alcance antes de chegar ao alcance corpo a corpo. |
| `UseHomunSSkillChase`, `UseHomunSSkillAttack` | 1, 1 | Permite habilidades de alvo único de Homúnculo S ao perseguir / atacar. Desative se muitas habilidades têm conjuração interrompível e você não consegue esquivar. |
| `AutoMobMode` | 2 | Habilidades de área: 0 desativado; 1 conta apenas monstros já atacando; 2 conta todos os monstros que a área atingiria. |
| `AutoMobCount` | 2 | Monstros (ponderados pela [tática Weight](#weight)) necessários para usar uma habilidade de área. 1 permite que uma habilidade de área concorra mesmo em alvo único. |
| `AoEFixedLevel` | 0 | 1 = sempre usa habilidades de área no nível definido na opção `...Level` da habilidade, ignorando o nível configurado nas táticas. |
| `AoEMaximizeTargets` | 0 | 1 = mira habilidades de área no ponto que atinge mais monstros, mesmo que não atinja o alvo atual. |
| `AoEReserveSP` | 0 | 1 = nunca gasta SP em outro ataque se isso deixar SP insuficiente para a habilidade de área. |
| `AttackSkillReserveSP` | 0 | SP em reserva: nenhuma habilidade ofensiva é usada se deixar menos que isso. Pode ser sobrescrito por monstro pela [tática SP](#sp-tactic). |
| `AutoSkillLimit` | 0 | Teto de habilidades ofensivas usadas contra um alvo, aplicado em cima da tática de habilidade do monstro (vale o menor dos dois números). 0 = sem limite. A contagem recomeça quando o alvo muda. |
| `AutoSkillDelay` | 200 (mercenário 400) | Espera adicional, em ms, entre habilidades, para evitar uso duplicado. 0 remove apenas essa espera, não a recarga nem o tempo de conjuração. |
| `CastTimeRatio` | .80 | Fração do tempo de conjuração que a IA espera, considerando suas reduções de conjuração. |
| `AutoComboMode` | 2 | Combos (Eleanor): 0 nunca; 1 apenas em monstros cuja classe de habilidade é de combo; 2 sempre. |
| `AutoComboSpheres` | 1 | Esferas necessárias para **iniciar** uma sequência automática. |
| `UseVanilCaprice` | 1 | Vanilmirth: usar Caprice. |
| `LintUserOptions` | 1 | **Novo.** Na inicialização, desativa opções de habilidades não aprendidas pelo homúnculo e reduz os níveis para o nível aprendido. |
| `TrustSkillProbe` | -1 | **Novo.** Define se confia na resposta do cliente sobre níveis de habilidades aprendidos. -1 = automático (não no servidor oficial, sim em servidores privados). |
| `ServerType`, `AutoDetectServerType` | oficial, 1 | **Novo.** Servidor oficial ou rAthena/Hercules. Veja o [manual](MANUAL.pt-BR.md#4-comportamento-básico). |

**Ativação e níveis de habilidades.** Cada habilidade tem `Use<Type><Skill>` (1 = ativada) e
`<Type><Skill>Level`. A IA não usa uma habilidade acima do nível aprendido: com `LintUserOptions`,
reduz os níveis configurados ao nível aprendido na inicialização.

| Homúnculo | Opções de habilidades ofensivas |
|---|---|
| Eira | `UseEiraXenoSlasher`, `UseEiraEraseCutter` (Eraser Cutter), `UseEiraTwisterCutter`, `UseEiraAbsoluteZephyr`, `UseEiraSilentBreeze` |
| Bayeri | `UseBayeriStahlHorn`, `UseBayeriHailegeStar`, `UseBayeriGlanzenSpies`, `UseBayeriHeiligePferd` |
| Sera | `UseSeraParalyze` (Needle of Paralyze), `UseSeraPoisonMist`, `UseSeraCallLegion`, `UseSeraToxinOfMandara`, `UseSeraNeedleStinger` |
| Dieter | `UseDieterLavaSlide`, `UseDieterVolcanicAsh`, `UseDieterAshAsAttack`, `UseDieterBlastForge` |
| Eleanor | `UseEleanorSonicClaw`, `EleanorSilverveinLevel`, `EleanorMidnightLevel`, `UseEleanorTinderBreaker`, `UseEleanorCBC`, `UseEleanorEQC`, `UseEleanorBlazingAndFurious`, `UseEleanorTheOneFighterRises` |

Silvervein Rush e Midnight Frenzy têm apenas nível, sem opção de ativação. Dicas para escolher
entre elas estão na [seção 6 do manual](MANUAL.pt-BR.md#6-dicas-por-homúnculo-s). `LavaSlideMode` e
`PoisonMistMode` (padrão 0) escolhem como usar essas duas habilidades: 0 usa como ataques; outro
valor mantém a habilidade ao seu redor como buff e indica quando usá-la (valores da seção 3.4).

<a id="options-buffs"></a>
### 3.4 Habilidades de buff

Uma opção de buff recebe um dos valores abaixo, que indicam **quando** a habilidade pode ser usada:

| Valor | Quando |
|---|---|
| -2 | Quando ociosa, se não houver mais nada a fazer |
| -1 | Ao perseguir (para não desperdiçar quando não há nada para matar) |
| 0 | Nunca |
| 1 | Quando ociosa |
| 2 | Ao perseguir ou atacar no modo Berserk |
| 3 | Assim que possível, interrompendo outras ações. O alvo atual só é abandonado para Painkiller e Provoke. |

Buffs de Homúnculo S não têm opção de nível, pois níveis mais altos são melhores. Homúnculos
básicos têm essa opção, pois níveis mais altos têm recargas maiores.

| Opção | Padrão | Controla |
|---|---|---|
| `UseOffensiveBuff` | 3 | Flitting, Mental Charge, Weapon Quicken, Bloodlust |
| `UseDefensiveBuff` | 3 | Amistr Bulwark, Accelerated Flight, Guard, Parrying, Urgent Escape |
| `UseAutoSight`, `UseAutoMag` | 1, 0 (interno) | **Só mercenário:** Sight (arqueiro nível 2), Magnificat (arqueiro nível 4) |
| `UseDieterGraniticArmor`, `UseDieterMagmaFlow`, `UseDieterPyroclastic` | 0, 3, 3 | A habilidade de Dieter com esse nome (`DieterPyroclasticLevel`, padrão 10) |
| `UseDieterTempering` | 3 | **Novo.** Tempering de Dieter |
| `UseEiraOveredBoost` | 0 | Overed Boost de Eira |
| `UseBayeriGoldenPherze`, `UseBayeriAngriffModus`, `UseBayeriGoldeneTone` | 0, 3, 3 | Golden Ferse, Angriff Modus e Goldene Tone de Bayeri |
| `UseProvokeOwner` | 0 | Provoke (mercenário) ou Pain Killer (Sera) em você |
| `DefensiveBuffOwnerMobbed` | 0 | Não usa em você com menos monstros que isso atacando você, exceto abaixo de 90% de HP (a tática Weight conta) |
| `UseSeraPainkiller` | 3 | Pain Killer de Sera |
| `UseSacrificeOwner` | 0 (interno) | **Só mercenário:** Sacrifice em você. Funciona com diferença de até 10 níveis do jogador e não é confiável devido a bugs do jogo. |
| `UseSmartBulwark` | 0 | Amistr: só usa Bulwark se restarem pelo menos 120 SP, para permitir Bloodlust depois. Ignorado abaixo de 160 SP máximo ou sem Bloodlust. |
| `AmiBulwarkLevel`, `FilirFlitLevel`, `FilirAccelLevel`, `LifEscapeLevel` | 5, 5, 5, 0 | Nível de Amistr Bulwark, Filir Flitting, Filir Accelerated Flight, Lif Urgent Escape. |
| `UseEiraLightOfRegene` | 0 | **Novo.** Eira se sacrifica para ressuscitar você. Veja o [manual](MANUAL.pt-BR.md#6-dicas-por-homúnculo-s). |

`LifEscapeLevel = 0` desativa Urgent Escape: a IA pula o uso em vez de enviar nível 0 ao servidor.
Isso é seguro, ao contrário do aviso do manual original. Use um nível aprendido de 1 a 5 para
ativá-la; `UseDefensiveBuff` determina quando usá-la.

**Cura** (Healing Hands de Lif, Chaotic Blessing de Vanilmirth; homúnculo):
`UseAutoHeal` = 0 desativado, 1 ativado, 2 só quando ocioso (inclusive entre mortes), 3 só quando
ocioso e sem mais nada a fazer. `HealOwnerHP` (50) é a porcentagem de HP abaixo da qual cura você
(Lif usa Healing Hands nível 5; Vanilmirth, Chaotic Blessing nível 3). `HealSelfHP` (50) é a
porcentagem de HP do próprio Vanilmirth para curar a si mesmo (nível 4; nível 5 quando ocioso).
Lif não consegue curar a si mesma.

<a id="options-other-skills"></a>
### 3.5 Outras habilidades

| Opção | Padrão | O que faz |
|---|---|---|
| `UseAutoPushback`, `AutoPushbackThreshold` | 0, 2 | **Só mercenário.** Empurra monstros para longe do mercenário com uma habilidade de empurrão (Arrow Repel, Skid Trap) quando chegam a essa distância. A tática Pushback também precisa permitir. |
| `UseCastleDefend`, `CastleDefendThreshold` | 0, 4 | Amistr: usa Castling quando pelo menos esta quantidade de monstros (Weight conta) ataca você, e menos atacam o Amistr. |
| `UseCastleRoute` | 0 | Amistr: usa Castling para mover você por uma rota absoluta (precisa de `UseIdleWalk` 5 ou 6 e `RelativeRoute` 0). |
| `UseBayeriSteinWand` | 1 | Bayeri: Stone Wall quando um grupo grande o suficiente ataca você ou ele (valores de buff, seção 3.4). |
| `UseSteinWandOwnerMob`, `UseSteinWandSelfMob` | 2, 2 | Monstros necessários atacando você / o Bayeri (Weight conta). |
| `BayeriSteinWandLevel` | 5 | Nível de Stone Wall. |
| `UseSteinWandTele`, `SteinWandTelePause` | 0, 3000 | Usa Stone Wall logo após você entrar no jogo ou chegar de um teleporte, e não repete. Combina com `DoNotChase` e a habilidade ofensiva de Bayeri; não combine com `UseBayeriSteinWand`. |

<a id="options-berserk"></a>
### 3.6 Modo Berserk

O modo Berserk faz a unidade lutar de forma mais agressiva após um comando ou contra um grupo
grande. Após um comando, termina quando o alvo morre. **A grafia funcional é `Berserk`** (o manual
original escreve “Berzerk”). O `M_Config.lua` distribuído usa esses nomes; um config que ainda
escreve as opções de mercenário como `Berzerk_*` é lido do mesmo jeito, e o log de inicialização
avisa.

| Opção | Padrão | O que faz |
|---|---|---|
| `UseBerserkSkill` | 0 | Entra em Berserk quando recebe ordem para usar uma habilidade ofensiva. |
| `UseBerserkAttack` | 0 | Entra em Berserk quando recebe ordem para atacar. |
| `UseBerserkMobbed` | 0 | 0 = desativado. Caso contrário, entra em Berserk enquanto esta quantidade ou mais de monstros luta contra você e seus amigos. |
| `Berserk_SkillAlways` | 0 | Sempre usa habilidades ofensivas em Berserk. |
| `Berserk_Dance` | 0 | Faz dança de ataque em Berserk (só homúnculo). |
| `Berserk_IgnoreMinSP` | 0 | Ignora `AttackSkillReserveSP` em Berserk. |
| `Berserk_ComboAlways` | 0 | Sempre permite combos em Berserk. |

<a id="options-kiting"></a>
### 3.7 Kiting

Kiting é afastar-se dos monstros enquanto os ataca, principalmente com mercenários arqueiros.
Os padrões foram escolhidos empiricamente: altere apenas com um motivo.

| Opção | Padrão | O que faz |
|---|---|---|
| `KiteMonsters` | 0 | Ativa kiting. A tática Kite do monstro também precisa permitir. |
| `KiteParanoid` | 0 | 1 = faz kiting antes de ser atacado. Táticas podem sobrescrever. |
| `KiteStep` | 5 | Células percorridas ao fazer kiting. |
| `KiteThreshold` | 3 | Distância do monstro que ativa kiting depois que ele atacou. |
| `KiteParanoidStep`, `KiteParanoidThreshold` | 2, 2 | O mesmo para kiting antes de ser atacado. |
| `KiteBounds` | 10 | Distância máxima de você ao fazer kiting. Precisa ser menor que os limites de movimento. |
| `ForceKite` | 0 | Permite kiting para um homúnculo sem ataque de longo alcance. |
| `KiteByRange` | 0 | **Novo.** Faz kiting conforme o alcance real de ataque do monstro. Veja o [manual](MANUAL.pt-BR.md#o-que-o-banco-de-monstros-muda-só-homúnculo). |

<a id="options-timing"></a>
### 3.8 Temporização e outras opções

| Opção | Padrão | O que faz |
|---|---|---|
| `SpawnDelay` | 1000 | Milissegundos de espera antes de agir após aparecer. Necessário para a proteção contra roubo de mortes funcionar durante teleportes. |
| `PVPmode` | 0 | 1 = ativa comportamento PVP ([táticas PVP](#5-advanced-settings)). |
| `CastInterruptLimit`, `CastInterruptPause`, `CastInterruptUnconfirmed` | 3, 5000, 0 | **Novo.** Pausa uma habilidade que sofre interrupções repetidas. Veja o [manual](MANUAL.pt-BR.md#7-interrupções-repetidas-de-cast). |
| `SphereTrackInterval`, `SphereTrackFactor`, `EleanorComboGap`, `EleanorComboDance`, `EleanorDoNotSwitchMode` | 800, 2, 500, 1, 0 | Estimativa de esferas e ritmo de combos de Eleanor. Veja o [manual](MANUAL.pt-BR.md#eleanor-combos-ou-somente-sonic-claw). |
| `OldHomunType` | 2 | Para um Homúnculo S, o tipo anterior: 1 Lif, 2 Amistr, 3 Filir, 4 Vanilmirth. Detectado automaticamente quando possível; a opção é apenas uma alternativa. |

Eleanor não deduz uma troca de estilo pela recusa de uma habilidade inicial:
falta de esferas, alcance ou timing também podem causar recusas. A recuperação
pode testar uma habilidade inicial habilitada do outro estilo no mesmo alvo,
no máximo uma vez a cada cinco segundos. Só o uso confirmado atualiza o estilo.
Style Change também exige confirmação; uma pausa nas chamadas da AI não basta.
A dança dos combos não muda.
Na inicialização e na ressurreição detectada, Sonic Claw habilitada prefere
Fighting, inclusive com ambas as habilidades iniciais habilitadas; apenas
Tinder Breaker prefere Grappling apenas com combos automáticos habilitados
(`AutoComboMode` diferente de zero ou `Berserk_ComboAlways=1`). Com combos
desabilitados, habilidades de área habilitadas preferem Fighting para gerar
esferas com ataques normais. `EleanorDoNotSwitchMode=1` impede essa troca
automática. A morte detectada limpa casts, alvos, combos e esferas estimadas;
a ressurreição reconstrói as esferas. O marcador de morte sobrevive à recarga
da AI. Se o cliente nunca informar a morte, uma pausa sozinha não permite
identificar a ressurreição.

<a id="4-tactics"></a>
## 4. Táticas

As táticas decidem como tratar cada tipo de monstro: ignorar plantas, priorizar os perigosos,
guardar SP para outros. Ficam em `H_Tactics.lua` e `M_Tactics.lua` e podem ser editadas nas GUIs.

O homúnculo também usa o banco de monstros para monstros sem uma entrada (veja o manual); um
**mercenário não consegue identificar monstros**, então, além da entrada padrão, precisa de um
arquivo MobID para o mapa e servidor (`Mob_ID.lua`) para que entradas específicas funcionem.
Sem ele, só funcionam a entrada padrão, `MyTact[10]` e `MyTact[11]`.

<a id="tactics-format"></a>
### 4.1 Formato do arquivo

```lua
MyTact[id] = {TACT_BASIC, TACT_SKILL, TACT_KITE, TACT_CAST, TACT_PUSHBACK, TACT_DEBUFF,
              TACT_SKILLCLASS, TACT_RESCUE, TACT_SP, TACT_SNIPE, TACT_KS, TACT_WEIGHT, TACT_CHASE}
```

`id` é o ID do tipo de monstro (de um site de banco de dados). Só pode haver uma entrada por
monstro. IDs especiais:

| id | Usado para |
|---|---|
| `MyTact[0]` | A entrada padrão |
| `MyTact[10]` | Monstros detectados como invocados |
| `MyTact[11]` | Monstros que parecem plantas, quando `AutoDetectPlant` está ativado (só mercenário) |

A entrada padrão distribuída é `{TACT_ATTACK_H, SKILL_ALWAYS, KITE_NEVER, CAST_REACT, PUSH_NEVER, DEBUFF_NEVER, CLASS_S, RESCUE_FRIEND, -1, SNIPE_DISABLE, KS_NEVER, 1, CHASE_CLEVER}`.
Na GUI: abra a aba de táticas, use **Adicionar** (o nome é apenas visual; o ID é o que importa),
escolha cada campo nas listas, **Remover** para excluir uma entrada e **Aplicar** para salvar.
Entradas removidas voltam a usar a entrada padrão.

<a id="tactics-basic"></a>
### 4.2 Tática básica (`TACT_BASIC`)

Quando e se deve atacar um monstro.

| Constante | Valor | Significado |
|---|---|---|
| `TACT_IGNORE` | 0 | Nunca ataca sem receber ordem. |
| `TACT_TANK` | -1 | Ataca uma vez e segura até outra coisa matar o monstro. |
| `TACT_TANKMOB` | -2 | Como Tank, mas, se monstros suficientes atacam a unidade, ataca um deles e usa habilidades de área no grupo. |
| `TACT_ATTACK_L` | 2 | Ataca apenas se não houver nada de prioridade maior, sem priorizá-lo nem quando ataca você ou um amigo. |
| `TACT_ATTACK_M` | 3 | Procura e ataca, exceto se algo tiver prioridade maior. |
| `TACT_ATTACK_H` | 4 | Procura e ataca, exceto se estiver reagindo a um monstro com tática React. |
| `TACT_ATTACK_TOP` | 15 | Procura e ataca, prioridade máxima. |
| `TACT_ATTACK_LAST` | 14 | Só ataca quando não há nada melhor a fazer (veja `AttackLastFullSP`). Bom para plantas, monstros de evento e outros alvos sem valor. |
| `TACT_REACT_L` | 5 | Ataca quando você, um amigo ou a unidade são atacados; prioridade baixa. Attack Medium e High vêm antes. |
| `TACT_REACT_M` | 7 | O mesmo, prioridade média. |
| `TACT_REACT_H` | 8 | O mesmo, prioridade alta. |
| `TACT_REACT_SELF` | 9 | Só reage quando **a própria unidade** é atacada, com prioridade máxima (exceto Attack Top). Para monstros que ameaçam a unidade. |
| `TACT_ATK_L_REACT_M` | 13 | Attack Low enquanto o monstro não ataca você ou um amigo; React Medium quando passa a atacar. Bom para monstros fracos que ajudam outros. |
| `TACT_SNIPE_L/M/H` | 10, 11, 12 | **Sniping**: tratado como ataque para ataques normais, mas, se a unidade luta contra outro monstro, usa uma habilidade ofensiva em um monstro próximo com tática snipe com o qual **não** está lutando, desde que nenhuma habilidade de área seja melhor. Funciona melhor com uma habilidade rápida e barata como Caprice. Se não matar em um golpe, atrai monstros extras. |

**Ordem de prioridade**, da maior para a menor (do manual original): Attack Top, React Self,
React High, React Medium, Attack High ou Snipe High, Attack Medium ou Snipe Medium, React Low,
Attack Low ou Snipe Low, Tank ou Tank & Mob, Attack Last. Esta versão acrescenta `BossAggro`,
`AttackMVPs`, `AttackOnlyMVPs` e `TargetPriority` a essa lógica (manual, seção 8).

<a id="tactics-fields"></a>
### 4.3 Os outros campos

<a id="skill-tactic"></a>**Skill (`TACT_SKILL`)**: quantas vezes usa uma habilidade ofensiva neste monstro.

| Valor | Significado |
|---|---|
| `SKILL_NEVER` (0) | Nunca usa habilidades. |
| `SKILL_ALWAYS` (100) | Sempre usa habilidades. |
| um número positivo N | Usa uma habilidade no máximo N vezes. |
| um número negativo -N | Um uso, no nível N (só homúnculo). Adequado para sniping com um bolt de nível baixo. |

**Kite (`TACT_KITE`)**: `KITE_NEVER` (0); `KITE_REACT` (1) só após ser atacado; `KITE_ALWAYS` (2)
sempre, recomendado para monstros agressivos. Só importa quando kiting está ativado.

**Cast (`TACT_CAST`)**: `CAST_REACT` (1) trata as conjurações do monstro em você, em um amigo ou
nele mesmo como hostis; `CAST_PASSIVE` (0) não. Útil principalmente para impedir a unidade de
matar Geographers invocados.

**Pushback (`TACT_PUSHBACK`)**: apenas mercenários com habilidade de empurrão (a única habilidade
de homúnculo com empurrão é Stahl Horn, inútil para isso por causa da conjuração).
`PUSH_NEVER` (0), `PUSH_SELF` (1) quando o mercenário é atacado, `PUSH_FRIEND` (2) quando você
ou um amigo são atacados.

**Debuff (`TACT_DEBUFF`)**: qual debuff usar (mercenário, além de Silent Breeze de Eira e
Volcanic Ash de Dieter). Na GUI, escolha a habilidade e se deve ser usada ao atacar ou perseguir.
No arquivo, use a constante da habilidade: positiva (`DEBUFF_CRASH_A`, `DEBUFF_PROVOKE_A`,
`DEBUFF_SANDMAN_A`, `DEBUFF_FREEZE_A`, `DEBUFF_DECAGI_A`, `DEBUFF_LEXDIV_A`, `DEBUFF_BREEZE_A`,
`DEBUFF_ASH_A`) ao atacar, ou a versão `_C` ao perseguir. `DEBUFF_ANY_A` (1) / `DEBUFF_ANY_C` (-1)
usam o que as entradas de `BasicDebuffs` permitem. `DEBUFF_NEVER` (0) não usa nenhum.

**Classe de habilidade (`TACT_SKILLCLASS`, chamada `TACT_SIZE` em arquivos antigos)**: qual tipo
de habilidade de alvo único usar.

| Constante | Valor | Significado |
|---|---|---|
| `CLASS_BOTH` | -1 | Qualquer tipo. |
| `CLASS_OLD` | 0 | Apenas habilidades anteriores ao S (Moonlight, Caprice). |
| `CLASS_S` | 1 | Apenas habilidades de Homúnculo S. Em mercenário, significa nenhuma habilidade; não use. |
| `CLASS_MOB` | 2 | Usa habilidades de área mesmo sem monstros suficientes por perto. |
| `CLASS_COMBO_1` | 3 | Combos de Eleanor: usa a segunda habilidade após a primeira, mas não a terceira. Os combos causam menos dano do que repetir Sonic Claw com 10 esferas; use apenas se quiser outro efeito do combo. |
| `CLASS_COMBO_2` | 4 | Completa o combo. |
| `CLASS_MINION` | 5 | Invoca criaturas neste alvo (Call Legion de Sera). Combina com tática snipe: as criaturas matam enquanto a unidade vai para outro lugar. |
| `CLASS_GRAPPLE` | 6 | **Novo.** Sequência de agarrão de Eleanor: apenas Tinder Breaker. |
| `CLASS_GRAPPLE_1` | 7 | **Novo.** Tinder Breaker e depois C.B.C. |
| `CLASS_GRAPPLE_2` | 8 | **Novo.** Sequência completa: Tinder Breaker, C.B.C., E.Q.C. |

<a id="rescue"></a>**Rescue (`TACT_RESCUE`)**: se a unidade abandona tudo para salvar você, seus
amigos ou sua outra unidade deste monstro. `RESCUE_NEVER` (0), `RESCUE_FRIEND` (1) amigos, mas não
você, `RESCUE_RETAINER` (2) seu outro homúnculo ou mercenário, `RESCUE_SELF` (3) defende a si
mesma, `RESCUE_OWNER` (4) você, `RESCUE_ALL` (5) todos esses, exceto a si mesma.
`RescueOwnerLowHP` adiciona uma condição de HP.

<a id="sp-tactic"></a>**SP (`TACT_SP`)**: um número positivo substitui `AttackSkillReserveSP`
para este monstro. Caso contrário (-1), usa a opção. Defina um valor maior em monstros fracos
para guardar SP para os fortes sem desativar habilidades contra os fracos.

**Snipe (`TACT_SNIPE`)**: `SNIPE_OK` (1) permite sniping em outros monstros enquanto luta contra
este; `SNIPE_DISABLE` (0) não permite, para matar este mais rápido.

**KS (`TACT_KS`)**: quão respeitosa a unidade é com outros jogadores em relação a este monstro.

| Constante | Valor | Significado |
|---|---|---|
| `KS_NEVER` | 0 | Não ataca se o monstro está atacando ou sendo atacado por outro jogador. |
| `KS_POLITE` | -1 | Não ataca enquanto ele se move, pois pode estar perseguindo alguém (como `DoNotAttackMoving`). |
| `KS_ALWAYS` | 1 | Livre para todos: ataca sem considerar outros jogadores. Você é responsável pelo que sua unidade faz: usar isso em monstros que não são livres para todos pode causar punição à sua conta. Fortemente desaconselhado. |

<a id="weight"></a>**Weight (`TACT_WEIGHT`)**: um número, padrão 1. Onde monstros são contados
(habilidades de área, Berserk, Stein Wand, Castling, Provoke...), este monstro conta como essa
quantidade. Dois monstros de peso 0.4, um de 0.5 e um de 1.2 contam como
2 x 0.4 + 0.5 + 1.2 = 2.5.

**Chase (`TACT_CHASE`)**: como `DoNotChase`, apenas para este monstro.

| Constante | Valor | Significado |
|---|---|---|
| `CHASE_NORMAL` | -1 | Segue `DoNotChase`. |
| `CHASE_ALWAYS` | 0 | Persegue mesmo com `DoNotChase` = 1. Boa escolha para Vanilmirth em OD2: configure os arqueiros orcs com `CHASE_ALWAYS` e `DoNotChase` = 1 para se mover só quando necessário. |
| `CHASE_NEVER` | 1 | Nunca persegue, mesmo com `DoNotChase` = 0. Ainda pode selecionar o monstro como alvo, então use com cuidado. |
| `CHASE_CLEVER` | 2 | Espera um pulso de SP (`ChaseSPPause`) ao perseguir, mesmo com essa opção desativada. |

<a id="5-advanced-settings"></a>
## 5. Configurações avançadas

Ficam em `H_Extra.lua` / `M_Extra.lua`, arquivos Lua carregados após todo o resto. Eles
sobrescrevem a GUI; confira-os primeiro quando uma alteração salva não faz efeito. Os arquivos
distribuídos contêm exemplos comentados.

| Configuração | O que faz |
|---|---|
| `NewAutoFriend = 0` | Descomente se a outra unidade usa outra IA (veja [Amigos](#2-friends)). |
| `FriendAttack[MOTION_...]` | Use 1 para cada movimento seu ou de um amigo que conta como sinal de ataque: atacar, usar habilidade, conjurar, arremessar (Spear Boom, poções), Acid Bomb, Full Blast. |
| `BasicDebuffs[...]` | Quais debuffs são usados quando o debuff está ativado, mas a tática não indica um (padrão: todos, exceto armadilhas e Provoke). 1 ativa um deles. |
| `AggressiveRelogTracking = 1`, `AggressiveRelogPath` | Registra relogins gravando constantemente um arquivo, para acompanhar durações de habilidades entre eles. Causa muita atividade de disco e pode deixar alguns sistemas lentos; não use sem um disco rápido. |
| `MyRoute={{x1,y1},{x2,y2},...}` | Rota para `UseIdleWalk` 5 ou 6: lista de células caminháveis, cada uma dentro dos limites de movimento das vizinhas. Com `RelativeRoute` 1, as células são deslocamentos em relação a você. Uma rota longa pode precisar de muitos pontos. |
| `GetMyTact(e)` | Exemplo comentado de alternância entre duas listas de táticas conforme uma condição. Funcionamento não garantido. |
| `LogEnable["AAI_..."]=1` | Mais registros, gravados como `AAI_*.log` na pasta do cliente: `AAI_SKILLFAIL`, `AAI_CLOSEST`, `AAI_DANCE`, `AAI_ACTORS`, `AAI_TARGETING` (retrato a cada 5 s), `AAI_MOBCOUNT`, `AAI_PROBE` (respostas brutas da sondagem de habilidades). `AAI_ERROR` está ativado por padrão; `LogEnable["AAI_ERROR"]=0` o oculta, mas, se o arquivo crescer demais, relate o problema. |

**PVP.** Com `PVPmode` = 1, outros jogadores são tratados por `H_PVP_Tact.lua` /
`M_PVP_Tact.lua`, editados como táticas normais, mas sem os campos SP, Chase, Weight e KS.
Classifique jogadores adicionando uma entrada com o ID da conta, ou uma linha em `A_Friends.lua`
com `KOS`, `ENEMY`, `NEUTRAL`, `FRIEND` ou `ALLY` e uma entrada de tática PVP para essas classes.

<a id="6-installation-notes-from-the-original"></a>
## 6. Notas de instalação do original

- **Windows com UAC (Vista e posteriores):** se Ragnarok Online está em uma pasta protegida,
  como *Arquivos de Programas*, a IA pode não funcionar corretamente. Mova a pasta inteira do RO
  para um local não protegido, como `C:\Games`, exclua o conteúdo de `USER_AI` e instale novamente.
  Isso é desnecessário se o UAC estiver desativado.
- **Atualização:** preserve `A_Friends.lua`. Para os outros arquivos, veja o
  [manual](MANUAL.pt-BR.md#atualizações-e-backups).
- **Verifique se funciona:** `AAIStartH.txt` / `AAIStartM.txt` aparece na pasta do RO quando a IA
  executa. Se não aparecer, a IA não está instalada corretamente.

<a id="7-what-changed-from-the-original-manual"></a>
## 7. O que mudou em relação ao manual original

| Tema | Manual original 1.50 | Esta versão |
|---|---|---|
| Servidores | Servidores privados ilegais não são suportados. | rAthena/Hercules podem ser selecionados com `ServerType` (testado apenas offline e em rAthena local, não em todos os servidores). Siga as regras do seu servidor sobre IA personalizada. |
| Tecla de espera | Alt+R em um trecho, Alt+T em outro. | Alt+T (homúnculo) e Ctrl+T (mercenário), como em `A_Friends.lua`. |
| Opções Berserk | Escritas como `Berzerk`. | Os nomes funcionais são `UseBerserk...` e `Berserk_...`. |
| `UseBayeriHailageStar`, `UseBayeriGoldenFerse`, `UseStienWandTele`, `FilerFlitLevel` | | `UseBayeriHailegeStar`, `UseBayeriGoldenPherze`, `UseSteinWandTele`, `FilirFlitLevel`. |
| Percurso de rota | `UseRouteWalk` e um arquivo `Route.lua`. | `UseIdleWalk` 5 ou 6 e `MyRoute` em `H_Extra.lua` / `M_Extra.lua`. |
| `OldHomunType` | Definido em `H_Extra.lua`; Vanilmirth detectado automaticamente. | Definido em `H_Config.lua`, como alternativa; o tipo básico é detectado pelas habilidades herdadas quando uma delas foi aprendida. |
| `AoEFixedLevel` | Padrão 1. | Padrão 0 no `H_Config.lua` distribuído. |
| `AggroHP` | Padrão 60. | 0 no `H_Config.lua` distribuído, permitindo iniciar combate com qualquer HP. |
| `UseAutoSkill_MinSP` (tática SP) | | `AttackSkillReserveSP`. |
| `AutoMobMode` | Não explicado. | 0 desativado, 1 conta apenas atacantes, 2 conta todos. |
| Arquivos de táticas PVP | `H_PVP_Tactics.lua` | `H_PVP_Tact.lua`, `M_PVP_Tact.lua` |
| Classes de habilidade de Eleanor | `CLASS_COMBO_1/2` | Mais `CLASS_GRAPPLE`, `_1`, `_2`. |
| Novas opções | | `ServerType`, `AutoDetectServerType`, `TrustSkillProbe`, `LintUserOptions`, `UseMobInfo` e opções do banco de monstros, `TargetPriority`, `KiteByRange`, `UseDamageDanger`, `CommandedFightStay`, `StayAwayDistance`, `StayAwayHold`, `CastInterrupt*`, opções de habilidades de Homúnculo S, predefinições por homúnculo em `Types/<Name>.lua`. |
| Histórico de versões | Changelog de 1.00 a 1.50. | Não repetido aqui. Mudanças desde o original 1.56 estão nas [changelog](../CHANGELOG.md#200). |

<a id="8-homunculus-scripting-primer"></a>
## 8. Introdução à criação de scripts para homúnculos

A IA controlada pelo cliente pode ser entendida como uma máquina de estados: fica ociosa,
segue o dono, persegue um alvo ou ataca. Esse é apenas um modelo para aprendizado — o AzzyAI
atual possui muitos outros estados e comportamentos. O ponto de entrada do homúnculo é
`AI.lua`; neste projeto, constantes compartilhadas, funções utilitárias e comportamento ficam
separados em arquivos como `Const_.lua`, `AzzyUtil.lua` e `AI_main.lua`.

Algumas funções fornecidas pelo cliente e usadas pelo código atual:

| Função | Finalidade |
|---|---|
| `GetV(V_..., id)` | Lê uma propriedade compatível de um objeto do jogo, como posição, HP, SP ou movimento. |
| `GetActors()` | Obtém os IDs de objetos visíveis atualmente pelo cliente. |
| `GetTick()` | Obtém o relógio do cliente em milissegundos, útil para agendamentos e timeouts. |
| `IsMonster(id)` | Verifica se um objeto é classificado como monstro. |
| `GetMsg(id)`, `GetResMsg(id)` | Recebem comandos estruturados da interface do cliente; não são mensagens de texto arbitrárias do chat. |
| `Move(id, x, y)`, `Attack(id, target)` | Solicitam movimento ou ataque. O AzzyAI envolve essas chamadas para aplicar suas próprias verificações e seu comportamento. |
| `SkillObject(id, level, skill, target)`, `SkillGround(id, level, skill, x, y)` | Solicitam uma habilidade com alvo ou de chão. As regras do cliente e do servidor ainda determinam se ela será executada. |

Esta lista é um ponto de partida, não um contrato completo da API válido em todos os
servidores. Os clientes variam, e os wrappers do AzzyAI acrescentam comportamento às chamadas
originais. Para extensões, prefira `H_Extra.lua` e os arquivos documentados de configuração e
táticas em vez de substituir arquivos centrais da IA. Os exemplos de configuração do guia
antigo usam outra estrutura de arquivos; não os copie diretamente para esta versão.

**Créditos:** Este resumo original foi inspirado no histórico
[Homunculus AI Script User's Guide](https://github.com/maxmx03/YggAI/blob/main/Homunculus%20AI%20Script%20User%20Guide.md),
que credita a Gravity como autora original e Max Del Canto pela atualização de 2024. O guia
está hospedado no [YggAI](https://github.com/maxmx03/YggAI); as notas acima foram parafraseadas
e adaptadas ao AzzyAI 2.0. O código atual deste projeto é a referência para nomes e comportamento.

---

[Manual](MANUAL.pt-BR.md) · [README](../README.pt-BR.md) · [English](REFERENCE.md) · [Español](REFERENCE.es.md)
