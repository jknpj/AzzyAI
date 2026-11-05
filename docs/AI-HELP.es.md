# Pide a una IA que te ayude a ajustar AzzyAI

[English](AI-HELP.md) | [Português (BR)](AI-HELP.pt-BR.md) | [Español](AI-HELP.es.md)

[Volver al README](../README.es.md) | [Manual del usuario](MANUAL.es.md)

¿No sabes por qué una habilidad nunca se usa, o qué cambiar? Pídele a cualquier
chat de IA (ChatGPT, Copilot, Gemini, Claude...) que lea tus archivos y te
explique. No necesitas instalar nada: abre un chat, adjunta tus archivos y pega
la solicitud de abajo.

## 1. Adjunta estos archivos

De la carpeta `AI\USER_AI` del juego, o de la carpeta del juego para los logs:

- `H_Config.lua`, los ajustes de tu homúnculo. Si usas un preset por tipo, también `Types\<Nombre>.lua`.
- `AAIStartH.txt`, el informe de inicio. Muestra el número de versión y si se cargaron los datos de monstruos.
- `AAI_ERROR.log`, si existe.

**No adjuntes** `A_Friends.lua`, la carpeta `data` ni nada con contraseñas o
datos de la cuenta. Si tienes dudas, abre antes los archivos en un editor de texto.

## 2. Pega esta solicitud

Completa las tres últimas líneas:

```text
Me estás ayudando a configurar AzzyAI, una IA de homúnculo para Ragnarok Online
(una continuación comunitaria del AzzyAI original).
Lee primero los archivos adjuntos y este manual:
https://github.com/jknpj/AzzyAI/blob/LATAM-HG/docs/MANUAL.es.md
(si no puedes abrir el enlace, avísame y pego el fragmento relevante).

Reglas para tu respuesta:
1. Usa palabras simples. No soy programador.
2. Para cada sugerencia indica: el nombre exacto de la opción en mi H_Config.lua,
   su valor actual, el valor que sugieres y el motivo, citando la línea o la
   sección del manual que lo respalda.
3. Sugiere solo opciones que existan en mis archivos o en el manual. Nunca inventes una.
4. Di claramente qué LEÍSTE en mis archivos y qué estás SUPONIENDO.
5. Si no puedes saberlo, dilo y hazme UNA pregunta en lugar de suponer.
6. Sugiere un cambio por vez y recuérdame hacer un respaldo de la carpeta AI y
   recargar la IA en el juego antes de probar.

Mi homúnculo: <Eira / Bayeri / Sera / Dieter / Eleanor / otro>
Mi servidor: <oficial RO LATAM / privado>
Qué está mal o qué quiero: <por ejemplo: "Dieter nunca usa Lava Slide contra grupos">
```

## 3. Revisa la respuesta

La IA no ve tu juego ni ejecuta la IA, y la solicitud de arriba no se ha probado
con todos los chats. Trata su respuesta como una sugerencia: revisa la evidencia
que cite, cambia una cosa, prueba con un monstruo fácil y vuelve al respaldo si
empeora.
