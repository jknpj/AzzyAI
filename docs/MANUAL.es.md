# Manual del usuario: AzzyAI HG

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Volver al README](../README.es.md) | [Notas técnicas en inglés](RELEASE_NOTES.md)

## 1. Antes de empezar

Usa la IA solo donde tu servidor lo permita. HG configura el comportamiento:
no desbloquea habilidades, elimina cooldowns ni evita interrupciones por daño.
Clientes y servidores pueden comportarse de manera diferente. Empieza con
monstruos fáciles.

El homúnculo y el mercenario tienen ajustes separados. Los nombres de las
opciones se mantienen en inglés para coincidir con las herramientas.

## 2. Instalar y activar

1. Cierra las ventanas de configuración y respalda toda la carpeta `AI` del juego.
2. Descarga un paquete de release, cuando esté disponible, o la rama **LATAM-HG**.
3. Coloca el contenido de `USER_AI` del proyecto en `AI\USER_AI` del juego.
   El resultado debe contener `AI\USER_AI\AI.lua`, sin otra carpeta anidada.
4. Escribe `/hoai` hasta seleccionar la IA personalizada del homúnculo.
   Para mercenarios, usa `/merai`.
5. Invoca nuevamente o reconéctate. Repite la recarga después de guardar ajustes.

Instalar HG reemplaza la IA de esa carpeta. Para conservar otra IA en un tipo
de unidad, preserva su archivo de entrada: `AI.lua` para homúnculos o `AI_M.lua`
para mercenarios. No mezcles versiones de los archivos de soporte de HG.

### Actualizaciones y respaldos

Antes de actualizar, conserva una copia de:

| Archivos | Contenido |
|---|---|
| `H_Config.lua`, `M_Config.lua` | Ajustes del homúnculo y mercenario |
| `H_Tactics.lua`, `M_Tactics.lua` | Comportamiento por monstruo |
| `H_PVP_Tact.lua`, `M_PVP_Tact.lua` | Comportamiento en PVP |
| `H_Extra.lua`, `M_Extra.lua` | Opciones extra y código personalizado |
| `A_Friends.lua` y `data` | Lista de amigos e información guardada por la IA |

Guarda el respaldo completo fuera de la carpeta de IA del juego.
No sobrescribas archivos personalizados sin revisarlos ni sustituyas el código
nuevo con la carpeta antigua completa. Compara/importa los ajustes en la
instalación actualizada, guarda, recarga y prueba.

La GUI de Windows puede hacerlo por ti (menú **Update**, sección 3). Nunca
sobrescribe los archivos de la tabla anterior (ni `Types\*.lua`, salvo que
marques la opción) y hace copia de seguridad de todo lo que reemplaza.

## 3. Configurar con las GUIs

### Aplicación Windows

Ejecuta `AzzyAIConfig.exe` en `AI\USER_AI` instalada en el juego. Edita los
archivos junto al ejecutable, incluso al abrirlo mediante un acceso directo.
Comprueba la ubicación de la configuración antes de guardar.

- **Homunculus / Mercenary:** comportamiento general, habilidades y buffs.
- **Homunculus Tactics / Mercenary Tactics:** comportamiento por monstruo.
- **PVP Tactics / Merc PVP Tactics:** comportamiento para jugadores.
- **Extra Options:** opciones avanzadas y Lua personalizado; evita editar código si no sabes.

La búsqueda encuentra nombres y descripciones. Los filtros por tipo ayudan a
encontrar opciones; **ocultar un ajuste no lo desactiva**. Usa **Clear filters**
para ver todo. **Apply** guarda; **Revert** descarta cambios pendientes.
Las etiquetas de la aplicación Windows están en inglés.

**Menú Update (solo en la aplicación Windows):**

- **Update AI from git...** hace `git pull --ff-only` en tu clon local o descarga
  la rama `LATAM-HG` de GitHub (sin necesitar git) y copia su `USER_AI` sobre la
  carpeta donde se ejecuta la GUI. Tus ajustes, tactics, extra y listas de avoid
  se conservan; con *Also overwrite the homunculus presets* marcado, también se
  reemplazan los `Types\*.lua`. Lo reemplazado se copia a `backup\update-<hora>\`.
  Un `AzzyAIConfig.exe` más nuevo se guarda como `.exe.new` y se cambia al
  aceptar el reinicio. Recarga la IA en el juego después.
- **Update monster/skill data from rAthena...** ejecuta `tools\harness\rathena_sync.py`
  en tu clon (necesita Python 3 e internet) y copia aquí el `MobInfo.lua`
  regenerado. El clon también recibe los fixtures del harness y
  `webgui\mobdata.js`, que haces commit si mantienes el repositorio.

La carpeta del clon se guarda en `AzzyAIConfig.update.txt`.

### Navegador

Abre [la web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Elige **AzzyAI HG**, tu idioma y **File > Open USER_AI folder** para abrir la
carpeta instalada en el juego.

- Chrome/Edge pueden escribir directamente si el acceso a carpetas está admitido y autorizado.
- Sin escritura directa, usa el ZIP descargado/exportado y extrae los archivos
  de configuración en `AI\USER_AI` del juego.
- Sin abrir la carpeta, la página no lee tus ajustes instalados.
- Una configuración recordada por el navegador puede no coincidir con los archivos actuales.

Haz clic en **Apply**, confirma la escritura/exportación y recarga la IA.
Algunos recursos del navegador necesitan conexión a internet.

### Configuración por tipo de homúnculo

Ambas GUIs tienen el selector **Configuración de** en la pestaña Homúnculo. *Global* edita `H_Config.lua`; elegir un tipo (Lif, Amistr, Filir, Vanilmirth, Eira, Bayeri, Sera, Dieter, Eleanor) edita `Types/<Nombre>.lua`, que solo guarda los valores distintos de los globales y se aplica después de `H_Config.lua` cuando ese homúnculo inicia. **Usar valores globales** quita los ajustes del tipo. El selector también reemplaza los antiguos filtros Homúnculo S / Homúnculo base: un Homúnculo S lista sus habilidades, uno base lista solo las suyas y *Global* lista todo. Borrar el archivo del tipo desactiva el preset. Dieter viene con `AutoMobCount = 1`, porque todos sus ataques son de área y el valor global 2 lo dejaba casi inactivo contra un solo monstruo.

## 4. Comportamiento básico

| Opción | Significado práctico |
|---|---|
| `ServerType` | **Official** para oficiales; **Private** para rAthena/Hercules. |
| `UseAttackSkill` | Permite habilidades ofensivas automáticas. |
| `UseHomunSSkillAttack` / `UseHomunSSkillChase` | Permite habilidades S al atacar / perseguir. |
| `AggroHP` / `AggroSP` | Porcentaje de HP/SP necesario para buscar objetivos. Aumentarlo puede reducir la agresividad. |
| `SuperPassive` | Impide el combate normal; revísalo si no ataca. |
| `DoNotChase` | Impide perseguir objetivos. Uno lejano puede no entrar en alcance. |
| `KiteMonsters` | Permite mantener distancia si las tácticas también lo permiten. |
| `UseDanceAttack` | Activa movimiento durante ataques. Puede cambiar la posición; no elimina cooldowns. |

La detección automática ayuda en algunos clientes privados, pero elige el tipo
de servidor explícitamente al investigar problemas. La detección del tipo
original y del nivel de habilidades ayuda con las heredadas, pero no sustituye
aprenderlas.

## 5. Habilidades, área y SP

Activa la habilidad deseada y usa un nivel no superior al aprendido.
Puede no seleccionarse por:

- SP insuficiente o la reserva configurada.
- Cooldown, alcance o elemento del objetivo.
- Requisitos de área o tácticas del monstruo.
- Otra habilidad disponible con mayor prioridad.
- Un lanzamiento, combo o pausa temporal en curso.

`AutoMobMode` controla ataques automáticos de área. `AutoMobCount` define la
cantidad necesaria; **1 puede hacer que las habilidades de área compitan incluso
contra un solo objetivo**. La eficacia elemental también puede influir en el
conteo. `AoEMaximizeTargets` ayuda a apuntar áreas; las habilidades centradas
en el homúnculo siguen necesitando monstruos cercanos.

`AttackSkillReserveSP` conserva SP en lugar de gastarlo todo en ataques.
`AutoSkillDelay` es una espera adicional de la IA. **0 elimina solo esa espera**,
no cooldowns, tiempo de lanzamiento ni interrupciones del servidor.
Menor no siempre es mejor. El preset incluido y los valores internos de reserva
de la IA pueden diferir; importa el valor del archivo instalado.

## 6. Consejos por Homúnculo S

| Homúnculo | Qué revisar |
|---|---|
| Eira | Activa los ataques deseados, como Twister Cutter o Absolute Zephyr. Light of Regenerate es opcional y está desactivada por defecto: exige lealtad, reduce intimidad al lanzarse y sacrifica a Eira para resucitar al dueño. |
| Bayeri | **Glanzen Spies** usa un objetivo; **Heilige Pferd** está centrada en Bayeri. El preset de Bayeri activa Pferd y Spies y elige según la DEF del objetivo (de `MobInfo.lua`): Spies solo por encima de DEF 320, si no Stahl Horn; con exactamente dos monstruos, Stange/Pferd solo desde DEF 110. |
| Sera | Toxin of Mandara y Needle Stinger tienen lanzamientos que pueden interrumpirse. Mantén el objetivo en alcance y revisa las pausas por fallos. Pain Killer y Summon Legion tienen sus propios ajustes. |
| Dieter | Activa ataques y buffs por separado. La resistencia o inmunidad al fuego puede impedir seleccionar habilidades de fuego. |
| Eleanor | Sonic Claw y la cadena de lucha exigen estilo de lucha; Tinder Breaker/CBC/EQC exigen agarre. Importan esferas, SP y tiempos del combo. The One Fighter Rises y Blazing And Furious son habilidades de área: siguen `AutoMobMode`/`AutoMobCount` en lugar de usarse contra un solo objetivo, salvo The One Fighter Rises contra un objetivo aislado con DEF 300 o más. Con 3+ monstruos omite las cadenas de Sonic Claw, y Blazing And Furious solo se usa con 6+ esferas. |
| Vanilmirth | **Caprice** (heredada también por los Homunculus S basados en Vanilmirth) se controla con `UseVanilCaprice` (activada por defecto). Chaotic Blessing sigue `UseAutoHeal`. Self Destruct nunca se usa automáticamente. |
Son consejos de configuración, no un ranking ni una build óptima. Las reglas
del servidor pueden variar, incluso el alcance y el tipo de objetivo.

El preset incluido usa `AutoMobCount` 2 y deja desactivadas **Twister Cutter**
(Eira) y **Needle Stinger** (Sera) (los presets por tipo en `Types/` activan la
Spies de Bayeri y la Blazing And Furious de Eleanor), porque la IA las prefiere a habilidades más baratas que
rindieron al menos igual en pruebas sin conexión, y activa **Blast Forge**
(Dieter). Estas elecciones vienen de una simulación, no de un servidor oficial:
activa cualquiera si lo prefieres. Todas las habilidades de ataque de Dieter son de área, así que con
`AutoMobCount` 2 no ataca a un solo monstruo con ellas: pon `AutoMobCount` en 1 si
luchas sobre todo contra un monstruo a la vez con Dieter (Bayeri y Sera pierden
daño contra monstruos aislados). Los detalles están en las
[notas técnicas](RELEASE_NOTES.md#default-preset-tuning) y en las
[tablas de comparación](preset-tuning/README.md).

### Eleanor: combos o solo Sonic Claw

Para combos, elige **Always** o **Tactics** en `AutoComboMode` y revisa
`AutoComboSpheres`: es el umbral de esferas para **iniciar** una cadena
automática, no un requisito repetido en cada paso. Un valor alto hace esperar
más. Las habilidades ofensivas nuevas pueden competir con el combo.

Para un experimento ofensivo solo con Sonic:

1. Activa `UseEleanorSonicClaw`.
2. Pon `AutoComboMode` en **Never** y desactiva `Berserk_ComboAlways`.
3. Desactiva Tinder Breaker, CBC, EQC, Blazing And Furious y The One Fighter Rises.
4. Pon a Eleanor en **estilo de lucha** y recarga la IA después de guardar.

Los ataques normales y buffs habilitados siguen activos. El seguimiento de
cambios manuales de estilo depende de lo que informa el cliente.
Si juego e IA discrepan después de un cambio manual de estilo, recarga y prueba
otra vez; los iconos de esferas no siempre son un conteo exacto en tiempo real.

`SphereTrackInterval` es un ajuste avanzado de estimación (predeterminado
**800 ms**), no de velocidad ni cooldown. Redúcelo solo después de confirmar
ataques normales más rápidos; demasiado bajo puede causar rechazos.
Una opción en `H_Extra.lua` tiene prioridad sobre el valor guardado por la GUI.

## 7. Interrupciones repetidas

Encuentra estas opciones en **AutoSkill Options**:

| Opción | Predeterminado | Qué hace |
|---|---|---|
| `CastInterruptLimit` | 3 | Omite temporalmente una habilidad tras ese número de fallos contados. **0 desactiva**. |
| `CastInterruptPause` | 5000 | Duración en milisegundos: **5000 = 5 segundos**. |
| `CastInterruptUnconfirmed` | Desactivada | Cuenta también fallos sin lanzamiento/finalización visible, con objetivo vivo en alcance y SP suficiente. |

Normalmente solo cuentan lanzamientos ofensivos observados e interrumpidos con
evidencia de daño. Un lanzamiento completado reinicia el contador de esa
habilidad. Durante la pausa, pueden elegirse otras habilidades disponibles o
ataques normales. Buffs, combos y solicitudes manuales de habilidades no
activan esta pausa.

Usa `CastInterruptUnconfirmed` solo si el cliente oculta lanzamientos y los
fallos repetidos no se cuentan. **Puede contar rechazos del servidor, no solo
interrupciones.** No la uses para ocultar ajustes incorrectos de SP, alcance o
habilidades. Contadores y pausas se reinician cuando el juego recarga la IA.
Estas opciones no vuelven los casts ininterrumpibles ni cambian sus reglas.

## 8. Tácticas de monstruos

En las pestañas de tácticas, decide qué monstruos ignorar, priorizar o atacar
y si usar habilidades. Revisa la fila predeterminada y cualquier fila
específica del objetivo.

La web GUI puede **Suggest tactics** por mapa o monstruo. Revisa las sugerencias:
la base de monstruos puede diferir de tu servidor.

- `UseMobInfo`: usa la información de monstruos incluida.
- `PlantSkills`: desactivada por defecto para no gastar habilidades en plantas.
- `BossAggro`: por defecto evita iniciar combate con MVPs, pero permite defenderse.
- `TargetPriority`: orden inteligente opcional entre objetivos.
- `KiteByRange`: distancia opcional basada en alcance, con kiting habilitado.

Para una prueba controlada con un muñeco, pon ese monstruo en **Ignore** para
la selección automática y ordena atacar con **Alt + clic derecho**.
Esto afecta a todos los monstruos con el mismo ID de táctica, no solo un muñeco.

## 9. Problemas frecuentes

| Síntoma | Revisa primero |
|---|---|
| No ataca nada | `/hoai`, carpeta instalada, recarga, modo pasivo, servidor, tácticas y umbrales de HP/SP. |
| Ataque manual funciona; automático no | Agresividad, tácticas del objetivo, alcance y restricciones de kill-steal. |
| Habilidad nueva nunca aparece | Nivel aprendido, activación, SP, alcance, umbral de área y habilidades competidoras. |
| El cast comienza y falla repetidamente | Interrupciones por daño, alcance, SP y pausas; no todo fallo es una interrupción. |
| La GUI no cambia el juego | Apply/exportación, carpeta correcta, recarga y opciones en `H_Extra.lua`. |
| Eleanor usa la cadena equivocada | Estilo, combos y esferas; recarga después de cambiar ajustes. |

`AAIStartH.txt`, en la carpeta del cliente, registra el inicio de la IA del
homúnculo. `AAI_ERROR.log` puede mostrar errores y pausas ofensivas temporales.
Si el archivo inicial no se actualiza, revisa permisos y activación de la IA.
Los pasos detallados están en las
[notas técnicas](RELEASE_NOTES.md#troubleshooting-automatic-attacks).

Al reportar, incluye servidor/cliente, homúnculo/tipo original, habilidades
activadas, pasos y logs relevantes. Elimina datos de cuenta, contraseñas e
información privada. Nunca envíes archivos de credenciales.
Usa [Issues](https://github.com/jknpj/AzzyAI/issues).
