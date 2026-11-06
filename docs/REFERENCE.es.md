# Referencia de AzzyAI: comandos, opciones y tácticas

[English](REFERENCE.md) | [Português (Brasil)](REFERENCE.pt-BR.md) | **Español**

[Volver al manual](MANUAL.es.md) | [README](../README.es.md) | [Notas técnicas en inglés](RELEASE_NOTES.md)

> **¿Estás empezando?** Usa el [manual del usuario](MANUAL.es.md) para la instalación,
> configuración y solución de problemas. Esta página sirve como consulta detallada de comandos,
> opciones y tácticas.

Esta referencia actualiza el contenido del manual original de AzzyAI 1.50 para la versión 2.0.0:

- Los nombres, valores predeterminados y valores de las opciones se comprobaron en
  `USER_AI/Defaults.lua`, `USER_AI/H_Config.lua` y `USER_AI/Const_.lua`. Cuando el manual original
  era incorrecto o estaba desactualizado, esta página sigue el código (las diferencias están en
  la [sección 7](#7-what-changed-from-the-original-manual)).
- **Predeterminado** es el valor del `H_Config.lua` distribuido (homúnculo). `Defaults.lua`
  contiene un segundo valor, interno, usado solo cuando falta la opción en la configuración.
  Los mercenarios tienen su propio `M_Config.lua`.
- **Nuevo** identifica una opción que no existe en el AzzyAI original.
- Tu `H_Extra.lua` / `M_Extra.lua` se carga al final y sobrescribe lo guardado por la GUI.
- Nada se volvió a probar dentro del juego para esta página; el comportamiento se consultó en
  el código y en el manual original.

## Contenido

1. [Comandos en el juego](#1-commanding-in-game)
2. [Amigos](#2-friends)
3. [Opciones](#3-options)
   - [Inicio y combate](#options-engaging)
   - [Movimiento y comportamiento inactivo](#options-movement)
   - [Habilidades ofensivas](#options-attack-skills)
   - [Buffs](#options-buffs)
   - [Otras habilidades](#options-other-skills)
   - [Modo Berserk](#options-berserk)
   - [Kiting](#options-kiting)
   - [Temporización y otras opciones](#options-timing)
4. [Tácticas](#4-tactics)
   - [Formato del archivo](#tactics-format)
   - [Táctica básica](#tactics-basic)
   - [Otros campos de táctica](#tactics-fields)
5. [Configuración avanzada](#5-advanced-settings)
6. [Notas de instalación](#6-installation-notes-from-the-original)
7. [Cambios respecto al original](#7-what-changed-from-the-original-manual)
8. [Introducción a los scripts](#8-homunculus-scripting-primer)

<a id="1-commanding-in-game"></a>
## 1. Comandos en el juego

El cliente permite enviar algunos comandos a la IA. La IA recibe comandos, nunca texto del
chat, por lo que no es posible escribirle un comando.

| Qué haces | Qué hace la IA | Opciones relacionadas |
|---|---|---|
| Tecla de **espera**: **Alt+T** (homúnculo), **Ctrl+T** (mercenario) | Alterna el modo de espera: la unidad deja de ser agresiva. Pulsa de nuevo para salir. | `DefendStandby` (sigue defendiéndote en espera), `StickyStandby` (vuelve a la espera tras un combate defensivo) |
| **Alt + clic derecho** en una celda | Se mueve hasta allí. También sirve para [añadir amigos](#2-friends). Un movimiento a más de 15 celdas de ti se ignora y se registra como error. | `MoveSticky`, `MoveStickyFight` |
| **Alt + doble clic derecho** en un monstruo | Interrumpe lo que estaba haciendo y ataca a ese monstruo. | `CommandedFightStay` (**nuevo**), `UseBerserkAttack` |
| Usas una habilidad **en un monstruo** | Ataca al objetivo, empezando con esa habilidad. | `UseBerserkSkill` |
| Usas una habilidad **en un jugador**, fuera de PVP | Se acerca hasta estar a alcance y usa la habilidad en él. | |
| Usas una habilidad sin objetivo o de suelo | La ejecuta el cliente sin avisar a la IA, por lo que esta puede perder el seguimiento del intervalo y la duración de esa habilidad. | |
| **Te sientas** | Deja de ser agresiva y se queda a tu lado. | `DoNotUseRest`, `RestXOff`, `RestYOff` |

El manual original menciona Alt+R para la espera en un pasaje; la lista anterior sigue el
`A_Friends.lua` distribuido y la descripción de `DefendStandby`. Los atajos se pueden cambiar
en el cliente, así que comprueba los tuyos.

Con `CommandedFightStay` activado (predeterminado), persigue y combate a un monstruo al que
ordenaste atacar hasta que muera o des otra orden, aunque te alejes. El abandono por límites de
movimiento y `AttackTimeLimit` no se aplican a ese objetivo. Usa 0 para recuperar el
comportamiento anterior.

`AllowSBR44` (predeterminado 0) hace que la IA bloquee el uso manual de SBR44, para proteger
contra clics accidentales. No puede hacer lo mismo con Self Destruct.

<a id="2-friends"></a>
## 2. Amigos

Un jugador añadido como amigo es defendido como si fuera el dueño, hasta que lo elimines.
Añadir el mercenario o el homúnculo de otro jugador solo dura hasta que vuelva a conectarse,
se teletransporte o lo invoque de nuevo, porque su ID cambia.

**Método de caminar** (`MirAIFriending` = 1, predeterminado):

1. Ordena al homúnculo caminar hasta la celda **justo al norte o al sur** del jugador.
2. Camina en **círculo** alrededor del objetivo si se añadió al amigo, o **va y vuelve en
   línea recta** si se eliminó.

**Método de espera** (`StandbyFriending` = 1, método antiguo de AzzyAI): pon la unidad en
espera; luego **tú** te sientas una celda **al oeste** de la persona para añadirla, o una celda
**al este** para eliminarla.

**A mano:** `A_Friends.lua` guarda la lista, una línea por jugador, usando el ID de la cuenta
(obtenido de un sitio como ROPD):

```lua
MyFriends[2322797]=1       -- un amigo
MyFriends[2322797]=KOS     -- listas PVP: NEUTRAL, KOS (atacar al avistar), ENEMY (hostil, no atacado), ALLY (aliado)
```

Conserva `A_Friends.lua` al actualizar; no se reconstruye.

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `AssumeHomun` | 1 | El homúnculo y tu mercenario se añaden como amigos automáticamente. Usa 0 solo si no quieres que se roben las bajas entre sí. |
| `NewAutoFriend` | 1 | Añade automáticamente como amigos a tu propio mercenario y homúnculo. En `H_Extra.lua`, usa 0 si la otra unidad utiliza otra IA; después invoca ambos y vuelve a conectarte o teletranspórtate para activarlo. |
| `FriendAttack[...]` | véase [5](#5-advanced-settings) | Qué movimientos de un amigo (atacar, conjurar...) hacen que la IA ataque al monstruo que tiene como objetivo. |

<a id="3-options"></a>
## 3. Opciones

Los nombres son los usados en los archivos de configuración; las GUIs muestran los mismos
nombres y tienen un cuadro de búsqueda.

<a id="options-engaging"></a>
### 3.1 Iniciar y mantener combates

La unidad normalmente busca monstruos permitidos por sus [tácticas](#4-tactics), cuando están
cerca de ti.

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `AggroHP`, `AggroSP` | 0, 0 | La unidad solo inicia combate mientras su porcentaje de HP / SP está **por encima** de estos valores. 100 significa que nunca inicia una pelea y solo defiende. (El valor interno de respaldo de `AggroHP` es 60.) |
| `StationaryAggroDist`, `MobileAggroDist` | 12, 12 | Distancia de ti, en celdas, dentro de la cual busca monstruos mientras estás quieto / en movimiento. |
| `SuperPassive` | 0 | 1 = nunca lucha sin una orden directa. |
| `DoNotChase` | 0 | 1 = nunca camina hacia un monstruo. Sigue atacando a distancia con habilidades de alcance, y las tácticas `CHASE_ALWAYS` ignoran esta opción. |
| `DoNotAttackMoving` | 0 | 1 = ignora monstruos en movimiento (ayuda a evitar robar bajas y monstruos rápidos; malo para subir de nivel AFK). |
| `OpportunisticTargeting` | 0 | 1 = durante la persecución, reevalúa los objetivos y cambia a uno mejor si aparece. |
| `AttackTimeLimit` | 10000 | Milisegundos atacando antes de considerar al objetivo inalcanzable y abandonarlo. 0 = nunca. No se aplica a un objetivo ordenado (`CommandedFightStay`). |
| `RescueOwnerLowHP` | 0 | Cambia el rescate según tu HP (véanse [tácticas de rescate](#rescue)). |
| `FleeHP` | 0 | Solo hace kiting por debajo de este porcentaje de HP. 0 ignora la condición. |
| `UseDanceAttack` | 1 | Solo homúnculo: “baila” durante los ataques normales para atacar un poco más rápido. El baile detiene la regeneración de SP durante la pelea; con una habilidad ofensiva suele ser mejor desactivarlo. |
| `DanceMinSP` | 0 | SP mínimo para bailar. Negativo = porcentaje del SP máximo. |
| `TankMonsterLimit` | 4 | Máximo de monstruos que intenta retener para otro atacante con la táctica `TACT_TANK`. |
| `AutoDetectPlant` | 1 | **Solo mercenario.** Trata a los monstruos que nunca vio moverse o atacar como plantas y usa `MyTact[11]` para ellos. Usa 0 contra monstruos estacionarios. (El homúnculo usa la base de datos de monstruos.) |
| `AttackLastFullSP` | 0 | 1 = los monstruos `TACT_ATTACK_LAST` solo se atacan con SP completo. |
| `UseAvoid` | 0 | **Solo homúnculo.** 1 = el cliente se cierra cuando aparece un monstruo de `H_Avoid.lua` (por defecto, MVP invocables y algunos monstruos peligrosos). Pruébalo antes: en algunas versiones de Windows aparece una ventana de error en vez de cerrarse. |
| `UseDamageDanger`, `DamageDangerDrop`, `DamageDangerHP` | 0, 30, 60 | **Nuevo.** Se aleja tras una caída repentina de HP, incluso con `KiteMonsters` desactivado. Véase el [manual](MANUAL.es.md#4-comportamiento-básico). |
| `UseMobInfo`, `SkillElementMinRate`, `PlantSkills`, `BossAggro`, `AttackMVPs`, `AttackOnlyMVPs`, `TargetPriority` | 1, 50, 0, 1, 0, 0, 0 | **Nuevo.** Opciones de la base de datos de monstruos; explicadas en el [manual](MANUAL.es.md#qué-cambia-la-base-de-datos-de-monstruos-solo-homúnculo). |

<a id="options-movement"></a>
### 3.2 Movimiento y comportamiento inactivo

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `FollowStayBack` | 2 | Celdas que se queda detrás de ti al seguirte. |
| `StationaryMoveBounds`, `MobileMoveBounds` | 14, 9 | Hasta qué distancia de ti puede ir para reaccionar (estando tú quieto / en movimiento). Más allá, abandona todo y vuelve. Mantén ambos en **14 o menos**. |
| `MoveSticky` | 0 | 1 = tras una orden de movimiento, se queda allí hasta recibir otra orden de movimiento. |
| `MoveStickyFight` | 0 | 1 = aun estando fijada así, sigue luchando normalmente. |
| `StayAwayDistance` | 0 | **Nuevo.** Mantiene aproximadamente esta distancia de ti mientras está inactiva (limitada por los límites de movimiento). 0 = desactivado. Útil en instancias donde el homúnculo retiene a los monstruos. |
| `StayAwayHold` | 0 | **Nuevo.** 1 = se queda en un punto y vuelve a él tras las peleas, en vez de seguirte. |
| `UseIdleWalk` | 4 | Camina mientras está inactiva, cuando HP supera `AggroHP`, SP supera `AggroSP` y supera `IdleWalkSP` por ciento. 0 desactivado; 1 círculo a tu alrededor; 2 cruz; 3 rectángulo; 4 aleatorio; 5 recorre `MyRoute` de ida y vuelta; 6 recorre `MyRoute` en bucle. |
| `IdleWalkDistance` | 4 | Distancia de ti para los recorridos de 1 a 4. |
| `IdleWalkSP` | 80 | Porcentaje de SP que debe superar para caminar cuando está inactiva. |
| `RelativeRoute` | 1 | 1 = las celdas de `MyRoute` son desplazamientos respecto a ti; 0 = celdas absolutas del mapa. Véase [recorrido de ruta](#5-advanced-settings). |
| `DoNotUseRest` | 0 | 1 = no deja de ser agresiva cuando te sientas. |
| `RestXOff`, `RestYOff` | 2, 0 | Desplazamiento respecto a ti donde espera cuando te sientas (puede ser negativo). |
| `ChaseSPPause` | 0 | 1 = retrasa perseguir y seguir hasta justo después de un pulso de regeneración de SP. La táctica `CHASE_ALWAYS` lo ignora; `CHASE_CLEVER` lo fuerza. |
| `ChaseSPPauseSP` | -60 | Solo pausa por debajo de este SP. Negativo = porcentaje. |
| `ChaseSPPauseTime` | 3000 | Espera máxima, en ms, hasta el siguiente pulso de SP. |

<a id="options-attack-skills"></a>
### 3.3 Habilidades ofensivas

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `UseAttackSkill` | 1 | Habilidades ofensivas automáticas. |
| `UseSkillOnly` | -1 | **-1 (perseguir, recomendado)** usa habilidades al perseguir y atacar. **0** solo al atacar. **1** ataca únicamente con habilidades y no usa una habilidad de largo alcance antes de llegar al alcance cuerpo a cuerpo. |
| `UseHomunSSkillChase`, `UseHomunSSkillAttack` | 1, 1 | Permite las habilidades de objetivo único de Homúnculo S al perseguir / atacar. Desactívalas si muchas habilidades tienen lanzamiento interrumpible y no puedes esquivar. |
| `AutoMobMode` | 2 | Habilidades de área: 0 desactivado; 1 cuenta solo monstruos que ya están atacando; 2 cuenta todos los monstruos que alcanzaría el área. |
| `AutoMobCount` | 2 | Monstruos (ponderados por la [táctica Weight](#weight)) necesarios para usar una habilidad de área. 1 permite que una habilidad de área compita incluso en un solo objetivo. |
| `AoEFixedLevel` | 0 | 1 = siempre usa habilidades de área al nivel definido en la opción `...Level` de la habilidad, ignorando el nivel configurado en las tácticas. |
| `AoEMaximizeTargets` | 0 | 1 = apunta las habilidades de área al punto que alcanza más monstruos, aunque no alcance al objetivo actual. |
| `AoEReserveSP` | 0 | 1 = nunca gasta SP en otro ataque si eso deja SP insuficiente para la habilidad de área. |
| `AttackSkillReserveSP` | 0 | SP en reserva: no usa una habilidad ofensiva si dejaría menos que esto. Se puede sobrescribir por monstruo mediante la [táctica SP](#sp-tactic). |
| `AutoSkillLimit` | 0 | Máximo de veces que usa una habilidad ofensiva en un objetivo. 0 = sin límite. Las tácticas pueden sobrescribirlo. |
| `AutoSkillDelay` | 200 | Espera adicional, en ms, entre habilidades, para evitar usos duplicados. 0 elimina solo esta espera, no la recarga ni el tiempo de lanzamiento. |
| `CastTimeRatio` | .80 | Fracción del tiempo de lanzamiento que espera la IA, considerando tus reducciones de lanzamiento. |
| `AutoComboMode` | 2 | Combos (Eleanor): 0 nunca; 1 solo en monstruos cuya clase de habilidad es de combo; 2 siempre. |
| `AutoComboSpheres` | 1 | Esferas necesarias para **iniciar** una cadena automática. |
| `UseVanilCaprice` | 1 | Vanilmirth: usar Caprice. |
| `LintUserOptions` | 1 | **Nuevo.** Al iniciar, desactiva opciones de habilidades que el homúnculo no ha aprendido y reduce los niveles al nivel aprendido. |
| `TrustSkillProbe` | -1 | **Nuevo.** Si debe confiar en la respuesta del cliente sobre los niveles de habilidades aprendidos. -1 = automático (no en el servidor oficial, sí en servidores privados). |
| `ServerType`, `AutoDetectServerType` | oficial, 1 | **Nuevo.** Servidor oficial o rAthena/Hercules. Véase el [manual](MANUAL.es.md#4-comportamiento-básico). |

**Activación y niveles de habilidades.** Cada habilidad tiene `Use<Type><Skill>` (1 = activada)
y `<Type><Skill>Level`. La IA no usa una habilidad por encima del nivel aprendido: con
`LintUserOptions`, reduce los niveles configurados al nivel aprendido al iniciar.

| Homúnculo | Opciones de habilidades ofensivas |
|---|---|
| Eira | `UseEiraXenoSlasher`, `UseEiraEraseCutter` (Eraser Cutter), `UseEiraTwisterCutter`, `UseEiraAbsoluteZephyr`, `UseEiraSilentBreeze` |
| Bayeri | `UseBayeriStahlHorn`, `UseBayeriHailegeStar`, `UseBayeriGlanzenSpies`, `UseBayeriHeiligePferd` |
| Sera | `UseSeraParalyze` (Needle of Paralyze), `UseSeraPoisonMist`, `UseSeraCallLegion`, `UseSeraToxinOfMandara`, `UseSeraNeedleStinger` |
| Dieter | `UseDieterLavaSlide`, `UseDieterVolcanicAsh`, `UseDieterAshAsAttack`, `UseDieterBlastForge` |
| Eleanor | `UseEleanorSonicClaw`, `EleanorSilverveinLevel`, `EleanorMidnightLevel`, `UseEleanorTinderBreaker`, `UseEleanorCBC`, `UseEleanorEQC`, `UseEleanorBlazingAndFurious`, `UseEleanorTheOneFighterRises` |

Silvervein Rush y Midnight Frenzy solo tienen nivel, no una opción de activación. Los consejos
para elegir entre ellas están en la [sección 6 del manual](MANUAL.es.md#6-consejos-por-homúnculo-s).
`LavaSlideMode` y `PoisonMistMode` (predeterminado 0) eligen cómo usar esas dos habilidades:
0 las usa como ataques; otro valor las mantiene a tu alrededor como buff e indica cuándo
usarlas (los valores de la sección 3.4).

<a id="options-buffs"></a>
### 3.4 Habilidades de buff

Una opción de buff recibe uno de estos valores, que indican **cuándo** se puede usar la habilidad:

| Valor | Cuándo |
|---|---|
| -2 | Estando inactiva, si no hay nada más que hacer |
| -1 | Al perseguir (para no desperdiciarla cuando no hay nada que matar) |
| 0 | Nunca |
| 1 | Estando inactiva |
| 2 | Al perseguir o atacar en modo Berserk |
| 3 | En cuanto sea posible, interrumpiendo otras acciones. El objetivo actual solo se abandona para Painkiller y Provoke. |

Los buffs de Homúnculo S no tienen opción de nivel, porque los niveles altos son mejores.
Los homúnculos básicos sí la tienen, porque los niveles altos tienen recargas mayores.

| Opción | Predeterminado | Controla |
|---|---|---|
| `UseOffensiveBuff` | 3 | Flitting, Mental Charge, Weapon Quicken, Bloodlust |
| `UseDefensiveBuff` | 3 | Amistr Bulwark, Accelerated Flight, Guard, Parrying, Urgent Escape |
| `UseAutoSight`, `UseAutoMag` | 1, 0 (interno) | **Solo mercenario:** Sight (arquero nivel 2), Magnificat (arquero nivel 4) |
| `UseDieterGraniticArmor`, `UseDieterMagmaFlow`, `UseDieterPyroclastic` | 0, 1, 1 | La habilidad de Dieter con ese nombre (`DieterPyroclasticLevel`, predeterminado 10) |
| `UseDieterTempering` | 1 | **Nuevo.** Tempering de Dieter |
| `UseEiraOveredBoost` | 0 | Overed Boost de Eira |
| `UseBayeriGoldenPherze`, `UseBayeriAngriffModus`, `UseBayeriGoldeneTone` | 0, 1, 1 | Golden Ferse, Angriff Modus y Goldene Tone de Bayeri |
| `UseProvokeOwner` | 0 | Provoke (mercenario) o Pain Killer (Sera) en ti |
| `ProvokeOwnerMobbed` | 3 | No se usa en ti con menos monstruos que esto atacándote, salvo por debajo del 90% de HP (cuenta la táctica Weight) |
| `UseSeraPainkiller` | 3 | Pain Killer de Sera |
| `UseSacrificeOwner` | 0 (interno) | **Solo mercenario:** Sacrifice en ti. Funciona con una diferencia de hasta 10 niveles del jugador y no es fiable debido a errores del juego. |
| `UseSmartBulwark` | 0 | Amistr: solo usa Bulwark si quedan al menos 120 SP, para permitir Bloodlust después. Se ignora por debajo de 160 SP máximo o sin Bloodlust. |
| `AmiBulwarkLevel`, `FilirFlitLevel`, `FilirAccelLevel`, `LifEscapeLevel` | 5, 5, 5, 0 | Nivel de Amistr Bulwark, Filir Flitting, Filir Accelerated Flight, Lif Urgent Escape. |
| `UseEiraLightOfRegene` | 0 | **Nuevo.** Eira se sacrifica para resucitarte. Véase el [manual](MANUAL.es.md#6-consejos-por-homúnculo-s). |

`LifEscapeLevel = 0` desactiva Urgent Escape: la IA omite el uso en vez de enviar nivel 0 al
servidor. Es seguro, al contrario de la advertencia del manual original. Usa un nivel aprendido
de 1 a 5 para activarla; `UseDefensiveBuff` determina cuándo usarla.

**Curación** (Healing Hands de Lif, Chaotic Blessing de Vanilmirth; homúnculo):
`UseAutoHeal` = 0 desactivado, 1 activado, 2 solo estando inactivo (incluso entre bajas), 3 solo
estando inactivo y sin nada más que hacer. `HealOwnerHP` (50) es tu porcentaje de HP por debajo
del cual te cura (Lif usa Healing Hands nivel 5; Vanilmirth, Chaotic Blessing nivel 3).
`HealSelfHP` (50) es el porcentaje de HP del propio Vanilmirth para curarse (nivel 4; nivel 5
estando inactivo). Lif no puede curarse a sí misma.

<a id="options-other-skills"></a>
### 3.5 Otras habilidades

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `UseAutoPushback`, `AutoPushbackThreshold` | 0, 2 | **Solo mercenario.** Empuja a los monstruos lejos del mercenario con una habilidad de empuje (Arrow Repel, Skid Trap) cuando se acercan a esta distancia. La táctica Pushback también debe permitirlo. |
| `UseCastleDefend`, `CastleDefendThreshold` | 0, 4 | Amistr: usa Castling cuando al menos esta cantidad de monstruos (Weight cuenta) te ataca y menos atacan al Amistr. |
| `UseCastleRoute` | 0 | Amistr: usa Castling para moverte por una ruta absoluta (requiere `UseIdleWalk` 5 o 6 y `RelativeRoute` 0). |
| `UseBayeriSteinWand` | 1 | Bayeri: Stone Wall cuando un grupo suficientemente grande te ataca a ti o a él (valores de buff, sección 3.4). |
| `UseSteinWandOwnerMob`, `UseSteinWandSelfMob` | 2, 2 | Monstruos necesarios atacándote a ti / al Bayeri (Weight cuenta). |
| `BayeriSteinWandLevel` | 5 | Nivel de Stone Wall. |
| `UseSteinWandTele`, `SteinWandTelePause` | 0, 3000 | Usa Stone Wall justo después de conectarte o llegar de un teletransporte, y no lo repite. Combina con `DoNotChase` y la habilidad ofensiva de Bayeri; no lo combines con `UseBayeriSteinWand`. |

<a id="options-berserk"></a>
### 3.6 Modo Berserk

El modo Berserk hace que la unidad luche más agresivamente tras un comando o contra un grupo
grande. Tras un comando, termina cuando muere el objetivo. **La grafía funcional es `Berserk`**
(el manual original escribe “Berzerk”).

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `UseBerserkSkill` | 0 | Entra en Berserk cuando recibe la orden de usar una habilidad ofensiva. |
| `UseBerserkAttack` | 0 | Entra en Berserk cuando recibe la orden de atacar. |
| `UseBerserkMobbed` | 0 | 0 = desactivado. En otro caso, entra en Berserk mientras esta cantidad o más de monstruos lucha contra ti y tus amigos. |
| `Berserk_SkillAlways` | 0 | Siempre usa habilidades ofensivas en Berserk. |
| `Berserk_Dance` | 0 | Baila al atacar en Berserk (solo homúnculo). |
| `Berserk_IgnoreMinSP` | 0 | Ignora `AttackSkillReserveSP` en Berserk. |
| `Berserk_ComboAlways` | 0 | Siempre permite combos en Berserk. |

<a id="options-kiting"></a>
### 3.7 Kiting

Kiting significa alejarse de los monstruos mientras se les ataca, principalmente con
mercenarios arqueros. Los valores predeterminados se eligieron empíricamente: cámbialos solo
con un motivo.

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `KiteMonsters` | 0 | Activa kiting. La táctica Kite del monstruo también debe permitirlo. |
| `KiteParanoid` | 0 | 1 = hace kiting antes de ser atacado. Las tácticas pueden sobrescribirlo. |
| `KiteStep` | 5 | Celdas recorridas al hacer kiting. |
| `KiteThreshold` | 3 | Distancia del monstruo que activa kiting después de que haya atacado. |
| `KiteParanoidStep`, `KiteParanoidThreshold` | 2, 2 | Lo mismo para kiting antes de ser atacado. |
| `KiteBounds` | 10 | Distancia máxima de ti al hacer kiting. Debe ser menor que los límites de movimiento. |
| `ForceKite` | 0 | Permite kiting para un homúnculo sin ataque de largo alcance. |
| `KiteByRange` | 0 | **Nuevo.** Hace kiting según el alcance real de ataque del monstruo. Véase el [manual](MANUAL.es.md#qué-cambia-la-base-de-datos-de-monstruos-solo-homúnculo). |

<a id="options-timing"></a>
### 3.8 Temporización y otras opciones

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `SpawnDelay` | 1000 | Milisegundos que espera antes de actuar tras aparecer. Necesario para que la protección contra robo de bajas funcione al teletransportarse. |
| `PVPmode` | 0 | 1 = activa el comportamiento PVP ([tácticas PVP](#5-advanced-settings)). |
| `CastInterruptLimit`, `CastInterruptPause`, `CastInterruptUnconfirmed` | 3, 5000, 0 | **Nuevo.** Pausa una habilidad que sufre interrupciones repetidas. Véase el [manual](MANUAL.es.md#7-interrupciones-repetidas). |
| `SphereTrackInterval`, `SphereTrackFactor`, `EleanorComboGap`, `EleanorComboDance`, `EleanorDoNotSwitchMode` | 800, 2, 500, 1, 0 | Estimación de esferas y ritmo de combos de Eleanor. Véase el [manual](MANUAL.es.md#eleanor-combos-o-solo-sonic-claw). |
| `OldHomunType` | 2 | Para un Homúnculo S, el tipo anterior: 1 Lif, 2 Amistr, 3 Filir, 4 Vanilmirth. Se detecta automáticamente cuando es posible; la opción es solo un respaldo. |

<a id="4-tactics"></a>
## 4. Tácticas

Las tácticas deciden cómo tratar a cada tipo de monstruo: ignorar plantas, priorizar los
peligrosos, guardar SP para otros. Están en `H_Tactics.lua` y `M_Tactics.lua` y pueden editarse
en las GUIs.

El homúnculo también usa la base de datos para monstruos sin una entrada (véase el manual);
un **mercenario no puede identificar monstruos**, así que, además de la entrada predeterminada,
necesita un archivo MobID para el mapa y servidor (`Mob_ID.lua`) para que funcionen las entradas
específicas. Sin él, solo funcionan la entrada predeterminada, `MyTact[10]` y `MyTact[11]`.

<a id="tactics-format"></a>
### 4.1 Formato del archivo

```lua
MyTact[id] = {TACT_BASIC, TACT_SKILL, TACT_KITE, TACT_CAST, TACT_PUSHBACK, TACT_DEBUFF,
              TACT_SKILLCLASS, TACT_RESCUE, TACT_SP, TACT_SNIPE, TACT_KS, TACT_WEIGHT, TACT_CHASE}
```

`id` es el ID del tipo de monstruo (de un sitio de base de datos). Solo puede haber una entrada
por monstruo. IDs especiales:

| id | Se usa para |
|---|---|
| `MyTact[0]` | La entrada predeterminada |
| `MyTact[10]` | Monstruos detectados como invocados |
| `MyTact[11]` | Monstruos que parecen plantas, cuando `AutoDetectPlant` está activado (solo mercenario) |

La entrada predeterminada distribuida es `{TACT_ATTACK_H, SKILL_ALWAYS, KITE_NEVER, CAST_REACT, PUSH_NEVER, DEBUFF_NEVER, CLASS_S, RESCUE_FRIEND, -1, SNIPE_DISABLE, KS_NEVER, 1, CHASE_CLEVER}`.
En la GUI: abre la pestaña de tácticas, usa **Añadir** (el nombre es solo visual; el ID es lo que
importa), elige cada campo en las listas, **Eliminar** para borrar una entrada y **Aplicar**
para guardar. Las entradas eliminadas vuelven a usar la entrada predeterminada.

<a id="tactics-basic"></a>
### 4.2 Táctica básica (`TACT_BASIC`)

Cuándo y si debe atacar a un monstruo.

| Constante | Valor | Significado |
|---|---|---|
| `TACT_IGNORE` | 0 | Nunca ataca sin recibir una orden. |
| `TACT_TANK` | -1 | Golpea una vez y retiene al monstruo hasta que otra cosa lo mate. |
| `TACT_TANKMOB` | -2 | Como Tank, pero, si suficientes monstruos atacan a la unidad, ataca a uno y usa habilidades de área sobre el grupo. |
| `TACT_ATTACK_L` | 2 | Ataca solo si no hay nada de mayor prioridad, sin priorizarlo ni cuando te ataca a ti o a un amigo. |
| `TACT_ATTACK_M` | 3 | Busca y ataca, salvo que algo tenga mayor prioridad. |
| `TACT_ATTACK_H` | 4 | Busca y ataca, salvo que esté respondiendo a un monstruo con táctica React. |
| `TACT_ATTACK_TOP` | 15 | Busca y ataca, prioridad máxima. |
| `TACT_ATTACK_LAST` | 14 | Solo ataca cuando no hay nada mejor que hacer (véase `AttackLastFullSP`). Bueno para plantas, monstruos de evento y otros objetivos sin valor. |
| `TACT_REACT_L` | 5 | Ataca cuando tú, un amigo o la unidad son atacados; prioridad baja. Attack Medium y High van antes. |
| `TACT_REACT_M` | 7 | Lo mismo, prioridad media. |
| `TACT_REACT_H` | 8 | Lo mismo, prioridad alta. |
| `TACT_REACT_SELF` | 9 | Solo reacciona cuando **la propia unidad** es atacada, con prioridad máxima (excepto Attack Top). Para monstruos que amenazan a la unidad. |
| `TACT_ATK_L_REACT_M` | 13 | Attack Low mientras el monstruo no te ataca a ti o a un amigo; React Medium cuando lo hace. Bueno para monstruos débiles que ayudan a otros. |
| `TACT_SNIPE_L/M/H` | 10, 11, 12 | **Sniping**: se trata como ataque para ataques normales, pero, si la unidad lucha contra otro monstruo, usa una habilidad ofensiva en un monstruo cercano con táctica snipe contra el que **no** está luchando, siempre que ninguna habilidad de área sea mejor. Funciona mejor con una habilidad rápida y barata como Caprice. Si no mata de un golpe, atrae monstruos adicionales. |

**Orden de prioridad**, de mayor a menor (del manual original): Attack Top, React Self, React
High, React Medium, Attack High o Snipe High, Attack Medium o Snipe Medium, React Low, Attack Low
o Snipe Low, Tank o Tank & Mob, Attack Last. Esta versión añade `BossAggro`, `AttackMVPs`,
`AttackOnlyMVPs` y `TargetPriority` a esa lógica (manual, sección 8).

<a id="tactics-fields"></a>
### 4.3 Los otros campos

<a id="skill-tactic"></a>**Skill (`TACT_SKILL`)**: cuántas veces usa una habilidad ofensiva en este monstruo.

| Valor | Significado |
|---|---|
| `SKILL_NEVER` (0) | Nunca usa habilidades. |
| `SKILL_ALWAYS` (100) | Siempre usa habilidades. |
| un número positivo N | Usa una habilidad como máximo N veces. |
| un número negativo -N | Un uso, al nivel N (solo homúnculo). Adecuado para sniping con un bolt de nivel bajo. |

**Kite (`TACT_KITE`)**: `KITE_NEVER` (0); `KITE_REACT` (1) solo tras ser atacado; `KITE_ALWAYS` (2)
siempre, recomendado para monstruos agresivos. Solo importa cuando kiting está activado.

**Cast (`TACT_CAST`)**: `CAST_REACT` (1) trata los lanzamientos del monstruo sobre ti, un amigo
o sí mismo como hostiles; `CAST_PASSIVE` (0) no. Útil principalmente para impedir que la unidad
mate Geographers invocados.

**Pushback (`TACT_PUSHBACK`)**: solo mercenarios con una habilidad de empuje (la única habilidad
de homúnculo con empuje es Stahl Horn, inútil para ello por su tiempo de lanzamiento).
`PUSH_NEVER` (0), `PUSH_SELF` (1) cuando atacan al mercenario, `PUSH_FRIEND` (2) cuando te atacan
a ti o a un amigo.

**Debuff (`TACT_DEBUFF`)**: qué debuff usar (mercenario, además de Silent Breeze de Eira y
Volcanic Ash de Dieter). En la GUI, elige la habilidad y si usarla al atacar o perseguir. En el
archivo, usa la constante de la habilidad: positiva (`DEBUFF_CRASH_A`, `DEBUFF_PROVOKE_A`,
`DEBUFF_SANDMAN_A`, `DEBUFF_FREEZE_A`, `DEBUFF_DECAGI_A`, `DEBUFF_LEXDIV_A`, `DEBUFF_BREEZE_A`,
`DEBUFF_ASH_A`) al atacar, o la versión `_C` al perseguir. `DEBUFF_ANY_A` (1) / `DEBUFF_ANY_C` (-1)
usan lo que permiten las entradas de `BasicDebuffs`. `DEBUFF_NEVER` (0) no usa ninguno.

**Clase de habilidad (`TACT_SKILLCLASS`, llamada `TACT_SIZE` en archivos antiguos)**: qué tipo
de habilidad de objetivo único usar.

| Constante | Valor | Significado |
|---|---|---|
| `CLASS_BOTH` | -1 | Cualquier tipo. |
| `CLASS_OLD` | 0 | Solo habilidades anteriores al S (Moonlight, Caprice). |
| `CLASS_S` | 1 | Solo habilidades de Homúnculo S. En un mercenario significa ninguna habilidad; no lo uses. |
| `CLASS_MOB` | 2 | Usa habilidades de área incluso sin suficientes monstruos cerca. |
| `CLASS_COMBO_1` | 3 | Combos de Eleanor: usa la segunda habilidad después de la primera, pero no la tercera. Sus combos causan menos daño que repetir Sonic Claw con 10 esferas; úsalo solo si quieres otro efecto del combo. |
| `CLASS_COMBO_2` | 4 | Completa el combo. |
| `CLASS_MINION` | 5 | Invoca criaturas sobre este objetivo (Call Legion de Sera). Combina con una táctica snipe: las criaturas matan mientras la unidad va a otro sitio. |
| `CLASS_GRAPPLE` | 6 | **Nuevo.** Cadena de agarre de Eleanor: solo Tinder Breaker. |
| `CLASS_GRAPPLE_1` | 7 | **Nuevo.** Tinder Breaker y luego C.B.C. |
| `CLASS_GRAPPLE_2` | 8 | **Nuevo.** Cadena completa: Tinder Breaker, C.B.C., E.Q.C. |

<a id="rescue"></a>**Rescue (`TACT_RESCUE`)**: si la unidad abandona todo para salvarte a ti, a
tus amigos o a tu otra unidad de este monstruo. `RESCUE_NEVER` (0), `RESCUE_FRIEND` (1) amigos,
pero no tú, `RESCUE_RETAINER` (2) tu otro homúnculo o mercenario, `RESCUE_SELF` (3) se defiende
a sí misma, `RESCUE_OWNER` (4) tú, `RESCUE_ALL` (5) todos esos excepto ella misma.
`RescueOwnerLowHP` añade una condición de HP.

<a id="sp-tactic"></a>**SP (`TACT_SP`)**: un número positivo sustituye `AttackSkillReserveSP`
para este monstruo. En otro caso (-1), se usa la opción. Define un valor mayor en monstruos
débiles para guardar SP para los fuertes sin desactivar habilidades contra los débiles.

**Snipe (`TACT_SNIPE`)**: `SNIPE_OK` (1) permite sniping en otros monstruos mientras lucha
contra este; `SNIPE_DISABLE` (0) no lo permite, para matar a este más rápido.

**KS (`TACT_KS`)**: lo respetuosa que es la unidad con otros jugadores respecto a este monstruo.

| Constante | Valor | Significado |
|---|---|---|
| `KS_NEVER` | 0 | No ataca si el monstruo está atacando o siendo atacado por otro jugador. |
| `KS_POLITE` | -1 | No ataca mientras se mueve, porque podría estar persiguiendo a alguien (como `DoNotAttackMoving`). |
| `KS_ALWAYS` | 1 | Libre para todos: ataca sin considerar a otros jugadores. Eres responsable de lo que hace tu unidad: usarlo en monstruos que no son libres para todos puede provocar sanciones a tu cuenta. Muy desaconsejado. |

<a id="weight"></a>**Weight (`TACT_WEIGHT`)**: un número, predeterminado 1. Donde se cuentan
monstruos (habilidades de área, Berserk, Stein Wand, Castling, Provoke...), este monstruo cuenta
como esa cantidad. Dos monstruos de peso 0.4, uno de 0.5 y uno de 1.2 cuentan como
2 x 0.4 + 0.5 + 1.2 = 2.5.

**Chase (`TACT_CHASE`)**: como `DoNotChase`, solo para este monstruo.

| Constante | Valor | Significado |
|---|---|---|
| `CHASE_NORMAL` | -1 | Sigue `DoNotChase`. |
| `CHASE_ALWAYS` | 0 | Persigue incluso con `DoNotChase` = 1. Buena opción para Vanilmirth en OD2: configura los arqueros orcos con `CHASE_ALWAYS` y `DoNotChase` = 1 para que solo se mueva cuando sea necesario. |
| `CHASE_NEVER` | 1 | Nunca persigue, incluso con `DoNotChase` = 0. Aún puede seleccionar al monstruo como objetivo, así que úsalo con cuidado. |
| `CHASE_CLEVER` | 2 | Espera un pulso de SP (`ChaseSPPause`) al perseguir, incluso si esa opción está desactivada. |

<a id="5-advanced-settings"></a>
## 5. Configuración avanzada

Está en `H_Extra.lua` / `M_Extra.lua`, archivos Lua cargados después de todo lo demás.
Sobrescriben la GUI; compruébalos primero cuando un cambio guardado no surte efecto.
Los archivos distribuidos contienen ejemplos comentados.

| Configuración | Qué hace |
|---|---|
| `NewAutoFriend = 0` | Descomenta si la otra unidad usa otra IA (véase [Amigos](#2-friends)). |
| `FriendAttack[MOTION_...]` | Usa 1 para cada movimiento tuyo o de un amigo que cuenta como señal de ataque: atacar, usar habilidad, conjurar, lanzar (Spear Boom, pociones), Acid Bomb, Full Blast. |
| `BasicDebuffs[...]` | Qué debuffs se usan cuando el debuff está activado, pero la táctica no indica uno (por defecto: todos excepto trampas y Provoke). 1 activa uno de ellos. |
| `AggressiveRelogTracking = 1`, `AggressiveRelogPath` | Registra reconexiones escribiendo constantemente un archivo, para seguir las duraciones de habilidades entre ellas. Causa mucha actividad de disco y puede ralentizar algunos sistemas; no lo uses sin un disco rápido. |
| `MyRoute={{x1,y1},{x2,y2},...}` | Ruta para `UseIdleWalk` 5 o 6: lista de celdas transitables, cada una dentro de los límites de movimiento de sus vecinas. Con `RelativeRoute` 1, las celdas son desplazamientos respecto a ti. Una ruta larga puede necesitar muchos puntos. |
| `GetMyTact(e)` | Ejemplo comentado de alternar entre dos listas de tácticas según una condición. Funcionamiento no garantizado. |
| `LogEnable["AAI_..."]=1` | Más registros, escritos como `AAI_*.log` en la carpeta del cliente: `AAI_SKILLFAIL`, `AAI_CLOSEST`, `AAI_DANCE`, `AAI_ACTORS`, `AAI_TARGETING` (captura cada 5 s), `AAI_MOBCOUNT`, `AAI_PROBE` (respuestas sin procesar del sondeo de habilidades). `AAI_ERROR` está activado por defecto; `LogEnable["AAI_ERROR"]=0` lo oculta, pero, si el archivo crece demasiado, informa del problema. |

**PVP.** Con `PVPmode` = 1, otros jugadores se gestionan mediante `H_PVP_Tact.lua` /
`M_PVP_Tact.lua`, editados como tácticas normales, pero sin los campos SP, Chase, Weight y KS.
Clasifica jugadores añadiendo una entrada con el ID de la cuenta, o una línea en `A_Friends.lua`
con `KOS`, `ENEMY`, `NEUTRAL`, `FRIEND` o `ALLY` y una entrada de táctica PVP para esas clases.

<a id="6-installation-notes-from-the-original"></a>
## 6. Notas de instalación del original

- **Windows con UAC (Vista y posteriores):** si Ragnarok Online está en una carpeta protegida,
  como *Archivos de programa*, la IA puede no funcionar correctamente. Mueve la carpeta completa
  de RO a una ubicación no protegida, como `C:\Games`, elimina el contenido de `USER_AI` e
  instala de nuevo. No es necesario si el UAC está desactivado.
- **Actualización:** conserva `A_Friends.lua`. Para los demás archivos, véase el
  [manual](MANUAL.es.md#actualizaciones-y-respaldos).
- **Comprueba que funciona:** `AAIStartH.txt` / `AAIStartM.txt` aparece en la carpeta de RO
  cuando se ejecuta la IA. Si no aparece, la IA no está instalada correctamente.

<a id="7-what-changed-from-the-original-manual"></a>
## 7. Qué cambió respecto al manual original

| Tema | Manual original 1.50 | Esta versión |
|---|---|---|
| Servidores | Los servidores privados ilegales no están soportados. | rAthena/Hercules se pueden seleccionar con `ServerType` (probado solo offline y en rAthena local, no en todos los servidores). Sigue las reglas de tu servidor sobre IA personalizada. |
| Tecla de espera | Alt+R en un pasaje, Alt+T en otro. | Alt+T (homúnculo) y Ctrl+T (mercenario), como en `A_Friends.lua`. |
| Opciones Berserk | Escritas como `Berzerk`. | Los nombres funcionales son `UseBerserk...` y `Berserk_...`. |
| `UseBayeriHailageStar`, `UseBayeriGoldenFerse`, `UseStienWandTele`, `FilerFlitLevel` | | `UseBayeriHailegeStar`, `UseBayeriGoldenPherze`, `UseSteinWandTele`, `FilirFlitLevel`. |
| Recorrido de ruta | `UseRouteWalk` y un archivo `Route.lua`. | `UseIdleWalk` 5 o 6 y `MyRoute` en `H_Extra.lua` / `M_Extra.lua`. |
| `OldHomunType` | Definido en `H_Extra.lua`; Vanilmirth detectado automáticamente. | Definido en `H_Config.lua`, como respaldo; el tipo básico se detecta por las habilidades heredadas cuando se ha aprendido una. |
| `AoEFixedLevel` | Predeterminado 1. | Predeterminado 0 en el `H_Config.lua` distribuido. |
| `AggroHP` | Predeterminado 60. | 0 en el `H_Config.lua` distribuido, permitiendo iniciar combate con cualquier HP. |
| `UseAutoSkill_MinSP` (táctica SP) | | `AttackSkillReserveSP`. |
| `AutoMobMode` | No explicado. | 0 desactivado, 1 cuenta solo atacantes, 2 cuenta todos. |
| Archivos de tácticas PVP | `H_PVP_Tactics.lua` | `H_PVP_Tact.lua`, `M_PVP_Tact.lua` |
| Clases de habilidad de Eleanor | `CLASS_COMBO_1/2` | Más `CLASS_GRAPPLE`, `_1`, `_2`. |
| Nuevas opciones | | `ServerType`, `AutoDetectServerType`, `TrustSkillProbe`, `LintUserOptions`, `UseMobInfo` y opciones de la base de datos de monstruos, `TargetPriority`, `KiteByRange`, `UseDamageDanger`, `CommandedFightStay`, `StayAwayDistance`, `StayAwayHold`, `CastInterrupt*`, opciones de habilidades de Homúnculo S, ajustes por homúnculo en `Types/<Name>.lua`. |
| Historial de versiones | Changelog de 1.00 a 1.50. | No se repite aquí. Los cambios desde el original 1.56 están en las [notas de versión en inglés](RELEASE_NOTES.md#changes-since-the-original-156-release-200). |

<a id="8-homunculus-scripting-primer"></a>
## 8. Introducción a los scripts de homúnculos

La IA controlada por el cliente puede entenderse como una máquina de estados: permanece
inactiva, sigue a su dueño, persigue un objetivo o ataca. Es solo un modelo didáctico: el
AzzyAI actual tiene muchos más estados y comportamientos. El punto de entrada del homúnculo es
`AI.lua`; en este proyecto, las constantes compartidas, las funciones auxiliares y el
comportamiento se distribuyen en archivos como `Const_.lua`, `AzzyUtil.lua` y `AI_main.lua`.

Algunas funciones proporcionadas por el cliente y usadas por el código actual:

| Función | Propósito |
|---|---|
| `GetV(V_..., id)` | Lee una propiedad compatible de un objeto del juego, como posición, HP, SP o movimiento. |
| `GetActors()` | Obtiene los ID de los objetos que el cliente ve actualmente. |
| `GetTick()` | Obtiene el reloj del cliente en milisegundos, útil para programaciones y tiempos de espera. |
| `IsMonster(id)` | Comprueba si un objeto está clasificado como monstruo. |
| `GetMsg(id)`, `GetResMsg(id)` | Reciben comandos estructurados de la interfaz del cliente; no son mensajes arbitrarios de texto del chat. |
| `Move(id, x, y)`, `Attack(id, target)` | Solicitan movimiento o un ataque. AzzyAI envuelve estas llamadas para aplicar sus propias comprobaciones y comportamiento. |
| `SkillObject(id, level, skill, target)`, `SkillGround(id, level, skill, x, y)` | Solicitan una habilidad con objetivo o de suelo. Las reglas del cliente y del servidor siguen determinando si se ejecuta. |

Esta lista es un punto de partida, no un contrato completo de API válido para todos los
servidores. Los clientes varían y los wrappers de AzzyAI añaden comportamiento a las llamadas
originales. Para extensiones, es preferible usar `H_Extra.lua` y los archivos documentados de
configuración y tácticas en vez de reemplazar archivos centrales de la IA. Los ejemplos de
configuración de la guía antigua usan otra estructura de archivos; no los copies directamente
a esta versión.

**Créditos:** Este resumen original se basa en conceptos de la histórica
[Homunculus AI Script User's Guide](https://github.com/maxmx03/YggAI/blob/main/Homunculus%20AI%20Script%20User%20Guide.md),
que acredita a Gravity como autora original y a Max Del Canto por su actualización de 2024.
La guía está alojada en [YggAI](https://github.com/maxmx03/YggAI); las notas anteriores están
parafraseadas y adaptadas a AzzyAI 2.0. El código actual de este proyecto es la referencia para
los nombres y el comportamiento.

---

[Manual](MANUAL.es.md) · [README](../README.es.md) · [English](REFERENCE.md) · [Português (Brasil)](REFERENCE.pt-BR.md)
