# AzzyAI HG para Ragnarok Online

[English](README.md) | [Português (BR)](README.pt-BR.md) | [Español](README.es.md)

Configura cómo tu **homúnculo o mercenario** te sigue, ataca monstruos y usa
habilidades, sin tener que editar código.

HG es una versión comunitaria basada en [AzzyAI original](https://github.com/SpenceKonde/AzzyAI),
no un producto oficial de Gravity. Esta rama está en pruebas para RO LATAM y
servidores privados; el comportamiento y las habilidades disponibles dependen
del servidor. Respeta sus reglas sobre IA personalizada.

## ¿Qué ofrece?

- Configuración de habilidades y buffs de Eira, Bayeri, Sera, Dieter y Eleanor.
- Comportamientos diferentes para cada monstruo.
- Opciones para atacar, seguir, mantener distancia o usar habilidades de área.
- Alternativas temporales cuando un lanzamiento ofensivo se interrumpe repetidamente.
- Configuración con una **aplicación Windows** o un **navegador**.

Las habilidades nuevas solo funcionan si tu servidor las admite y tu homúnculo
las ha aprendido. Activar una habilidad no garantiza que se use en cada ataque:
el alcance, SP, cooldown, elemento y otros ajustes también cuentan.

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

Los cooldowns del servidor, costos de SP, interrupciones y requisitos siguen
vigentes. HG no elimina esas reglas.

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
