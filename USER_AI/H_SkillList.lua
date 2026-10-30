-- H_SkillList.lua	v1.50.2
-- Converted to Lua by Dr. Azzy from
-- Mercenary skill list posted by Neo Saro -and-
-- Homun S skill list ganked from some pserver site.
-- All skill list and skill info values laboriously hand entered by Dr. Azzy
-- This may be distributed and used in AIs freely

MS_BASH			= 8201
MS_MAGNUM		= 8202
MS_BOWLINGBASH		= 8203
MS_PARRYING		= 8204
MS_REFLECTSHIELD	= 8205
MS_BERSERK		= 8206
MA_DOUBLE		= 8207
MA_SHOWER		= 8208
MA_SKIDTRAP		= 8209
MA_LANDMINE		= 8210
MA_SANDMAN		= 8211
MA_FREEZINGTRAP		= 8212
MA_REMOVETRAP		= 8213
MA_CHARGEARROW		= 8214
MA_SHARPSHOOTING	= 8215
ML_PIERCE		= 8216
ML_BRANDISH		= 8217
ML_SPIRALPIERCE		= 8218
ML_DEFENDER		= 8219
ML_AUTOGUARD		= 8220
ML_DEVOTION		= 8221
MER_MAGNIFICAT		= 8222
MER_QUICKEN		= 8223
MER_SIGHT		= 8224
MER_CRASH		= 8225
MER_REGAIN		= 8226
MER_TENDER		= 8227
MER_BENEDICTION		= 8228
MER_RECUPERATE		= 8229
MER_MENTALCURE		= 8230
MER_COMPRESS		= 8231
MER_PROVOKE		= 8232
MER_AUTOBERSERK		= 8233
MER_DECAGI		= 8234
MER_SCAPEGOAT		= 8235
MER_LEXDIVINA		= 8236
MER_ESTIMATION		= 8237

-- Homunculus Skills
HLIF_HEAL		= 8001
HLIF_AVOID		= 8002
HLIF_CHANGE		= 8004
HAMI_CASTLE		= 8005
HAMI_DEFENCE		= 8006
HAMI_BLOODLUST		= 8008
HFLI_MOON		= 8009
HFLI_FLEET		= 8010
HFLI_SPEED		= 8011
HFLI_SBR44		= 8012
HVAN_CAPRICE		= 8013
HVAN_CHAOTIC		= 8014
HVAN_SELFDESTRUCT	= 8016

--Homun S Skills
MUTATION_BASEJOB 	= 8017
MH_SUMMON_LEGION 	= 8018
MH_NEEDLE_OF_PARALYZE 	= 8019
MH_POISON_MIST 		= 8020
MH_PAIN_KILLER 		= 8021
MH_LIGHT_OF_REGENE 	= 8022
MH_OVERED_BOOST		= 8023
MH_ERASER_CUTTER 	= 8024
MH_XENO_SLASHER 	= 8025
MH_SILENT_BREEZE 	= 8026
MH_STYLE_CHANGE 	= 8027
MH_SONIC_CRAW 		= 8028
MH_SONIC_CLAW 		= 8028
MH_SILVERVEIN_RUSH 	= 8029
MH_MIDNIGHT_FRENZY 	= 8030
MH_STAHL_HORN 		= 8031
MH_GOLDENE_FERSE 	= 8032
MH_STEINWAND		= 8033
MH_HEILIGE_STANGE 	= 8034
MH_ANGRIFFS_MODUS 	= 8035
MH_TINDER_BREAKER 	= 8036
MH_CBC 			= 8037
MH_EQC 			= 8038
MH_MAGMA_FLOW 		= 8039
MH_GRANITIC_ARMOR 	= 8040
MH_LAVA_SLIDE 		= 8041
MH_PYROCLASTIC 		= 8042
MH_VOLCANIC_ASH 	= 8043

--Homun S New Skills
MH_BLAST_FORGE = 8044
MH_TEMPERING = 8045
MH_TWISTER_CUTTER = 8047
MH_ABSOLUTE_ZEPHYR = 8048
MH_BLAZING_AND_FURIOUS = 8050
MH_THE_ONE_FIGHTER_RISES = 8051
MH_TOXIN_OF_MANDARA = 8053
MH_NEEDLE_STINGER = 8054
MH_GLANZEN_SPIES = 8056
MH_HEILIGE_PFERD = 8057
MH_GOLDENE_TONE = 8058

--SkillList[homuntype][skillid]=level
SkillList={}
SkillList[LIF]={}
SkillList[LIF][HLIF_HEAL]=5
SkillList[LIF][HLIF_AVOID]=5
SkillList[LIF][HLIF_CHANGE]=3
SkillList[AMISTR]={}
SkillList[AMISTR][HAMI_CASTLE]=5
SkillList[AMISTR][HAMI_DEFENCE]=5
SkillList[AMISTR][HAMI_BLOODLUST]=3
SkillList[FILIR]={}
SkillList[FILIR][HFLI_MOON]=5
SkillList[FILIR][HFLI_FLEET]=5
SkillList[FILIR][HFLI_SPEED]=5
SkillList[FILIR][HFLI_SBR44]=3
SkillList[VANILMIRTH]={}
SkillList[VANILMIRTH][HVAN_CAPRICE]=5
SkillList[VANILMIRTH][HVAN_CHAOTIC]=5
SkillList[VANILMIRTH][HVAN_SELFDESTRUCT]=3
SkillList[SERA]={}
SkillList[SERA][MH_SUMMON_LEGION]=5
SkillList[SERA][MH_NEEDLE_OF_PARALYZE]=10
SkillList[SERA][MH_POISON_MIST]=5
SkillList[SERA][MH_PAIN_KILLER]=10
SkillList[SERA][MH_TOXIN_OF_MANDARA]=10
SkillList[SERA][MH_NEEDLE_STINGER]=10
SkillList[EIRA]={}
SkillList[EIRA][MH_LIGHT_OF_REGENE]=5
SkillList[EIRA][MH_OVERED_BOOST]=5
SkillList[EIRA][MH_ERASER_CUTTER]=10
SkillList[EIRA][MH_XENO_SLASHER]=10
SkillList[EIRA][MH_SILENT_BREEZE]=5
SkillList[EIRA][MH_TWISTER_CUTTER]=10
SkillList[EIRA][MH_ABSOLUTE_ZEPHYR]=10
SkillList[ELEANOR]={}
SkillList[ELEANOR][MH_STYLE_CHANGE]=1
SkillList[ELEANOR][MH_SONIC_CRAW]=5
SkillList[ELEANOR][MH_SILVERVEIN_RUSH]=10
SkillList[ELEANOR][MH_MIDNIGHT_FRENZY]=10
SkillList[ELEANOR][MH_TINDER_BREAKER]=5
SkillList[ELEANOR][MH_CBC]=5
SkillList[ELEANOR][MH_EQC]=5
SkillList[ELEANOR][MH_BLAZING_AND_FURIOUS]=10
SkillList[ELEANOR][MH_THE_ONE_FIGHTER_RISES]=10
SkillList[BAYERI]={}
SkillList[BAYERI][MH_STAHL_HORN]=10
SkillList[BAYERI][MH_GOLDENE_FERSE]=5
SkillList[BAYERI][MH_STEINWAND]=5
SkillList[BAYERI][MH_HEILIGE_STANGE]=10
SkillList[BAYERI][MH_ANGRIFFS_MODUS]=5
SkillList[BAYERI][MH_GLANZEN_SPIES]=10
SkillList[BAYERI][MH_HEILIGE_PFERD]=10
SkillList[BAYERI][MH_GOLDENE_TONE]=10
SkillList[DIETER]={}
SkillList[DIETER][MH_MAGMA_FLOW]=5
SkillList[DIETER][MH_GRANITIC_ARMOR]=5
SkillList[DIETER][MH_LAVA_SLIDE]=10
SkillList[DIETER][MH_PYROCLASTIC]=10
SkillList[DIETER][MH_VOLCANIC_ASH]=5
SkillList[DIETER][MH_TEMPERING]=10
SkillList[DIETER][MH_BLAST_FORGE]=10

--SkillListM[mertype][skillid]=level




--SkillInfo[id]={name,range,SP cost,Cast Time (fixed),Cast Time (variable),Delay,targetmode,Duration,reuse delay)
--SkillInfo[id][1]=name
--SkillInfo[id][2]=range
--SkillInfo[id][3]=sp cost
--SkillInfo[id][4]=Fixed cast (ms)
--SkillInfo[id][5]=Variable cast (ms)
--SkillInfo[id][6]=Delay (ms)
--SkillInfo[id][7]=target mode (0 = self targeted, 1 = enemy targeted, 2 = ground targeted) 
--SkillInfo[id][8]=Duration (ms)
--SkillInfo[id][9]=Reuse Delay (ms)

-- Official profile target: LATAM Episode 19 Isgard (4th class balance patch, 2026-10-27/29), which
-- already includes kRO's 2025-06-18 homunculus update and Homunculus S Lv250 (2026-05-28).
-- kRO release notes: ro.gnjoy.com/news/notice/View.asp?seq=7186 (2019-11-20),
-- seq=7288 (2020-08-19), seq=7374 (2021-03-03). LATAM dates: https://browiki.org/wiki/Epis%C3%B3dios.
-- Primary source: the current kRO client skill descriptions (skilldescript.lub, English translation at
-- github.com/llchrisll/ROenglishRE), checked on 2026-10-29. They state durations, AoE sizes, some
-- cooldowns and Healing Hands' SP; the latest kRO data wins wherever it gives a number.
-- kRO's client text omits range, cast times, delays and most SP costs, so those come from the
-- bROWiki pages (browiki.org/wiki/<PT name>). Where the wiki's formula and level table disagree, the
-- value also used by rAthena was kept. Per-level cast times of Needle of Paralyze and Lava Slide are
-- interpolated from the wiki's lv 1/4/7/10 values. Private overrides use rAthena Renewal.
-- Poison Mist (3-15s) and Volcanic Ash (8-40s) durations confirmed by kRO. Still unconfirmed:
-- Xeno Slasher / CBC / Pyroclastic SP; EQC stays self-targeted (kRO text silent, rAthena TargetType Self).
SkillInfo={}
SkillInfo[0]={"No Skill",{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0}}
-- Homunculus Skills
-- Durations/cooldowns checked against kRO and bROWiki on 2026-10-29; except Flitting / Accelerated
-- Flight cooldowns (see ServerSkillOverrides), they also match rAthena Renewal.
SkillInfo[HLIF_HEAL]={"Healing Hands",{0,0,0,0,0},{13,16,19,22,25},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{0,0,0,0,0},{20000,20000,20000,20000,20000}}
SkillInfo[HLIF_AVOID]={"Urgent Escape",{0,0,0,0,0},{20,25,30,35,40},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{40000,35000,30000,25000,20000},{35000,35000,35000,35000,35000}}
SkillInfo[HLIF_CHANGE]={"Mental Charge",{0,0,0},{100,100,100},{0,0,0},{0,0,0},{2000,2000,2000},0,{60000,180000,300000},{600000,900000,1200000}}
SkillInfo[HAMI_CASTLE]={"Castling",{0,0,0,0,0},{10,10,10,10,10},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{0,0,0,0,0},{1000,1000,1000,1000,1000}}
SkillInfo[HAMI_DEFENCE]={"Amistr Bulwark",{0,0,0,0,0},{20,25,30,35,40},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{40000,35000,30000,25000,20000},{30000,30000,30000,30000,30000}}
SkillInfo[HAMI_BLOODLUST]={"Bloodlust",{0,0,0},{120,120,120},{0,0,0},{0,0,0},{2000,2000,2000},0,{60000,180000,300000},{300000,600000,900000}}
SkillInfo[HFLI_MOON]={"Moonlight",{1,1,1,1,1},{4,8,12,16,20},{0,0,0,0,0},{0,0,0,0,0},{0,0,0,0,0},1,{0,0,0,0,0},{2000,2000,2000,2000,2000}}
SkillInfo[HFLI_FLEET]={"Flitting",{0,0,0,0,0},{30,40,50,60,70},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{60000,55000,50000,45000,40000},{60000,70000,80000,90000,120000}}
SkillInfo[HFLI_SPEED]={"Accellerated Flight",{0,0,0,0,0},{30,40,50,60,70},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{60000,55000,50000,45000,40000},{60000,70000,80000,90000,120000}}
SkillInfo[HFLI_SBR44]={"SBR 44",{9,9,9},{1,1,1},{0,0,0},{0,0,0},{0,0,0},1,{0,0,0},{1000,1000,1000}}
SkillInfo[HVAN_CAPRICE]={"Caprice",{9,9,9,9,9},{22,24,26,28,30},{0,0,0,0,0},{0,0,0,0,0},{0,0,0,0,0},1,{0,0,0,0,0},{2000,2200,2400,2600,2800}}
SkillInfo[HVAN_CHAOTIC]={"Chaotic Blessing",{0,0,0,0,0},{40,40,40,40,40},{0,0,0,0,0},{0,0,0,0,0},{0,0,0,0,0},0,{0,0,0,0,0},{3000,3000,3000,3000,3000}}
SkillInfo[HVAN_SELFDESTRUCT]={"Self Destruct",{0,0,0},{1,1,1},{0,0,0},{0,0,0},{0,0,0},0,{0,0,0},{1000,1000,1000}}
-- Sera Homunculus S Skills
SkillInfo[MH_SUMMON_LEGION] = {"Summon Legion",{9,9,9,9,9},{60,80,100,120,140},{400,600,800,1000,1200},{1600,1400,1200,1000,800},{0,0,0,0,0},1,{20000,30000,40000,50000,60000},{60000,60000,60000,60000,60000}}
SkillInfo[MH_NEEDLE_OF_PARALYZE] = {"Needle of Paralyze",{5,5,5,5,5,5,5,5,5,5},{42,48,54,60,66,72,78,84,90,96},{500,500,500,400,400,400,300,300,300,200},{1000,1000,1000,1100,1100,1100,1200,1200,1200,1300},{0,0,0,0,0,0,0,0,0,0},1,{3000,3500,4000,4500,5000,5500,6000,6500,7000,7500},{0,0,0,0,0,0,0,0,0,0}}
SkillInfo[MH_POISON_MIST] = {"Poison Mist",{5,5,5,5,5},{65,75,85,95,105},{500,500,500,500,500},{1000,1000,1000,1000,1000},{0,0,0,0,0},2,{3000,6000,9000,12000,15000},{10000,10000,10000,10000,10000}}
SkillInfo[MH_PAIN_KILLER] = {"Pain Killer",{5,5,5,5,5,5,5,5,5,5},{48,52,56,60,64,68,72,76,80,84},{1000,900,800,700,600,500,400,300,200,100},{1000,1100,1200,1300,1400,1500,1600,1700,1800,1900},{0,0,0,0,0,0,0,0,0,0},1,{330000,360000,390000,420000,450000,480000,510000,540000,570000,600000},{60000,60000,60000,60000,60000,60000,60000,60000,60000,60000}}
-- Eira Homunculus S Skills
SkillInfo[MH_LIGHT_OF_REGENE] = {"Light of Regenerate",{0,0,0,0,0},{40,50,60,70,80},{1600,1400,1200,1000,800},{0,0,0,0,0},{0,0,0,0,0},0,{360000,420000,480000,540000,600000},{60000,60000,60000,60000,60000}}
SkillInfo[MH_OVERED_BOOST] = {"Overed Boost",{0,0,0,0,0},{70,90,110,130,150},{200,300,400,500,600},{800,700,600,500,400},{0,0,0,0,0},0,{30000,45000,60000,75000,90000},{60000,60000,60000,60000,60000}}
SkillInfo[MH_ERASER_CUTTER] = {"Eraser Cutter",{7,7,7,7,7,7,7,7,7,7},{25,30,35,40,45,50,55,60,65,70},{0,0,0,0,0,0,0,0,0,0},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500},{0,0,0,0,0,0,0,0,0,0},1,{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0}}
SkillInfo[MH_XENO_SLASHER] = {"Xeno Slasher",{7,7,7,7,7,7,7,7,7,7},{85,90,95,100,105,110,115,120,125,130},{0,0,0,0,0,0,0,0,0,0},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500},{0,0,0,0,0,0,0,0,0,0},2,{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0}}
SkillInfo[MH_SILENT_BREEZE] = {"Silent Breeze",{5,5,7,7,9},{45,54,63,72,81},{1000,800,600,400,200},{1000,1200,1400,1600,1800},{0,0,0,0,0},1,{9000,12000,15000,18000,21000},{20000,20000,20000,20000,20000}}
-- Eleanor Homunculus S Skills
SkillInfo[MH_STYLE_CHANGE] = {"Style Change",{0},{35},{0},{0},{1000},0,{0},{0}}
SkillInfo[MH_SONIC_CRAW] = {"Sonic Claw",{3,3,3,3,3},{20,25,30,35,40},{0,0,0,0,0},{0,0,0,0,0},{0,0,0,0,0},1,{0,0,0,0,0},{500,500,500,500,500}}
SkillInfo[MH_SILVERVEIN_RUSH] = {"Silvervein Rush",{3,3,3,3,3,3,3,3,3,3},{17,19,21,23,25,27,29,31,33,35},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},1,{0,0,0,0,0,0,0,0,0,0},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500}}
SkillInfo[MH_MIDNIGHT_FRENZY] = {"Midnight Frenzy",{3,3,3,3,3,3,3,3,3,3},{18,21,24,27,30,33,36,39,42,45},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},1,{0,0,0,0,0,0,0,0,0,0},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500}}
SkillInfo[MH_TINDER_BREAKER] = {"Tinder Breaker",{3,4,5,6,7},{20,25,30,35,40},{0,0,0,0,0},{1000,1000,1000,1000,1000},{0,0,0,0,0},1,{0,0,0,0,0},{500,500,500,500,500}}
SkillInfo[MH_CBC] = {"Continuous Brandishing Combo",{1,1,1,1,1},{10,20,30,40,50},{0,0,0,0,0},{0,0,0,0,0},{0,0,0,0,0},1,{0,0,0,0,0},{300,300,300,300,300}}
SkillInfo[MH_EQC] = {"Eternal Quickening Combo",{1,1,1,1,1},{24,28,32,36,40},{0,0,0,0,0},{0,0,0,0,0},{1000,1000,1000,1000,1000},0,{0,0,0,0,0},{300,300,300,300,300}}
-- Bayeri Homunculus S Skills
SkillInfo[MH_STAHL_HORN] = {"Stahl Horn",{5,5,6,6,7,7,8,8,9,9},{43,46,49,52,55,58,61,64,67,70},{500,500,500,500,500,500,500,500,500,500},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},1,{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0}}
SkillInfo[MH_GOLDENE_FERSE] = {"Goldene Ferse",{0,0,0,0,0},{60,65,70,75,80},{0,0,0,0,0},{1000,1200,1400,1600,1800},{0,0,0,0,0},0,{30000,45000,60000,75000,90000},{30000,30000,30000,30000,30000}}
SkillInfo[MH_STEINWAND] = {"Stein Wand",{0,0,0,0,0},{80,90,100,110,120},{0,0,0,0,0},{1000,1000,1000,1000,1000},{0,0,0,0,0},0,{200000,300000,400000,500000,600000},{4000,8000,12000,16000,20000}}
SkillInfo[MH_HEILIGE_STANGE] = {"Heilige Stange",{9,9,9,9,9,9,9,9,9,9},{48,54,60,66,72,78,84,90,96,102},{2000,1800,1600,1400,1200,1000,800,600,400,200},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500},{0,0,0,0,0,0,0,0,0,0},1,{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0}}
SkillInfo[MH_ANGRIFFS_MODUS] = {"Angriffs Modus",{0,0,0,0,0},{60,65,70,75,80},{0,0,0,0,0},{200,400,600,800,1000},{0,0,0,0,0},0,{30000,45000,60000,75000,90000},{30000,30000,30000,30000,30000}}
-- Dieter Homunculus S Skills
SkillInfo[MH_MAGMA_FLOW] = {"Magma Flow",{0,0,0,0,0},{34,38,42,46,50},{2000,1500,1000,500,0},{2000,2500,3000,3500,4000},{1000,1000,1000,1000,1000},0,{30000,45000,60000,75000,90000},{0,0,0,0,0}}
SkillInfo[MH_GRANITIC_ARMOR] = {"Granitic Armor",{0,0,0,0,0},{54,58,62,66,70},{1000,1000,1000,1000,1000},{5000,4500,4000,3500,3000},{1000,1000,1000,1000,1000},0,{60000,60000,60000,60000,60000},{60000,60000,60000,60000,60000}}
SkillInfo[MH_LAVA_SLIDE] = {"Lava Slide",{7,7,7,7,7,7,7,7,7,7},{40,45,50,55,60,65,70,75,80,85},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},{5000,5000,5000,4500,4500,4500,4000,4000,4000,3500},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},2,{6000,7000,8000,9000,10000,11000,12000,13000,14000,15000},{3000,3000,3000,3000,3000,3000,3000,3000,3000,3000}}
SkillInfo[MH_PYROCLASTIC] = {"Pyroclastic",{0,0,0,0,0,0,0,0,0,0},{20,28,36,44,52,56,60,64,66,70},{200,200,200,200,200,200,200,200,200,200},{1000,1500,2000,2500,3000,3500,4000,4500,5000,5500},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},0,{330000,360000,390000,420000,450000,480000,510000,540000,570000,600000},{60000,60000,60000,60000,60000,60000,60000,60000,60000,60000}}
SkillInfo[MH_VOLCANIC_ASH] = {"Volcanic Ash",{7,7,7,7,7},{60,65,70,75,80},{1000,1000,1000,1000,1000},{4000,3500,3000,2500,2000},{1000,1000,1000,1000,1000},2,{8000,16000,24000,32000,40000},{10000,10000,10000,10000,10000}}
-- New Homunculus S Skills
-- Added with the kRO 2021-03-03 Lv250 update. Values differing in rAthena Renewal are
-- in ServerSkillOverrides below and replace these when ServerType = SERVER_PRIVATE.
-- Range of self-centered AoE skills (Toxin, TOFR, Heilige Pferd) is their splash radius, so the AI only
-- fires them when the target is actually inside the area.
SkillInfo[MH_TOXIN_OF_MANDARA]={"Toxin Of Mandara",{1,1,1,2,2,2,3,3,3,4},{60,65,70,75,80,85,90,95,100,105},{700,700,700,700,700,700,700,700,700,700},{0,0,0,0,0,0,0,0,0,0},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},0,{2500,3000,3500,4000,4500,5000,5500,6000,6500,7000},{2000,2000,2000,2000,2000,2000,2000,2000,2000,2000}};
SkillInfo[MH_NEEDLE_STINGER]={"Needle Stinger",{7,7,7,7,7,7,7,7,7,7},{74,82,90,98,106,114,122,130,138,146},{300,300,300,300,300,300,300,300,300,300},{0,0,0,0,0,0,0,0,0,0},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},1,{0,0,0,0,0,0,0,0,0,0},{250,250,250,250,250,250,250,250,250,250}};
SkillInfo[MH_TWISTER_CUTTER]={"Twister Cutter",{7,7,7,7,7,7,7,7,7,7},{106,112,118,124,130,136,142,148,154,160},{0,0,0,0,0,0,0,0,0,0},{1500,1500,1500,1500,1500,1500,1500,1500,1500,1500},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},1,{0,0,0,0,0,0,0,0,0,0},{200,200,200,200,200,200,200,200,200,200}};
SkillInfo[MH_ABSOLUTE_ZEPHYR]={"Absolute Zephyr",{7,7,7,7,7,7,7,7,7,7},{122,129,136,143,150,157,164,171,178,185},{300,300,300,300,300,300,300,300,300,300},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},1,{0,0,0,0,0,0,0,0,0,0},{300,300,300,300,300,300,300,300,300,300}};
SkillInfo[MH_BLAZING_AND_FURIOUS]={"Blazing And Furious",{7,7,7,7,7,7,7,7,7,7},{103,108,113,118,123,128,133,138,143,148},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},{500,500,500,500,500,500,500,500,500,500},1,{0,0,0,0,0,0,0,0,0,0},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000}};
SkillInfo[MH_THE_ONE_FIGHTER_RISES]={"The One Fighter Rises",{1,1,1,1,2,2,2,2,3,3},{100,106,112,118,124,130,136,142,148,154},{0,0,0,0,0,0,0,0,0,0},{0,0,0,0,0,0,0,0,0,0},{500,500,500,500,500,500,500,500,500,500},0,{0,0,0,0,0,0,0,0,0,0},{3800,3600,3400,3200,3000,2800,2600,2400,2200,2000}};
SkillInfo[MH_GLANZEN_SPIES]={"Glanzen Spies",{3,3,3,3,3,3,3,3,3,3},{60,65,70,75,80,85,90,95,100,105},{700,700,700,700,700,700,700,700,700,700},{0,0,0,0,0,0,0,0,0,0},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},1,{0,0,0,0,0,0,0,0,0,0},{250,250,250,250,250,250,250,250,250,250}};
SkillInfo[MH_HEILIGE_PFERD]={"Heilige Pferd",{1,1,1,1,2,2,2,2,3,3},{122,129,136,143,150,157,164,171,178,185},{300,300,300,300,300,300,300,300,300,300},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},0,{0,0,0,0,0,0,0,0,0,0},{500,500,500,500,500,500,500,500,500,500}}
SkillInfo[MH_GOLDENE_TONE]={"Goldene Tone",{9,9,9,9,9,9,9,9,9,9},{124,133,142,151,160,169,178,187,196,205},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},{2000,2000,2000,2000,2000,2000,2000,2000,2000,2000},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},0,{30000,40000,50000,60000,70000,80000,90000,100000,110000,120000},{120000,120000,120000,120000,120000,120000,120000,120000,120000,120000}}
SkillInfo[MH_TEMPERING] = {"Tempering",{0,0,0,0,0,0,0,0,0,0},{83,91,99,107,115,123,131,139,147,155},{2000,2000,2000,2000,2000,2000,2000,2000,2000,2000},{0,0,0,0,0,0,0,0,0,0},{500,500,500,500,500,500,500,500,500,500},0,{45000,60000,75000,90000,105000,120000,135000,150000,165000,180000},{120000,120000,120000,120000,120000,120000,120000,120000,120000,120000}}
SkillInfo[MH_BLAST_FORGE] = {"Blast Forge",{1,1,1,1,1,1,1,1,1,1},{52,59,66,73,80,87,94,101,108,115},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},{0,0,0,0,0,0,0,0,0,0},{1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},2,{5000,5000,5000,5000,5000,5000,5000,5000,5000,5000},{3000,3000,3000,3000,3000,3000,3000,3000,3000,3000}}

--SkillAOEInfo[skill]={AoE size (size on a side of the AoE - by level),Target Type (0=centered on enemy, 1=centered on caster, skill-id=special - handled in GetAOECount)), other fields for future use}
SkillAOEInfo={}
SkillAOEInfo[0]={{0,0,0,0,0},0}
SkillAOEInfo[HVAN_SELFDESTRUCT]={{9,9,9},1}
SkillAOEInfo[MH_POISON_MIST]={{7,7,7,7,7},0}
SkillAOEInfo[MH_XENO_SLASHER]={{3,3,3,5,5,5,7,7,7,9},0}
SkillAOEInfo[MH_HEILIGE_STANGE]={{3,3,3,3,5,5,5,5,7,7},0}
-- kRO 2020-08-19 limits Lava Slide to 7x7; bROWiki lists this size at every level (rAthena: Unit Layout 3, also 7x7).
SkillAOEInfo[MH_LAVA_SLIDE]={{7,7,7,7,7,7,7,7,7,7},0}
SkillAOEInfo[MH_VOLCANIC_ASH]={{3,3,3,3,3},0}
-- New Homunculus S Skills
SkillAOEInfo[MH_TOXIN_OF_MANDARA]={{3,3,3,5,5,5,7,7,7,9},1}
SkillAOEInfo[MH_ABSOLUTE_ZEPHYR]={{3,3,3,5,5,5,7,7,7,9},0}
SkillAOEInfo[MH_BLAZING_AND_FURIOUS]={{3,3,3,3,5,5,5,5,7,7},0}
SkillAOEInfo[MH_THE_ONE_FIGHTER_RISES]={{3,3,3,3,5,5,5,5,7,7},1}
SkillAOEInfo[MH_HEILIGE_PFERD]={{3,3,3,3,5,5,5,5,7,7},1}
SkillAOEInfo[MH_BLAST_FORGE]={{3,3,3,3,5,5,5,5,7,7},0}

-- Spirit spheres Eleanor's skills need (rAthena SpiritSphereCost, checked in skill_isNotOk_hom);
-- the server refuses the cast without them. Blazing and Furious also uses up every sphere.
SkillSphereCost={}
SkillSphereCost[MH_SONIC_CRAW]=1
SkillSphereCost[MH_SILVERVEIN_RUSH]=1
SkillSphereCost[MH_MIDNIGHT_FRENZY]=1
SkillSphereCost[MH_TINDER_BREAKER]=1
SkillSphereCost[MH_CBC]=1
SkillSphereCost[MH_EQC]=2
SkillSphereCost[MH_BLAZING_AND_FURIOUS]=1

-- rAthena Renewal (db/re/skill_db.yml) values that differ from official LATAM, applied by ApplyServerType()
-- when ServerType = SERVER_PRIVATE. info = {[SkillInfo index]=value}, aoe = replacement SkillAOEInfo entry.
-- Audited against rAthena master on 2026-10-29: every homunculus skill (8001-8059) compared field by field.
-- Field mapping: Range=[2], SpCost=[3], FixedCastTime=[4], CastTime=[5], AfterCastActDelay=[6],
-- Duration1=[8], Cooldown=[9], SplashArea/Unit Layout N = AoE side 2N+1.
-- Deliberately not copied:
--  * Missing rAthena cooldowns: [9] also keeps the AI from recasting Summon Legion, Poison Mist and
--    Lava Slide while they are still active, and is unused by buffs, so the LATAM values are kept.
--  * Missing AfterCastActDelay / AfterCastWalkDelay: a longer [6] only makes the AI wait a bit more.
--  * HFLI_MOON / HFLI_SBR44 Range 15: keeping LATAM's 1 / 9 avoids changing Filir's positioning.
ServerSkillOverrides={}
ServerSkillOverrides[SERVER_PRIVATE]={
	-- rAthena: older Flitting / Accelerated Flight cooldowns (kRO 60/70/80/90/120s)
	[HFLI_FLEET]={info={[9]={60000,75000,90000,105000,120000}}},
	[HFLI_SPEED]={info={[9]={60000,75000,90000,105000,120000}}},
	-- rAthena: Bio Explosion 11x11 splash (LATAM 9x9)
	[HVAN_SELFDESTRUCT]={aoe={{11,11,11},1}},
	-- rAthena: 30-90s Safety Wall (kRO 200-600s buff)
	[MH_STEINWAND]={info={[8]={30000,45000,60000,75000,90000}}},
	-- rAthena has no fixed cast on these; its single CastTime is variable
	[MH_NEEDLE_OF_PARALYZE]={info={[4]={0,0,0,0,0,0,0,0,0,0},[5]={1000,1000,1000,1000,1000,1000,1000,1000,1000,1000},[8]={12000,12000,12000,14000,14000,14000,16000,16000,16000,18000}}},
	[MH_POISON_MIST]={info={[4]={0,0,0,0,0}}},
	[MH_PAIN_KILLER]={info={[4]={0,0,0,0,0,0,0,0,0,0},[5]={2500,2500,2500,2500,2500,2500,2500,2500,2500,2500}}},
	[MH_STAHL_HORN]={info={[4]={0,0,0,0,0,0,0,0,0,0},[5]={500,500,500,500,500,500,500,500,500,500}}},
	[MH_HEILIGE_STANGE]={info={[4]={0,0,0,0,0,0,0,0,0,0}}},
	[MH_LAVA_SLIDE]={info={[4]={0,0,0,0,0,0,0,0,0,0},[5]={2000,2000,2000,2000,2000,2000,2000,2000,2000,2000}}},
	[MH_PYROCLASTIC]={info={[4]={0,0,0,0,0,0,0,0,0,0},[5]={2500,2500,2500,2500,2500,2500,2500,2500,2500,2500}}},
	-- rAthena: 1s cooldown (LATAM 0.5s)
	[MH_SONIC_CRAW]={info={[9]={1000,1000,1000,1000,1000}}},
	-- rAthena: 12-20s ash duration (AzzyAI's LATAM entry uses the 8-40s effect duration)
	[MH_VOLCANIC_ASH]={info={[8]={12000,14000,16000,18000,20000}}},
	-- rAthena: enemy-targeted, range 2, splash centered on the target
	[MH_TOXIN_OF_MANDARA]={info={[2]={2,2,2,2,2,2,2,2,2,2},[7]=1},aoe={{3,3,3,5,5,5,7,7,7,9},0}},
}
