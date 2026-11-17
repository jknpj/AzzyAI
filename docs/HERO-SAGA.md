# Branch `hero` — adaptações para o Hero Saga

Este branch contém mudanças **específicas do servidor Hero Saga** (cliente próprio,
lançado em 13/11/2026). Não são alterações gerais: foram feitas para fazer a
Eleanor usar o combo de grappling **Tinder Breaker → CBC → EQC** neste servidor.

## Objetivo

O EQC (Eternal Quick Combo / Estado Traumático) dá ao mestre **+50% de dano
crítico por 10 minutos**. O objetivo é manter a Eleanor usando esse combo.

## MVPs customizados (Hero Saga)

A wiki do servidor mostra que boa parte do conteúdo (Singularidade, Jardim dos
Elementais, Pesadelo dos Heróis, Trial etc.) usa monstros marcados como
**MVP**. No AzzyAI o padrão é `BossAggro = 1`: a IA **nunca inicia luta** com
MVP, só reage se o monstro atacar. Num servidor onde o "mob comum" do conteúdo
é MVP, isso deixa a homúnculo parada exatamente onde ela deveria trabalhar.

Por isso o preset da Eleanor usa `AttackMVPs = 1`: ele **vence sobre o
`BossAggro`** e transforma táticas de reação em ataque, para que ela engaje
esses monstros normalmente. É uma decisão **específica do servidor** — o
`main` mantém `AttackMVPs = 0` (comportamento original: não começar briga com
MVP).

**Efeito colateral e exceções:** com `AttackMVPs = 1`, qualquer MVP vira alvo
em potencial, inclusive um que você talvez queira deixar em paz. Se isso
acontecer em algum conteúdo específico, use `H_Tactics.lua` com `TACT_IGNORE`
no ID daquele monstro — a tática por monstro continua tendo prioridade sobre o
preset global.

Fonte observada: `https://wiki.rpgherosaga.com/sobre/modificacoes-hero` e as
páginas de conteúdo (revisado em 17/11/2026).

## O que está diferente daqui

### `USER_AI/Types/Eleanor.lua`
- `UseEleanorSonicClaw = 0` — nunca entra no combo de Fighting.
- `UseVanilCaprice = 0` — skill herdada de Vanilmirth desligada.
- `UseEleanorTheOneFighterRises = 1` — TOFR repõe 10 esferas e inicia o ciclo.
- `UseEleanorBlazingAndFurious = 0` — consome esferas.
- `AutoMobCount = 1` — TOFR precisa funcionar em alvo único.
- `AutoComboSpheres = 4` — esferas para Tinder (1) + CBC (1) + EQC (2).
- `SkillInfo[MH_TINDER_BREAKER][2] = {1,1,1,1,1}` — alcance 1 (CBC/EQC são range 1).
- `SkillInfo[MH_TINDER_BREAKER][5] = {0,0,0,0,0}` — sem barra de cast neste cliente.
- `MyTact[0][TACT_SKILLCLASS] = CLASS_GRAPPLE_2` — libera o EQC (CLASS_S só permite Tinder/CBC).
- `EleanorComboGap = 1100`, `EleanorFollowGap = 500`.
- `AttackMVPs = 1` — ver seção "MVPs customizados" acima; liga o ataque a MVPs que,
  no padrão da IA (`BossAggro=1`), só receberiam reação.

### `USER_AI/AI_main.lua`
- Fallback de Sonic Claw (que trocava para Fighting) agora exige `UseEleanorSonicClaw == 1`.
- TOFR credita **10 esferas** no sucesso confirmado (o cliente só reporta +1).
- Quando o watcher falha em confirmar CBC/EQC, a janela de combo **não é limpa**
  (o cliente não reporta motion/SP confiável; limpar matava o EQC).

### `USER_AI/AzzyUtil.lua`
- `GetGrappleSkill`: Tinder não reinicia enquanto a janela de combo está aberta.
- `DoSkill`: `EleanorFollowGap` aplicado ao CBC → EQC (500 ms em vez de 1100 ms).
- `SkillCastResult`: CBC/EQC nunca sofrem step-down de nível (confirmação unreliable).
- `GetNewMobSkill`: TOFR não é escolhido enquanto `ComboSCTimeout`/`ComboSVTimeout`
  estão ativos — senão o `DoSkill` do TOFR zera as janelas e drena SP (154 por cast).

## Estado

A cadeia funciona com razoável consistência, mas ainda falha às vezes. A janela
de combo é **server-side** e não está em nenhum arquivo do cliente; o `skill_db`
do Hero Saga seria necessário para acertar os gaps de vez.

** Não tentar de novo:** um sequenciador `SkillObject` direto no `H_Extra.lua`
que assumia o controle total da Eleanor. Com a IA normal junto, os dois
competiam e o ataque normal cancelava o combo; sem a IA, a Eleanor parava de
agir. Foi removido.

## Dados do cliente (para referência)

`ai/webgui/skilldata.js` descreve:
- 8036 Tinder Breaker: consome 1 esfera, só em Garra.
- 8037 CBC (Dreno Traumático): só após Tinder, consome 1 (EN) / 2 (PT) esferas.
- 8038 EQC: só após CBC, consome 2 esferas.
- 8051 TOFR: "Summons the maximum amount of spirit sphere upon using the skill" → 0→10.

## Diagnóstico

Logs na pasta do cliente Hero Saga:
- `AAIStartH.txt` — config aplicada, ServerType, logs.
- `AAI_SKILLFAIL.log` — watcher, confirmações por SP, níveis aprendidos.
- `AAI_ELEANOR_DEBUG.log` — (quando habilitado) modo/esferas/estado.
- `ai/USER_AI/data/H_<owner>Timeouts.lua` — modo/esferas/cooldowns salvos.
- `ai/USER_AI/data/H_<owner>Skills.lua` — cache de níveis aprendidos (limpar se envenenado).
