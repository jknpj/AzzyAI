# Ask an AI to help you tune AzzyAI

[English](AI-HELP.md) | [Português (BR)](AI-HELP.pt-BR.md) | [Español](AI-HELP.es.md)

[Back to README](../README.md) | [User manual](MANUAL.md)

Not sure why a skill is never used, or what to change? Ask any AI chat
(ChatGPT, Copilot, Gemini, Claude...) to read your files and explain. You need no
extra software: open a chat, attach your files and paste the request below.

## 1. Attach these files

From your game's `AI\USER_AI` folder, or from the game folder for the logs:

- `H_Config.lua`, your homunculus settings. For a type preset, also `Types\<Name>.lua`.
- `AAIStartH.txt`, the startup report. It shows the release number and whether the monster data loaded.
- `AAI_ERROR.log`, if it exists.

**Do not attach** `A_Friends.lua`, the `data` folder or anything with passwords or
account details. Open the files in a text editor first if you are unsure.

## 2. Paste this request

Fill in the last three lines:

```text
You are helping me configure AzzyAI, a Ragnarok Online homunculus AI (a community
continuation of the original AzzyAI).
Read the attached files and this manual first:
https://github.com/jknpj/AzzyAI/blob/main/docs/MANUAL.md
(if you cannot open the link, tell me and I will paste the relevant part).

Rules for your answer:
1. Use simple words. I am not a programmer.
2. For each suggestion give: the exact option name from my H_Config.lua, its
   current value, the value you suggest, and why, quoting the line or the manual
   section that supports it.
3. Only suggest options that exist in my files or in the manual. Never invent one.
4. Say clearly what you READ in my files and what you are GUESSING.
5. If you cannot tell, say so and ask me ONE question instead of guessing.
6. Suggest one change at a time, and remind me to back up the AI folder and reload
   the AI in game before testing.

My homunculus: <Eira / Bayeri / Sera / Dieter / Eleanor / other>
My server: <official RO LATAM / private>
What is wrong or what I want: <for example: "Dieter never uses Lava Slide on a pack">
```

## 3. Check the answer

The AI cannot see your game or run the AI, and the request above has not been
tried with every chat. Treat its answer as a suggestion: check the evidence it
quotes, change one thing, test on an easy monster, and go back to your backup if
it gets worse.
