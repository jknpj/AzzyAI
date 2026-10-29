// Default AzzyAI config files embedded for the web GUI (copied from GUI/AzzyAIConfig).
window.AzzyTemplates = {
  'H_Config.lua': String.raw`AggroHP                  = 0
AggroSP                  = 0
OldHomunType             = 3
UseSkillOnly             = -1 
UseAttackSkill           = 1
OpportunisticTargeting   = 0
DoNotChase               = 0
UseDanceAttack           = 0
SuperPassive             = 0
UseIdleWalk              = 0
IdleWalkSP               = 80
UseCastleRoute           = 0
RelativeRoute            = 1
UseCastleDefend          = 0
CastleDefendThreshold    = 4
IdleWalkDistance         = 4
FleeHP                   = 0
RescueOwnerLowHP         = 0
LiveMobID                = 0

AttackSkillReserveSP     = 0 
AutoMobMode              = 2 
AutoMobCount             = 2
AutoComboMode            = 2
AutoComboSkill		=0
AutoComboSpheres         = 10
UseHomunSSkillChase      = 1
UseHomunSSkillAttack     = 1 
AutoSkillDelay           = 100 
AoEMaximizeTargets       = 0
CastTimeRatio		= .80
UseEiraSilentBreeze      = 0
EiraSilentBreezeLevel    = 5 
UseBayeriStahlHorn       = 1
BayeriStahlHornLevel     = 10
UseBayeriHailegeStar     = 1
BayeriHailegeStarLevel   = 10
UseSeraParalyze          = 0
SeraParalyzeLevel        = 10
UseSeraPoisonMist        = 0 
SeraPoisonMistLevel      = 5
UseEleanorSonicClaw      = 1
EleanorSonicClawLevel    = 5
EleanorSilverveinLevel   = 10
EleanorMidnightLevel     = 10
UseDieterLavaSlide       = 1 
UseDieterVolcanicAsh     = 1
DieterLavaSlideLevel     = 10
UseDefensiveBuff         = 1
UseOffensiveBuff         = 1 
UseProvokeOwner          = 0
ProvokeOwnerMobbed       = 3
LifEscapeLevel           = 5
FilirFlitLevel           = 5
FilirAccelLevel          = 5
AmiBulwarkLevel          = 5
UseBayeriAngriffModus    = 0
UseBayeriGoldenPherze    = 1
UseDieterMagmaFlow       = 1
UseDieterGraniticArmor   = 0
UseDieterPyroclastic     = 1
DieterPyroclasticLevel   = 10
UseEiraOveredBoost       = 0
HealSelfHP               = 50 
HealOwnerHP              = 50 
UseAutoHeal              = 0 
FollowStayBack           = 2
StationaryAggroDist      = 12
MobileAggroDist          = 12
StationaryMoveBounds     = 14
MobileMoveBounds         = 9
DoNotUseRest             = 0
RestXOff                 = 2
RestYOff                 = 0	
MoveSticky               = 0
MoveStickyFight          = 0
KiteMonsters             = 0
KiteBounds               = 10 
KiteStep                 = 5
KiteParanoidStep         = 2
KiteThreshold            = 3
KiteParanoidThreshold    = 2
KiteParanoid             = 0 
DefendStandby            = 0 
StickyStandby            = 1 
SpawnDelay               = 1000
AutoSkillDelay           = 100
StandbyFriending         = 1  
MirAIFriending           = 1  
UseAvoid                 = 0  
TankMonsterLimit         = 4
LavaSlideMode            = 1
PoisonMistMode           = 1
AoEReserveSP             = 0

ChaseSPPause             = 0
ChaseSPPauseSP 			= -60
ChaseSPPauseTime         = 3000
AttackTimeLimit          = 10000

UseBayeriSteinWand       = 0
BayeriSteinWandLevel     = 5
UseSteinWandSelfMob      = 2
UseSteinWandOwnerMob     = 2
UseSteinWandTele         = 0 
SteinWandTelePause       = 3000
AssumeHomun              = 1
AttackLastFullSP         = 0
DanceMinSP               = 0
AutoSkillLimit           = 100
UseAutoPushback          = 0
AutoPushbackThreshold    = 2
AoEFixedLevel            = 0
UseSeraCallLegion        = 1
SeraCallLegionLevel      = 5
ForceKite                = 0
UseBerserkMobbed         = 0
UseBerserkSkill          = 0
UseBerserkAttack         = 0
Berserk_SkillAlways      = 0
Berserk_Dance            = 0
Berserk_IgnoreMinSP      = 0
Berserk_ComboAlways      = 0
PVPmode                  = 0
UseSeraPainkiller        = 1
DefensiveBuffOwnerMobbed = 0
PainkillerFriends        = 1
PainkillerFriendsSave    = 1
EleanorDoNotSwitchMode   = 0
LastSavedDate            = "15/07/2025 16:06:10"
UseEiraXenoSlasher       = 1
EiraXenoSlasherLevel     = 10
UseEiraEraseCutter       = 0
EiraEraseCutterLevel     = 10
LagReduction             = 0
HealOwnerBreeze          = 0
SteinWandTelePause       = 0
DoNotAttackMoving        = 0
UseSmartBulwark          = 0
`,
  'M_Config.lua': String.raw`
AggroHP                  = 60
AggroSP                  = 0
OldHomunType			= 3
UseSkillOnly			=-1 
UseAttackSkill			=1
OpportunisticTargeting  =0
DoNotChase				=0
UseDanceAttack			=0
SuperPassive			=0
PVPMode					=0
AttackLastHPSP			=80
UseIdleWalk				=1
IdleWalkDistance		=3
RelativeRoute			=0
AutoDetectPlants		=1
FleeHP					=0

-- Attack skill options
AttackSkillReserveSP  = 0 
AutoMobMode			=1 
AutoMobCount		=5
AutoComboMode		=1
AutoComboSkill		=0
AutoComboSpheres	=10
UseAutoPushback		=0
AutoPushbackThreshold	=2
UseHomunSSkillChase = 1
UseHomunSSkillAttack = 1 
AutoSkillDelay           = 400 
AoEMaximizeTargets			=  0
CastTimeRatio		= .80


--Autobuff options

UseDefensiveBuff		=1
UseOffensiveBuff		=1 
UseProvokeOwner     =1
UseProvokeSelf		=0
UseSacrificeOwner   =0
UseAutoMag			=0
UseAutoSight        =1


FollowStayBack 		=2
StationaryAggroDist 	=12
MobileAggroDist			=7
StationaryMoveBounds    =14
MobileMoveBounds		=9
DoNotUseRest =0
RestXOff		=0
RestYOff		=2	

MoveSticky               = 0
MoveStickyFight          = 0

--Kiting options:
KiteMonsters 				=0
KiteBounds               = 10 
KiteStep                 = 5
KiteParanoidStep         = 2
KiteThreshold            = 3
KiteParanoidThreshold    = 2
KiteParanoid             = 0 

--Standby Behavior:
DefendStandby            = 0 
StickyStandby            = 1 


--Advanced Timing settings 
SpawnDelay               = 1000
AutoSkillDelay		= 400
AttackTimeLimit		= 10000
--Berzerk Settings:

UseBerzerkMobbed         = 0 
UseBerzerkSkill          = 0 
UseBerzerkAttack         = 0 
Berzerk_SkillAlways      = 0 
Berzerk_Dance            = 0
Berzerk_IgnoreMinSP      = 0 


--Misc settings:
StandbyFriending         = 1  
MirAIFriending           = 1  
TankMonsterLimit	= 4
`,
  'H_Tactics.lua': String.raw`MyTact={}
MyTact[0]={TACT_ATTACK_M,SKILL_ALWAYS,KITE_NEVER,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Default
MyTact[13]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Treasure Box
MyTact[1005]={TACT_ATTACK_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0.5,CHASE_NORMAL} --Familiar
MyTact[1042]={TACT_ATTACK_L,-1,KITE_REACT,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_OLD,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Steel Chonchon
MyTact[1111]={TACT_SNIPE_M,SKILL_NEVER,KITE_ALWAYS,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_OLD,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Drainliar
MyTact[1153]={TACT_ATTACK_H,-3,KITE_ALWAYS,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_OLD,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Orc Zombie
MyTact[1152]={TACT_SNIPE_M,-3,KITE_ALWAYS,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_OLD,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_CLEVER} --Orc Skeleton
MyTact[1177]={TACT_SNIPE_H,-4,KITE_NEVER,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_OLD,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_CLEVER} --Zenorc
MyTact[1189]={TACT_REACT_H,SKILL_ALWAYS,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_S,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,2,CHASE_ALWAYS} --Orc Archer
MyTact[1078]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Red Plant
MyTact[1079]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Blue Plant
MyTact[1080]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Green Plant
MyTact[1081]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Yellow Plant
MyTact[1082]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --White Plant
MyTact[1083]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Shining Plant
MyTact[1084]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Red Mushroom
MyTact[1085]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0,CHASE_NORMAL} --Black Mushroom
MyTact[1589]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0.1,CHASE_NORMAL} --Mandragora Summon
MyTact[1579]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0.3,CHASE_NORMAL} --Hydra Summon
MyTact[1575]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0.6,CHASE_NORMAL} --Flora Summon
MyTact[1555]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Parasite Summon
MyTact[1590]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Geographer Summon
MyTact[2158]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,0.5,CHASE_NORMAL} --Sera Legion (Hornet)
MyTact[2159]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Sera Legion (Giant)
MyTact[2160]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,2,CHASE_NORMAL} --Sera Legion (Vespa)
MyTact[1105]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Pierre
MyTact[1160]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Andre
MyTact[1095]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Deniro
MyTact[1176]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Vitata
MyTact[1121]={TACT_REACT_L,SKILL_NEVER,KITE_NEVER,CAST_REACT,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Giearth


`,
  'M_Tactics.lua': String.raw`MyTact={}
MyTact[0]={TACT_ATTACK_M,SKILL_ALWAYS,KITE_NEVER,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_BOTH,RESCUE_RETAINER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Default
MyTact[10]={TACT_REACT_M,SKILL_ALWAYS,KITE_REACT,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Default Summon
MyTact[11]={TACT_REACT_M,3,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER,-1,SNIPE_OK,KS_NEVER,1,CHASE_NORMAL} --Autodetect Plant
`,
  'H_PVP_Tact.lua': String.raw`MyPVPTact={}
MyPVPTact[0]={TACT_REACT_L,SKILL_ALWAYS,KITE_NEVER,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[KOS]={TACT_ATTACK_H,SKILL_ALWAYS,KITE_ALWAYS,CAST_REACT,PUSH_SELF,1,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[ENEMY]={TACT_REACT_H,SKILL_ALWAYS,KITE_ALWAYS,CAST_REACT,PUSH_SELF,1,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[NEUTRAL]={TACT_REACT_M,SKILL_ALWAYS,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[FRIEND]={TACT_IGNORE,SKILL_ALWAYS,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[ALLY]={TACT_IGNORE,SKILL_NEVER,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[RETAINER]={TACT_IGNORE,SKILL_NEVER,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
`,
  'M_PVP_Tact.lua': String.raw`MyPVPTact={}
MyPVPTact[0]={TACT_REACT_L,SKILL_ALWAYS,KITE_NEVER,CAST_REACT,PUSH_SELF,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[KOS]={TACT_ATTACK_H,SKILL_ALWAYS,KITE_ALWAYS,CAST_REACT,PUSH_SELF,1,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[ENEMY]={TACT_REACT_H,SKILL_ALWAYS,KITE_ALWAYS,CAST_REACT,PUSH_SELF,1,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[NEUTRAL]={TACT_REACT_M,SKILL_ALWAYS,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[FRIEND]={TACT_IGNORE,SKILL_ALWAYS,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[ALLY]={TACT_IGNORE,SKILL_NEVER,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
MyPVPTact[RETAINER]={TACT_IGNORE,SKILL_NEVER,KITE_NEVER,CAST_PASSIVE,PUSH_NEVER,DEBUFF_NEVER,CLASS_BOTH,RESCUE_NEVER}
`,
  'H_Extra.lua': String.raw`-- HOMUN S STUFF THAT DOESN'T HAVE GUI OPTIONS YET:
-- This is the Extra Options file
-- NewAutoFriend = 0 -- uncomment this if you're not using AzzyAI for your homun.

FriendAttack={}			--Set these to 1 to have homun attack 
				--the target of a friend/owner when the friend is:
FriendAttack[MOTION_ATTACK]=1	--Attacking normally
FriendAttack[MOTION_ATTACK2]=1	--Attacking normally
FriendAttack[MOTION_SKILL]=1	--Uses a skill (which has the normal skill animation)
FriendAttack[MOTION_CASTING]=1	--Is casting a skill with a casting time
FriendAttack[MOTION_TOSS]=0	--Uses SpearBoom/AidPot/other "throwing" things
FriendAttack[MOTION_BIGTOSS]=1	--Uses Acid Bomb
FriendAttack[MOTION_FULLBLAST]=1	--Uses Full Blast


BasicDebuffs={}
BasicDebuffs[MER_CRASH]=	1
BasicDebuffs[MER_LEXDIVINA]=	1
BasicDebuffs[MER_DECAGI]=	1
BasicDebuffs[MA_SANDMAN]=	1
BasicDebuffs[MA_FREEZINGTRAP]=	1

--[[
Tactics switching:
You may desire to switch your tactics based on conditions. The can be accomplished by making a second tactics file, and including a require "./AI/USER_AI/(your tactics file name)", and uncommenting the lines below
Replace the "1==1" test with a test of your choice (you can test if a player of a specific ID is on screen with Players[player id]~=nil (this will be true if the player is on the screen)) and so on. 
This feature is not guaranteed and may not function as intended.
Dr. Azzy takes no responsibility for your lua programming; I barely take responsibility or my own. 
]]--
--function GetMyTact(e)
--	if (1==1) then 
--		return MyTact[e]
--	else if MyAltTact[e]~= nil then
--		return MyAltTact[e]
--	end
--end
`,
  'M_Extra.lua': String.raw`-- This is the Extra Options file
-- NewAutoFriend = 0 -- uncomment this if you're not using AzzyAI for your homun.

FriendAttack={}			--Set these to 1 to have homun attack 
				--the target of a friend/owner when the friend is:
FriendAttack[MOTION_ATTACK]=1	--Attacking normally
FriendAttack[MOTION_ATTACK2]=1	--Attacking normally
FriendAttack[MOTION_SKILL]=1	--Uses a skill (which has the normal skill animation)
FriendAttack[MOTION_CASTING]=1	--Is casting a skill with a casting time
FriendAttack[MOTION_TOSS]=0	--Uses SpearBoom/AidPot/other "throwing" things
FriendAttack[MOTION_BIGTOSS]=1	--Uses Acid Bomb
FriendAttack[MOTION_FULLBLAST]=1	--Uses Full Blast


BasicDebuffs={}
BasicDebuffs[MER_CRASH]=	1
BasicDebuffs[MER_LEXDIVINA]=	1
BasicDebuffs[MER_DECAGI]=	1
BasicDebuffs[MA_SANDMAN]=	1
BasicDebuffs[MA_FREEZINGTRAP]=	1
`,
};
