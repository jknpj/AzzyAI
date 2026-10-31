# Manual do usuário: AzzyAI HG

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Voltar ao README](../README.pt-BR.md) | [Notas técnicas em inglês](RELEASE_NOTES.md)

## 1. Antes de começar

Use a IA somente onde seu servidor permitir. HG configura o comportamento:
não desbloqueia habilidades, remove cooldowns ou impede interrupções por dano.
Clientes e servidores podem se comportar de formas diferentes. Comece em
monstros fáceis.

Homúnculo e mercenário têm configurações separadas. Os nomes das opções ficam
em inglês para corresponder às ferramentas.

## 2. Instalar e ativar

1. Feche as janelas de configuração e faça backup de toda a pasta `AI` do jogo.
2. Baixe um pacote de release, quando disponível, ou a branch **LATAM-HG**.
3. Coloque o conteúdo de `USER_AI` do projeto em `AI\USER_AI` do jogo.
   O resultado deve conter `AI\USER_AI\AI.lua`, sem outra pasta aninhada.
4. Digite `/hoai` até selecionar a IA personalizada do homúnculo.
   Para mercenários, use `/merai`.
5. Invoque novamente ou reconecte. Repita a recarga após salvar configurações.

Instalar HG substitui a IA daquela pasta. Para manter outra IA em um dos tipos
de unidade, preserve o arquivo de entrada correspondente: `AI.lua` para
homúnculos ou `AI_M.lua` para mercenários. Não misture versões dos arquivos
de suporte da HG.

### Atualizações e backups

Antes de atualizar, guarde uma cópia de:

| Arquivos | Conteúdo |
|---|---|
| `H_Config.lua`, `M_Config.lua` | Configurações do homúnculo e mercenário |
| `H_Tactics.lua`, `M_Tactics.lua` | Comportamento por monstro |
| `H_PVP_Tact.lua`, `M_PVP_Tact.lua` | Comportamento em PVP |
| `H_Extra.lua`, `M_Extra.lua` | Opções extras e código personalizado |
| `A_Friends.lua` e `data` | Lista de amigos e informações salvas pela IA |

Guarde o backup completo fora da pasta de IA do jogo.
Não sobrescreva arquivos personalizados sem conferir nem substitua o código
novo pela pasta antiga inteira. Compare/importe as configurações na instalação
atualizada, salve, recarregue e teste.

## 3. Configurar pelas GUIs

### Aplicativo Windows

Execute `AzzyAIConfig.exe` na pasta `AI\USER_AI` instalada no jogo. Ele edita
os arquivos ao lado do executável, mesmo aberto por atalho. Confira o caminho
da configuração antes de salvar.

- **Homunculus / Mercenary:** comportamento geral, habilidades e buffs.
- **Homunculus Tactics / Mercenary Tactics:** comportamento por monstro.
- **PVP Tactics / Merc PVP Tactics:** comportamento para jogadores.
- **Extra Options:** opções avançadas e Lua personalizado; não edite código se não souber.

A busca encontra nomes e descrições. Os filtros por tipo facilitam encontrar
opções; **esconder uma configuração não a desativa**. Use **Clear filters** para
ver tudo. **Apply** salva; **Revert** descarta alterações pendentes.
Os rótulos do aplicativo Windows estão em inglês.

### Navegador

Abra a [web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Escolha **AzzyAI HG**, o idioma e **File > Open USER_AI folder** para abrir a
pasta instalada no jogo.

- Chrome/Edge podem gravar diretamente quando o acesso à pasta é suportado e autorizado.
- Sem gravação direta, use o ZIP baixado/exportado e extraia os arquivos de
  configuração para `AI\USER_AI` do jogo.
- Sem abrir a pasta, a página não está lendo as configurações instaladas.
- Uma configuração lembrada pelo navegador pode não ser igual aos arquivos atuais.

Clique em **Apply**, confirme a gravação/exportação e recarregue a IA no jogo.
Alguns recursos do navegador precisam de conexão com a internet.

## 4. Comportamento básico

| Opção | Significado prático |
|---|---|
| `ServerType` | **Official** para oficiais; **Private** para rAthena/Hercules. |
| `UseAttackSkill` | Permite habilidades ofensivas automáticas. |
| `UseHomunSSkillAttack` / `UseHomunSSkillChase` | Permite habilidades S ao atacar / perseguir. |
| `AggroHP` / `AggroSP` | Porcentagem de HP/SP necessária para procurar alvos. Aumentar pode reduzir a agressividade. |
| `SuperPassive` | Impede o combate normal; confira se ninguém está atacando. |
| `DoNotChase` | Impede perseguir alvos. Um alvo distante pode nunca entrar no alcance. |
| `KiteMonsters` | Permite manter distância quando as táticas também permitem. |
| `UseDanceAttack` | Ativa movimento durante ataques. Pode mudar o posicionamento; não remove cooldowns. |

A detecção automática ajuda em alguns clientes privados, mas escolha o tipo
de servidor explicitamente ao investigar problemas. A detecção do tipo original
e do nível das habilidades ajuda com as herdadas, mas não substitui aprendê-las.

## 5. Habilidades, área e SP

Ative a habilidade desejada e use um nível não superior ao aprendido.
Uma habilidade pode não ser escolhida por:

- Falta de SP ou reserva configurada.
- Cooldown, alcance ou elemento do alvo.
- Requisitos de área ou táticas do monstro.
- Outra habilidade disponível com prioridade maior.
- Cast, combo ou pausa temporária ainda em andamento.

`AutoMobMode` controla habilidades automáticas em área. `AutoMobCount` define
a quantidade necessária; **1 pode fazer habilidades de área competirem mesmo
contra um único alvo**. A eficácia do elemento também pode influenciar a
contagem. `AoEMaximizeTargets` ajuda a mirar a área; habilidades centradas no
homúnculo continuam exigindo monstros próximos dele.

`AttackSkillReserveSP` preserva SP em vez de gastar tudo no ataque.
`AutoSkillDelay` é uma espera extra da IA. **0 remove só essa espera**, não o
cooldown, cast ou interrupções do servidor. Menor nem sempre é melhor.
O preset incluído e os valores internos de reserva da IA podem diferir; vale o
valor do arquivo instalado.

## 6. Dicas por Homúnculo S

| Homúnculo | O que conferir |
|---|---|
| Eira | Ative os ataques desejados, como Twister Cutter ou Absolute Zephyr. Light of Regenerate é opcional e desligada por padrão: exige lealdade, reduz intimidade ao conjurar e sacrifica Eira para ressuscitar o dono. |
| Bayeri | **Glanzen Spies** usa um alvo; **Heilige Pferd** é centrada no Bayeri. Pferd está desligada no preset incluído: ative se quiser. Heilige Stange e outras áreas podem ter prioridade sobre Spies. |
| Sera | Toxin of Mandara e Needle Stinger têm casts que podem ser interrompidos. Mantenha o alvo no alcance e confira as pausas de falha abaixo. Pain Killer e Summon Legion têm opções próprias. |
| Dieter | Ative ataques e buffs separadamente. Resistência ou imunidade ao fogo pode impedir a escolha de habilidades de fogo. |
| Eleanor | Sonic Claw e a sequência de luta exigem estilo de luta; Tinder Breaker/CBC/EQC exigem agarrar. Esferas, SP e tempo do combo importam. |

São dicas de configuração, não um ranking ou uma build ideal. As regras do
servidor podem variar, inclusive alcance e forma de mirar habilidades.

### Eleanor: combos ou somente Sonic Claw

Para combos, escolha **Always** ou **Tactics** em `AutoComboMode` e confira
`AutoComboSpheres`: é o limite de esferas para **iniciar** a sequência automática,
não uma exigência repetida em cada etapa. Um limite alto faz esperar mais.
As novas habilidades ofensivas podem competir com o combo.

Para um experimento ofensivo só com Sonic:

1. Ative `UseEleanorSonicClaw`.
2. Coloque `AutoComboMode` em **Never** e desligue `Berserk_ComboAlways`.
3. Desligue Tinder Breaker, CBC, EQC, Blazing And Furious e The One Fighter Rises.
4. Coloque Eleanor em **estilo de luta** e recarregue a IA após salvar.

Ataques normais e buffs habilitados continuam ativos. O acompanhamento de
trocas manuais de estilo depende das informações fornecidas pelo cliente.
Se jogo e IA discordarem após uma troca manual de estilo, recarregue e teste
novamente; os ícones de esferas não são necessariamente uma contagem exata
em tempo real.

`SphereTrackInterval` é uma opção avançada de estimativa (padrão **800 ms**),
não um ajuste de velocidade ou cooldown. Reduza somente após confirmar
ataques normais mais rápidos; um valor baixo demais pode causar recusas.
Uma opção em `H_Extra.lua` prevalece sobre o valor salvo pela GUI.

## 7. Interrupções repetidas de cast

Encontre estas opções em **AutoSkill Options**:

| Opção | Padrão | O que faz |
|---|---|---|
| `CastInterruptLimit` | 3 | Ignora temporariamente uma habilidade após esse número de falhas contadas. **0 desativa**. |
| `CastInterruptPause` | 5000 | Duração em milissegundos: **5000 = 5 segundos**. |
| `CastInterruptUnconfirmed` | Desligada | Conta também falhas sem cast/conclusão visível, com alvo vivo no alcance e SP suficiente. |

Normalmente só contam casts ofensivos observados e interrompidos com indício
de dano. Um cast concluído zera o contador daquela habilidade. Na pausa, outra
habilidade disponível ou ataques normais podem ser escolhidos.
Buffs, combos e pedidos manuais de skill não acionam essa pausa.

Use `CastInterruptUnconfirmed` somente se o cliente esconder casts e as falhas
repetidas não forem contadas. **Ela pode contar recusas do servidor, não só
interrupções.** Não use para mascarar SP, alcance ou habilidades mal configuradas.
Contadores e pausas reiniciam quando o jogo recarrega a IA.
Essas opções não tornam casts ininterruptíveis nem alteram regras das skills.

## 8. Táticas de monstros

Nas abas de táticas, escolha quais monstros ignorar, priorizar ou atacar e se
deve usar habilidades. Confira a linha padrão e uma eventual linha específica
para seu alvo.

A web GUI pode **Suggest tactics** por mapa ou monstro. Revise antes de aplicar:
a base de monstros pode não corresponder exatamente ao seu servidor.

- `UseMobInfo`: usa as informações de monstros incluídas.
- `PlantSkills`: desligada por padrão para não desperdiçar skills em plantas.
- `BossAggro`: por padrão evita iniciar combate com MVPs, mas permite defesa.
- `TargetPriority`: ordenação inteligente opcional entre alvos.
- `KiteByRange`: distância baseada em alcance, opcional, com kiting habilitado.

Para um teste controlado em boneco, configure aquele monstro como **Ignore**
na escolha automática e mande o homúnculo atacar com **Alt + botão direito**.
Isso afeta todos os monstros com o mesmo ID de tática, não só um boneco.

## 9. Problemas comuns

| Sintoma | Confira primeiro |
|---|---|
| Não ataca nada | `/hoai`, pasta instalada, recarga, modo passivo, servidor, táticas e limites de HP/SP. |
| Ataque manual funciona, automático não | Agressividade, táticas do alvo, alcance e restrições de kill-steal. |
| Habilidade nova nunca aparece | Nível aprendido, ativação, SP, alcance, limite de área e habilidades concorrentes. |
| Cast começa sempre e falha | Interrupções por dano, alcance, SP e opções de pausa; nem toda falha é interrupção. |
| GUI não muda o jogo | Apply/exportação, pasta correta, recarga e opções em `H_Extra.lua`. |
| Eleanor usa a sequência errada | Estilo, combos e esferas; recarregue após mudanças. |

`AAIStartH.txt`, na pasta do cliente, registra o carregamento da IA do homúnculo.
`AAI_ERROR.log` pode mostrar erros e pausas temporárias de skills ofensivas.
Se o arquivo inicial não atualizar, confira permissões e ativação da IA.
Passos detalhados de diagnóstico estão nas
[notas técnicas](RELEASE_NOTES.md#troubleshooting-automatic-attacks).

Ao relatar, inclua servidor/cliente, homúnculo/tipo original, skills ativadas,
passos e trechos relevantes dos logs. Remova informações de conta, senhas e
dados privados. Nunca envie arquivos de credenciais.
Use [Issues](https://github.com/jknpj/AzzyAI/issues).
