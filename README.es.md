# AzzyAI HG para Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

Una IA más inteligente para tu **homúnculo o mercenario**. Decide a quién
atacar, qué habilidad usar y cuándo, y tú lo ajustas en una ventana o en el
navegador, sin editar código.

HG es una versión comunitaria basada en [AzzyAI original](https://github.com/SpenceKonde/AzzyAI),
no un producto oficial de Gravity. Esta rama está en pruebas para RO LATAM y
servidores privados; el comportamiento y las habilidades disponibles dependen
del servidor. Respeta sus reglas sobre IA personalizada.

## ¿Por qué usar esta?

El AzzyAI original (la rama `master`) sigue siendo una buena IA. HG conserva lo
que hace y añade lo siguiente. Esto es lo que notarías en el juego:

| | AzzyAI original | HG |
|---|---|---|
| **¿Qué monstruo es?** | Sin datos de monstruos. Tú escribes una fila de táctica para cada uno. | Conoce el elemento, la raza y si es jefe/MVP/planta de cada monstruo (unos 2.700), así que funciona bien con monstruos que nunca configuraste. |
| **Habilidades y elemento** | Usa la habilidad incluso contra un monstruo que la resiste. | Omite la habilidad que haría menos de la mitad del daño (ajustable) y usa otro ataque. Dieter no lanza Lava Slide contra un monstruo de Fuego. |
| **Habilidades de área** | Cuenta monstruos, sea cual sea su elemento. | Cuenta solo los monstruos que realmente recibirían daño y apunta a ellos, incluso cuando el de enfrente es inmune. |
| **Plantas y hongos** | Adivina las plantas por su comportamiento (`AutoDetectPlant`). | Sabe exactamente cuáles son plantas (reciben 1 de daño de todo) y guarda el SP para los demás. |
| **MVP** | Solo lo que pongas en las tácticas. | Por defecto no *empieza* peleas con un MVP, pero igual te defiende. Opcional: atacar siempre a los MVP. |
| **Un lanzamiento que se interrumpe una y otra vez** | Un límite fijo de reintentos (`SkillRetryLimit`), nada más inteligente. | Pausa la habilidad unos segundos tras algunas interrupciones y usa otra cosa. |
| **Niveles de habilidades** | La mayoría de las habilidades de Homúnculo S se detienen en el nivel 5 en su tabla. | Niveles máximos actuales (10 en la mayoría) y solo lanza niveles que el homúnculo ya aprendió. |
| **Habilidades de Homúnculo S** | Lo básico de Eira, Bayeri, Sera, Dieter y Eleanor. | También Twister Cutter, Absolute Zephyr, Toxin of Mandara, Needle Stinger, Glanzen Spies, Heilige Pferd, Goldene Tone, Blazing And Furious, The One Fighter Rises, Blast Forge y Tempering. |
| **Eleanor** | Cambio de estilo y combos. | También esferas según las reglas del servidor (cuánto cuesta cada habilidad), tiempos de los combos y las dos habilidades nuevas. |
| **Los números de tu servidor** | Un único conjunto de valores. | Datos oficiales (RO LATAM, Episodio 19) o de rAthena, elegidos con `ServerType`, para que cooldowns, alcances y costos coincidan con tu servidor. |
| **Configuración** | Aplicación Windows. | La aplicación Windows y una **página en el navegador** en English, Português y Español, con descripciones de habilidades, presets por homúnculo y sugerencia de tácticas de monstruos por mapa. |
| **Actualizaciones** | Manuales. | La aplicación Windows actualiza la IA y los datos de monstruos por ti, sin tocar tus ajustes. |
| **Pruebas** | Sin suite de pruebas sin conexión. | Cada cambio se ejecuta contra un servidor simulado (decenas de escenarios) antes de publicarse. |

Las funciones que reconocen monstruos son para **homúnculos**. Los mercenarios
reciben las herramientas y las correcciones de errores, pero no la base de datos
de monstruos.

## ¿Vale la pena cambiar?

**Te conviene si** juegas con Eira, Bayeri, Sera, Dieter o Eleanor, quieres
ajustar el comportamiento sin leer código, o cazas grupos de monstruos y quieres
que las habilidades de área se usen bien.

**Quizá no lo necesites si** tu IA actual ya hace lo que quieres, o si tu
servidor tiene reglas de habilidades distintas y prefieres no probar.

**Probarla es seguro:** haz un respaldo de la carpeta `AI` antes. Volver atrás
es restaurar ese respaldo. Tus tácticas y tu lista de amigos se pueden conservar
(consulta el [checklist de actualización](docs/MANUAL.es.md#actualizaciones-y-respaldos)).

## Qué cambia por defecto

Esto viene activado desde el inicio; cada punto se puede desactivar en la
herramienta de configuración.

- **Base de datos de monstruos** (`UseMobInfo`): se tienen en cuenta el elemento, las plantas y los jefes.
- **MVP** (`BossAggro`): el homúnculo no empieza peleas con MVP.
- **Peleas ordenadas** (`CommandedFightStay`): un monstruo al que le ordenas atacar se combate hasta que muera.

Extras opcionales, desactivados por defecto: elección de objetivo más
inteligente (`TargetPriority`), mantener distancia según el alcance real del
monstruo (`KiteByRange`) y escapar cuando el HP baja rápido (`UseDamageDanger`).
Las pruebas sin conexión los cubren, pero aún no se han verificado en el juego.

## Estado real

HG se prueba sin conexión y en un servidor rAthena local. Eso no garantiza que
funcione en todo cliente o servidor oficial. Las habilidades nuevas solo
funcionan si el servidor las admite y el homúnculo las ha aprendido. Activar una
habilidad no garantiza que se use en cada ataque: el alcance, SP, cooldown y
elemento también cuentan. Los cooldowns del servidor, costos de SP e
interrupciones de lanzamiento siguen vigentes.

## Instalación rápida

1. **Respalda toda la carpeta `AI` del juego**, especialmente si ya usas otra IA.
2. Descarga un paquete desde [Releases](https://github.com/jknpj/AzzyAI/releases),
   cuando esté disponible, o el código de la rama **LATAM-HG**.
3. Copia el **contenido de [USER_AI de este proyecto](USER_AI)** en
   `AI\USER_AI` dentro del juego. No crees `AI\USER_AI\USER_AI`.
4. En el juego, usa **`/hoai`** hasta que el mensaje confirme la IA personalizada
   del homúnculo. Para mercenarios, usa **`/merai`**.
5. Invoca nuevamente al homúnculo o reconéctate para cargar los archivos.
6. Configura el servidor y las habilidades; prueba con un monstruo fácil.

**¿Estás actualizando una instalación?** Conserva ajustes, tácticas y lista de
amigos. Consulta el [checklist de actualización](docs/MANUAL.es.md#actualizaciones-y-respaldos).

## Elige tu herramienta

| Herramienta | Cómo abrirla |
|---|---|
| GUI Windows | Ejecuta [AzzyAIConfig.exe](USER_AI/AzzyAIConfig.exe) **desde `AI\USER_AI` de tu juego**. |
| Web GUI | Abre [AzzyAI Web Configuration](https://jknpj.github.io/AzzyAI-webgui/), selecciona **AzzyAI HG** y abre `USER_AI` del juego. |

Usa **Apply** para guardar. Si el navegador no puede escribir en la carpeta,
exporta/descarga la configuración y cópiala al juego. Después recarga la IA.
La web GUI ofrece inglés, portugués brasileño y español; los nombres de las
opciones siguen en inglés. La GUI Windows usa etiquetas en inglés; el manual
traducido explica los ajustes principales.

**Comprueba qué carpeta estás editando:** el juego no lee una copia aislada en
descargas o en el código fuente.

## Para empezar

- En `ServerType`, elige **Official** (oficial) o **Private** (privado).
- Activa solo las habilidades deseadas, en los niveles ya aprendidos.
- Revisa ataque y uso de área antes de cambiar opciones avanzadas.
- Cambia una cosa por vez y recarga la IA entre pruebas.

## Guías y ayuda

| Idioma | Manual del usuario |
|---|---|
| Español | [Instalación, ajustes, homúnculos y problemas frecuentes](docs/MANUAL.es.md) |
| English | [Installation, settings, homunculi and troubleshooting](docs/MANUAL.md) |
| Português (BR) | [Instalação, configurações, homúnculos e problemas comuns](docs/MANUAL.pt-BR.md) |

Para reportar un problema, indica homúnculo, servidor/cliente, habilidades
activadas y lo que ocurrió. **Nunca compartas contraseñas, credenciales ni
archivos privados.** Usa [Issues](https://github.com/jknpj/AzzyAI/issues).

Para desarrolladores y usuarios avanzados:
[notas de versión y referencia técnica, en inglés](docs/RELEASE_NOTES.md).
La documentación antigua incluida es una referencia histórica, no el manual
actual de HG.

## Créditos

Basado en AzzyAI de Dr. Azzy / [SpenceKonde](https://github.com/SpenceKonde/AzzyAI),
con contribuciones de [samufacanha2](https://github.com/samufacanha2/AzzyAI),
[Kisaro](https://github.com/Kisaro/AzzyAI) y
[vorpal1337](https://github.com/vorpal1337). Sitio original: [drazzy.com](http://drazzy.com/ro/ai).
