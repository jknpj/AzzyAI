# Manual do usuário: AzzyAI

[English](MANUAL.md) | [Português (BR)](MANUAL.pt-BR.md) | [Español](MANUAL.es.md)

[Voltar ao README](../README.pt-BR.md) | [Referência técnica em inglês](TECHNICAL.md) | [Referência de comandos, opções e táticas](REFERENCE.pt-BR.md)

## 1. Antes de começar

Use a IA somente onde seu servidor permitir. O AzzyAI configura o comportamento:
não desbloqueia habilidades, remove cooldowns ou impede interrupções por dano.
Clientes e servidores podem se comportar de formas diferentes. Comece em
monstros fáceis.

Homúnculo e mercenário têm configurações separadas. Os nomes das opções ficam
em inglês para corresponder às ferramentas.

## 2. Instalar e ativar

1. Feche as janelas de configuração e faça backup de toda a pasta `AI` do jogo.
2. Baixe um pacote de release, quando disponível, ou a branch **prerenewal**.
3. Coloque o conteúdo de `USER_AI` do projeto em `AI\USER_AI` do jogo.
   O resultado deve conter `AI\USER_AI\AI.lua`, sem outra pasta aninhada.
4. Digite `/hoai` até selecionar a IA personalizada do homúnculo.
   Para mercenários, use `/merai`.
5. Invoque novamente ou reconecte. Repita a recarga após salvar configurações.

Instalar o AzzyAI substitui a IA daquela pasta. Para manter outra IA em um dos tipos
de unidade, preserve o arquivo de entrada correspondente: `AI.lua` para
homúnculos ou `AI_M.lua` para mercenários. Não misture versões dos arquivos
de suporte do AzzyAI.

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

O aplicativo desktop faz isso por você (**Update AI from GitHub...**, seção 3). Ele nunca
sobrescreve os arquivos da tabela acima (nem `Types\*.lua`, salvo se você marcar
a opção) e faz backup de tudo que substitui.

## 3. Configurar pelas GUIs

### Aplicativo Windows

Execute `AzzyAIConfig.exe` na pasta `AI\USER_AI` instalada. Ele edita os arquivos dessa
pasta, mesmo quando aberto por um atalho. Confira o caminho mostrado no rodapé antes de
salvar.

A janela de configuração mostra o conjunto de opções Pre-Renewal: quatro homúnculos e sem grupos de Homúnculo S.

O app ainda não tem assinatura de código, então o SmartScreen pode mostrar "O Windows
protegeu o computador" em um download novo (um antivírus também pode sinalizar um
executável pequeno e pouco baixado). O código está neste repositório e o CI gera esse
arquivo a partir dele; para continuar, clique em **Mais informações > Executar assim
mesmo**. A configuração pelo navegador não precisa de download.

- **Homunculus / Mercenary:** comportamento geral, habilidades e buffs.
- **Homunculus Tactics / Mercenary Tactics:** comportamento para cada monstro.
- **PVP Tactics / Merc PVP Tactics:** comportamento para jogadores.
- **Extra Options:** opções avançadas e Lua personalizado; evite mexer no código se não
  souber o que está fazendo.

A busca encontra nomes e descrições. Os filtros de tipo ajudam a encontrar opções;
**ocultar uma configuração não a desliga**. Use **Limpar filtros** para ver tudo. Clique
em **Aplicar configurações** para salvar; **Recarregar** descarta as edições pendentes.
A interface e as descrições das opções estão em inglês, português e espanhol; escolha o
idioma na barra superior.

**Update AI from GitHub...** (barra superior): baixa o zip oficial do branch `prerenewal` e
copia sobre a pasta de destino mostrada na janela (editável, então dá para atualizar
outra instalação). Suas configurações, tactics, extras e listas de avoid são mantidos;
com *Replace Types/ presets too* marcado, `Types\*.lua` também é substituído. Os arquivos
substituídos vão para `backup\update-<hora>\`. Um `AzzyAIConfig.exe` mais novo é salvo
como `.exe.new`, e **Restart now** fecha a janela, troca o executável e abre o novo
build. Recarregue a IA no jogo depois. Se não houver edições pendentes, a janela
recarrega os arquivos sozinha.

No startup a janela confere a tag do release mais recente e, se for mais nova que o
`CVersion` instalado, mostra uma barra de **Update**.

**Files** (barra superior): exporta todos os arquivos de configuração (incluindo edições
ainda não aplicadas), importa uma pasta ou exporta um diagnóstico. O diagnóstico tem o
mesmo conteúdo do navegador: o relatório de startup, as últimas linhas dos logs da IA, a
configuração, os arquivos de tactics e extras, `Types\*.lua` e os arquivos de `data`, com
os IDs de dono trocados por `<owner>`. **Help** mostra a versão e os links para a
documentação, a web GUI e o formulário de problema.

### Navegador (sem Windows)

Sem Windows, ou prefere o navegador? Abra a [web GUI](https://jknpj.github.io/AzzyAI-webgui/).
Escolha **AzzyAI** (não o Classic), o idioma e **File > Open USER_AI folder** para abrir a
pasta instalada no jogo. **Help > Export diagnostics for a bug report...** monta o mesmo pacote do
aplicativo Windows (um `.zip` com o relatório de inicialização, os logs, a configuração e os níveis de
skill aprendidos, IDs de dono trocados por `<owner>`) a partir da pasta do jogo que você escolher, e
em seguida abre o formulário de issue no GitHub.

- Chrome/Edge podem gravar diretamente quando o acesso à pasta é suportado e autorizado.
- Sem gravação direta, use o ZIP baixado/exportado e extraia os arquivos de
  configuração para `AI\USER_AI` do jogo.
- Sem abrir a pasta, a página não está lendo as configurações instaladas.
- Uma configuração lembrada pelo navegador pode não ser igual aos arquivos atuais.

Clique em **Apply**, confirme a gravação/exportação e recarregue a IA no jogo.
Alguns recursos do navegador precisam de conexão com a internet.

### Configurações por tipo de homúnculo

As duas GUIs têm o seletor **Configurações de** na aba Homúnculo. *Global* edita o `H_Config.lua`; escolher um tipo (Lif, Amistr, Filir, Vanilmirth) edita `Types/<Nome>.lua`, que guarda só os valores diferentes dos globais e é aplicado depois do `H_Config.lua` quando aquele homúnculo aparece. **Usar valores globais** remove as mudanças do tipo. Apagar o arquivo do tipo desativa o preset.

## 4. Comportamento básico

| Opção | Significado prático |
|---|---|
| `ServerType` | **Official** para oficiais; **Private** para rAthena/Hercules. |
| `UseAttackSkill` | Permite habilidades ofensivas automáticas. |
| `AggroHP` / `AggroSP` | Porcentagem de HP/SP necessária para procurar alvos. Aumentar pode reduzir a agressividade. |
| `SuperPassive` | Impede o combate normal; confira se ninguém está atacando. |
| `DoNotChase` | Impede perseguir alvos. Um alvo distante pode nunca entrar no alcance. |
| `KiteMonsters` | Permite manter distância quando as táticas também permitem. |
| `UseDamageDanger` | Desligado por padrão. Se o homúnculo perder `DamageDangerDrop`% (30) do HP máximo em 3 s com HP abaixo de `DamageDangerHP`% (60), ele kita mesmo com `KiteMonsters` desligado. Ele só enxerga o próprio HP, não os buffs dos inimigos. |
| `UseDanceAttack` | Ativa movimento durante ataques. Pode mudar o posicionamento; não remove cooldowns. |
| `CommandedFightStay` | Ligado por padrão. Um monstro que você mandou o homúnculo atacar é combatido até morrer ou até você dar outra ordem, mesmo que você se afaste. Use 0 para ele desistir e voltar quando o alvo sair dos limites de movimento ou o tempo limite de ataque passar. |
| `StationaryMoveBounds` / `MobileMoveBounds` | Até que distância de você pode ir para reagir (com você parado / em movimento). Ambos vêm em 14; diminua `MobileMoveBounds` se a unidade ficar para trás quando você corre. |
| `AutoSkillLimit` | Teto de habilidades ofensivas usadas contra um alvo (0 = sem limite). Vale em cima da tática de habilidade do monstro: o menor dos dois números vence. A contagem recomeça quando o alvo muda. |

A detecção automática ajuda em alguns clientes privados, mas escolha o tipo
de servidor explicitamente ao investigar problemas. A detecção do tipo original
e do nível das habilidades ajuda com as herdadas, mas não substitui aprendê-las.

## 5. Habilidades, área e SP

Ative a habilidade desejada e use um nível não superior ao aprendido.
Uma habilidade pode não ser escolhida por:

- Falta de SP ou reserva configurada.
- Cooldown, alcance ou elemento do alvo.
- Requisitos de área ou táticas do monstro.
- Outra habilidade disponível com pontuação maior (veja abaixo).
- Cast, combo ou pausa temporária ainda em andamento.

As habilidades de ataque são usadas na ordem configurada (a habilidade de ataque
normal, depois as prioridades por tipo e as táticas), não pontuadas por dano: o
modelo de dano Renewal que as comparava não existe no Pre-Renewal. Uma habilidade
ainda é pulada quando o elemento do alvo derrubaria o dano abaixo de
`SkillElementMinRate` (`MobInfo.lua`), quando o SP não basta ou quando a tática
não permite. `SkillElementMinRate` em 0 ignora a checagem de elemento.

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

## 6. Homúnculos nesta versão

Esta é a versão Pre-Renewal, então o homúnculo é um dos quatro originais:
**Lif, Amistr, Filir ou Vanilmirth**. Cada um compra as próprias habilidades com
os pontos de evolução que o jogo dá, e as opções seguem as habilidades que ele
aprendeu.

Não há habilidades além das que os quatro originais aprendem, então não há nada
extra para configurar; as opções que as controlariam não existem (nem nas GUIs). Veja [PRERENEWAL.md](PRERENEWAL.md)
para o que esta versão inclui.

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

### O que o banco de monstros muda (só homúnculo)

Esta versão traz uma lista com cerca de 2.700 monstros, com elemento, raça e se cada um
é chefe, MVP ou planta. O homúnculo lê qual monstro está vendo e usa essa lista,
então age de forma sensata até em monstros para os quais você nunca escreveu uma
linha de tática. Mercenários não usam isso.

| Opção | Padrão | Em palavras simples |
|---|---|---|
| `UseMobInfo` | Ligado | Chave geral. Desligada = comportamento antigo, e nenhuma das opções abaixo faz efeito. |
| `SkillElementMinRate` | 50 | A habilidade é pulada quando o elemento do monstro reduziria o dano dela abaixo dessa porcentagem, e a IA ataca de outro jeito. Monstros imunes são sempre pulados. Diminua o número para usar habilidades com mais liberdade; 0 pula só os imunes. |
| `PlantSkills` | Desligado | Plantas e cogumelos recebem 1 de dano de tudo, então nenhuma habilidade é gasta neles. Ligue para usar habilidades neles mesmo assim. |
| `BossAggro` | Reagir a MVPs | O homúnculo nunca *começa* uma luta com um MVP, mas ainda defende você e a si mesmo. `React to bosses` inclui os mini-chefes; `Tactics` deixa as suas táticas decidirem. |
| `AttackMVPs` | Desligado | Ataca sempre os MVPs. Vale mais que o `BossAggro`. |
| `AttackOnlyMVPs` | Desligado | `1` = os outros monstros só recebem reação, `2` = os outros monstros são ignorados. Útil para limpar uma instância. |
| `TargetPriority` | Desligado | Escolhe o próximo alvo por mais do que a distância. Entre monstros com a mesma prioridade de tática: primeiro os que atacam você ou um amigo, plantas por último, depois os monstros que suas habilidades conseguem ferir, depois o centro do maior grupo quando você tem habilidade em área e, por fim, o mais próximo. |
| `KiteByRange` | Desligado | Com kiting ligado, mantém distância pelo alcance real do monstro: um arqueiro faz o homúnculo recuar mais cedo e mais longe do que um monstro corpo a corpo. Não foge de um monstro cujo alcance chega ao `KiteBounds`, porque não haveria para onde ir. Só importa quando o kiting já funciona para você: `KiteMonsters` ligado, a tática de kite do monstro definida e `ForceKite` para homúnculos sem ataque à distância. |

A lista de monstros vem do rAthena (Renewal), então alguns monstros podem ser
diferentes no seu servidor. `TargetPriority` e `KiteByRange` são novas e ainda
não foram verificadas no jogo; teste primeiro em monstros fáceis.

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

`AAIStartH.txt`, na pasta do cliente, registra o carregamento da IA do homúnculo.
`AAI_ERROR.log` pode mostrar erros e pausas temporárias de skills ofensivas.
Se o arquivo inicial não atualizar, confira permissões e ativação da IA.
Passos detalhados de diagnóstico estão nas
[notas técnicas](TECHNICAL.md#troubleshooting-automatic-attacks).

Ao relatar, inclua servidor/cliente, homúnculo/tipo original, skills ativadas,
passos e trechos relevantes dos logs. Remova informações de conta, senhas e
dados privados. Nunca envie arquivos de credenciais.
Use [Issues](https://github.com/jknpj/AzzyAI/issues).
