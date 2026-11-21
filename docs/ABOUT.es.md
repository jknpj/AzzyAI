# Qué añade esta versión de AzzyAI

[English](ABOUT.md) | [Português (BR)](ABOUT.pt-BR.md) | [Español](ABOUT.es.md)

[Volver al README](../README.es.md) | [Manual del usuario](MANUAL.es.md)

## Cómo funciona

[![Cómo decide qué hacer AzzyAI: aparece un monstruo, se consulta en la lista, la IA decide y lanza dentro de las reglas del servidor](hg-how-es.svg)](hg-how-es.svg)

El homúnculo lee qué monstruo está viendo y lo consulta en una lista de unos
2.700 monstruos. Tus ajustes y tácticas deciden entonces a quién atacar y qué
habilidad usar, y los datos del servidor mantienen cada lanzamiento dentro de lo
que tu servidor acepta. [Abrir la figura en tamaño completo](hg-how-es.svg).

## Qué puedes configurar

| Homúnculo | Habilidades que puede usar | Conviene saber |
|---|---|---|
| **Mercenarios** | Tácticas, habilidades y kiting, como en el AzzyAI original | No usan la base de datos de monstruos. |

Las habilidades solo funcionan si tu servidor las ofrece y el homúnculo las ha
aprendido. El [manual del usuario](MANUAL.es.md#6-consejos-por-homúnculo-s) tiene notas sobre cada uno.

## Comparación con el AzzyAI original

El AzzyAI original (la rama `legacy/classic`) sigue siendo una buena IA. Esta versión conserva lo
que hace y añade lo siguiente. Esto es lo que notarías en el juego:

| | AzzyAI original | Esta versión |
|---|---|---|
| **¿Qué monstruo es?** | Sin datos de monstruos. Tú escribes una fila de táctica para cada uno. | Conoce el elemento, la raza y si es jefe/MVP/planta de cada monstruo (unos 2.700), así que funciona bien con monstruos que nunca configuraste. |
| **Habilidades de área** | Cuenta monstruos, sea cual sea su elemento. | Cuenta solo los monstruos que realmente recibirían daño y apunta a ellos, incluso cuando el de enfrente es inmune. |
| **Plantas y hongos** | Adivina las plantas por su comportamiento (`AutoDetectPlant`). | Sabe exactamente cuáles son plantas (reciben 1 de daño de todo) y guarda el SP para los demás. |
| **MVP** | Solo lo que pongas en las tácticas. | Por defecto no *empieza* peleas con un MVP, pero igual te defiende. Opcional: atacar siempre a los MVP. |
| **Un lanzamiento que se interrumpe una y otra vez** | Un límite fijo de reintentos (`SkillRetryLimit`), nada más inteligente. | Pausa la habilidad unos segundos tras algunas interrupciones y usa otra cosa. |
| **Los números de tu servidor** | Un único conjunto de valores. | Datos oficiales (RO LATAM, Episodio 19) o de rAthena, elegidos con `ServerType`, para que cooldowns, alcances y costos coincidan con tu servidor. |
| **Configuración** | Aplicación Windows. | La aplicación Windows y una **página en el navegador** en English, Português y Español, con descripciones de habilidades, presets por homúnculo y sugerencia de tácticas de monstruos por mapa. |
| **Actualizaciones** | Manuales. | La aplicación Windows actualiza la IA y los datos de monstruos por ti, sin tocar tus ajustes. |
| **Pruebas** | Sin suite de pruebas sin conexión. | Cada cambio se ejecuta contra un servidor simulado (más de 130 escenarios) antes de publicarse. |

Las funciones que reconocen monstruos son para **homúnculos**. Los mercenarios
reciben las herramientas y las correcciones de errores, pero no la base de datos
de monstruos.

## ¿Vale la pena cambiar?

ajustar el comportamiento sin leer código, o cazas grupos de monstruos y quieres
que las habilidades de área se usen bien.

**Quizá no lo necesites si** tu IA actual ya hace lo que quieres, o si tu
servidor tiene reglas de habilidades distintas y prefieres no probar.

**Probarla es seguro:** haz un respaldo de la carpeta `AI` antes. Volver atrás
es restaurar ese respaldo. Tus tácticas y tu lista de amigos se pueden conservar
(consulta el [checklist de actualización](MANUAL.es.md#actualizaciones-y-respaldos)).

## Qué cambia por defecto

Esto viene activado desde el inicio; cada punto se puede desactivar en la
herramienta de configuración.

- **Base de datos de monstruos** (`UseMobInfo`): se tienen en cuenta el elemento, las plantas y los jefes.
- **MVP** (`BossAggro`): el homúnculo no empieza peleas con MVP.
- **Peleas ordenadas** (`CommandedFightStay`): un monstruo al que le ordenas atacar se combate hasta que muera.

Extras opcionales, desactivados por defecto: elección de objetivo más
inteligente (`TargetPriority`), mantener distancia según el alcance real del
monstruo (`KiteByRange`), kitear antes del primer golpe (`KiteParanoid`) y
escapar cuando el HP baja rápido (`UseDamageDanger`).
Las pruebas sin conexión los cubren, pero aún no se han verificado en el juego.

## Estado real

Esta versión se prueba sin conexión y en un servidor rAthena local. Eso no garantiza que
funcione en todo cliente o servidor oficial. Las habilidades nuevas solo
funcionan si el servidor las admite y el homúnculo las ha aprendido. Activar una
habilidad no garantiza que se use en cada ataque: el alcance, SP, cooldown y
elemento también cuentan. Los cooldowns del servidor, costos de SP e
interrupciones de lanzamiento siguen vigentes.
