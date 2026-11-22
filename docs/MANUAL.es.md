# Manual del usuario: AzzyAI

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Volver al README](../README.es.md) | [Referencia técnica en inglés](TECHNICAL.md) | [Referencia de comandos, opciones y tácticas](REFERENCE.es.md)

## 1. Antes de empezar

Usa la IA solo donde tu servidor lo permita. AzzyAI configura el comportamiento:
no desbloquea habilidades, elimina cooldowns ni evita interrupciones por daño.
Clientes y servidores pueden comportarse de manera diferente. Empieza con
monstruos fáciles.

El homúnculo y el mercenario tienen ajustes separados. Los nombres de las
opciones se mantienen en inglés para coincidir con las herramientas.

## 2. Instalar y activar

1. Cierra las ventanas de configuración y respalda toda la carpeta `AI` del juego.
2. Descarga un paquete de release, cuando esté disponible, o la rama **prerenewal**.
3. Coloca el contenido de `USER_AI` del proyecto en `AI\USER_AI` del juego.
   El resultado debe contener `AI\USER_AI\AI.lua`, sin otra carpeta anidada.
4. Escribe `/hoai` hasta seleccionar la IA personalizada del homúnculo.
   Para mercenarios, usa `/merai`.
5. Invoca nuevamente o reconéctate. Repite la recarga después de guardar ajustes.

Instalar AzzyAI reemplaza la IA de esa carpeta. Para conservar otra IA en un tipo
de unidad, preserva su archivo de entrada: `AI.lua` para homúnculos o `AI_M.lua`
para mercenarios. No mezcles versiones de los archivos de soporte de AzzyAI.

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

La aplicación de escritorio puede hacerlo por ti (**Update AI from GitHub...**, sección 3). Nunca
sobrescribe los archivos de la tabla anterior (ni `Types\*.lua`, salvo que
marques la opción) y hace copia de seguridad de todo lo que reemplaza.

## 3. Configurar con las GUIs

### Aplicación Windows

Ejecuta `AzzyAIConfig.exe` desde la carpeta `AI\USER_AI` instalada. Edita los archivos de
esa carpeta, incluso si se abre desde un acceso directo. Revisa la ruta que muestra el pie
antes de guardar.

La ventana de configuración muestra el conjunto de opciones Pre-Renewal: cuatro homúnculos y sin grupos de Homúnculo S.

La aplicación aún no tiene firma de código, así que SmartScreen puede mostrar "Windows
protegió tu PC" en una descarga nueva (un antivirus también puede marcar un ejecutable
pequeño y poco descargado). El código está en este repositorio y el CI genera ese archivo
a partir de él; para continuar, haz clic en **Más información > Ejecutar de todos modos**.
La configuración por el navegador no necesita descarga.

- **Homunculus / Mercenary:** comportamiento general, habilidades y buffs.
- **Homunculus Tactics / Mercenary Tactics:** comportamiento para cada monstruo.
- **PVP Tactics / Merc PVP Tactics:** comportamiento para jugadores.
- **Extra Options:** opciones avanzadas y Lua personalizado; evita tocar el código si no
  sabes lo que haces.

La búsqueda encuentra nombres y descripciones. Los filtros de tipo ayudan a encontrar
opciones; **ocultar una opción no la desactiva**. Usa **Limpiar filtros** para ver todo.
Haz clic en **Aplicar configuración** para guardar; **Recargar** descarta los cambios
pendientes. La interfaz y las descripciones de las opciones están en inglés, portugués y
español; elige el idioma en la barra superior.

**Update AI from GitHub...** (barra superior): descarga el zip oficial de la rama `prerenewal` y
lo copia sobre la carpeta de destino que muestra la ventana (editable, así puedes
actualizar otra instalación). Tu configuración, tácticas, extras y listas de avoid se
conservan; con *Replace Types/ presets too* marcado, `Types\*.lua` también se reemplaza.
Los archivos reemplazados van a `backup\update-<hora>\`. Un `AzzyAIConfig.exe` más nuevo
se guarda como `.exe.new`, y **Restart now** cierra la ventana, lo cambia y abre la nueva
versión. Recarga la IA en el juego después. Si no hay cambios pendientes, la ventana
recarga los archivos sola.

Al iniciar, la ventana consulta la etiqueta del último release y, si es más nueva que el
`CVersion` instalado, muestra una barra de **Update**.

**Files** (barra superior): exporta todos los archivos de configuración (incluidos los
cambios sin aplicar), importa una carpeta o exporta un diagnóstico. El diagnóstico tiene
el mismo contenido que el del navegador: el informe de inicio, las últimas líneas de los
logs, la configuración, las tácticas y extras, `Types\*.lua` y los archivos de `data`, con
los IDs de dueño reemplazados por `<owner>`. **Help** muestra la versión y los enlaces a la
documentación, la web GUI y el formulario de problema.

### Navegador (sin Windows)

¿Sin Windows, o prefieres el navegador? Abre [la web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Elige **AzzyAI** (no el Classic), tu idioma y **File > Open USER_AI folder** para abrir la
carpeta instalada en el juego. **Help > Export diagnostics for a bug report...** arma el mismo paquete
que la aplicación de Windows (un `.zip` con el informe de inicio, los logs, la configuración y los
niveles de habilidad aprendidos, IDs de dueño cambiados por `<owner>`) desde la carpeta del juego que
elijas, y luego abre el formulario de incidencia en GitHub.

- Chrome/Edge pueden escribir directamente si el acceso a carpetas está admitido y autorizado.
- Sin escritura directa, usa el ZIP descargado/exportado y extrae los archivos
  de configuración en `AI\USER_AI` del juego.
- Sin abrir la carpeta, la página no lee tus ajustes instalados.
- Una configuración recordada por el navegador puede no coincidir con los archivos actuales.

Haz clic en **Apply**, confirma la escritura/exportación y recarga la IA.
Algunos recursos del navegador necesitan conexión a internet.

### Configuración por tipo de homúnculo

Ambas GUIs tienen el selector **Configuración de** en la pestaña Homúnculo. *Global* edita `H_Config.lua`; elegir un tipo (Lif, Amistr, Filir, Vanilmirth) edita `Types/<Nombre>.lua`, que solo guarda los valores distintos de los globales y se aplica después de `H_Config.lua` cuando ese homúnculo aparece. **Usar valores globales** elimina las modificaciones del tipo. Borrar el archivo del tipo desactiva su preset.

## 4. Comportamiento básico

| Opción | Significado práctico |
|---|---|
| `ServerType` | **Official** para oficiales; **Private** para rAthena/Hercules. |
| `UseAttackSkill` | Permite habilidades ofensivas automáticas. |
| `AggroHP` / `AggroSP` | Porcentaje de HP/SP necesario para buscar objetivos. Aumentarlo puede reducir la agresividad. |
| `SuperPassive` | Impide el combate normal; revísalo si no ataca. |
| `DoNotChase` | Impide perseguir objetivos. Uno lejano puede no entrar en alcance. |
| `KiteMonsters` | Permite mantener distancia si las tácticas también lo permiten. |
| `UseDamageDanger` | Desactivado por defecto. Si el homúnculo pierde `DamageDangerDrop`% (30) de su HP máximo en 3 s con HP por debajo de `DamageDangerHP`% (60), kitea aunque `KiteMonsters` esté desactivado. Solo ve su propio HP, no los buffs de los enemigos. |
| `UseDanceAttack` | Activa movimiento durante ataques. Puede cambiar la posición; no elimina cooldowns. |
| `CommandedFightStay` | Activado por defecto. Un monstruo que ordenas atacar al homúnculo se combate hasta que muera o des otra orden, aunque te alejes. Usa 0 para que desista y vuelva cuando el objetivo salga de los límites de movimiento o pase el tiempo límite de ataque. |
| `StationaryMoveBounds` / `MobileMoveBounds` | Hasta qué distancia de ti puede ir para reaccionar (estando tú quieto / en movimiento). Ambos vienen en 14; reduce `MobileMoveBounds` si la unidad se queda atrás cuando corres. |
| `AutoSkillLimit` | Tope de habilidades ofensivas usadas contra un objetivo (0 = sin límite). Se aplica sobre la táctica de habilidad del monstruo: gana el número menor de los dos. La cuenta se reinicia cuando cambia el objetivo. |

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
- Otra habilidad disponible con mejor puntuación (ver abajo).
- Un lanzamiento, combo o pausa temporal en curso.

Las habilidades de ataque se usan en el orden configurado (la habilidad de ataque
habitual, luego las prioridades por tipo y las tácticas), no puntuadas por daño:
el modelo de daño Renewal que las comparaba no existe en Pre-Renewal. Una
habilidad se sigue omitiendo cuando el elemento del objetivo bajaría su daño por
debajo de `SkillElementMinRate` (`MobInfo.lua`), cuando no alcanza el SP o cuando
la táctica no lo permite. `SkillElementMinRate` en 0 ignora la comprobación de
elemento.

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

## 6. Homúnculos en esta versión

Esta es la versión Pre-Renewal, así que el homúnculo es uno de los cuatro
originales: **Lif, Amistr, Filir o Vanilmirth**. Cada uno compra sus propias
habilidades con los puntos de evolución que da el juego, y las opciones siguen
las habilidades que ha aprendido.

No hay habilidades más allá de las que aprenden los cuatro originales, así que
no hay nada extra que configurar; las opciones que las controlarían no existen
(tampoco en las GUIs). Véase
[PRERENEWAL.md](PRERENEWAL.md) para saber qué incluye esta versión.

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

### Qué cambia la base de datos de monstruos (solo homúnculo)

Esta versión incluye una lista de unos 2.700 monstruos con su elemento, raza y si cada uno
es jefe, MVP o planta. El homúnculo lee qué monstruo está viendo y usa esa lista,
así que actúa con sentido incluso con monstruos para los que nunca escribiste una
fila de táctica. Los mercenarios no la usan.

| Opción | Por defecto | En palabras simples |
|---|---|---|
| `UseMobInfo` | Activada | Interruptor general. Desactivada = el comportamiento anterior, y ninguna de las opciones siguientes hace efecto. |
| `SkillElementMinRate` | 50 | La habilidad se omite cuando el elemento del monstruo reduciría su daño por debajo de este porcentaje, y la IA ataca de otra forma. Los monstruos inmunes siempre se omiten. Baja el número para usar habilidades con más libertad; 0 omite solo los inmunes. |
| `PlantSkills` | Desactivada | Las plantas y los hongos reciben 1 de daño de todo, así que no se gastan habilidades en ellos. Actívala para usar habilidades en ellos de todos modos. |
| `BossAggro` | Reaccionar a MVPs | El homúnculo nunca *empieza* una pelea con un MVP, pero igual te defiende a ti y a sí mismo. `React to bosses` incluye los mini-jefes; `Tactics` deja que decidan tus tácticas. |
| `AttackMVPs` | Desactivada | Ataca siempre a los MVP. Pesa más que `BossAggro`. |
| `AttackOnlyMVPs` | Desactivada | `1` = los demás monstruos solo reciben una reacción, `2` = los demás monstruos se ignoran. Útil para limpiar una instancia. |
| `TargetPriority` | Desactivada | Elige el siguiente objetivo por algo más que la distancia. Entre monstruos con la misma prioridad de táctica: primero los que te atacan a ti o a un amigo, las plantas al final, luego los monstruos que tus habilidades pueden dañar, luego el centro del grupo más grande si tienes una habilidad de área y, por último, el más cercano. |
| `KiteByRange` | Desactivada | Con el kiting activado, mantiene la distancia según el alcance real del monstruo: un arquero hace que retroceda antes y más lejos que un monstruo cuerpo a cuerpo. No huye de un monstruo cuyo alcance llega a `KiteBounds`, porque no habría adónde ir. Solo importa cuando el kiting ya funciona para ti: `KiteMonsters` activado, la táctica de kite del monstruo definida y `ForceKite` para homúnculos sin ataque a distancia. |

La lista de monstruos viene de rAthena (Renewal), así que algunos monstruos
pueden ser distintos en tu servidor. `TargetPriority` y `KiteByRange` son nuevas
y aún no se han verificado en el juego; pruébalas primero con monstruos fáciles.

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

`AAIStartH.txt`, en la carpeta del cliente, registra el inicio de la IA del
homúnculo. `AAI_ERROR.log` puede mostrar errores y pausas ofensivas temporales.
Si el archivo inicial no se actualiza, revisa permisos y activación de la IA.
Los pasos detallados están en las
[notas técnicas](TECHNICAL.md#troubleshooting-automatic-attacks).

Al reportar, incluye servidor/cliente, homúnculo/tipo original, habilidades
activadas, pasos y logs relevantes. Elimina datos de cuenta, contraseñas e
información privada. Nunca envíes archivos de credenciales.
Usa [Issues](https://github.com/jknpj/AzzyAI/issues).
